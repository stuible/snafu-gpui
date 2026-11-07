//! Compile gate for the generated AppWindow caption bindings: includes the generated
//! output verbatim (never hand-edited) against the exact support-crate closure the
//! generator emits. Checked for x64 Windows via `cargo xwin check`.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

include!("../../bindings.rs");
