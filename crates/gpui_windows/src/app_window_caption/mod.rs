//! Loop-owned native AppWindow caption host over the existing GPUI HWND.
//!
//! The native caption is on by default. `SNAFU_DISABLE_NATIVE_CAPTION=1` turns it
//! off for support diagnostics: then every entry point here is a no-op, and
//! window destruction keeps the established GPUI behavior (executor destroy on
//! drop, `DefWindowProc` close). Otherwise ONE host is created on the GPUI
//! foreground thread before the launch callback runs and before any SDK window
//! mutation. The host owns the staged SDK runtime — Bootstrap initialization,
//! STA WinRT initialization and one `DispatcherQueueController` created on the
//! current thread — for the whole run: ordinary window close and reopen never
//! tear it down, and the approved terminal exit winds it down once.
//!
//! Failure policy: a caption failure never exits the process and never panics.
//! The decisions live in `support::window_caption_policy`:
//! - Runtime start-up failure: every window keeps its app-owned header and no
//!   attach is attempted.
//! - Attach or region failure whose rollback restores the captured originals (or
//!   that wrote nothing): the window shows its app-owned header and the native
//!   attach is not retried for that identity.
//! - Rollback that cannot restore a live window: the unfinished native state is
//!   retained in the host, the window shows its app-owned header, and no further
//!   SDK call is issued for that window. Nothing is reported as restored.
//! - Teardown that cannot be verified, or a queue rundown that fails: the runtime
//!   is held instead of released, so no Ro/Bootstrap balance or COM release runs
//!   over live work. The process reclaims it at exit.
//! Each such failure logs one `FAILED` line. Detailed traces appear only in debug
//! builds or with `SNAFU_CAPTION_TRACE=1` set for support diagnostics.
//!
//! Threading and ownership: everything here runs on the thread that owns the
//! HWNDs, their STA `OleInitialize` and the message pump. The host lives in
//! `WindowsPlatform::run` as a local; the only shared state is this module's
//! thread-local command inbox, which message handlers and window drops push plain
//! commands onto (keyed by the window's native-lifetime token, never SDK objects),
//! followed by one coalesced platform wake. Commands run between outer
//! `DispatchMessageW` turns, after a consumed pretranslated message, and at the
//! pump epilogue — never inside a message dispatch, never under an `App`/platform/
//! window borrow, and never with a GPUI `RefCell` or registry lock held across an
//! SDK call, `QI`, `AddRef`, `Release` or any other COM-bearing drop. Live
//! commands require their token still alive before any work, and attach, refresh,
//! restore and destroy revalidate that token after every potentially reentrant SDK
//! boundary, so a destroyed or reused HWND never receives a stale attach, restore
//! or destroy.
//!
//! Queue contract (proven by the isolated capability probe on Windows 11 ARM64
//! build 26300 with Windows App SDK 2.5.1): queue-free, `AppWindow`/title bar
//! succeeds but `InputNonClientPointerSource::GetForWindowId` fails with
//! `E_ACCESSDENIED`; with an owned queue created via `CreateOnCurrentThread`, the
//! input source and `SetRegionRects` succeed. The host therefore creates its
//! controller only when the typed `DispatcherQueue::GetForCurrentThread` probe
//! answers S_OK with a null queue — an arbitrary HRESULT error is a failure, not
//! "no queue" — and refuses an unexpected foreign queue. Teardown runs in a fixed
//! order: verify retirement → enter the terminal phase BEFORE the first SDK
//! boundary (native creation is then refused at `WM_NCCREATE`, and only retirement
//! is admitted) → synchronous `IDispatcherQueueController2::ShutdownQueue` →
//! consume cleanup and verify → release the controller interfaces → consume
//! release-triggered commands and verify again → close admission (every later
//! enqueue rejected, wake target cleared) → balance Ro and Bootstrap. No async
//! join, no `WaitForSingleObject`, no COM in `WM_NCDESTROY`, and never
//! `AssociateWithDispatcherQueue` or an SDK-created window for GPUI's borrowed HWND.
//!
//! Region and input ownership: while a session is attached, GPUI's own
//! non-client hit-test classification, NCCALCSIZE rewrite and NC mouse emulation
//! are bypassed for that window — the SDK's subclass chain plus `DefWindowProc`
//! own the routing, and the save-aware `WM_CLOSE` path is untouched. The app's
//! Caption comes only from the completed header Drag rectangle of a current frame,
//! stamped with the native identity and a metric revision, and clipped to the
//! measured title band with the SDK's own button/icon/border rectangles excluded.
//! Application is transactional per kind through the shared stage journal: both
//! kinds' originals are captured before the first write, and a partial failure
//! rolls the whole session back to them.
//!
//! Close admission is owned by the window's lifecycle token (`Idle`/
//! `CallbackRunning`/`DestroyQueued`, `Dead` terminal): a nested `WM_CLOSE` during a
//! running callback and a callback-less close are answered without destruction and
//! never fall through to `DefWindowProc`; an approved close queues exactly one
//! retained Destroy, deduplicated with the owner-drop authorization.

mod runtime;
mod session;
pub(crate) mod support;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread_local;

use appwindow_caption_bindings::Error;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
use windows::core::HRESULT;

use super::window::WindowsWindowInner;
use crate::app_window_caption::support::window_caption_policy;
use crate::app_window_caption::support::window_lifetime::WindowLifetime;
use crate::app_window_caption::support::window_terminal::{self, TerminalLedger};

use runtime::{PreTranslate, RuntimeOwner, acquire_runtime};
use session::{AttachProgress, WindowSession};

const LOG_PREFIX: &str = "[snafu-appwindow-caption]";

/// One-line trace: emitted only in debug builds or when `SNAFU_CAPTION_TRACE=1`.
fn log(stage: &str, detail: &str) {
    if trace_enabled() {
        eprintln!("{LOG_PREFIX} {stage}: {detail}");
    }
}

/// One-line diagnostic for a failure. Always emitted; never more than one per failure.
fn log_failed(stage: &str, detail: &str) {
    eprintln!("{LOG_PREFIX} FAILED stage={stage}: {detail}");
}

/// Diagnostics for `windows` 0.61 (old core) HRESULT values.
fn os_hr(hr: HRESULT) -> String {
    format!("HRESULT=0x{:08X}", hr.0)
}

/// Diagnostics for windows-core 0.100 errors, normalized through `.code()`;
/// the two HRESULT types are never transmuted into each other.
fn sdk_error(error: &Error) -> String {
    format!("HRESULT=0x{:08X}", error.code().0)
}

/// Whether detailed traces are emitted: always in debug builds, and in release
/// builds only when `SNAFU_CAPTION_TRACE=1` is set for support diagnostics.
fn trace_enabled() -> bool {
    static TRACE: OnceLock<bool> = OnceLock::new();
    cfg!(debug_assertions)
        || *TRACE.get_or_init(|| {
            std::env::var_os("SNAFU_CAPTION_TRACE").is_some_and(|value| value == "1")
        })
}

/// Whether the native caption is enabled for this run. Read once: the answer is
/// fixed for the process, and it is consulted on every enqueue and by the
/// renderer's composition-layer selection.
pub(super) fn native_caption_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        window_caption_policy::native_caption_enabled(
            std::env::var_os(window_caption_policy::DISABLE_ENV).as_deref(),
        )
    })
}

/// Whether the window destruction and close seams take the loop-owner lifecycle.
/// False only with the support opt-out, where the established GPUI paths run.
pub(super) fn lifecycle_enabled() -> bool {
    native_caption_enabled()
}

// ---------------------------------------------------------------------------
// Commands: the only thing message handlers and drop sites ever do.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum CommandKind {
    /// Native window creation: register the HWND in the ledger before anything
    /// else can happen to it.
    Register,
    /// First show of a live identity: attempt the staged attach once.
    Attach,
    /// Size/DPI/activation/presenter change: re-read actual SDK geometry and
    /// reapply the region plan from the latest current layout payload (or
    /// suspend the app caption when none is).
    Refresh,
    /// A completed GPUI frame published fresh control geometry: apply the
    /// latest payload for this identity if it is still current.
    ApplyRegions,
    /// Approved close or owner drop: live detach, then the existing
    /// RevokeDragDrop/DestroyWindow continuation, once. The command OWNS the
    /// window inner until the continuation completes in the loop owner.
    Destroy,
    /// `WM_NCDESTROY` observed (the ONLY ledger-retiring acknowledgment):
    /// release the retired identity's session — no SDK mutation and no HWND
    /// work ever runs from the destruction path, and the command stays
    /// executable after the inner object dies.
    Cleanup,
}

/// One inbox command, keyed by the window's native-lifetime token. Live kinds
/// (Attach/Refresh/Destroy) carry the window inner; Destroy owns its `Rc` for
/// the continuation's whole life. Register and Cleanup are token-only: they
/// must stay executable when the inner is already dead, and Cleanup never
/// resurrects or touches an HWND.
struct Command {
    kind: CommandKind,
    lifetime: Rc<WindowLifetime>,
    hwnd_value: isize,
    live: Option<Rc<WindowsWindowInner>>,
}

thread_local! {
    /// Foreground-thread command inbox. Message handlers and drop sites push
    /// plain commands here; only the loop-owned host drains it. With the native
    /// caption disabled nothing is ever enqueued, and enqueueing never touches
    /// the SDK.
    static INBOX: RefCell<Vec<Command>> = const { RefCell::new(Vec::new()) };
    /// The platform inbox window and validation number, recorded once at host
    /// creation so `enqueue` can post a plain coalesced wake.
    static WAKE_TARGET: RefCell<Option<(HWND, usize)>> = const { RefCell::new(None) };
}

/// Set when a wake has been posted and not yet consumed by a host drive, so
/// any number of enqueues share one posted message: at most one wake is ever
/// unconsumed, which is the explicit bound on coalescing.
static WAKE_PENDING: AtomicBool = AtomicBool::new(false);

/// The terminal-barrier phase latch, written only by the loop-owned host on
/// the foreground thread: `PHASE_OPEN` until the approved terminal exit,
/// `PHASE_TERMINAL` from just before the first SDK rundown/release boundary
/// (native creation refused at WM_NCCREATE; only retirement acknowledgments
/// admitted), and `PHASE_CLOSED` at final close admission (every enqueue
/// rejected) for the runtime balance. Never rewritten backwards. Every
/// admission decision goes through [`window_terminal::admits`].
static TERMINAL_PHASE: AtomicU8 = AtomicU8::new(window_terminal::PHASE_OPEN);

/// Posts one plain platform wake (the existing GPUI main-thread dispatch
/// message, carrying the validation number) so a pump waiting in
/// `GetMessageW` returns and the host can drive the inbox between turns.
/// Coalesced to at most one unconsumed post via `WAKE_PENDING`. A failed post
/// is logged and the flag is cleared so the next enqueue retries; queued
/// commands are still drained at the next pump turn.
fn wake() {
    if WAKE_PENDING.swap(true, Ordering::SeqCst) {
        return;
    }
    let target = WAKE_TARGET.with_borrow(Clone::clone);
    let Some((hwnd, validation_number)) = target else {
        // Only reachable after close admission cleared the target, when
        // enqueues are already rejected.
        WAKE_PENDING.store(false, Ordering::SeqCst);
        return;
    };
    if let Err(error) = unsafe {
        PostMessageW(
            Some(hwnd),
            super::events::WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD,
            WPARAM(validation_number),
            LPARAM(0),
        )
    } {
        WAKE_PENDING.store(false, Ordering::SeqCst);
        log_failed(
            "wake",
            &format!("PostMessageW failed: {error}; queued caption work waits for the next pump turn"),
        );
    }
}

/// The two gates every enqueue passes: the native caption must be enabled and
/// the terminal barrier must still admit the command kind. During the terminal
/// phase only a retirement acknowledgment is admitted, through
/// [`window_terminal::admits`]. A rejected live command's window stays in the
/// ledger, and a rejected registration cannot exist at all (native creation is
/// refused at WM_NCCREATE), so rejection never hides live work from the barrier.
fn enqueue_admitted(kind: CommandKind) -> bool {
    if !native_caption_enabled() {
        return false;
    }
    let phase = TERMINAL_PHASE.load(Ordering::SeqCst);
    if !window_terminal::admits(phase, matches!(kind, CommandKind::Cleanup)) {
        log(
            "enqueue",
            "rejected by the terminal barrier; native work cannot reopen",
        );
        return false;
    }
    true
}

/// Whether native window creation may proceed, consulted at WM_NCCREATE — the
/// actual native creation boundary. In the terminal phase the barrier can no
/// longer register, destroy or verify a new HWND, so creation is refused before
/// any window state is built. The guard is a plain latch read — no SDK call and
/// no host borrow — so it is safe inside the message dispatch. With the native
/// caption disabled this is always true.
pub(super) fn native_creation_admitted() -> bool {
    if !native_caption_enabled() {
        return true;
    }
    if window_terminal::creation_admitted(TERMINAL_PHASE.load(Ordering::SeqCst)) {
        return true;
    }
    log_failed(
        "native-creation",
        "refused at WM_NCCREATE during the terminal phase; the window was not created",
    );
    false
}

/// Pushes one admitted command and requests the coalesced wake. Only called
/// after `enqueue_admitted` — admission is the barrier contract.
fn dispatch(command: Command) {
    INBOX.with_borrow_mut(|inbox| inbox.push(command));
    wake();
}

fn enqueue(kind: CommandKind, inner: &Rc<WindowsWindowInner>) {
    if !enqueue_admitted(kind) {
        return;
    }
    dispatch(Command {
        kind,
        lifetime: Rc::clone(&inner.lifetime),
        hwnd_value: inner.caption_hwnd_value(),
        live: Some(Rc::clone(inner)),
    });
}

fn enqueue_token(kind: CommandKind, lifetime: Rc<WindowLifetime>, hwnd_value: isize) {
    if !enqueue_admitted(kind) {
        return;
    }
    dispatch(Command {
        kind,
        lifetime,
        hwnd_value,
        live: None,
    });
}

pub(super) fn enqueue_register(lifetime: Rc<WindowLifetime>, hwnd_value: isize) {
    enqueue_token(CommandKind::Register, lifetime, hwnd_value);
}

pub(super) fn enqueue_attach(inner: &Rc<WindowsWindowInner>) {
    enqueue(CommandKind::Attach, inner);
}

/// The open-transition seam: queues the attach for a still-hidden
/// `params.show` window and defers its first show to the host's
/// continuation. Returns whether the command was admitted — the caller
/// falls back to the immediate `SetWindowPlacement` exactly when this is
/// false (native caption disabled, or the terminal barrier refusing). The show
/// request is marked on the identity BEFORE the command is pushed, so the host
/// can never observe the command without it. A close that beat the continuation
/// is never shown.
pub(super) fn enqueue_prepare_for_show(inner: &Rc<WindowsWindowInner>) -> bool {
    if !enqueue_admitted(CommandKind::Attach) {
        return false;
    }
    inner.lifetime.request_deferred_show();
    dispatch(Command {
        kind: CommandKind::Attach,
        lifetime: Rc::clone(&inner.lifetime),
        hwnd_value: inner.caption_hwnd_value(),
        live: Some(Rc::clone(inner)),
    });
    true
}

pub(super) fn enqueue_refresh(inner: &Rc<WindowsWindowInner>) {
    enqueue(CommandKind::Refresh, inner);
}

/// Bumps the window's metric revision — retiring every stored layout payload
/// from before the change — and queues the re-measuring refresh. Every size,
/// DPI, activation and fullscreen-style invalidation goes through here, so a
/// payload computed under old metrics can never apply afterwards.
pub(super) fn enqueue_metric_invalidation(inner: &Rc<WindowsWindowInner>) {
    inner.invalidate_metric_revision();
    enqueue_refresh(inner);
}

/// Queues the application of the latest published control layout. Drawing
/// queues plain data only; this runs in the loop owner, never inside a draw.
pub(super) fn enqueue_apply_regions(inner: &Rc<WindowsWindowInner>) {
    enqueue(CommandKind::ApplyRegions, inner);
}

/// Queues the authorized destruction continuation. The command takes its own
/// strong reference, so the window inner survives until the loop owner runs
/// the continuation — independent of GPUI's registry, the quit callback's
/// borrows, and the OLE drag-drop registration's incidental reference.
pub(super) fn enqueue_destroy(inner: &Rc<WindowsWindowInner>) {
    enqueue(CommandKind::Destroy, inner);
}

/// Retires one destroyed window: keyed by the native-lifetime token captured
/// at `WM_NCDESTROY` entry (before dispatch), executable even when the window
/// inner is already dead, and never able to match a later window allocated at
/// the same HWND value.
pub(super) fn enqueue_cleanup(lifetime: Rc<WindowLifetime>, hwnd_value: isize) {
    enqueue_token(CommandKind::Cleanup, lifetime, hwnd_value);
}

// ---------------------------------------------------------------------------
// The loop-owned host.
// ---------------------------------------------------------------------------

/// Loop-owned host: staged runtime, per-generation sessions, and the inbox
/// drain points. Owned as a local by `WindowsPlatform::run`. Cleanup is never
/// performed from `Drop`: only the explicit epilogue releases the runtime, and
/// a teardown that cannot be verified holds it instead.
pub(super) struct CaptionHost {
    runtime: Option<RuntimeOwner>,
    sessions: HashMap<isize, WindowSession>,
    /// The pure retirement ledger ([`TerminalLedger`]): every identity from
    /// native creation until its acknowledged retirement, plus any recorded
    /// terminal-phase live work. The epilogue releases the runtime only when the
    /// ledger is balanced.
    ledger: TerminalLedger,
    /// One attach attempt per live generation; a failed generation keeps its
    /// failure stage so later commands never retry it. A fresh window (new
    /// generation) gets a fresh attempt.
    attempts: HashMap<u64, String>,
    /// Rollbacks that failed while their identity stayed live, keyed by the
    /// identity. The unfinished native state is retained here as the only cleanup
    /// capability for the partially mutated title bar: nothing is published as
    /// restored, no further SDK call is made for the window, and the identity's
    /// retired cleanup releases the state without live restoration.
    failed_held: HashMap<u64, AttachProgress>,
    pretranslate: PreTranslate,
    sealed: bool,
}

impl CaptionHost {
    /// Creates the host BEFORE the launch callback and any SDK window mutation:
    /// staged runtime acquisition happens here, on the foreground thread. A
    /// start-up failure leaves the host without a runtime; every window then
    /// keeps its app-owned header. Also records the platform wake target for the
    /// coalesced inbox wake.
    pub(super) fn new(platform_hwnd: HWND, validation_number: usize) -> CaptionHost {
        let mut host = CaptionHost {
            runtime: None,
            sessions: HashMap::new(),
            ledger: TerminalLedger::new(),
            attempts: HashMap::new(),
            failed_held: HashMap::new(),
            pretranslate: PreTranslate::Unresolved,
            sealed: false,
        };
        if !native_caption_enabled() {
            // Opt-out: record no wake target, acquire no runtime; every entry
            // point below early-returns, so the established lifecycle is untouched.
            return host;
        }
        WAKE_TARGET.with_borrow_mut(|target| {
            *target = Some((platform_hwnd, validation_number));
        });
        match acquire_runtime() {
            Ok(runtime) => {
                log("host", "staged runtime acquired before launch");
                host.runtime = Some(runtime);
            }
            Err(message) => {
                log_failed("host-create", &message);
                log_failed(
                    "host",
                    "native caption unavailable for this run; every window keeps its app-owned header",
                );
            }
        }
        host
    }

    /// Drives every queued command to completion, including commands queued by
    /// earlier commands in the same drain (a destroy's `WM_NCDESTROY` queues a
    /// plain cleanup, and the SDK rundown itself can enqueue more). Consumes the
    /// coalesced wake before draining so enqueues during the drain can request a
    /// fresh one.
    fn drive_all(&mut self) {
        WAKE_PENDING.store(false, Ordering::SeqCst);
        loop {
            let batch = INBOX.with_borrow_mut(std::mem::take);
            if batch.is_empty() {
                return;
            }
            for command in batch {
                self.execute(command);
            }
        }
    }

    /// Public turn boundary for `WindowsPlatform::run` between message turns.
    /// With the native caption disabled this is a true no-op.
    pub(super) fn drive(&mut self) {
        if !native_caption_enabled() {
            return;
        }
        self.drive_all();
    }

    /// Runs one queued command, keyed by the native-lifetime token. Live
    /// kinds require the token still alive: a retired identity can never
    /// attach, refresh or destroy, and a reused HWND's fresh token never
    /// matches an older command. Register and Cleanup run on the token alone
    /// and must work when the inner is already dead.
    fn execute(&mut self, command: Command) {
        match command.kind {
            CommandKind::Register => {
                self.register(&command.lifetime, command.hwnd_value);
            }
            CommandKind::Cleanup => {
                self.cleanup(&command.lifetime, command.hwnd_value);
            }
            CommandKind::Attach
            | CommandKind::Refresh
            | CommandKind::ApplyRegions
            | CommandKind::Destroy => {
                let Some(inner) = command.live else {
                    log(
                        "identity",
                        &format!(
                            "{:?} command arrived without its live window reference; skipped",
                            command.kind
                        ),
                    );
                    return;
                };
                if !command.lifetime.is_alive() {
                    // Retired (WM_NCDESTROY observed) before this command
                    // ran: a dead identity never touches any HWND — in
                    // particular not one a later window now owns.
                    return;
                }
                match command.kind {
                    CommandKind::Attach => {
                        self.attach(command.hwnd_value, &command.lifetime, &inner);
                        self.show_after_prepare(&command.lifetime, &inner);
                    }
                    CommandKind::Refresh => {
                        self.refresh(command.hwnd_value, &command.lifetime, &inner)
                    }
                    CommandKind::ApplyRegions => {
                        self.apply_regions(command.hwnd_value, &command.lifetime, &inner)
                    }
                    CommandKind::Destroy => {
                        // The command owns this Rc until the native
                        // destruction continuation below completes; the
                        // reference is released afterwards, outside every
                        // App/host borrow.
                        self.destroy(command.hwnd_value, &command.lifetime, &inner)
                    }
                    // Routed by the token-only arms above.
                    CommandKind::Register | CommandKind::Cleanup => {}
                }
            }
        }
    }

    /// Gives the App SDK's `ContentPreTranslateMessage` the first chance at
    /// each pumped message while a session is attached, so its subclass chain
    /// services non-client input before `DispatchMessageW`.
    pub(super) fn pretranslate(
        &mut self,
        msg: &windows::Win32::UI::WindowsAndMessaging::MSG,
    ) -> bool {
        if self.sessions.is_empty() {
            return false;
        }
        let PreTranslate::Resolved(pfn) = &self.pretranslate else {
            return false;
        };
        if (*pfn).is_null() {
            return false;
        }
        // Signature per the pinned Microsoft.UI.Dispatching.Interop.h:
        // BOOL __stdcall(const MSG*).
        let pretranslate_msg: unsafe extern "system" fn(
            *const windows::Win32::UI::WindowsAndMessaging::MSG,
        ) -> i32 = unsafe { core::mem::transmute(*pfn) };
        unsafe { pretranslate_msg(core::ptr::from_ref(msg)) != 0 }
    }

    /// Stops admitting new attachments at the approved terminal exit.
    pub(super) fn seal(&mut self) {
        self.sealed = true;
    }

    /// Enters the terminal phase BEFORE the first SDK rundown/release boundary:
    /// from here on native creation is refused at WM_NCCREATE, only retirement
    /// acknowledgments are admitted, and live work reaching the terminal drain is
    /// recorded as a barrier violation. The process-wide latch ([`TERMINAL_PHASE`])
    /// is what the enqueue and creation guards read; it is set here, on the one
    /// foreground thread, and never rewritten backwards.
    fn enter_terminal_phase(&mut self) {
        self.sealed = true;
        TERMINAL_PHASE.store(window_terminal::PHASE_TERMINAL, Ordering::SeqCst);
        log(
            "terminal",
            "terminal phase entered before rundown: native creation is refused and only \
             retirement is admitted",
        );
    }

    /// The approved-terminal-exit epilogue, run by the loop owner AFTER the quit
    /// callback returned (so every `App` borrow is gone). Each step runs only
    /// after the previous one succeeded (see the module documentation):
    ///
    /// 1. drain the recorded destruction continuations and verify every
    ///    identity retired and every session gone;
    /// 2. enter the terminal phase before the first SDK boundary;
    /// 3. run the synchronous SDK queue rundown once;
    /// 4. consume cleanup the rundown enqueued, then verify;
    /// 5. release the controller/queue interfaces;
    /// 6. consume release-triggered commands and verify again;
    /// 7. close admission, then balance Ro and Bootstrap.
    ///
    /// A pump failure is logged but the rundown is still attempted: it is
    /// synchronous and does not depend on the app pump. Any unverified state or
    /// failed rundown holds the runtime (see [`CaptionHost::hold_runtime`]) and
    /// returns; it never exits the process.
    pub(super) fn epilogue(&mut self, pump_failed: bool) {
        if !native_caption_enabled() {
            return;
        }
        if pump_failed {
            log_failed(
                "pump",
                "message pump exited with an error; the synchronous queue rundown is still attempted",
            );
        }
        self.drive_all();
        if !self.retirement_settled("before the queue rundown") {
            self.hold_runtime();
            return;
        }
        self.enter_terminal_phase();
        let Some(runtime) = self.runtime.take() else {
            // Native caption on, but no SDK runtime was acquired: nothing to
            // rundown or balance. The ledger was verified above.
            self.close_admission();
            log("shutdown", "no SDK runtime owned; terminal admission closed");
            return;
        };
        if let Err(message) = runtime.shutdown_queue() {
            log_failed("epilogue", &format!("{message}; the queue is held"));
            std::mem::forget(runtime);
            self.close_admission();
            return;
        }
        // Terminal drain: cleanup only. Any other command is recorded as a
        // barrier violation, so the verification below fails explicitly.
        self.terminal_drive();
        if !self.retirement_settled("after the queue rundown") {
            std::mem::forget(runtime);
            self.close_admission();
            return;
        }
        let released = runtime.release_interfaces();
        // The Release calls may enqueue callback work: consume it while the
        // terminal phase still admits retirement, then verify again.
        self.terminal_drive();
        if !self.retirement_settled("after the final interface release") {
            // The interfaces are already released; the Ro/Bootstrap balance is
            // skipped so no unload runs while live work may remain.
            self.close_admission();
            return;
        }
        self.close_admission();
        released.balance_runtime();
        log(
            "shutdown",
            "runtime teardown complete (queue rundown consumed, interfaces released, \
             admission closed, WinRT, Bootstrap)",
        );
    }

    /// The terminal-phase drain: cleanup acknowledgments are still consumed,
    /// but any other command means native work tried to reopen during the
    /// final balance — it is rejected and recorded in the ledger, so the next
    /// verification fails. A rejected Register in particular can never appear
    /// in the outstanding set (native creation is refused at WM_NCCREATE), which
    /// is why the violation record, not the log line, enforces the lifetime.
    fn terminal_drive(&mut self) {
        WAKE_PENDING.store(false, Ordering::SeqCst);
        loop {
            let batch = INBOX.with_borrow_mut(std::mem::take);
            if batch.is_empty() {
                return;
            }
            for command in batch {
                match command.kind {
                    CommandKind::Cleanup => self.cleanup(&command.lifetime, command.hwnd_value),
                    other => {
                        self.ledger.note_violation();
                        log(
                            "terminal",
                            &format!(
                                "{other:?} command rejected during the terminal phase and \
                                 recorded as a barrier violation; native work cannot reopen"
                            ),
                        );
                    }
                }
            }
        }
    }

    /// Single retirement check: every registered identity must have its
    /// NCDESTROY acknowledgment, every session must be gone, no failed rollback
    /// state may stay retained, and no terminal-phase live work may be recorded.
    /// On failure this logs one line and returns false; the caller holds the
    /// runtime rather than tearing down over live work.
    fn retirement_settled(&self, when: &str) -> bool {
        if self.sessions.is_empty() && self.failed_held.is_empty() && self.ledger.is_balanced() {
            return true;
        }
        log_failed(
            "epilogue",
            &format!(
                "outstanding identities {:?} / sessions {} / retained rollbacks {} / terminal \
                 live work {} {when}",
                self.ledger.sorted_outstanding(),
                self.sessions.len(),
                self.failed_held.len(),
                if self.ledger.is_violated() {
                    "recorded"
                } else {
                    "none"
                },
            ),
        );
        false
    }

    /// Holds the runtime instead of releasing it: the queue and its references
    /// are leaked on purpose, so no release or Ro/Bootstrap balance runs over
    /// possibly live work. The process reclaims them at exit. Closes admission
    /// so nothing else can be posted or driven afterwards.
    fn hold_runtime(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            std::mem::forget(runtime);
            log_failed(
                "epilogue",
                "SDK runtime held: no queue rundown or Ro/Bootstrap balance runs; the process reclaims it at exit",
            );
        }
        self.close_admission();
    }

    /// Close admission: the terminal phase is already established (entered
    /// before the first rundown/release boundary); this advances the barrier to
    /// closed — every later enqueue is rejected, even a retirement
    /// acknowledgment — and clears the wake target, so nothing can be posted or
    /// driven during the final runtime balance.
    fn close_admission(&mut self) {
        self.sealed = true;
        TERMINAL_PHASE.store(window_terminal::PHASE_CLOSED, Ordering::SeqCst);
        WAKE_TARGET.with_borrow_mut(|target| *target = None);
    }
}
