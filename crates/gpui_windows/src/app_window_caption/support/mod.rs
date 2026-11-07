//! SDK-free rule modules for the Windows AppWindow caption host, kept in one
//! support home next to the native host that drives them. Each child is plain
//! std-only data and functions so the caption transitions and planning rules
//! stay unit-testable off-Windows; the host's SDK calls live in the parent
//! `app_window_caption` module. The caption geometry rules live in `gpui::window_caption_geometry`,
//! shared with the frame publisher.

#[cfg(any(target_os = "windows", test))]
pub(crate) mod window_attach_txn;

#[cfg(any(target_os = "windows", test))]
pub(crate) mod window_lifetime;

#[cfg(any(target_os = "windows", test))]
pub(crate) mod window_terminal;

#[cfg(any(target_os = "windows", test))]
pub(crate) mod window_caption_policy;
