//! Windows App SDK runtime ownership for the AppWindow caption host: Bootstrap
//! initialization, STA WinRT initialization, the owned `DispatcherQueueController`
//! and the `ContentPreTranslateMessage` export. The host in the module root decides
//! when each stage runs; this module owns how it is acquired and balanced.

use appwindow_caption_bindings::{
    DispatcherQueue, DispatcherQueueController, Error, GUID, IDispatcherQueueController2,
    IDispatcherQueueControllerStatics, IDispatcherQueueStatics, Interface, imp,
};
use windows::Win32::Foundation::{GetLastError, HMODULE};
use windows::Win32::System::LibraryLoader::{
    GetModuleFileNameW, GetModuleHandleW, GetProcAddress, LoadLibraryW,
};
use windows::Win32::System::WinRT::RO_INIT_SINGLETHREADED;
use windows::core::{HRESULT, HSTRING, PCSTR, PCWSTR, s, w};

use super::{CaptionHost, log, log_failed, os_hr, sdk_error};

// Release-phase constant from the pinned WindowsAppSDK-VersionInfo.h: 2.5.
const BOOTSTRAP_MAJOR_MINOR: u32 = 0x0002_0005;
// Minimum package version 2.5.1.0, encoded as the documented PACKAGE_VERSION union.
const BOOTSTRAP_MIN_VERSION: u64 = 0x0002_0005_0001_0000;

const BOOTSTRAP_DLL: PCWSTR = w!("Microsoft.WindowsAppRuntime.Bootstrap.dll");
const FRAMEWORK_UDK_DLL: PCWSTR = w!("Microsoft.Internal.FrameworkUdk.dll");
// Hosts the input pretranslation export per the pinned InterOp evidence; the
// WindowId helper above stays on FrameworkUdk per the official interop header.
const WINDOWING_CORE_DLL: PCWSTR = w!("Microsoft.UI.Windowing.Core.dll");

// `RPC_E_CHANGED_MODE`, written as an explicit unsigned literal converted to
// the HRESULT representation (`0x80010106`).
const RPC_E_CHANGED_MODE: i32 = 0x8001_0106u32 as i32;
const S_OK: i32 = 0;
const S_FALSE: i32 = 1;

// Raw WinRT initialization/activation entry points. `RoInitialize` is bound
// directly (rather than through the `windows` crate mapping) because the
// success code distinguishes `S_OK` from `S_FALSE` ("already initialized")
// and both must be balanced; `windows` collapses both into `Ok(())`. Per the
// official SDK mapping these symbols live in the WinRT api set whose import
// library is `ole32.lib`; the STA constant itself comes from the official
// `windows` crate (`RO_INIT_SINGLETHREADED = 0`, roapi.h), never a manual
// number.
#[link(name = "ole32")]
unsafe extern "system" {
    fn RoInitialize(grfinitflags: u32) -> HRESULT;
    fn RoGetActivationFactory(
        activatableclassid: *mut core::ffi::c_void,
        iid: *const FactoryGuid,
        factory: *mut *mut core::ffi::c_void,
    ) -> HRESULT;
    fn RoUninitialize();
}

// Package identity query (appmodel.h). A process without package identity gets
// APPMODEL_ERROR_NO_PACKAGE; a packaged one asks for a buffer.
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: *mut u16) -> u32;
}
const APPMODEL_ERROR_NO_PACKAGE: u32 = 15700;

/// Whether this process runs with package identity (an installed MSIX). A
/// packaged Snafu takes the Windows App SDK runtime from its manifest's
/// framework dependency; the Bootstrap API is for unpackaged runs only.
fn has_package_identity() -> bool {
    let mut length = 0u32;
    // SAFETY: a zero length with a null buffer only asks for the required size.
    let status = unsafe { GetCurrentPackageFullName(&mut length, core::ptr::null_mut()) };
    status != APPMODEL_ERROR_NO_PACKAGE
}

/// `RoGetActivationFactory`'s ABI GUID. Kept as a local plain-data struct so
/// the 0.100 `GUID` value crosses the boundary field by field — no interface
/// or string object ever crosses between windows-core versions.
#[repr(C)]
struct FactoryGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

fn factory_guid(iid: &GUID) -> FactoryGuid {
    FactoryGuid {
        data1: iid.data1,
        data2: iid.data2,
        data3: iid.data3,
        data4: iid.data4,
    }
}

/// Acquires a locally owned activation factory for `class_id` through the raw
/// `RoGetActivationFactory` ABI, typed with the generated statics interface.
/// The returned object owns its reference and releases it on drop — unlike the
/// generated class-static accessors, whose `FactoryCache` forgets factories
/// for process lifetime with no clear operation.
pub(super) fn activation_factory<I: Interface + Clone>(class_id: &str) -> Result<I, String> {
    unsafe {
        let class_id = HSTRING::from(class_id);
        let iid = factory_guid(&I::IID);
        let mut raw: *mut core::ffi::c_void = core::ptr::null_mut();
        // The ABI takes the HSTRING *handle*, not a UTF-16 buffer pointer; the
        // generated bindings pass exactly this way (transmute_copy of the
        // repr(transparent) handle value, never a generic object transmute).
        let class_id_abi: *mut core::ffi::c_void = core::mem::transmute_copy(&class_id);
        let hr = RoGetActivationFactory(class_id_abi, &iid, &mut raw);
        if hr.is_err() {
            return Err(os_hr(hr));
        }
        // Typed, null-checked transfer of the returned reference through the
        // installed core 0.100 conversion: the raw reference moves into I, and
        // a null out with a success HRESULT is an explicit error — never an
        // unchecked generic transmute.
        <I as imp::Type<I>>::from_abi(raw)
            .map_err(|e| format!("null activation factory for {class_id}: {}", sdk_error(&e)))
    }
}

// ---------------------------------------------------------------------------
// Staged runtime: Bootstrap → STA WinRT → typed queue probe → owned
// controller → IDispatcherQueueController2 queried before lifecycle use.
// ---------------------------------------------------------------------------

/// Foreground-owned SDK runtime for the whole run. Both interfaces are kept
/// alive until the epilogue: `ShutdownQueue` runs on the controller2 view, and
/// the controller reference itself must outlive the queue it owns.
pub(super) struct RuntimeOwner {
    controller: DispatcherQueueController,
    _controller2: IDispatcherQueueController2,
    /// Whether this run initialized the Bootstrap API (unpackaged runs only),
    /// so the epilogue balances exactly what was initialized.
    bootstrapped: bool,
}

impl RuntimeOwner {
    /// Rundown phase one, run exactly once from the pump epilogue: the
    /// synchronous, documented SDK drain of the owned queue. Every reference
    /// stays alive; on failure the caller holds the runtime instead of
    /// releasing or unloading anything.
    pub(super) fn shutdown_queue(&self) -> Result<(), String> {
        if let Err(e) = self._controller2.ShutdownQueue() {
            return Err(format!(
                "IDispatcherQueueController2::ShutdownQueue HRESULT=0x{:08X}",
                e.code().0
            ));
        }
        log(
            "queue-shutdown",
            "IDispatcherQueueController2::ShutdownQueue completed; the SDK drained its queue",
        );
        Ok(())
    }

    /// Rundown phase two: release the controller/queue interface references
    /// WITHOUT touching the apartment. The Release calls themselves can
    /// enqueue callback work, so the host consumes that under the terminal
    /// phase before anything is balanced; Ro/Bootstrap balance happens only
    /// through the [`ReleasedRuntime`] ticket this returns.
    pub(super) fn release_interfaces(self) -> ReleasedRuntime {
        let Self {
            controller,
            _controller2,
            bootstrapped,
        } = self;
        drop(_controller2);
        drop(controller);
        ReleasedRuntime { bootstrapped }
    }
}

/// Token that the controller/queue interfaces are released and every
/// release-triggered command was consumed: only the Ro/Bootstrap balance
/// remains, and no path can call it while a controller is still alive.
pub(super) struct ReleasedRuntime {
    bootstrapped: bool,
}

impl ReleasedRuntime {
    /// The final balance, run only after close admission: both S_OK and
    /// S_FALSE initialize an owned reference that is balanced exactly here;
    /// the thread remains an STA (GPUI's OleInitialize), so a mode change
    /// cannot occur at this point.
    pub(super) fn balance_runtime(self) {
        unsafe { RoUninitialize() };
        if self.bootstrapped {
            bootstrap_shutdown();
        }
    }
}

pub(super) fn acquire_runtime() -> Result<RuntimeOwner, String> {
    // Staged guard for the PRE-QUEUE stages: every fallible stage is balanced
    // exactly as far as it succeeded, in reverse. No stage below creates the
    // queue, so this unwind never touches a live queue.
    let mut bootstrap_ok = false;
    let mut ro_ok = false;
    let result = (|| -> Result<DispatcherQueueController, String> {
        if has_package_identity() {
            log(
                "bootstrap-init",
                "packaged run: the manifest's framework dependency supplies the runtime",
            );
        } else {
            initialize_bootstrap()?;
            bootstrap_ok = true;
        }
        initialize_winrt()?;
        ro_ok = true;

        // Typed probe on the real interface: S_OK with a null queue is the
        // only "no queue yet" answer. An arbitrary HRESULT error is a
        // failure, and an existing foreign queue is refused explicitly —
        // never reused, never released late.
        let probe: Result<DispatcherQueue, Error> = {
            let statics: IDispatcherQueueStatics =
                activation_factory("Microsoft.UI.Dispatching.DispatcherQueue")
                    .map_err(|m| stage("queue-factory", &m))?;
            let mut out = core::ptr::null_mut();
            unsafe {
                (Interface::vtable(&statics).GetForCurrentThread)(
                    Interface::as_raw(&statics),
                    &mut out,
                )
            }
            // SAFETY: the successful typed SDK call transfers this interface
            // reference; a null out with S_OK maps to Err(S_OK) via from_abi —
            // exactly the documented "no queue for this thread" answer.
            .and_then(|| unsafe { imp::Type::from_abi(out) })
        };
        match probe {
            Ok(queue) => {
                drop(queue);
                return Err(
                    "unexpected foreign DispatcherQueue on the foreground thread; \
                     refusing the owned-queue lifecycle explicitly"
                        .to_string(),
                );
            }
            Err(e) if e.code().0 == S_OK => {}
            Err(e) => return Err(stage("queue-probe", &sdk_error(&e))),
        }

        let statics: IDispatcherQueueControllerStatics =
            activation_factory("Microsoft.UI.Dispatching.DispatcherQueueController")
                .map_err(|m| stage("queue-controller-factory", &m))?;
        let mut out = core::ptr::null_mut();
        let controller: DispatcherQueueController = unsafe {
            (Interface::vtable(&statics).CreateOnCurrentThread)(
                Interface::as_raw(&statics),
                &mut out,
            )
        }
        // SAFETY: the successful typed SDK call transfers this interface reference.
        .and_then(|| unsafe { imp::Type::from_abi(out) })
        .map_err(|e| stage("queue-create", &sdk_error(&e)))?;
        Ok(controller)
    })();
    let controller = match result {
        Ok(controller) => controller,
        Err(message) => {
            // Pre-queue partial-init unwind: balance exactly what succeeded,
            // in reverse. No queue exists, so this is an ordinary staged
            // failure and the native caption is unavailable for this run.
            if ro_ok {
                unsafe { RoUninitialize() };
                log("unwind", "RoUninitialize balanced after partial init");
            }
            if bootstrap_ok {
                bootstrap_shutdown();
            }
            return Err(message);
        }
    };
    log(
        "dispatcher-queue",
        "created owned DispatcherQueueController on the foreground thread",
    );

    // Post-queue stages cannot be unwound: the queue is live on this thread
    // from here on, and the only safe teardown is the queried synchronous
    // ShutdownQueue. A failed query therefore holds the queue — its references
    // are leaked deliberately, and no Ro/Bootstrap unload runs under it — and
    // the native caption stays unavailable for this run.
    let _controller2: IDispatcherQueueController2 = match controller.cast() {
        Ok(controller2) => controller2,
        Err(e) => {
            log_failed(
                "controller2-cast",
                &format!("{}; the created DispatcherQueue is held", sdk_error(&e)),
            );
            std::mem::forget(controller);
            return Err("IDispatcherQueueController2 cast failed; the queue is held".to_string());
        }
    };
    log(
        "dispatcher-queue",
        "IDispatcherQueueController2 queried before lifecycle use",
    );

    Ok(RuntimeOwner {
        controller,
        _controller2,
        bootstrapped: bootstrap_ok,
    })
}

/// Resolution state of `ContentPreTranslateMessage` (pinned interop header:
/// `BOOL __stdcall(const MSG*)`), resolved only from
/// `Microsoft.UI.Windowing.Core.dll` — never FrameworkUdk, never a silent
/// fallback. A missing export fails the attach before ACTIVE.
pub(super) enum PreTranslate {
    Unresolved,
    Absent,
    Resolved(*const core::ffi::c_void),
}

impl CaptionHost {
    /// Resolves the pretranslation contract BEFORE the first attach can
    /// publish ACTIVE. Missing export = failed attach, never a silent
    /// non-pretranslated run.
    pub(super) fn ensure_pretranslate(&mut self) -> Result<(), String> {
        match self.pretranslate {
            PreTranslate::Resolved(_) => Ok(()),
            PreTranslate::Absent => Err(
                "ContentPreTranslateMessage export missing from Microsoft.UI.Windowing.Core.dll; \
                 the SDK input pretranslation contract is absent"
                    .to_string(),
            ),
            PreTranslate::Unresolved => {
                let Some(module) = windowing_core_module() else {
                    self.pretranslate = PreTranslate::Absent;
                    log_failed(
                        "pretranslate",
                        "Microsoft.UI.Windowing.Core.dll could not be resolved",
                    );
                    return Err("Microsoft.UI.Windowing.Core.dll could not be resolved".to_string());
                };
                let resolved = proc_address(module, s!("ContentPreTranslateMessage"));
                match resolved {
                    Some(pfn) => {
                        log(
                            "pretranslate",
                            &format!(
                                "ContentPreTranslateMessage resolved from {}",
                                module_path(module)
                            ),
                        );
                        self.pretranslate = PreTranslate::Resolved(pfn);
                        Ok(())
                    }
                    None => {
                        self.pretranslate = PreTranslate::Absent;
                        log_failed(
                            "pretranslate",
                            "ContentPreTranslateMessage export missing from \
                             Microsoft.UI.Windowing.Core.dll; refusing to attach",
                        );
                        Err("ContentPreTranslateMessage export missing from \
                             Microsoft.UI.Windowing.Core.dll"
                            .to_string())
                    }
                }
            }
        }
    }
}

pub(super) fn stage(name: &str, message: &str) -> String {
    format!("{name}: {message}")
}

fn initialize_bootstrap() -> Result<(), String> {
    let bootstrap = bootstrap_module().ok_or_else(|| {
        let detail = format!(
            "Microsoft.WindowsAppRuntime.Bootstrap.dll not found next to the executable: GetLastError={}",
            unsafe { GetLastError().0 }
        );
        log_failed("bootstrap-load", &detail);
        detail
    })?;
    let initialize2: unsafe extern "system" fn(u32, PCWSTR, u64, u32) -> HRESULT = unsafe {
        proc_address(bootstrap, s!("MddBootstrapInitialize2")).map(|p| core::mem::transmute(p))
    }
    .ok_or_else(|| {
        let detail = "MddBootstrapInitialize2 export missing from Bootstrap DLL".to_string();
        log_failed("bootstrap-init", &detail);
        detail
    })?;

    // Signature and constants per the pinned MddBootstrap.h; PACKAGE_VERSION is
    // an 8-byte union passed by value, and for release 2+ the minor version is
    // ignored (best installed runtime is selected).
    let hr = unsafe {
        initialize2(
            BOOTSTRAP_MAJOR_MINOR,
            PCWSTR::null(),
            BOOTSTRAP_MIN_VERSION,
            0,
        )
    };
    if hr.is_err() {
        let detail = os_hr(hr);
        log_failed("bootstrap-init", &detail);
        return Err(detail);
    }
    log("bootstrap-init", &format!("initialized ({})", os_hr(hr)));
    Ok(())
}

/// Balances MddBootstrapShutdown; only ever called when the Bootstrap
/// initialization is known to have succeeded.
fn bootstrap_shutdown() {
    if let Some(bootstrap) = bootstrap_module() {
        if let Some(shutdown_fn) = proc_address(bootstrap, s!("MddBootstrapShutdown")) {
            let shutdown_fn: unsafe extern "system" fn() =
                unsafe { core::mem::transmute(shutdown_fn) };
            unsafe { shutdown_fn() };
            log("unwind", "MddBootstrapShutdown balanced");
        }
    }
}

fn initialize_winrt() -> Result<(), String> {
    // STA constant from the official `windows` mapping (roapi.h:
    // RO_INIT_SINGLETHREADED = 0); GPUI already called OleInitialize(STA) on
    // this thread, so S_FALSE ("already initialized") is the expected result
    // and is balanced exactly like S_OK.
    let hr = unsafe { RoInitialize(RO_INIT_SINGLETHREADED.0 as u32) };
    match hr.0 {
        S_OK => log("ro-initialize", "RoInitialize(STA) = S_OK"),
        S_FALSE => log(
            "ro-initialize",
            "RoInitialize(STA) = S_FALSE (balanced at the epilogue)",
        ),
        RPC_E_CHANGED_MODE => {
            let detail =
                "RoInitialize returned RPC_E_CHANGED_MODE; foreground thread is not a WinRT STA";
            log_failed("ro-initialize", detail);
            return Err(detail.to_string());
        }
        _ => {
            let detail = os_hr(hr);
            log_failed("ro-initialize", &detail);
            return Err(detail);
        }
    }
    Ok(())
}

/// Module handles only; never freed (borrowed or process-lifetime by design —
/// the official interop header keeps FrameworkUdk loaded for process lifetime).
fn module_handle(name: PCWSTR) -> Option<HMODULE> {
    unsafe {
        if let Ok(module) = GetModuleHandleW(name) {
            return Some(module);
        }
        LoadLibraryW(name).ok()
    }
}

fn bootstrap_module() -> Option<HMODULE> {
    module_handle(BOOTSTRAP_DLL)
}

pub(super) fn framework_udk() -> Option<HMODULE> {
    module_handle(FRAMEWORK_UDK_DLL)
}

fn windowing_core_module() -> Option<HMODULE> {
    module_handle(WINDOWING_CORE_DLL)
}

pub(super) fn proc_address(module: HMODULE, name: PCSTR) -> Option<*const core::ffi::c_void> {
    // FARPROC is Option<fn>; mapping it exposes the function pointer directly.
    unsafe {
        GetProcAddress(module, name).map(|proc| proc as *const () as *const core::ffi::c_void)
    }
}

/// Resolved module path, for pretranslation provenance in the log.
fn module_path(module: HMODULE) -> String {
    let mut path = [0u16; 260];
    let len = unsafe { GetModuleFileNameW(Some(module), &mut path) } as usize;
    String::from_utf16_lossy(&path[..len.min(path.len())])
}
