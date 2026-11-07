//! Per-window transactions run by the loop-owned host: registration, attach, show,
//! refresh, region application, rollback and the authorized destroy. Each method
//! runs one queued command against one identity's session.

use std::rc::Rc;

use gpui_util::ResultExt;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{RDW_INVALIDATE, RedrawWindow};
use windows::Win32::System::Ole::RevokeDragDrop;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyWindow, GetClientRect, IsWindowVisible, SW_HIDE, ShowWindow,
};

use gpui::Pixels;
use gpui::CaptionChromeState;
use crate::app_window_caption::support::window_caption_policy::{
    AttachGate, after_failure, attach_gate,
};
use crate::app_window_caption::support::window_lifetime::WindowLifetime;
use crate::window::WindowsWindowInner;

use super::super::{CaptionHost, log, log_failed, sdk_error, trace_enabled};
use super::attach::build_session;
use super::regions::{ApplyOutcome, RegionApplyFailure, RegionCommand, apply_session_regions};
use super::{AttachFailure, AttachProgress, RestoreOutcome, WindowSession, stage_log_name};

impl CaptionHost {
    // -- command execution ---------------------------------------------------

    /// Registers a window in the ledger. Every native creation passes here —
    /// attached or not, successful or failed, sealed or not — so the epilogue's
    /// retirement check covers every HWND the host saw, not only attached
    /// sessions. Registration happened before this command ran (WM_NCCREATE
    /// ordering), so live windows are always accounted for.
    pub(in super::super) fn register(&mut self, lifetime: &WindowLifetime, hwnd_value: isize) {
        self.ledger.register(lifetime.id(), hwnd_value);
        log(
            "register",
            &format!(
                "hwnd=0x{hwnd_value:X} identity={} (ledger entry)",
                lifetime.id()
            ),
        );
    }

    pub(in super::super) fn attach(
        &mut self,
        hwnd_value: isize,
        lifetime: &Rc<WindowLifetime>,
        inner: &WindowsWindowInner,
    ) {
        if self.sealed {
            log("attach", "sealed at terminal exit; attach refused");
            return;
        }
        if let Some(existing) = self.sessions.get(&hwnd_value) {
            if existing.lifetime.id() == lifetime.id() {
                return;
            }
            // A stale session from a previous identity is still parked at
            // this reused hwnd, awaiting its retired cleanup: never attach
            // against it — the raw HWND value alone is never a match.
            log(
                "attach",
                &format!(
                    "hwnd=0x{hwnd_value:X} still hosts identity {}'s session; attach skipped",
                    existing.lifetime.id()
                ),
            );
            return;
        }
        let identity = lifetime.id();
        if let Some(previous) = self.attempts.get(&identity) {
            log(
                "attach",
                &format!("skipped: identity {identity} already attempted ({previous})"),
            );
            return;
        }
        self.attempts.insert(identity, "in progress".to_string());
        log(
            "attach",
            &format!("hwnd=0x{hwnd_value:X} identity={identity}"),
        );
        // The attach rewrites the frame contract (content extension, height,
        // regions): every layout payload published before this point is stale.
        // From here the first current frame re-publishes, and until then the
        // suspension plan — never a guessed strip — is what the SDK sees.
        inner.invalidate_metric_revision();
        trace_window_frames(hwnd_value, "before attach");

        if attach_gate(self.runtime.is_some()) == AttachGate::FallBack {
            let message = "runtime unavailable (host creation failed earlier)";
            self.attempts.insert(identity, message.to_string());
            log_failed("attach", message);
            self.notify_failure(inner, lifetime);
            return;
        }
        // The pretranslation contract is required before anything is mutated.
        if let Err(message) = self.ensure_pretranslate() {
            self.attempts.insert(identity, message.clone());
            log_failed("attach", &message);
            self.notify_failure(inner, lifetime);
            return;
        }

        match build_session(hwnd_value, lifetime) {
            Ok(session) => {
                if !lifetime.is_alive() {
                    // The identity was retired while the session was being
                    // built (WM_NCDESTROY through SDK reentry): release the
                    // session without any native restore — a dead HWND is
                    // never restored — and publish nothing. The WM_NCDESTROY
                    // cleanup owns the ledger entry.
                    drop(session);
                    self.attempts
                        .insert(identity, "identity retired during attach".to_string());
                    log(
                        "attach",
                        &format!(
                            "hwnd=0x{hwnd_value:X} identity={identity} retired while \
                             attaching; session released without restore"
                        ),
                    );
                    return;
                }
                // The ledger entry already exists from this window's
                // registration at native creation; attach adds only the
                // session, keyed by the same token.
                self.sessions.insert(hwnd_value, session);
                self.attempts.remove(&identity);
                let state = self.snapshot_for(hwnd_value);
                inner.set_caption_state(state);
                notify_frame(hwnd_value);
                trace_window_frames(hwnd_value, "after attach");
                log(
                    "attach",
                    &format!(
                        "native AppWindow caption ACTIVE hwnd=0x{hwnd_value:X}; GPUI's \
                         non-client hit-test, NCCALCSIZE rewrite and NC mouse emulation are \
                         bypassed for this window while attached"
                    ),
                );
            }
            Err(AttachFailure::Dead) => {
                self.attempts
                    .insert(identity, "identity retired during attach".to_string());
                log(
                    "attach",
                    &format!(
                        "hwnd=0x{hwnd_value:X} identity={identity} retired during attach; \
                         nothing was restored or published"
                    ),
                );
            }
            Err(AttachFailure::Failed(message, mut progress)) => {
                let outcome = progress.restore(lifetime);
                let disposition = after_failure(outcome.rollback());
                match outcome {
                    RestoreOutcome::Restored => {
                        self.attempts.insert(identity, message.clone());
                        log_failed(
                            "attach",
                            &format!(
                                "{message}; native state restored, window stays on its app-owned header"
                            ),
                        );
                    }
                    RestoreOutcome::Retired => {
                        // The identity retired mid-rollback: the progress is
                        // released without further native restore (a dead
                        // HWND is never restored) and nothing is published.
                        self.attempts
                            .insert(identity, "identity retired during rollback".to_string());
                        log(
                            "attach",
                            &format!(
                                "hwnd=0x{hwnd_value:X} identity={identity} retired during \
                                 rollback; the rest is released without restore"
                            ),
                        );
                    }
                    RestoreOutcome::FailedHeld { failures } => {
                        // The window stayed live but the rollback is incomplete:
                        // the unfinished state is retained as the ONLY cleanup
                        // capability, nothing is published as restored, and no
                        // further SDK call is made for this identity.
                        for (stage, error) in &failures {
                            log_failed(
                                "restore",
                                &format!("{}: {}", stage_log_name(*stage), error),
                            );
                        }
                        self.failed_held.insert(identity, progress);
                        self.attempts.insert(
                            identity,
                            format!("{message} (rollback incomplete; state retained)"),
                        );
                        log_failed(
                            "attach",
                            &format!(
                                "{message}; hwnd=0x{hwnd_value:X} identity={identity} rollback \
                                 incomplete on a live window: state retained, window held on \
                                 its app-owned header"
                            ),
                        );
                    }
                }
                if disposition.publishes_app_owned() {
                    self.notify_failure(inner, lifetime);
                }
            }
        }
    }

    /// The prepared first show, run by the loop owner after an attach command —
    /// never inside a message dispatch and never under a GPUI borrow. A close that
    /// already owns the window's end consumes and discards the request inside
    /// [`WindowLifetime::take_deferred_show`], so an approved close or owner drop
    /// that beat the first host drive is never shown. Every other prepared window
    /// shows, including one whose rollback was held: the failure path has already
    /// published its app-owned header, so the user is never left with a hidden
    /// window. A failed `set_window_placement` is logged and reported; no further
    /// recovery is attempted.
    pub(in super::super) fn show_after_prepare(
        &self,
        lifetime: &Rc<WindowLifetime>,
        inner: &Rc<WindowsWindowInner>,
    ) {
        if !lifetime.take_deferred_show() {
            return;
        }
        if !lifetime.is_alive() {
            return;
        }
        if let Err(error) = inner.set_window_placement() {
            log_failed(
                "show-continuation",
                &format!(
                    "hwnd=0x{:X} identity={} first show failed: {error}; the window stays \
                     hidden and this is reported instead of silently hanging it",
                    inner.caption_hwnd_value(),
                    lifetime.id()
                ),
            );
        }
    }

    pub(in super::super) fn refresh(
        &mut self,
        hwnd_value: isize,
        lifetime: &Rc<WindowLifetime>,
        inner: &WindowsWindowInner,
    ) {
        let Some(session) = self.sessions.get_mut(&hwnd_value) else {
            return;
        };
        if session.lifetime.id() != lifetime.id() {
            // A stale session from a previous identity is still parked at
            // this reused hwnd, awaiting its retired cleanup: never refresh
            // it against the new window.
            return;
        }
        // Every SDK call below is a potentially reentrant boundary, so each
        // one is revalidated with a plain token read before the next. On
        // death nothing further is read and nothing is published — the
        // WM_NCDESTROY cleanup owns the retirement.
        if !lifetime.is_alive() {
            return;
        }
        // Re-read the SDK's actual geometry; physical measurements are never
        // cached across size/DPI/activation changes.
        let hwnd = HWND(hwnd_value as *mut _);
        let height = match session.title_bar.Height() {
            Ok(height) => height,
            Err(e) => {
                log_failed("refresh-height", &sdk_error(&e));
                return;
            }
        };
        if !lifetime.is_alive() {
            return;
        }
        let left_inset = match session.title_bar.LeftInset() {
            Ok(left_inset) => left_inset,
            Err(e) => {
                log_failed("refresh-insets", &sdk_error(&e));
                return;
            }
        };
        if !lifetime.is_alive() {
            return;
        }
        let right_inset = match session.title_bar.RightInset() {
            Ok(right_inset) => right_inset,
            Err(e) => {
                log_failed("refresh-insets", &sdk_error(&e));
                return;
            }
        };
        if !lifetime.is_alive() {
            return;
        }
        session.caption_height = height;
        session.left_inset = left_inset;
        session.right_inset = right_inset;
        // The metrics changed under this window: the previously applied region
        // sets were computed for the OLD extent, and no payload published
        // before the invalidation may apply. The stored payload is read (not
        // consumed) and applied when exactly current; a missing or stale one
        // means the suspension plan — no app caption, whole-client
        // passthrough — applies until the next current frame. Never a guessed
        // strip, and never the stale old regions left standing.
        let stored = inner.stored_layout();
        match apply_session_regions(
            hwnd_value,
            session,
            lifetime,
            inner,
            stored,
            RegionCommand::Refresh,
        ) {
            Ok(report) => {
                if !matches!(report.outcome, ApplyOutcome::Applied) {
                    log(
                        "refresh",
                        &format!(
                            "hwnd=0x{hwnd_value:X} identity={} regions unchanged \
                             (height={height} insets={left_inset}/{right_inset}); nothing re-sent",
                            lifetime.id()
                        ),
                    );
                }
                let dpi = unsafe { GetDpiForWindow(hwnd) };
                inner.set_caption_state(CaptionChromeState {
                    active: true,
                    caption_height: Some(logical_px(height, dpi)),
                    left_inset: Some(logical_px(left_inset, dpi)),
                    right_inset: Some(logical_px(right_inset, dpi)),
                });
                notify_frame(hwnd_value);
            }
            Err(RegionApplyFailure::Retired) => {
                // The identity died across an SDK boundary: publish nothing
                // for a dead HWND; the cleanup owns retirement.
            }
            Err(RegionApplyFailure::Failed(message)) => {
                // A failed region write leaves the native region state
                // UNKNOWN — never "previous regions intact". Roll the whole
                // session back to its captured originals; a clean rollback
                // detaches to the established default chrome, an incomplete
                // one is retained by the rollback helper's held policy.
                log_failed("refresh", &message);
                let session = self.sessions.remove(&hwnd_value);
                if let Some(session) = session {
                    self.rollback_live_session(hwnd_value, lifetime, inner, session, "refresh");
                }
            }
        }
    }

    /// Applies the latest completed-frame control layout for a live session.
    /// The payload must be exactly current — this session's identity, the
    /// current metric revision, the ACTUAL client extent and DPI scale, and
    /// no older frame than the one already applied — anything else is
    /// rejected as stale (the next refresh or frame will supply a current
    /// one). With nothing stored there is nothing new: the applied plan
    /// stands, whatever it is. An unchanged plan is deduplicated: no SDK
    /// call at all.
    pub(in super::super) fn apply_regions(
        &mut self,
        hwnd_value: isize,
        lifetime: &Rc<WindowLifetime>,
        inner: &WindowsWindowInner,
    ) {
        let Some(session) = self.sessions.get_mut(&hwnd_value) else {
            // No session (yet/anymore): the payload stays stored for the
            // attach or is dropped with the window.
            return;
        };
        if session.lifetime.id() != lifetime.id() || !lifetime.is_alive() {
            return;
        }
        let stored = inner.stored_layout();
        match apply_session_regions(
            hwnd_value,
            session,
            lifetime,
            inner,
            stored,
            RegionCommand::ApplyRegions,
        ) {
            Ok(report) => {
                let outcome = match report.outcome {
                    ApplyOutcome::Applied => format!(
                        "applied {} caption + {} passthrough rects",
                        report.caption_rects, report.passthrough_rects
                    ),
                    ApplyOutcome::Unchanged => {
                        "deduplicated (unchanged plan); nothing re-sent".to_string()
                    }
                    ApplyOutcome::RetainedCurrent => {
                        "no new layout payload stored; the applied regions stand".to_string()
                    }
                };
                log(
                    "apply-regions",
                    &format!(
                        "hwnd=0x{hwnd_value:X} identity={} {}",
                        lifetime.id(),
                        outcome
                    ),
                );
            }
            Err(RegionApplyFailure::Retired) => {}
            Err(RegionApplyFailure::Failed(message)) => {
                // Same contract as refresh: unknown region state rolls the
                // session back to its captured originals; a clean rollback
                // detaches, an incomplete one is held.
                log_failed("apply-regions", &message);
                let session = self.sessions.remove(&hwnd_value);
                if let Some(session) = session {
                    self.rollback_live_session(
                        hwnd_value,
                        lifetime,
                        inner,
                        session,
                        "apply-regions",
                    );
                }
            }
        }
    }

    /// Rolls a LIVE session back to its captured originals after a failed region
    /// update. The caller has already removed the session, so no further SDK call
    /// is made for it. A clean rollback detaches to the app-owned header. An
    /// incomplete one retains the unfinished state in the host and still publishes
    /// the app-owned header: the window is never reported as restored, and it is
    /// never driven natively again.
    fn rollback_live_session(
        &mut self,
        hwnd_value: isize,
        lifetime: &Rc<WindowLifetime>,
        inner: &WindowsWindowInner,
        session: WindowSession,
        context: &str,
    ) {
        let identity = lifetime.id();
        let mut progress = AttachProgress::from_session(session);
        let outcome = progress.restore(lifetime);
        let disposition = after_failure(outcome.rollback());
        match outcome {
            RestoreOutcome::Restored => {
                log(
                    context,
                    &format!(
                        "hwnd=0x{hwnd_value:X} identity={identity} rolled back to the \
                         captured originals; session detached"
                    ),
                );
            }
            RestoreOutcome::Retired => {
                log(
                    context,
                    &format!(
                        "hwnd=0x{hwnd_value:X} identity={identity} retired during rollback; \
                         the rest is released without restore and nothing is published"
                    ),
                );
            }
            RestoreOutcome::FailedHeld { failures } => {
                for (stage, error) in &failures {
                    log_failed("restore", &format!("{}: {}", stage_log_name(*stage), error));
                }
                self.failed_held.insert(identity, progress);
                log_failed(
                    context,
                    &format!(
                        "hwnd=0x{hwnd_value:X} identity={identity} rollback incomplete on a \
                         live window: state retained, window held on its app-owned header"
                    ),
                );
            }
        }
        if disposition.publishes_app_owned() {
            self.notify_failure(inner, lifetime);
        }
    }

    pub(in super::super) fn destroy(
        &mut self,
        hwnd_value: isize,
        lifetime: &Rc<WindowLifetime>,
        inner: &WindowsWindowInner,
    ) {
        if let Some(session) = self.sessions.remove(&hwnd_value) {
            if session.lifetime.id() != lifetime.id() {
                // Not this command's session: put it back and refuse to
                // touch either the session or the HWND.
                log_failed(
                    "destroy",
                    &format!(
                        "session for hwnd=0x{hwnd_value:X} belongs to identity {}, \
                         not {}; skipped",
                        session.lifetime.id(),
                        lifetime.id()
                    ),
                );
                self.sessions.insert(hwnd_value, session);
                return;
            }
            if !lifetime.is_alive() {
                // Retired between dispatch and here: release the session
                // without native restore — a dead HWND is never restored —
                // and never touch the HWND. The WM_NCDESTROY cleanup owns
                // the ledger entry.
                drop(session);
                return;
            }
            // The approved close must not let the restore repaint the
            // captured originals over the custom chrome on a visible
            // window — that repaint is the unstyled flash at close. The
            // final hide precedes the first restore stage, so the user's
            // last sight of the window is the styled presentation and the
            // originals are re-applied invisibly. Only an approved close
            // or owner drop reaches here (a canceled close or failed save
            // queues nothing), and the hide is skipped for a window that
            // is already invisible. ShowWindow dispatches messages
            // (WM_SHOWWINDOW, WM_ACTIVATE) — a reentry boundary like the
            // restore stages and RevokeDragDrop below — so the token is
            // revalidated before anything else touches the HWND.
            let hwnd = HWND(hwnd_value as *mut _);
            if unsafe { IsWindowVisible(hwnd) }.as_bool() {
                unsafe { ShowWindow(hwnd, SW_HIDE) }.ok().log_err();
                if !lifetime.is_alive() {
                    return;
                }
            }
            log("detach", &format!("hwnd=0x{hwnd_value:X}"));
            let mut progress = AttachProgress::from_session(session);
            match progress.restore(lifetime) {
                RestoreOutcome::Restored => {
                    drop(progress);
                    if !lifetime.is_alive() {
                        // The identity died across the restore stages or the
                        // interface drops (each Release is a reentry point):
                        // its WM_NCDESTROY cleanup owns the ledger entry, and
                        // the HWND is never touched again.
                        return;
                    }
                    inner.set_caption_state(CaptionChromeState::default());
                    notify_frame(hwnd_value);
                }
                RestoreOutcome::Retired => {
                    // The identity died across the restore stages or the
                    // interface drops: the retired cleanup releases the rest
                    // without native restore, and the HWND is never touched.
                    return;
                }
                RestoreOutcome::FailedHeld { failures } => {
                    // The window is live but the rollback is incomplete. The
                    // unfinished state is retained for this identity's cleanup
                    // and is never restored or reported as restored. The window
                    // is being closed on an approved request, so the destruction
                    // continuation below still runs; the retained state is
                    // released without live restoration at WM_NCDESTROY.
                    for (stage, error) in &failures {
                        log_failed("restore", &format!("{}: {}", stage_log_name(*stage), error));
                    }
                    self.failed_held.insert(lifetime.id(), progress);
                    log_failed(
                        "detach",
                        &format!(
                            "hwnd=0x{hwnd_value:X} identity={} rollback incomplete on a live \
                             window; the retained state is released at window destruction",
                            lifetime.id()
                        ),
                    );
                }
            }
        }
        if !lifetime.is_alive() {
            return;
        }
        // The existing destroy continuation, executed exactly once by the loop
        // owner outside any message dispatch: WM_DESTROY/WM_NCDESTROY arrive
        // synchronously from this call and enqueue the retiring cleanup.
        let hwnd = HWND(hwnd_value as *mut _);
        unsafe {
            RevokeDragDrop(hwnd).log_err();
        }
        if !lifetime.is_alive() {
            // RevokeDragDrop is a reentry point: if the window died across
            // it, the WM_NCDESTROY cleanup already owns retirement and the
            // HWND must not be destroyed again.
            return;
        }
        if let Err(error) = unsafe { DestroyWindow(hwnd) } {
            // A failed destruction must never be treated as retired: the
            // ledger entry stays until a matching NCDESTROY acknowledgment.
            // The runtime is then held at the epilogue instead of torn down.
            log_failed(
                "destroy",
                &format!(
                    "DestroyWindow failed for hwnd=0x{hwnd_value:X}: {error}; the identity stays \
                     unretired and the runtime will be held at exit"
                ),
            );
        }
        // Nothing is removed from the ledger here: only the matching
        // NCDESTROY cleanup retires the identity.
    }

    /// Retires a destroyed window on its native-lifetime token alone — the
    /// window inner may already be dead. This is the ONLY ledger-retiring
    /// acknowledgment (it is enqueued at `WM_NCDESTROY` entry, before the
    /// message dispatch). Releases the session without any restore mutation
    /// or HWND work; a newer occupant at the same hwnd value is never
    /// touched.
    pub(in super::super) fn cleanup(&mut self, lifetime: &WindowLifetime, hwnd_value: isize) {
        let identity = lifetime.id();
        match self.sessions.remove(&hwnd_value) {
            Some(session) if session.lifetime.id() == identity => {
                log(
                    "cleanup",
                    &format!(
                        "hwnd=0x{hwnd_value:X} identity={identity} destroyed; \
                         session released without restore"
                    ),
                );
                drop(session);
            }
            Some(session) => {
                // A different identity's session lives at this reused hwnd:
                // not ours to release.
                log(
                    "cleanup",
                    &format!(
                        "hwnd=0x{hwnd_value:X} hosts identity {}'s session; retired identity \
                         {identity} left it untouched",
                        session.lifetime.id()
                    ),
                );
                self.sessions.insert(hwnd_value, session);
            }
            None => {}
        }
        if let Some(progress) = self.failed_held.remove(&identity) {
            // The retired identity's retained rollback state is released from
            // the outer owner WITHOUT live restoration — a dead HWND is never
            // restored; only its references are dropped here.
            log(
                "cleanup",
                &format!(
                    "hwnd=0x{hwnd_value:X} identity={identity} destroyed; the retained \
                     failed-rollback state is released without live restoration"
                ),
            );
            drop(progress);
        }
        self.attempts.remove(&identity);
        self.ledger.retire(identity);
    }

    // -- helpers -------------------------------------------------------------

    /// Publishes the copyable per-window snapshot (never a global UI flag) and
    /// requests a frame so the change is visible on the next draw.
    fn snapshot_for(&self, hwnd_value: isize) -> CaptionChromeState {
        let Some(session) = self.sessions.get(&hwnd_value) else {
            return CaptionChromeState::default();
        };
        let dpi = unsafe { GetDpiForWindow(HWND(hwnd_value as *mut _)) };
        // The session's stored measurements — refreshed on every metric
        // invalidation, never re-read through extra SDK getters here.
        CaptionChromeState {
            active: true,
            caption_height: Some(logical_px(session.caption_height, dpi)),
            left_inset: Some(logical_px(session.left_inset, dpi)),
            right_inset: Some(logical_px(session.right_inset, dpi)),
        }
    }

    /// Publishes the app-owned header for a window whose native caption is not
    /// kept, and requests a frame so the change shows on the next draw.
    fn notify_failure(&self, inner: &WindowsWindowInner, lifetime: &WindowLifetime) {
        inner.set_caption_state(CaptionChromeState::default());
        if lifetime.is_alive() {
            notify_frame(inner.caption_hwnd_value());
        }
    }
}

fn logical_px(physical: i32, dpi: u32) -> Pixels {
    gpui::px(physical as f32 * 96.0 / dpi as f32)
}

/// Existing request-frame notification: an async invalidate, so the redraw
/// happens on a later pump turn rather than inside the host.
fn notify_frame(hwnd_value: isize) {
    let hwnd = HWND(hwnd_value as *mut _);
    unsafe {
        let _ = RedrawWindow(Some(hwnd), None, None, RDW_INVALIDATE);
    }
}

/// Detailed window-frame trace (style, extended style and window/client bounds)
/// at a named moment, read straight from the OS. Emitted only when traces are
/// enabled, so release builds perform none of these reads.
#[cfg(target_os = "windows")]
fn trace_window_frames(hwnd_value: isize, when: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{
        GWL_EXSTYLE, GWL_STYLE, GetWindowLongW, GetWindowRect,
    };
    if !trace_enabled() {
        return;
    }
    let hwnd = HWND(hwnd_value as *mut _);
    let mut window = RECT::default();
    let mut client = RECT::default();
    let window_ok = unsafe { GetWindowRect(hwnd, &mut window) }.is_ok();
    let client_ok = unsafe { GetClientRect(hwnd, &mut client) }.is_ok();
    let style = unsafe { GetWindowLongW(hwnd, GWL_STYLE) };
    let ex_style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };
    log(
        "frame",
        &format!(
            "hwnd=0x{hwnd_value:X} {when}: style=0x{style:08X} ex_style=0x{ex_style:08X} \
             window={window_ok}?({},{})-({},{}) client={client_ok}?({},{})-({},{})",
            window.left,
            window.top,
            window.right,
            window.bottom,
            client.left,
            client.top,
            client.right,
            client.bottom,
        ),
    );
}
