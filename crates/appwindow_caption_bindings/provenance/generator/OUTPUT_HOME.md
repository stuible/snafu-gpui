# Compiled output home

`bindings.rs` in this directory remains the canonical generator output and
provenance record (`bindings.sha256`). Since G5 the *compiled* copy lives at
`vendor/appwindow-caption-bindings/src/generated.rs` (byte-identical, inside a
private ABI-isolation crate that owns the pinned windows-core/future/reference
0.100 support crates under their normal names — required because the generated
`define_interface!` macro expands absolute `::windows_core` paths that cannot be
aliased from inside gpui). Regenerate here, then copy verbatim to that path.
