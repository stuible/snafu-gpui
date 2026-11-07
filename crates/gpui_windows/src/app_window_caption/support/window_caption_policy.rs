//! Production failure policy for the native AppWindow caption, kept SDK-free so
//! every decision is unit-tested off-Windows. The host in the parent module asks
//! these functions what to do; it never picks an outcome of its own.
//!
//! The policy: the native caption is on by default and never exits the process.
//! A window whose native caption cannot be kept falls back to the app-owned
//! header. Where a rollback could not restore a live window, the unfinished
//! native state is held by the host and the window is not touched natively again.

use std::ffi::OsStr;

/// Opt-out for support diagnostics: the exact value `1` turns the native caption
/// off, and the established GPUI window paths are used instead.
pub(crate) const DISABLE_ENV: &str = "SNAFU_DISABLE_NATIVE_CAPTION";

/// Whether the native caption is enabled, given the raw value of [`DISABLE_ENV`].
/// It is on unless the variable is set to exactly `1`.
pub(crate) fn native_caption_enabled(disable_value: Option<&OsStr>) -> bool {
    disable_value.is_none_or(|value| value != "1")
}

/// What a window's native state is after a failed attach or region update, as
/// reported by its staged rollback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Rollback {
    /// Every journaled stage was restored, or nothing had been written yet.
    Restored,
    /// The window was destroyed during the rollback; nothing may be published.
    Retired,
    /// The window is live but at least one stage could not be restored.
    Held,
}

/// What the host does with a window after a failure has been settled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Disposition {
    /// The native state is restored (or was never written). The window shows its
    /// app-owned header, and the native attach is not retried for that identity.
    AppOwned,
    /// The rollback could not restore a live window. The host retains the
    /// unfinished native state, the window shows its app-owned header, and no
    /// further SDK call is issued for it.
    Held,
    /// The window was destroyed during the failure. Its cleanup owns retirement,
    /// and nothing is published for a dead window.
    Released,
}

impl Disposition {
    /// Whether the window's caption state is switched to the app-owned header.
    pub(crate) fn publishes_app_owned(self) -> bool {
        match self {
            Disposition::AppOwned | Disposition::Held => true,
            Disposition::Released => false,
        }
    }
}

/// The decision for a failed attach or region update, given how its rollback ended.
pub(crate) fn after_failure(rollback: Rollback) -> Disposition {
    match rollback {
        Rollback::Restored => Disposition::AppOwned,
        Rollback::Held => Disposition::Held,
        Rollback::Retired => Disposition::Released,
    }
}

/// Whether an attach may run for a window at all. A runtime that failed to start
/// fails every window the same way: each keeps its app-owned header and no SDK
/// call is made for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AttachGate {
    Attempt,
    FallBack,
}

pub(crate) fn attach_gate(runtime_ready: bool) -> AttachGate {
    if runtime_ready {
        AttachGate::Attempt
    } else {
        AttachGate::FallBack
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::*;

    #[test]
    fn native_caption_is_on_without_the_opt_out_variable() {
        assert!(native_caption_enabled(None));
    }

    #[test]
    fn only_the_exact_value_one_opts_out() {
        assert!(!native_caption_enabled(Some(OsStr::new("1"))));
        assert!(native_caption_enabled(Some(OsStr::new("0"))));
        assert!(native_caption_enabled(Some(OsStr::new(""))));
        assert!(native_caption_enabled(Some(OsStr::new("true"))));
    }

    #[test]
    fn a_restored_rollback_falls_back_to_the_app_owned_header() {
        let disposition = after_failure(Rollback::Restored);
        assert_eq!(disposition, Disposition::AppOwned);
        assert!(disposition.publishes_app_owned());
    }

    #[test]
    fn an_incomplete_rollback_holds_the_window_on_its_app_owned_header() {
        let disposition = after_failure(Rollback::Held);
        assert_eq!(disposition, Disposition::Held);
        assert!(disposition.publishes_app_owned());
    }

    #[test]
    fn a_retired_window_publishes_nothing() {
        let disposition = after_failure(Rollback::Retired);
        assert_eq!(disposition, Disposition::Released);
        assert!(!disposition.publishes_app_owned());
    }

    #[test]
    fn a_runtime_that_failed_to_start_keeps_every_window_off_the_sdk() {
        assert_eq!(attach_gate(false), AttachGate::FallBack);
    }

    #[test]
    fn a_ready_runtime_lets_attach_run() {
        assert_eq!(attach_gate(true), AttachGate::Attempt);
    }
}
