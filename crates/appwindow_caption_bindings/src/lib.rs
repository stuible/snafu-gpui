//! Private ABI-isolation crate for Snafu's native AppWindow caption.
//!
//! Purpose: the windows-bindgen 0.100.0 output calls `windows_core::imp::define_interface!`,
//! whose macro expands *absolute* `::windows_core` paths. Inside gpui those paths would
//! resolve to gpui's own windows-core 0.61 and implement the wrong `Interface` trait —
//! a local alias inside a gpui module cannot isolate an external macro. Housing the
//! verbatim generated body here, with normally named pinned 0.100 support crates,
//! isolates macro/ABI versioning from gpui's 0.61 dependency. This crate exists for
//! ABI/macro version isolation, not as a size abstraction; gpui is its only consumer
//! and the re-exports below stay narrow.
//!
//! Provenance: generated verbatim by the windows-bindgen 0.100.0 recipe in
//! `vendor/appwindow-caption-bindings/provenance/generator` from the pinned Microsoft.WindowsAppSDK
//! 2.5.1 WinMD set; body SHA256
//! `4534ab3e8f06398314883cebba0280221d7bd6b725b893a96113640719f1c2e7`
//! (recorded in `generator/bindings.sha256`). Never hand-edit `src/generated.rs`.
// Preserve metadata-derived API names and the unused projection surface here only.
// The handwritten adapter and this crate root retain ordinary lint coverage.
#[allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod generated;

pub use generated::*;

// Narrow core re-exports for the adapter's Interface/factory plumbing; the
// 0.100 objects themselves never cross into gpui's windows-core 0.61 types.
// HRESULT is named so the adapter's Windows-only counted-fake test can spell
// the 0.100 vtable signatures — distinct from gpui's windows 0.61 HRESULT.
pub use windows_core::{Error, GUID, HRESULT, Interface, imp};

// The caption adapter's nullable original capture must adopt a successful
// non-null button-color result as an owned typed reference so the value is
// read and the reference released on the 0.100 ABI: the generated getters
// return `Color` and collapse a successful null (the unset default) into an
// error, so absence cannot be expressed through them.
pub use windows_reference::IReference;
