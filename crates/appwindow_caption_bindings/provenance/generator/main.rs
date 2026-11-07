//! Generates the private AppWindow caption bindings from pinned official Windows App SDK
//! 2.x metadata. The recipe is reproducible without hardcoded scratch paths:
//!
//!   WASDK_IE_METADATA   InteractiveExperiences versioned metadata dir, e.g.
//!                       <extracted 2.1.9>/metadata/10.0.17763.0 (default assumes /tmp/wasdk)
//!   WASDK_CONTRACTS_REF Windows SDK Contracts `ref/netstandard2.0` dir supplying the
//!                       Windows.* contract types the Microsoft.* winmds reference
//!   BINDINGS_OUT        output .rs path (default: bindings.rs beside this crate)
//!
//! Filter style is deliberate: exact type names only. Whole-namespace entries make
//! windows-bindgen take its top-down path, which emits name-only shells for types the
//! closure does not fully resolve; exact entries use TypeClosure and emit complete,
//! typed definitions. Regenerate; never hand-edit the output.
use std::path::Path;

fn main() {
    let ie_metadata = std::env::var("WASDK_IE_METADATA").unwrap_or_else(|_| {
        "/tmp/wasdk/microsoft.windowsappsdk.interactiveexperiences.2.1.9/metadata/10.0.17763.0"
            .into()
    });
    let contracts_ref = std::env::var("WASDK_CONTRACTS_REF")
        .unwrap_or_else(|_| "/tmp/wasdk/contracts/ref/netstandard2.0".into());
    let out = std::env::var("BINDINGS_OUT").unwrap_or_else(|_| {
        Path::new(file!())
            .parent()
            .map(|p| p.join("bindings.rs"))
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "bindings.rs".into())
    });
    windows_bindgen::builder()
        .input(&ie_metadata)
        .input(&contracts_ref)
        .output(&out)
        .flat()
        .minimal()
        .filters([
            // Microsoft.UI value types
            "Microsoft.UI.WindowId",
            // Windowing: classes, their instance interfaces, statics, and the Changed args
            "Microsoft.UI.Windowing.AppWindow",
            "Microsoft.UI.Windowing.IAppWindow",
            "Microsoft.UI.Windowing.IAppWindow2",
            "Microsoft.UI.Windowing.IAppWindow3",
            "Microsoft.UI.Windowing.IAppWindowStatics",
            "Microsoft.UI.Windowing.AppWindowChangedEventArgs",
            "Microsoft.UI.Windowing.AppWindowTitleBar",
            "Microsoft.UI.Windowing.IAppWindowTitleBar",
            "Microsoft.UI.Windowing.IAppWindowTitleBar2",
            "Microsoft.UI.Windowing.IAppWindowTitleBar3",
            "Microsoft.UI.Windowing.IAppWindowTitleBarStatics",
            // Input: the non-client source used for Caption/Passthrough regions
            "Microsoft.UI.Input.InputNonClientPointerSource",
            "Microsoft.UI.Input.IInputNonClientPointerSource",
            "Microsoft.UI.Input.IInputNonClientPointerSourceStatics",
            // The input object's queue accessor: association verification only,
            // never an ownership claim (null contracts stay at the raw boundary).
            "Microsoft.UI.Input.IInputObject",
            "Microsoft.UI.Input.NonClientRegionKind",
            // Dispatching: queue/controller used on the GPUI foreground thread
            "Microsoft.UI.Dispatching.DispatcherQueue",
            "Microsoft.UI.Dispatching.IDispatcherQueue",
            "Microsoft.UI.Dispatching.DispatcherQueueController",
            "Microsoft.UI.Dispatching.IDispatcherQueueController",
            // Synchronous ShutdownQueue: the loop owner calls it once after the
            // pump exits; the async variant is never used.
            "Microsoft.UI.Dispatching.IDispatcherQueueController2",
            "Microsoft.UI.Dispatching.IDispatcherQueueControllerStatics",
            "Microsoft.UI.Dispatching.IDispatcherQueueStatics",
            // Windows.* types referenced by the projected surface
            "Windows.Graphics.RectInt32",
            "Windows.UI.Color",
            "Windows.Foundation.Rect",
            "Windows.Foundation.TimeSpan",
        ])
        .write();
    println!("wrote {out}");
}
