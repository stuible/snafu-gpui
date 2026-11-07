# AppWindow native caption — G2 binding-closure checkpoint (2026-10-27)

Follow-up unit on the G1 checkpoint (c7e11800): fix the generated binding dependency
closure only. Branch `test/windows-appwindow-caption`, base `b9d5a160`. Scope honored:
one bounded regeneration + ONE scratch x64 Windows check (`cargo xwin check`) in this
worktree's own target; no app build, no runtime install, no UI, no src/vendor changes.

## Result

- **Generator route reproduced with exact-type filters.** `generator/main.rs` is
  env-configurable (`WASDK_IE_METADATA`, `WASDK_CONTRACTS_REF`, `BINDINGS_OUT`; defaults
  documented in-file) and runnable from this directory:
  `cargo run --manifest-path vendor/appwindow-caption-bindings/provenance/generator/Cargo.toml`.
  `[[bin]] path = "main.rs"` fixes G1's packaging defect (main.rs was unreachable).
- **New canonical output: `generator/bindings.rs`** (2,764 lines, exact-type TypeClosure
  filters, whole-namespace entries removed so bindgen no longer takes its top-down
  shell path). SHA-256 in `generator/bindings.sha256`:
  `1f9caf58e67aacf048f6504a873c89c26a920c20b135b7e64ab28629689e1643`.
  G1's `bindings.narrowed-run.rs` is kept untouched as review evidence (its hash
  `dd9ec930…` matches the review).
- **Scratch x64 Windows check PASSED** (first run): `rustup run 1.98.1 cargo xwin check
  --target x86_64-pc-windows-msvc` in `generator/binding-check/`, `CARGO_TARGET_DIR` set
  to this worktree's own `target/`. Full log: `binding-check-g2.log`. The check crate
  includes the generated file verbatim — no hand patches — against the exact dep closure.
- **Support-crate closure (resolved versions in generator/binding-check/Cargo.lock):**
  windows-core 0.100.0, windows-future 0.100.0, windows-reference 0.100.0 required by the
  emitted code; windows-time 0.100.0 appears transitively. windows-collections is NOT
  needed by this closure (G1's broad output had required it).

## Typed call path (from the generated file — no ABI invented)

- `AppWindow::GetFromWindowId(WindowId) -> Result<AppWindow>`: class static resolving
  through the generated `IAppWindowStatics` factory accessor (`Self::IAppWindowStatics(..)`,
  a FactoryCache — see Factory-lifetime note below).
- `AppWindow::TitleBar(&self) -> Result<AppWindowTitleBar>`; `AppWindow` derefs to its
  default `IAppWindow`; `AppWindowTitleBar::IsCustomizationSupported()` is likewise a class
  static; `AppWindowTitleBar` derefs to `IAppWindowTitleBar`.
- `SetRegionRects(&self, NonClientRegionKind, &[RectInt32])` on `IInputNonClientPointerSource`
  is now fully typed (G1 finding 1 fixed: `Windows.Graphics.RectInt32` is filtered by exact
  name and emitted as a real struct; no more opaque `usize` slot).
- Height/`LeftInset`/`RightInset` are on `IAppWindowTitleBar`; `PreferredHeightOption`
  (type `TitleBarHeightOption`) is on `IAppWindowTitleBar2`; `PreferredTheme` (type
  `TitleBarTheme`) is on `IAppWindowTitleBar3` — reach them with
  `windows_core::Interface::cast::<IAppWindowTitleBar2/3>(&bar)`; do not guess slots.
- `windows_future` appears only for `DispatcherQueue::ShutdownQueueAsync -> IAsyncAction`.
- Input source: `InputNonClientPointerSource::GetForWindowId(WindowId)` class static;
  events/args the closure needs came along typed.

## Hashes and inventories (real files this time)

- `generator/bindings.sha256` — canonical generated output.
- `winmd-inventory.sha256` — SHA-256 of the three InteractiveExperiences 2.1.9
  `10.0.17763.0` winmds actually consumed, plus the contracts winmd file count.
- `binding-check-g2.log` — the exact check run (status + tail also in the handoff).

## Corrections to the G1 README (kept here for the record)

- The G1 claim "No custom WM_NCHITTEST/HTCAPTION/DWM experiments exist in this base" was
  WRONG: base `b9d5a160` already carries the coordinate-aware custom NCHITTEST hit-testing
  in `vendor/gpui/src/platform/windows/events.rs` (roughly lines 861–951, mapping header
  hitboxes to HTCAPTION/HT*BUTTON) plus WM_NCCALCSIZE/NC-input interception. Any adapter
  must define routing that lets the SDK consume caption input without the app-drawn
  button emulation duplicating actions, and must not overwrite GPUI's
  WndProc/GWLP_USERDATA storage (window.rs ~1174) or bypass existing subclasses.
- The G1 handoff's "planned exception: WinRT init leak-for-process-lifetime" is
  retracted — no leak is planned or approved; teardown/ownership is next-unit work under
  AGENTS review rules.
- `bindings.sha256` was referenced in G1 but absent; it is a real file now.

## Explicitly NOT in this unit (next-unit scope per review findings 3–4)

- Factory lifetime: generated class statics use static FactoryCache (`forget(factory)`,
  no clear). If teardown requires locally owned factories, the adapter must use
  RoGetActivationFactory with the generated statics IIDs/vtables and release explicitly.
  Not implemented here.
- Attachment/input chain, DispatcherQueue/ContentPreTranslateMessage pump interaction,
  subclass ordering, native Caption/Passthrough region placement in physical pixels,
  staged failure diagnostics vs. disabled-control semantics: next unit.
- Windows 10 dark/`PreferredTheme` behavior, paint/accessibility gates: unverified, root-owned.

## Artifact provenance (unchanged from G1, independently verified by review)

NuGet pins + SHA-256s as listed in the G1 review (all matched): umbrella 2.5.1, Base 2.0.4,
Foundation 2.3.12, InteractiveExperiences 2.1.9, WinUI 2.3.9, Runtime 2.5.1, Contracts
10.0.28000.2705; headers copied under `vendor-headers/` byte-match originals; Foundation
owns the x64 Bootstrap DLL; FrameworkUdk interop signature as documented there.

## G6 regeneration addendum (2026-10-27, lifecycle unit)

The canonical `generator/bindings.rs` was regenerated once (the unit's single
bounded generator run, offline, in the lifecycle worktree) after adding three
exact filters to `generator/main.rs`:

- `Microsoft.UI.Dispatching.IDispatcherQueueController2` — synchronous
  `ShutdownQueue() -> Result<()>` for the loop-owner epilogue (the async
  `ShutdownQueueAsync` is no longer used by the adapter).
- `Microsoft.UI.Windowing.IAppWindow3` and `Microsoft.UI.Input.IInputObject` —
  association verification only (`DispatcherQueue()` null checks at the raw
  boundary); no ownership claim and no `AssociateWithDispatcherQueue` call.

All three full type names were verified against the pinned InteractiveExperiences
2.1.9 `Microsoft.UI.winmd` TypeDef table (namespace + name) before the run. New
canonical SHA-256: `4534ab3e8f06398314883cebba0280221d7bd6b725b893a96113640719f1c2e7`
(`generator/bindings.sha256`), copied verbatim to
`vendor/appwindow-caption-bindings/src/generated.rs` (byte-identical).
