//! Staged construction of a per-window caption session: captures the pre-mutation
//! originals and configures the title bar and input source, handing back a
//! `WindowSession` only after every stage succeeded.

use std::rc::Rc;

use appwindow_caption_bindings::{
    AppWindow, Color, DispatcherQueue, Error, IAppWindow3, IAppWindowStatics, IAppWindowTitleBar2,
    IAppWindowTitleBar3, IAppWindowTitleBarStatics, IInputNonClientPointerSourceStatics,
    IReference, InputNonClientPointerSource, Interface, NonClientRegionKind, RectInt32,
    TitleBarHeightOption, TitleBarTheme, WindowId, imp,
};
use windows::Win32::Foundation::HWND;
use windows::core::{HRESULT, s};

use crate::app_window_caption::support::window_attach_txn;
use crate::app_window_caption::support::window_attach_txn::{
    AssociationState, Stage, StageOutcome, TransactionOutcome,
};
use crate::app_window_caption::support::window_lifetime::WindowLifetime;

use super::super::runtime::{activation_factory, framework_udk, proc_address, stage};
use super::super::{log, log_failed, os_hr, sdk_error};
use super::regions::{initial_passthrough_plan, set_region_rects};
use super::{
    AttachFailure, AttachProgress, CapturedTitleBar, TRANSPARENT_BUTTON_BACKGROUND, WindowSession,
    ensure_alive, stage_log_name,
};

/// Classifies one caption button background color's exact nullable original
/// from the raw ABI pair (HRESULT, out-pointer null-ness) of the generated
/// getter. The generated getters return `Color` and collapse a successful
/// null — the unset default — into an error, so absence must never be read
/// through them: a failing HRESULT stays a failed capture (never an inferred
/// `None`), a successful null is `None`, and a successful reference is
/// adopted as an owned `IReference<Color>` whose value is extracted before
/// the reference is released at scope end. A failing HRESULT never touches
/// the out pointer's ownership — the ABI contract leaves nothing to release
/// and the SDK error is classified, never swallowed. The identity token is
/// revalidated before the value read and after the release; on the non-null
/// path the transferred reference is adopted into the owning value FIRST —
/// `Type::from_abi` is a local move that takes the reference over without
/// an AddRef — so a lifetime retired during the getter returns Dead with
/// the reference released exactly once at scope end instead of leaked.
fn capture_optional_button_color(
    lifetime: &Rc<WindowLifetime>,
    progress: &mut AttachProgress,
    stage_name: &'static str,
    raw_get: impl FnOnce(*mut *mut core::ffi::c_void) -> Result<(), Error>,
) -> Result<Option<Color>, AttachFailure> {
    let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
    raw_get(&mut out).map_err(|e| fail_progress(progress, stage_name, sdk_error(&e)))?;
    if out.is_null() {
        ensure_alive(lifetime)?; // the getter is a boundary; nothing to adopt
        return Ok(None);
    }
    // SAFETY: the successful getter transferred one non-null IReference<Color>
    // reference to its caller. `Type::from_abi` adopts that reference as-is
    // (no AddRef), so this adoption must precede every liveness-failure
    // return: from here on, each path drops the owning value exactly once.
    let reference: IReference<Color> = unsafe { imp::Type::from_abi(out) }
        .map_err(|e| fail_progress(progress, stage_name, sdk_error(&e)))?;
    ensure_alive(lifetime)?; // the getter is a boundary; the adoption was a local move
    let value = reference
        .Value()
        .map_err(|e| fail_progress(progress, stage_name, sdk_error(&e)))?;
    drop(reference);
    ensure_alive(lifetime)?; // the reference's Release is a reentry boundary
    Ok(Some(value))
}

/// Packs a failed stage with the progress so far; the caller's `progress` is
/// left empty because the failure returns immediately.
fn fail_progress(
    progress: &mut AttachProgress,
    stage_name: &str,
    message: String,
) -> AttachFailure {
    AttachFailure::Failed(stage(stage_name, &message), std::mem::take(progress))
}

pub(super) fn build_session(
    raw_hwnd: isize,
    lifetime: &Rc<WindowLifetime>,
) -> Result<WindowSession, AttachFailure> {
    let mut progress = AttachProgress::new();

    // Locally owned factories; each is released at the end of this function,
    // while the host runtime stays ready for other windows. Every boundary
    // below — factory acquisition, interop call, QI, getter, setter, AddRef
    // and each later drop of an owned interface — is revalidated with a
    // plain token read before the next live-HWND action; a retired identity
    // ends the build through [`AttachFailure::Dead`] and nothing is restored
    // or published for it.
    let app_window: AppWindow = {
        let statics: IAppWindowStatics = activation_factory("Microsoft.UI.Windowing.AppWindow")
            .map_err(|m| fail_progress(&mut progress, "appwindow-factory", m))?;
        ensure_alive(lifetime)?;
        let window_id = window_id_from_hwnd(raw_hwnd)
            .map_err(|m| fail_progress(&mut progress, "interop", m))?;
        ensure_alive(lifetime)?;
        let mut out = core::ptr::null_mut();
        unsafe {
            (Interface::vtable(&statics).GetFromWindowId)(
                Interface::as_raw(&statics),
                window_id,
                &mut out,
            )
            .and_then(|| imp::Type::from_abi(out))
        }
        .map_err(|e| fail_progress(&mut progress, "appwindow-get", sdk_error(&e)))?
    };
    ensure_alive(lifetime)?; // the factory above is released at the block end

    let title_bar = app_window
        .TitleBar()
        .map_err(|e| fail_progress(&mut progress, "titlebar-get", sdk_error(&e)))?;
    ensure_alive(lifetime)?;
    progress.title_bar = Some(title_bar.clone());
    ensure_alive(lifetime)?; // the clone's AddRef is a reentry boundary

    let supported = {
        let statics: IAppWindowTitleBarStatics =
            activation_factory("Microsoft.UI.Windowing.AppWindowTitleBar")
                .map_err(|m| fail_progress(&mut progress, "titlebar-factory", m))?;
        ensure_alive(lifetime)?;
        let mut out = false;
        unsafe {
            (Interface::vtable(&statics).IsCustomizationSupported)(
                Interface::as_raw(&statics),
                &mut out,
            )
        }
        .map(|| out)
        .map_err(|e| fail_progress(&mut progress, "titlebar-customization", sdk_error(&e)))?
    };
    ensure_alive(lifetime)?; // call boundary and factory release
    if !supported {
        return Err(fail_progress(
            &mut progress,
            "titlebar-customization",
            "AppWindowTitleBar.IsCustomizationSupported() returned false".to_string(),
        ));
    }

    // Capture the exact pre-mutation values through the SDK's own getters, so
    // any unwind re-applies what was really there — never a blind default.
    // Each capture is its own scoped block: the QI'd interface view releases
    // at the block end and every boundary (QI, getter, Release) is
    // revalidated with a plain token read before the next SDK call — no
    // grouped answer covers calls within the group.
    let extends_content = {
        let extends_content = title_bar
            .ExtendsContentIntoTitleBar()
            .map_err(|e| fail_progress(&mut progress, "capture-extend", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the getter is a boundary
        extends_content
    };
    let height_option = {
        let title_bar2: IAppWindowTitleBar2 = title_bar
            .cast()
            .map_err(|e| fail_progress(&mut progress, "capture-height-cast", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the capture QI is a boundary
        let height_option = title_bar2
            .PreferredHeightOption()
            .map_err(|e| fail_progress(&mut progress, "capture-height", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the getter is a boundary; the QI'd view releases at block end
        height_option
    };
    ensure_alive(lifetime)?; // the height interface released at the block end
    let theme = {
        let title_bar3: IAppWindowTitleBar3 = title_bar
            .cast()
            .map_err(|e| fail_progress(&mut progress, "capture-theme-cast", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the capture QI is a boundary
        let theme = title_bar3
            .PreferredTheme()
            .map_err(|e| fail_progress(&mut progress, "capture-theme", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the getter is a boundary; the QI'd view releases at block end
        theme
    };
    ensure_alive(lifetime)?; // the theme interface released at the block end
    // The two button background originals are captured through the raw
    // nullable classification (the generated getters cannot express
    // absence), each its own helper call so one failure never covers the
    // other.
    let button_background = capture_optional_button_color(
        lifetime,
        &mut progress,
        "capture-button-background",
        |out| unsafe {
            (Interface::vtable(&title_bar).ButtonBackgroundColor)(
                Interface::as_raw(&title_bar),
                out,
            )
            .ok()
        },
    )?;
    let button_inactive_background = capture_optional_button_color(
        lifetime,
        &mut progress,
        "capture-button-inactive-background",
        |out| unsafe {
            (Interface::vtable(&title_bar).ButtonInactiveBackgroundColor)(
                Interface::as_raw(&title_bar),
                out,
            )
            .ok()
        },
    )?;
    progress.captured = Some(CapturedTitleBar {
        extends_content,
        height_option,
        theme,
        button_background,
        button_inactive_background,
        caption_regions: Vec::new(),
        passthrough_regions: Vec::new(),
    });
    log(
        "capture",
        &format!(
            "extend={extends_content} height_option={height_option:?} theme={theme:?} \
             button_background={button_background:?} \
             button_inactive_background={button_inactive_background:?}"
        ),
    );

    // The title-bar stages run through the shared transaction runner: each
    // stage is journaled as attempted BEFORE its setter (an HRESULT failure
    // never implies "no native effect"), the identity is revalidated before
    // and after every stage, and each stage's own QI → setter gap carries its
    // own plain token read.
    let title_bar_stages = window_attach_txn::run_stages(
        &[
            Stage::ExtendContent,
            Stage::Height,
            Stage::Theme,
            Stage::ButtonBackground,
            Stage::ButtonInactiveBackground,
        ],
        &mut progress.journal,
        || lifetime.is_alive(),
        |stage| match stage {
            Stage::ExtendContent => match title_bar.SetExtendsContentIntoTitleBar(true) {
                Ok(()) => StageOutcome::Applied,
                Err(e) => StageOutcome::Failed(sdk_error(&e)),
            },
            Stage::Height => match title_bar.cast::<IAppWindowTitleBar2>() {
                Ok(title_bar2) => {
                    if !lifetime.is_alive() {
                        // retired between the QI and the setter
                        return StageOutcome::Retired;
                    }
                    match title_bar2.SetPreferredHeightOption(TitleBarHeightOption::Standard) {
                        Ok(()) => StageOutcome::Applied,
                        Err(e) => StageOutcome::Failed(sdk_error(&e)),
                    }
                }
                Err(e) => StageOutcome::Failed(format!("cast: {}", sdk_error(&e))),
            },
            Stage::Theme => match title_bar.cast::<IAppWindowTitleBar3>() {
                Ok(title_bar3) => {
                    if !lifetime.is_alive() {
                        // retired between the QI and the setter
                        return StageOutcome::Retired;
                    }
                    match title_bar3.SetPreferredTheme(TitleBarTheme::Dark) {
                        Ok(()) => StageOutcome::Applied,
                        Err(e) => StageOutcome::Failed(sdk_error(&e)),
                    }
                }
                Err(e) => StageOutcome::Failed(format!("cast: {}", sdk_error(&e))),
            },
            Stage::ButtonBackground => {
                // The documented transparent background lets Snafu's own
                // title area show through the caption buttons while the
                // native glyph, hover, pressed and Close behavior stay.
                match title_bar.SetButtonBackgroundColor(Some(TRANSPARENT_BUTTON_BACKGROUND)) {
                    Ok(()) => StageOutcome::Applied,
                    Err(e) => StageOutcome::Failed(sdk_error(&e)),
                }
            }
            Stage::ButtonInactiveBackground => {
                match title_bar
                    .SetButtonInactiveBackgroundColor(Some(TRANSPARENT_BUTTON_BACKGROUND))
                {
                    Ok(()) => StageOutcome::Applied,
                    Err(e) => StageOutcome::Failed(sdk_error(&e)),
                }
            }
            // The region stages run in their own later pass, after the input
            // source is owned and the association gate passed.
            Stage::Regions | Stage::PassthroughRegions => {
                StageOutcome::Failed("regions run in a later pass".to_string())
            }
        },
    );
    match title_bar_stages {
        TransactionOutcome::Complete => {}
        TransactionOutcome::Retired => return Err(AttachFailure::Dead),
        TransactionOutcome::Failed { stage, error } => {
            return Err(fail_progress(&mut progress, stage_log_name(stage), error));
        }
    }

    // Metrics are read per getter with a token read between each — the three
    // reads are never covered by one grouped check.
    let height = {
        let height = title_bar
            .Height()
            .map_err(|e| fail_progress(&mut progress, "titlebar-height-read", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the getter is a boundary
        height
    };
    let left_inset = {
        let left_inset = title_bar
            .LeftInset()
            .map_err(|e| fail_progress(&mut progress, "titlebar-insets-read", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the getter is a boundary
        left_inset
    };
    let right_inset = {
        let right_inset = title_bar
            .RightInset()
            .map_err(|e| fail_progress(&mut progress, "titlebar-insets-read", sdk_error(&e)))?;
        ensure_alive(lifetime)?; // the getter is a boundary
        right_inset
    };
    progress.caption_height = Some(height);
    progress.left_inset = Some(left_inset);
    progress.right_inset = Some(right_inset);
    log(
        "titlebar-geometry",
        &format!("height={height} left_inset={left_inset} right_inset={right_inset}"),
    );

    let input_source: InputNonClientPointerSource = {
        let statics: IInputNonClientPointerSourceStatics =
            activation_factory("Microsoft.UI.Input.InputNonClientPointerSource")
                .map_err(|m| fail_progress(&mut progress, "input-factory", m))?;
        ensure_alive(lifetime)?;
        let window_id = window_id_from_hwnd(raw_hwnd)
            .map_err(|m| fail_progress(&mut progress, "interop", m))?;
        ensure_alive(lifetime)?;
        let mut out = core::ptr::null_mut();
        unsafe {
            (Interface::vtable(&statics).GetForWindowId)(
                Interface::as_raw(&statics),
                window_id,
                &mut out,
            )
            .and_then(|| imp::Type::from_abi(out))
        }
        .map_err(|e| fail_progress(&mut progress, "input-source-get", sdk_error(&e)))?
    };
    ensure_alive(lifetime)?; // the input factory above is released at the block end
    // Own the input source in the progress immediately: the region stage
    // below mutates through it, so every later failure must be able to
    // restore or release it — never None while a region mutation can exist.
    progress.input_source = Some(input_source.clone());
    ensure_alive(lifetime)?; // the clone's AddRef is a reentry boundary

    // Association gate: the AppWindow must NOT be associated with any
    // DispatcherQueue — this route never calls AssociateWithDispatcherQueue.
    // The gate fails CLOSED: a failed IAppWindow3 QI, a failing accessor
    // HRESULT and a non-null association are all blockers; only a success
    // HRESULT with a null out passes. The raw vtable pair (HRESULT, out) is
    // classified by window_attach_txn::classify_association. Adopt any
    // successfully returned reference before a lifetime check can return,
    // so every rejection/retirement path releases that owned reference.
    match app_window.cast::<IAppWindow3>() {
        Ok(app_window3) => {
            ensure_alive(lifetime)?; // the association QI is a boundary
            let mut out: *mut core::ffi::c_void = core::ptr::null_mut();
            let hr = unsafe {
                (Interface::vtable(&app_window3).DispatcherQueue)(
                    Interface::as_raw(&app_window3),
                    &mut out,
                )
            };
            let association = window_attach_txn::classify_association(hr.0, out.is_null());
            let queue = if hr.is_ok() && !out.is_null() {
                // SAFETY: the successful typed SDK accessor transfers one
                // non-null DispatcherQueue reference to its caller.
                Some(unsafe { DispatcherQueue::from_raw(out) })
            } else {
                None
            };
            ensure_alive(lifetime)?; // the accessor call is a boundary
            drop(queue);
            ensure_alive(lifetime)?; // releasing the returned queue can reenter
            match association {
                AssociationState::Unassociated => log(
                    "appwindow-association",
                    "verified unassociated before the Caption mutation",
                ),
                AssociationState::Associated => {
                    return Err(fail_progress(
                        &mut progress,
                        "appwindow-association",
                        "AppWindow unexpectedly reports a DispatcherQueue association".to_string(),
                    ));
                }
                AssociationState::CheckFailed => {
                    return Err(fail_progress(
                        &mut progress,
                        "appwindow-association",
                        format!("accessor HRESULT=0x{:08X}", hr.0),
                    ));
                }
            }
        }
        Err(e) => {
            // Fail closed: without the typed accessor the association cannot
            // be proven either way, and the attach must not proceed.
            return Err(fail_progress(
                &mut progress,
                "appwindow-association",
                format!("IAppWindow3 unavailable: {}", sdk_error(&e)),
            ));
        }
    }

    ensure_alive(lifetime)?; // the association interface released at the match end

    // Own the AppWindow in the progress from here on: the association gate
    // above was its last direct use, and from the first region stage onward
    // every retained form must hold the same interface set a session taken
    // back through [`AttachProgress::from_session`] would. The move
    // transfers the reference — no clone, no AddRef, no SDK call.
    progress.app_window = Some(app_window);

    // Capture the exact original Caption region set BEFORE the first Caption
    // write, so the rollback re-applies precisely what was there. A capture
    // failure stops BEFORE the mutation — it never assumes empty originals.
    let original_regions = {
        let Some(input_source_ref) = progress.input_source.as_ref() else {
            return Err(fail_progress(
                &mut progress,
                "caption-region-capture",
                "input source interface missing".to_string(),
            ));
        };
        let array = input_source_ref
            .GetRegionRects(NonClientRegionKind::Caption)
            .map_err(|e| fail_progress(&mut progress, "caption-region-capture", sdk_error(&e)))?;
        let regions: Vec<RectInt32> = array.to_vec();
        ensure_alive(lifetime)?; // the getter is a boundary; the native array frees at block end
        regions
    };
    log(
        "caption-region-capture",
        &format!("{} original Caption rects", original_regions.len()),
    );
    if let Some(captured) = progress.captured.as_mut() {
        captured.caption_regions = original_regions;
    }

    // The second kind's original is captured the same way, BEFORE its first
    // write, so a rollback re-applies exactly what was there for each kind
    // independently — one capture never covers the other.
    let original_passthrough = {
        let Some(input_source_ref) = progress.input_source.as_ref() else {
            return Err(fail_progress(
                &mut progress,
                "passthrough-region-capture",
                "input source interface missing".to_string(),
            ));
        };
        let array = input_source_ref
            .GetRegionRects(NonClientRegionKind::Passthrough)
            .map_err(|e| {
                fail_progress(&mut progress, "passthrough-region-capture", sdk_error(&e))
            })?;
        let regions: Vec<RectInt32> = array.to_vec();
        ensure_alive(lifetime)?; // the getter is a boundary; the native array frees at block end
        regions
    };
    log(
        "passthrough-region-capture",
        &format!("{} original Passthrough rects", original_passthrough.len()),
    );
    if let Some(captured) = progress.captured.as_mut() {
        captured.passthrough_regions = original_passthrough;
    }

    // The region stages run through the same transaction runner, journaled
    // as attempted before each setter, identity revalidated around them.
    // The INITIAL publication is deliberately the suspension plan: the frame
    // that will define the app's drag geometry has not been drawn under the
    // attached SDK yet (its layout payload is stale by the revision bump the
    // caller made), so no app Caption is published — the nonreserved band
    // routes as client — until a current frame arrives through ApplyRegions.
    let input_source_ref = progress.input_source.as_ref();
    let hwnd = HWND(raw_hwnd as *mut _);
    let region_stages = window_attach_txn::run_stages(
        &[Stage::Regions, Stage::PassthroughRegions],
        &mut progress.journal,
        || lifetime.is_alive(),
        |stage| {
            let Some(input_source) = input_source_ref else {
                return StageOutcome::Failed("input source interface missing".to_string());
            };
            if !lifetime.is_alive() {
                // retired between the journaling and the setter
                return StageOutcome::Retired;
            }
            let (kind, rects) = match stage {
                Stage::Regions => (NonClientRegionKind::Caption, Vec::new()),
                Stage::PassthroughRegions => (
                    NonClientRegionKind::Passthrough,
                    initial_passthrough_plan(hwnd),
                ),
                // The title-bar stages ran in the earlier pass.
                other => return StageOutcome::Failed(format!("unexpected stage {other:?}")),
            };
            match set_region_rects(input_source, kind, &rects) {
                Ok(()) => StageOutcome::Applied,
                Err(message) => StageOutcome::Failed(message),
            }
        },
    );
    match region_stages {
        TransactionOutcome::Complete => {}
        TransactionOutcome::Retired => return Err(AttachFailure::Dead),
        TransactionOutcome::Failed { stage, error } => {
            return Err(fail_progress(&mut progress, stage_log_name(stage), error));
        }
    }
    log(
        "attach",
        "suspension regions applied (no app caption before the first current frame); \
         session complete",
    );
    // Last reentry checkpoint: SetRegionRects ran, and every interface not
    // owned by the progress releases at this function's end before the
    // caller may publish ACTIVE.
    ensure_alive(lifetime)?;

    Ok(progress.into_session(lifetime))
}

/// Resolves the HWND through the official FrameworkUdk helper
/// (`Windowing_GetWindowIdFromWindow`), never by casting the handle.
fn window_id_from_hwnd(raw_hwnd: isize) -> Result<WindowId, String> {
    let module = framework_udk().ok_or_else(|| {
        let detail = "FrameworkUdk module unavailable".to_string();
        log_failed("framework-udk-load", &detail);
        detail
    })?;
    let pfn: unsafe extern "system" fn(isize, *mut WindowId) -> HRESULT = unsafe {
        proc_address(module, s!("Windowing_GetWindowIdFromWindow")).map(|p| core::mem::transmute(p))
    }
    .ok_or_else(|| {
        let detail = "Windowing_GetWindowIdFromWindow export missing".to_string();
        log_failed("framework-udk-proc", &detail);
        detail
    })?;
    let mut window_id = WindowId::default();
    let hr = unsafe { pfn(raw_hwnd, &mut window_id) };
    if hr.is_err() {
        let detail = os_hr(hr);
        log_failed("window-id-from-hwnd", &detail);
        return Err(detail);
    }
    Ok(window_id)
}

/// Ownership tests for [`capture_optional_button_color`]. The `raw_get`
/// seam alone cannot observe COM ownership, so these run against a narrow
/// counted COM fake: a hand-built `IReference<Color>` object whose vtable
/// matches the generated `IReference<Color>` layout (IUnknown, then
/// IInspectable, then `Value`) and counts every AddRef, Release and Value
/// call. Windows-only like the whole module, and compile-proven only off
/// Windows: nothing here claims anything about native appearance.
#[cfg(test)]
mod tests {
    use super::*;
    use appwindow_caption_bindings::GUID;
    use appwindow_caption_bindings::HRESULT as AbiHresult;
    use std::cell::Cell;

    const NOT_IMPL: AbiHresult = AbiHresult(0x8000_4001u32 as i32);
    const FAKE_COLOR: Color = Color {
        a: 0xAB,
        r: 0x11,
        g: 0x22,
        b: 0x33,
    };

    /// Counters shared between the fake COM object and its test.
    #[derive(Default)]
    struct FakeCounters {
        add_refs: Cell<u32>,
        release_calls: Cell<u32>,
        value_calls: Cell<u32>,
    }

    #[repr(C)]
    struct FakeReferenceVtbl {
        query_interface: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *const GUID,
            *mut *mut core::ffi::c_void,
        ) -> AbiHresult,
        add_ref: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
        release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
        get_iids: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut u32,
            *mut *mut GUID,
        ) -> AbiHresult,
        get_runtime_class_name: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> AbiHresult,
        get_trust_level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> AbiHresult,
        value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> AbiHresult,
    }

    /// A refcounted fake `IReference<Color>`: the vtable pointer is the first
    /// field, matching the generated interface's object layout, and every
    /// AddRef, Release and Value call is reported through `counters`.
    #[repr(C)]
    struct FakeReference {
        vtable: &'static FakeReferenceVtbl,
        count: Cell<u32>,
        counters: Rc<FakeCounters>,
    }

    unsafe extern "system" fn fake_query_interface(
        _: *mut core::ffi::c_void,
        _: *const GUID,
        _: *mut *mut core::ffi::c_void,
    ) -> AbiHresult {
        NOT_IMPL // never called by the helper
    }

    unsafe extern "system" fn fake_get_iids(
        _: *mut core::ffi::c_void,
        _: *mut u32,
        _: *mut *mut GUID,
    ) -> AbiHresult {
        NOT_IMPL // never called by the helper
    }

    unsafe extern "system" fn fake_get_runtime_class_name(
        _: *mut core::ffi::c_void,
        _: *mut *mut core::ffi::c_void,
    ) -> AbiHresult {
        NOT_IMPL // never called by the helper
    }

    unsafe extern "system" fn fake_get_trust_level(
        _: *mut core::ffi::c_void,
        _: *mut i32,
    ) -> AbiHresult {
        NOT_IMPL // never called by the helper
    }

    unsafe extern "system" fn fake_add_ref(this: *mut core::ffi::c_void) -> u32 {
        let object = unsafe { &*(this as *const FakeReference) };
        let count = object.count.get() + 1;
        object.count.set(count);
        object
            .counters
            .add_refs
            .set(object.counters.add_refs.get() + 1);
        count
    }

    unsafe extern "system" fn fake_release(this: *mut core::ffi::c_void) -> u32 {
        let object = unsafe { &*(this as *const FakeReference) };
        object
            .counters
            .release_calls
            .set(object.counters.release_calls.get() + 1);
        // Overflow checks are on in tests: a Release below zero fails loudly.
        let remaining = object.count.get() - 1;
        object.count.set(remaining);
        if remaining == 0 {
            drop(unsafe { Box::from_raw(this as *mut FakeReference) });
        }
        remaining
    }

    unsafe extern "system" fn fake_value(
        this: *mut core::ffi::c_void,
        result: *mut Color,
    ) -> AbiHresult {
        let object = unsafe { &*(this as *const FakeReference) };
        object
            .counters
            .value_calls
            .set(object.counters.value_calls.get() + 1);
        unsafe { result.write(FAKE_COLOR) };
        AbiHresult(0)
    }

    static FAKE_REFERENCE_VTABLE: FakeReferenceVtbl = FakeReferenceVtbl {
        query_interface: fake_query_interface,
        add_ref: fake_add_ref,
        release: fake_release,
        get_iids: fake_get_iids,
        get_runtime_class_name: fake_get_runtime_class_name,
        get_trust_level: fake_get_trust_level,
        value: fake_value,
    };

    /// Creates the counted fake with its initial +1 reference and returns the
    /// counters with the raw object pointer to transfer through `raw_get`'s
    /// out parameter.
    fn fake_reference() -> (Rc<FakeCounters>, *mut core::ffi::c_void) {
        let counters = Rc::new(FakeCounters::default());
        let object = Box::into_raw(Box::new(FakeReference {
            vtable: &FAKE_REFERENCE_VTABLE,
            count: Cell::new(1),
            counters: Rc::clone(&counters),
        }));
        (counters, object.cast())
    }

    /// The required regression: the getter succeeds with an owned non-null
    /// reference and the lifetime retires before the helper returns. The
    /// transferred reference must be released exactly once — adopted, never
    /// AddRef'd, never read through `Value`, never leaked — and classified
    /// Dead.
    #[test]
    fn retired_getter_adopts_then_releases_once() {
        let lifetime = Rc::new(WindowLifetime::allocate());
        let (counters, object) = fake_reference();
        let result = capture_optional_button_color(
            &lifetime,
            &mut AttachProgress::new(),
            "capture-button-background",
            |out| {
                lifetime.retire();
                unsafe { *out = object };
                Ok(())
            },
        );
        assert!(matches!(result, Err(AttachFailure::Dead)));
        assert_eq!(counters.add_refs.get(), 0);
        assert_eq!(counters.release_calls.get(), 1);
        assert_eq!(counters.value_calls.get(), 0);
    }

    /// Live success: the adopted reference's value is read and then the
    /// reference is released exactly once.
    #[test]
    fn live_getter_reads_value_then_releases_once() {
        let lifetime = Rc::new(WindowLifetime::allocate());
        let (counters, object) = fake_reference();
        let result = capture_optional_button_color(
            &lifetime,
            &mut AttachProgress::new(),
            "capture-button-background",
            |out| {
                unsafe { *out = object };
                Ok(())
            },
        );
        assert!(matches!(result, Ok(Some(color)) if color == FAKE_COLOR));
        assert_eq!(counters.add_refs.get(), 0);
        assert_eq!(counters.release_calls.get(), 1);
        assert_eq!(counters.value_calls.get(), 1);
    }

    /// A successful null out pointer is absence — `None`, no reference
    /// created, no liveness inference.
    #[test]
    fn successful_null_is_none() {
        let lifetime = Rc::new(WindowLifetime::allocate());
        let result = capture_optional_button_color(
            &lifetime,
            &mut AttachProgress::new(),
            "capture-button-background",
            |out| {
                unsafe { *out = core::ptr::null_mut() };
                Ok(())
            },
        );
        assert!(matches!(result, Ok(None)));
    }

    /// A successful null with a lifetime retired during the getter stays
    /// Dead — never reclassified into a value or absence answer.
    #[test]
    fn successful_null_with_retired_lifetime_is_dead() {
        let lifetime = Rc::new(WindowLifetime::allocate());
        let result = capture_optional_button_color(
            &lifetime,
            &mut AttachProgress::new(),
            "capture-button-background",
            |out| {
                lifetime.retire();
                unsafe { *out = core::ptr::null_mut() };
                Ok(())
            },
        );
        assert!(matches!(result, Err(AttachFailure::Dead)));
    }

    /// A failing HRESULT stays a failed capture naming its stage — never an
    /// inferred `None`, never Dead — and the out parameter's ownership is
    /// untouched (nothing to release; no fake was created).
    #[test]
    fn hresult_failure_is_failed_capture() {
        let lifetime = Rc::new(WindowLifetime::allocate());
        let mut progress = AttachProgress::new();
        let result = capture_optional_button_color(
            &lifetime,
            &mut progress,
            "capture-button-background",
            |_| Err(Error::empty()),
        );
        match result {
            Err(AttachFailure::Failed(stage, _)) => {
                assert!(stage.starts_with("capture-button-background:"));
            }
            _ => panic!("expected a failed capture"),
        }
    }
}
