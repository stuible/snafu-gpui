//! Caption region translation: turns the published header layout into the SDK's
//! Caption and Passthrough rectangles for the current extent, reads the SDK's own
//! reserved button rectangles, and applies both kinds through the session journal.

use std::rc::Rc;

use appwindow_caption_bindings::{InputNonClientPointerSource, NonClientRegionKind, RectInt32};
use windows::Win32::Foundation::{GetLastError, HWND, RECT};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, USER_DEFAULT_SCREEN_DPI};

use gpui::WindowControlLayout;
use crate::app_window_caption::support::window_attach_txn;
use crate::app_window_caption::support::window_attach_txn::{
    Stage, StageOutcome, TransactionOutcome,
};
use gpui::window_caption_geometry::{
    LayoutAction, RegionMetrics, RegionRect, logical_to_physical_inward,
    logical_to_physical_outward, plan_regions, queued_layout_action,
};
use crate::app_window_caption::support::window_lifetime::WindowLifetime;
use crate::window::WindowsWindowInner;

use super::super::{log, log_failed, sdk_error};
use super::{WindowSession, stage_log_name};

/// Applies one region kind's rect set: cleared per-kind when empty, set
/// otherwise. `ClearRegionRects` per kind — never `ClearAllRegionRects` — so
/// the SDK's own button/border kinds are always left untouched.
pub(super) fn set_region_rects(
    input_source: &InputNonClientPointerSource,
    kind: NonClientRegionKind,
    rects: &[RectInt32],
) -> Result<(), String> {
    let result = if rects.is_empty() {
        input_source.ClearRegionRects(kind)
    } else {
        input_source.SetRegionRects(kind, rects)
    };
    result.map_err(|e| sdk_error(&e))
}

/// The physical client rect of `hwnd` as a planning rectangle, or `None`
/// when GetClientRect fails (logged).
pub(super) fn client_region_rect(hwnd: HWND) -> Option<RegionRect> {
    let mut rect = RECT::default();
    // GetClientRect returns Option<()> in windows 0.61 (BOOL checked).
    match unsafe { GetClientRect(hwnd, &mut rect) } {
        Ok(()) => Some(RegionRect {
            x: 0,
            y: 0,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        }),
        Err(_) => {
            log_failed(
                "client-rect",
                &format!("GetClientRect failed: GetLastError={}", unsafe {
                    GetLastError().0
                }),
            );
            None
        }
    }
}

/// The suspension Passthrough plan: with the window live, the whole client
/// area routes as client. Used before the first current frame under the
/// attached SDK, and whenever a metric-invalidating change waits for its next
/// frame — never a guessed title strip.
pub(super) fn initial_passthrough_plan(hwnd: HWND) -> Vec<RectInt32> {
    let Some(client) = client_region_rect(hwnd) else {
        return Vec::new();
    };
    plan_regions(client, None, &[], &[], &[])
        .passthrough
        .iter()
        .map(|rect| RectInt32 {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        })
        .collect()
}

/// What one region-application pass did to the native region sets.
pub(super) enum ApplyOutcome {
    /// A fresh plan was written to the SDK.
    Applied,
    /// The computed plan equals the region sets already applied: nothing was
    /// sent.
    Unchanged,
    /// An apply command found no stored layout payload: nothing was computed
    /// or sent — the applied plan stands, whatever it is.
    RetainedCurrent,
}

/// The outcome of one region-plan application.
pub(super) struct ApplyReport {
    pub(super) outcome: ApplyOutcome,
    pub(super) caption_rects: usize,
    pub(super) passthrough_rects: usize,
}

/// Why a region application did not complete. `Failed` means the native
/// region state is UNKNOWN — the caller must roll the session back, never
/// claim the previous regions are still intact.
pub(super) enum RegionApplyFailure {
    Retired,
    Failed(String),
}

/// Which queued command is applying regions. It gives a MISSING layout
/// payload its meaning: a refresh — the re-measurement after a metric
/// invalidation — suspends without one, so the old extent's regions never
/// stand while the new metrics wait for their frame; an apply command
/// retains the applied plan, so a duplicate queued apply (or one whose
/// payload a refresh already applied) can never clear valid drag regions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RegionCommand {
    Refresh,
    ApplyRegions,
}

/// Computes both region kinds for a live session — from the stored layout
/// payload when it is exactly current (this identity, this metric revision,
/// the ACTUAL client extent and DPI scale read here, and no older frame than
/// the one already applied), otherwise the suspension plan — and applies
/// them transactionally through the session's own journal: each kind is
/// journaled as attempted BEFORE its setter, so a partial failure (including
/// a second-kind failure after the first applied) restores the captured
/// originals for every journaled kind.
pub(super) fn apply_session_regions(
    hwnd_value: isize,
    session: &mut WindowSession,
    lifetime: &Rc<WindowLifetime>,
    inner: &WindowsWindowInner,
    stored: Option<WindowControlLayout>,
    command: RegionCommand,
) -> Result<ApplyReport, RegionApplyFailure> {
    let identity = session.lifetime.id();
    let hwnd = HWND(hwnd_value as *mut _);
    // The currency decision compares the payload against the window's
    // ACTUAL current measurements, read here like every other native fact —
    // never a cached extent or scale. An empty explicit SDK region array
    // later in the plan is never read as "the buttons have no geometry": the
    // measured insets are reserved regardless.
    let sdk_rects = sdk_reserved_rects(&session.input_source, lifetime);
    if !lifetime.is_alive() {
        return Err(RegionApplyFailure::Retired);
    }
    let Some(client) = client_region_rect(hwnd) else {
        return Err(RegionApplyFailure::Failed(
            "GetClientRect failed; the client extent for region planning is unknown".to_string(),
        ));
    };
    if !lifetime.is_alive() {
        return Err(RegionApplyFailure::Retired);
    }
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    let actual_scale_factor = dpi as f32 / USER_DEFAULT_SCREEN_DPI as f32;
    let action = queued_layout_action(
        command == RegionCommand::Refresh,
        stored.as_ref(),
        identity,
        inner.current_metric_revision(),
        client,
        actual_scale_factor,
        session.last_applied_frame,
    );
    let (drag, annotated_passthrough, payload_frame) = match action {
        LayoutAction::RetainCurrent => {
            log(
                "apply-regions",
                &format!(
                    "identity={} no new layout payload stored; the applied regions stand \
                     ({} caption / {} passthrough rects)",
                    identity,
                    session.last_caption.len(),
                    session.last_passthrough.len(),
                ),
            );
            return Ok(ApplyReport {
                outcome: ApplyOutcome::RetainedCurrent,
                caption_rects: session.last_caption.len(),
                passthrough_rects: session.last_passthrough.len(),
            });
        }
        LayoutAction::ApplySuspension { reason } => {
            match reason {
                Some(reason) => log(
                    "apply-regions",
                    &format!(
                        "identity={} rejected the stored layout payload ({} mismatch); the \
                         suspension plan applies",
                        identity,
                        reason.text()
                    ),
                ),
                None => log(
                    "apply-regions",
                    &format!(
                        "identity={} refresh found no stored layout payload under the \
                         metric invalidation; the suspension plan applies",
                        identity
                    ),
                ),
            }
            (Vec::new(), Vec::new(), None)
        }
        LayoutAction::ApplyPayload => {
            // The action proved the payload current; plan from it.
            let payload = stored.expect("current action proved a stored payload");
            let drag = payload
                .drag
                .iter()
                .filter_map(|bounds| logical_to_physical_inward(*bounds, payload.scale_factor))
                .collect();
            let annotated_passthrough = payload
                .passthrough
                .iter()
                .filter_map(|bounds| logical_to_physical_outward(*bounds, payload.scale_factor))
                .collect();
            (drag, annotated_passthrough, Some(payload.frame))
        }
    };
    let metrics = (session.caption_height > 0).then(|| RegionMetrics {
        caption_height: session.caption_height,
        left_inset: session.left_inset,
        right_inset: session.right_inset,
    });
    let plan = plan_regions(client, metrics, &drag, &annotated_passthrough, &sdk_rects);
    log(
        "apply-regions",
        &format!(
            "identity={} frame {payload_frame:?} dpi={dpi} client={}x{} metrics={metrics:?} \
             sdk_rects={} plan: {} caption / {} passthrough rects",
            identity,
            client.width,
            client.height,
            sdk_rects.len(),
            plan.caption.len(),
            plan.passthrough.len(),
        ),
    );
    let caption: Vec<RectInt32> = plan
        .caption
        .iter()
        .map(|rect| RectInt32 {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        })
        .collect();
    let passthrough: Vec<RectInt32> = plan
        .passthrough
        .iter()
        .map(|rect| RectInt32 {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        })
        .collect();
    if caption == session.last_caption && passthrough == session.last_passthrough {
        return Ok(ApplyReport {
            outcome: ApplyOutcome::Unchanged,
            caption_rects: caption.len(),
            passthrough_rects: passthrough.len(),
        });
    }
    let caption_rects = caption.len();
    let passthrough_rects = passthrough.len();
    let input_source = &session.input_source;
    let journal = &mut session.journal;
    let outcome = window_attach_txn::run_stages(
        &[Stage::Regions, Stage::PassthroughRegions],
        journal,
        || lifetime.is_alive(),
        |stage| {
            if !lifetime.is_alive() {
                // retired between the journaling and the setter
                return StageOutcome::Retired;
            }
            let (kind, rects): (NonClientRegionKind, &[RectInt32]) = match stage {
                Stage::Regions => (NonClientRegionKind::Caption, &caption),
                Stage::PassthroughRegions => (NonClientRegionKind::Passthrough, &passthrough),
                other => return StageOutcome::Failed(format!("unexpected stage {other:?}")),
            };
            match set_region_rects(input_source, kind, rects) {
                Ok(()) => StageOutcome::Applied,
                Err(message) => StageOutcome::Failed(message),
            }
        },
    );
    match outcome {
        TransactionOutcome::Complete => {
            session.last_caption = caption;
            session.last_passthrough = passthrough;
            // A suspension (`payload_frame` None) retires the stamp: the next
            // current frame applies whatever frame it carries.
            session.last_applied_frame = payload_frame;
            Ok(ApplyReport {
                outcome: ApplyOutcome::Applied,
                caption_rects,
                passthrough_rects,
            })
        }
        TransactionOutcome::Retired => Err(RegionApplyFailure::Retired),
        TransactionOutcome::Failed { stage, error } => Err(RegionApplyFailure::Failed(format!(
            "{}: {}",
            stage_log_name(stage),
            error
        ))),
    }
}

/// The SDK's own reserved non-client rectangles for its button/icon/border
/// kinds, read per kind with a plain token read between each (every getter is
/// a reentry boundary). A failed getter excludes only that kind's rectangles
/// from planning and is logged; an EMPTY result is passed through as-is — it
/// is never treated as proof that the native buttons have no geometry (the
/// measured inset strips are the conservative reservation for that case).
/// Only the SDK-owned kinds are read; Caption and Passthrough are never set
/// here.
pub(super) fn sdk_reserved_rects(
    input_source: &InputNonClientPointerSource,
    lifetime: &WindowLifetime,
) -> Vec<RegionRect> {
    let kinds = [
        NonClientRegionKind::Close,
        NonClientRegionKind::Maximize,
        NonClientRegionKind::Minimize,
        NonClientRegionKind::Icon,
        NonClientRegionKind::TopBorder,
        NonClientRegionKind::LeftBorder,
        NonClientRegionKind::BottomBorder,
        NonClientRegionKind::RightBorder,
    ];
    let mut rects = Vec::new();
    for kind in kinds {
        if !lifetime.is_alive() {
            return rects;
        }
        match input_source.GetRegionRects(kind) {
            Ok(array) => {
                let kind_rects: Vec<RegionRect> = array
                    .to_vec()
                    .iter()
                    .map(|rect| RegionRect {
                        x: rect.x,
                        y: rect.y,
                        width: rect.width,
                        height: rect.height,
                    })
                    .collect();
                log(
                    "sdk-regions",
                    &format!("kind {kind:?}: {} rect(s)", kind_rects.len()),
                );
                rects.extend(kind_rects);
            }
            Err(e) => log(
                "sdk-regions",
                &format!("kind {kind:?}: {} (excluded from planning)", sdk_error(&e)),
            ),
        }
    }
    rects
}
