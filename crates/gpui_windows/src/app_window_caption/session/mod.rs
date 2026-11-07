//! Per-window caption session state: the attached SDK session, the staged attach
//! progress that rolls back journal entry by entry, and the outcome types the
//! per-window transactions return. Attach construction, region translation and the
//! transaction methods live in child modules.

mod attach;
mod regions;
mod transaction;

use std::rc::Rc;

use appwindow_caption_bindings::{
    AppWindow, AppWindowTitleBar, Color, IAppWindowTitleBar2, IAppWindowTitleBar3,
    InputNonClientPointerSource, Interface, NonClientRegionKind, RectInt32, TitleBarHeightOption,
    TitleBarTheme,
};

use crate::app_window_caption::support::window_attach_txn;
use crate::app_window_caption::support::window_attach_txn::{
    Stage, StageJournal, StageOutcome,
};
use crate::app_window_caption::support::window_caption_policy::Rollback;
use crate::app_window_caption::support::window_lifetime::WindowLifetime;

use super::{log, sdk_error};
use regions::set_region_rects;

/// The pre-mutation originals, captured through the SDK's own getters before
/// anything is changed; restore re-applies these instead of blindly resetting
/// to defaults. `caption_regions` is the EXACT original Caption region array
/// and `passthrough_regions` the EXACT original Passthrough array, each
/// captured before that kind's first write — an empty original set is restored
/// with `ClearRegionRects` for its kind, any other set verbatim;
/// `ClearAllRegionRects` is never used, so the SDK's own button/border/other
/// region kinds are always preserved. `button_background`/
/// `button_inactive_background` are the EXACT nullable originals of the two
/// caption button background colors, captured before their setters run:
/// `None` is the unset property (the SDK default, restored by writing a null
/// reference), `Some` a previously customized color, re-applied verbatim.
#[derive(Clone)]
struct CapturedTitleBar {
    extends_content: bool,
    height_option: TitleBarHeightOption,
    theme: TitleBarTheme,
    button_background: Option<Color>,
    button_inactive_background: Option<Color>,
    caption_regions: Vec<RectInt32>,
    passthrough_regions: Vec<RectInt32>,
}

/// Fully transparent ARGB: the documented way to let the app's own title
/// area show through the caption button backgrounds while content is extended
/// into the title bar. A local constant keeps every app palette color out of
/// the vendor tree.
const TRANSPARENT_BUTTON_BACKGROUND: Color = Color {
    a: 0,
    r: 0,
    g: 0,
    b: 0,
};

/// The per-window caption state. `lifetime` is the native-lifetime token the
/// session was attached under; cleanup matches it against the retired token
/// so a reused HWND's newer session is never released by an older command.
/// The remaining fields are dropped in declaration order during release: the
/// input source (region ownership) first, then the title bar, then the
/// AppWindow itself. `caption_height`/`left_inset`/`right_inset` are the
/// latest SDK measurements (physical pixels) — re-read on every refresh,
/// never cached across metric invalidations — `last_caption`/
/// `last_passthrough` are the region sets actually applied last, so an
/// unchanged plan is never re-sent to the SDK, and `last_applied_frame` is
/// the published frame that plan came from (`None` after a suspension), so
/// a queued command can never apply a frame older than the applied one.
pub(super) struct WindowSession {
    lifetime: Rc<WindowLifetime>,
    input_source: InputNonClientPointerSource,
    title_bar: AppWindowTitleBar,
    _app_window: AppWindow,
    captured: CapturedTitleBar,
    journal: StageJournal,
    caption_height: i32,
    left_inset: i32,
    right_inset: i32,
    last_caption: Vec<RectInt32>,
    last_passthrough: Vec<RectInt32>,
    last_applied_frame: Option<u64>,
}

/// The outcome of a staged rollback.
enum RestoreOutcome {
    /// Every journaled stage was restored; nothing is retained.
    Restored,
    /// The identity retired mid-restore: the remaining interfaces are
    /// released (by this drop or the retired cleanup) without any further
    /// native restore — a dead HWND is never restored.
    Retired,
    /// The identity is still live but some stages failed to restore: the
    /// unfinished mutation/resource state stays retained in the host, never
    /// published as restored or defaulted, and no destroy continues through
    /// it.
    FailedHeld { failures: Vec<(Stage, String)> },
}

impl RestoreOutcome {
    /// The rollback result the caption failure policy decides on.
    fn rollback(&self) -> Rollback {
        match self {
            RestoreOutcome::Restored => Rollback::Restored,
            RestoreOutcome::Retired => Rollback::Retired,
            RestoreOutcome::FailedHeld { .. } => Rollback::Held,
        }
    }
}

/// The log-stage name of a transaction stage, for failure diagnostics.
fn stage_log_name(stage: Stage) -> &'static str {
    match stage {
        Stage::ExtendContent => "titlebar-extend-content",
        Stage::Height => "titlebar-height",
        Stage::Theme => "titlebar-theme",
        Stage::ButtonBackground => "titlebar-button-background",
        Stage::ButtonInactiveBackground => "titlebar-button-inactive-background",
        Stage::Regions => "caption-region-set",
        Stage::PassthroughRegions => "passthrough-region-set",
    }
}

// ---------------------------------------------------------------------------
// Staged session construction with independent per-stage unwind.
// ---------------------------------------------------------------------------

/// A session under construction — or the retained form of one whose rollback
/// failed on a live window. Every interface is owned here as soon as it is
/// acquired, and the journal records each mutating stage as attempted before
/// its SDK call, so a failure after any stage unwinds exactly that far and a
/// failed unwind keeps exactly what is still unfinished.
#[derive(Default)]
pub(super) struct AttachProgress {
    app_window: Option<AppWindow>,
    title_bar: Option<AppWindowTitleBar>,
    input_source: Option<InputNonClientPointerSource>,
    captured: Option<CapturedTitleBar>,
    journal: StageJournal,
    caption_height: Option<i32>,
    left_inset: Option<i32>,
    right_inset: Option<i32>,
}

impl AttachProgress {
    fn new() -> AttachProgress {
        AttachProgress::default()
    }

    /// Rolls back exactly the journaled stages, in reverse, while the
    /// identity stays live. Each stage is attempted independently — one
    /// failure never blocks the others and never publishes a restored state
    /// by itself — and a retired identity stops the pass immediately: a dead
    /// HWND is never restored, its cleanup releases the rest instead. Only
    /// stages that failed to restore stay journaled, which is exactly the
    /// unfinished state a FailedHeld retention keeps. The token checks are
    /// plain reads: no SDK call and no host borrow.
    fn restore(&mut self, lifetime: &WindowLifetime) -> RestoreOutcome {
        let (Some(captured), Some(title_bar)) = (self.captured.as_ref(), self.title_bar.as_ref())
        else {
            // Nothing was ever captured, so nothing was mutated: the journal
            // is empty by construction (stages are journaled only after the
            // capture completed).
            return RestoreOutcome::Restored;
        };
        let input_source = self.input_source.as_ref();
        let report = window_attach_txn::run_restore(
            &mut self.journal,
            || lifetime.is_alive(),
            |stage| match stage {
                Stage::Regions => {
                    let Some(input_source) = input_source else {
                        return StageOutcome::Failed("input source interface missing".to_string());
                    };
                    // Restore the exact original Caption set: an empty
                    // original is cleared per-kind, any other is re-applied
                    // verbatim. ClearAllRegionRects is never used — it would
                    // wipe the SDK's own button/border region kinds.
                    match set_region_rects(
                        input_source,
                        NonClientRegionKind::Caption,
                        &captured.caption_regions,
                    ) {
                        Ok(()) => {
                            log("restore", "re-applied the original Caption region set");
                            StageOutcome::Applied
                        }
                        Err(message) => StageOutcome::Failed(message),
                    }
                }
                Stage::PassthroughRegions => {
                    let Some(input_source) = input_source else {
                        return StageOutcome::Failed("input source interface missing".to_string());
                    };
                    // Same per-kind rule for the second kind: the exact
                    // original Passthrough set.
                    match set_region_rects(
                        input_source,
                        NonClientRegionKind::Passthrough,
                        &captured.passthrough_regions,
                    ) {
                        Ok(()) => {
                            log("restore", "re-applied the original Passthrough region set");
                            StageOutcome::Applied
                        }
                        Err(message) => StageOutcome::Failed(message),
                    }
                }
                Stage::ButtonInactiveBackground => {
                    // Restore the exact nullable original: None writes a null
                    // reference, which is the SDK's own unset default.
                    match title_bar
                        .SetButtonInactiveBackgroundColor(captured.button_inactive_background)
                    {
                        Ok(()) => {
                            log(
                                "restore",
                                "re-applied the captured inactive button background",
                            );
                            StageOutcome::Applied
                        }
                        Err(e) => StageOutcome::Failed(sdk_error(&e)),
                    }
                }
                Stage::ButtonBackground => {
                    match title_bar.SetButtonBackgroundColor(captured.button_background) {
                        Ok(()) => {
                            log(
                                "restore",
                                "re-applied the captured active button background",
                            );
                            StageOutcome::Applied
                        }
                        Err(e) => StageOutcome::Failed(sdk_error(&e)),
                    }
                }
                Stage::Theme => match title_bar.cast::<IAppWindowTitleBar3>() {
                    Ok(title_bar3) => {
                        if !lifetime.is_alive() {
                            // retired between the QI and the setter
                            return StageOutcome::Retired;
                        }
                        match title_bar3.SetPreferredTheme(captured.theme) {
                            Ok(()) => {
                                log("restore", "re-applied captured title bar theme");
                                StageOutcome::Applied
                            }
                            Err(e) => StageOutcome::Failed(sdk_error(&e)),
                        }
                    }
                    Err(e) => StageOutcome::Failed(format!("cast: {}", sdk_error(&e))),
                },
                Stage::Height => match title_bar.cast::<IAppWindowTitleBar2>() {
                    Ok(title_bar2) => {
                        if !lifetime.is_alive() {
                            // retired between the QI and the setter
                            return StageOutcome::Retired;
                        }
                        match title_bar2.SetPreferredHeightOption(captured.height_option) {
                            Ok(()) => {
                                log("restore", "re-applied captured height option");
                                StageOutcome::Applied
                            }
                            Err(e) => StageOutcome::Failed(sdk_error(&e)),
                        }
                    }
                    Err(e) => StageOutcome::Failed(format!("cast: {}", sdk_error(&e))),
                },
                Stage::ExtendContent => {
                    match title_bar.SetExtendsContentIntoTitleBar(captured.extends_content) {
                        Ok(()) => {
                            log("restore", "re-applied captured content extension");
                            StageOutcome::Applied
                        }
                        Err(e) => StageOutcome::Failed(sdk_error(&e)),
                    }
                }
            },
        );
        if report.retired {
            return RestoreOutcome::Retired;
        }
        if report.failures.is_empty() && self.journal.is_empty() {
            RestoreOutcome::Restored
        } else {
            RestoreOutcome::FailedHeld {
                failures: report.failures,
            }
        }
    }

    /// Resolves a fully built progress into the session. Every field is
    /// present by construction once the build reached its end.
    fn into_session(mut self, lifetime: &Rc<WindowLifetime>) -> WindowSession {
        WindowSession {
            lifetime: Rc::clone(lifetime),
            input_source: self.input_source.take().expect("input source owned"),
            title_bar: self.title_bar.take().expect("title bar owned"),
            _app_window: self.app_window.take().expect("app window owned"),
            captured: self.captured.take().expect("originals captured"),
            journal: std::mem::take(&mut self.journal),
            caption_height: self.caption_height.expect("caption height measured"),
            left_inset: self.left_inset.expect("left inset measured"),
            right_inset: self.right_inset.expect("right inset measured"),
            last_caption: Vec::new(),
            last_passthrough: Vec::new(),
            last_applied_frame: None,
        }
    }

    /// Takes a live session back into the retained progress form, so a
    /// failed destroy rollback keeps every interface and the journal.
    fn from_session(session: WindowSession) -> AttachProgress {
        AttachProgress {
            app_window: Some(session._app_window),
            title_bar: Some(session.title_bar),
            input_source: Some(session.input_source),
            captured: Some(session.captured),
            journal: session.journal,
            caption_height: Some(session.caption_height),
            left_inset: Some(session.left_inset),
            right_inset: Some(session.right_inset),
        }
    }
}

/// The outcome of a preempted or failed session build.
enum AttachFailure {
    /// A live failure at a named stage: the caller rolls back exactly the
    /// stages mutated so far.
    Failed(String, AttachProgress),
    /// The identity was retired through a reentrant boundary while the
    /// session was being built: the progress is released WITHOUT native
    /// restore — a dead HWND is never restored — and nothing is published.
    Dead,
}

/// The plain liveness revalidation between SDK boundaries: a token read
/// only — no SDK call, no IsWindow, no host or App borrow.
fn ensure_alive(lifetime: &WindowLifetime) -> Result<(), AttachFailure> {
    if lifetime.is_alive() {
        Ok(())
    } else {
        Err(AttachFailure::Dead)
    }
}
