//! Pure native-window lifetime identity: an immutable, globally unique id
//! plus explicit alive/dead state, allocated once at native window creation
//! (WM_NCCREATE) and shared by the GPUI userdata record, the window inner,
//! the AppWindow caption host's ledger, sessions and commands. The token is
//! deliberately SDK-free and Win32-free so the identity transitions can be
//! unit-tested off-Windows; it lives here rather than in the Windows-only
//! module tree for exactly that reason. A retired identity is never
//! incremented into another live one — ids are only ever allocated fresh.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Allocation counter for [`WindowLifetime::allocate`]; monotonic for the
/// process, never reused.
static NEXT_WINDOW_ID: AtomicU64 = AtomicU64::new(1);

/// The native caption's close-admission phase, owned by the same lifecycle
/// token as the identity: a WM_CLOSE may invoke the `should_close`
/// callback only from `Idle`, an approved close queues the retained
/// destroy exactly once through `DestroyQueued`, and `Dead` is terminal.
/// `CallbackRunning` is the explicit pending guard: a nested `WM_CLOSE`
/// while the callback runs — or while a destroy is already queued — is
/// answered without invoking the callback again and without falling
/// through to the default destruction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ClosePhase {
    /// No close request is in flight; the callback is installable and a
    /// fresh close may be admitted.
    Idle,
    /// The `should_close` callback was taken and is running: nested
    /// closes are suppressed until it returns.
    CallbackRunning,
    /// One retained destroy has been queued (by an approved close or the
    /// separately authorized owner drop, whichever came first): no
    /// further destroy may be enqueued for this identity.
    DestroyQueued,
    /// The identity retired; every close transition is refused.
    Dead,
}

#[derive(Debug)]
pub(crate) struct WindowLifetime {
    id: u64,
    alive: AtomicBool,
    /// Foreground-thread-only close phase; `Cell` because every
    /// transition happens on the thread that owns the HWND.
    close: Cell<ClosePhase>,
    /// The pending first show, foreground-thread-only: a single slot set by
    /// the caption host's prepare-before-show enqueue (the window
    /// constructor, its only production caller, for a `params.show` window
    /// created hidden) and consumed once by the attach command's show
    /// continuation. `Cell` for the same reason as `close`.
    show_request: Cell<bool>,
}

impl WindowLifetime {
    /// Allocates the next globally unique identity in the alive state.
    pub(crate) fn allocate() -> WindowLifetime {
        WindowLifetime {
            id: NEXT_WINDOW_ID.fetch_add(1, Ordering::Relaxed),
            alive: AtomicBool::new(true),
            close: Cell::new(ClosePhase::Idle),
            show_request: Cell::new(false),
        }
    }

    /// The immutable identity key. Equal only to itself — never to a
    /// later window allocated at the same HWND value.
    pub(crate) fn id(&self) -> u64 {
        self.id
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed)
    }

    /// Marks the identity dead; returns true if THIS call retired it.
    /// Later calls are no-ops: there is no way to revive or transform a
    /// dead identity, and no live operation may key off it again. The
    /// pending show dies with the identity: a retired HWND is never shown
    /// by a leftover continuation.
    pub(crate) fn retire(&self) -> bool {
        let retired = self.alive.swap(false, Ordering::SeqCst);
        self.close.set(ClosePhase::Dead);
        self.show_request.set(false);
        retired
    }

    /// Requests that the identity's first show wait for the caption host's
    /// prepared continuation instead of an immediate `SetWindowPlacement`:
    /// the window stays hidden while the attach (or its fallback) decides
    /// the chrome, and only then shows. The flag is a single pending-slot,
    /// not a permanent one-shot: admitted while no request is pending and
    /// the identity is live, and the slot clears when the continuation
    /// consumes it. The production caller is the window constructor's
    /// prepare enqueue, which runs once per identity, so a duplicate while
    /// pending (a second creation-path enqueue) can never schedule a second
    /// show; a retired identity refuses regardless.
    pub(crate) fn request_deferred_show(&self) -> bool {
        if !self.alive.load(Ordering::Relaxed) || self.show_request.get() {
            return false;
        }
        self.show_request.set(true);
        true
    }

    /// Whether a deferred show is still outstanding (read by `activate`, so
    /// a manual activation cannot consume the placement and show the window
    /// before its chrome is prepared).
    pub(crate) fn deferred_show_pending(&self) -> bool {
        self.alive.load(Ordering::Relaxed) && self.show_request.get()
    }

    /// Consumes the deferred-show request: true only while the identity is
    /// live and its real close phase has not already queued destruction.
    /// The request is consumed either way — an approved close or owner drop
    /// that beat the first host drive owns the window's end, and its
    /// pending first show is discarded, never shown. A close that has not
    /// decided (callback running) or was canceled (back to `Idle`) keeps
    /// the legitimate show. False when nothing was requested (a
    /// `params.show=false` window keeps its normal activation-owned show).
    pub(crate) fn take_deferred_show(&self) -> bool {
        let requested = self.show_request.replace(false);
        requested
            && self.alive.load(Ordering::Relaxed)
            && matches!(
                self.close.get(),
                ClosePhase::Idle | ClosePhase::CallbackRunning
            )
    }

    /// Admits one close request: `Idle → CallbackRunning`. Refused in
    /// every other phase — a nested close during a running callback or an
    /// already-queued destroy must never invoke the callback again, and a
    /// retired identity admits nothing.
    pub(crate) fn begin_close_callback(&self) -> bool {
        match self.close.get() {
            ClosePhase::Idle => {
                self.close.set(ClosePhase::CallbackRunning);
                true
            }
            _ => false,
        }
    }

    /// Returns a canceled close to `Idle` so the callback stays
    /// installable and a later close is admitted fresh. A destroy queued
    /// by another authorization, or a retired identity, is never undone.
    pub(crate) fn cancel_close_callback(&self) {
        if self.close.get() == ClosePhase::CallbackRunning {
            self.close.set(ClosePhase::Idle);
        }
    }

    /// An approved close: `CallbackRunning → DestroyQueued`. Returns true
    /// only on this transition, so the retained destroy is enqueued
    /// exactly once per identity no matter how often the close is
    /// approved.
    pub(crate) fn approve_close(&self) -> bool {
        match self.close.get() {
            ClosePhase::CallbackRunning => {
                self.close.set(ClosePhase::DestroyQueued);
                true
            }
            _ => false,
        }
    }

    /// The separately authorized owner drop: `Idle` or `CallbackRunning →
    /// DestroyQueued`, deduplicating with an already-queued destroy (from
    /// an approved close or a previous drop). A canceled close never
    /// queues anything — only the drop itself admits here.
    pub(crate) fn admit_owner_destroy(&self) -> bool {
        match self.close.get() {
            ClosePhase::Idle | ClosePhase::CallbackRunning => {
                self.close.set(ClosePhase::DestroyQueued);
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WindowLifetime;
    use std::rc::Rc;

    #[test]
    fn identities_are_globally_unique_and_start_alive() {
        let a = Rc::new(WindowLifetime::allocate());
        let b = Rc::new(WindowLifetime::allocate());
        assert!(a.is_alive() && b.is_alive());
        assert_ne!(a.id(), b.id(), "every native creation allocates a fresh id");
    }

    #[test]
    fn retire_is_once_only_and_never_reborn() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.retire(), "the first retirement wins");
        assert!(!token.is_alive());
        assert!(!token.retire(), "a second retirement is a no-op");
        assert!(
            !token.is_alive(),
            "a dead identity never becomes live again"
        );
    }

    #[test]
    fn a_retired_token_never_matches_a_replacement() {
        // An old window's cleanup, arriving late, must never match the
        // window later created at the same HWND value: keys are
        // immutable, so the retired token can only retire itself.
        let old = Rc::new(WindowLifetime::allocate());
        let replacement = Rc::new(WindowLifetime::allocate());
        old.retire();
        assert_ne!(old.id(), replacement.id());
        assert!(replacement.is_alive());
    }

    #[test]
    fn a_nested_close_during_a_running_callback_cannot_bypass() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.begin_close_callback(), "the first close is admitted");
        // The nested WM_CLOSE (the callback was taken) is refused before
        // any callback could be invoked again — and its handler returns
        // handled right there, never reaching an approval or the default
        // destruction:
        assert!(!token.begin_close_callback());
        // Cancel returns to Idle with the window untouched: a later
        // close is admitted fresh.
        token.cancel_close_callback();
        assert!(
            token.begin_close_callback(),
            "cancel returns the token to Idle"
        );
    }

    #[test]
    fn cancel_preserves_the_live_window_and_queues_nothing() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.begin_close_callback());
        token.cancel_close_callback();
        assert!(
            !token.approve_close(),
            "a canceled close never queues a destroy"
        );
        assert!(
            token.begin_close_callback(),
            "cancel returns the token to Idle: a later close is admitted"
        );
        token.cancel_close_callback();
        // A cancel outside a running callback changes nothing.
        token.cancel_close_callback();
        assert!(token.begin_close_callback(), "the token never gets stuck");
    }

    #[test]
    fn an_approved_close_queues_the_retained_destroy_exactly_once() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.begin_close_callback());
        assert!(token.approve_close(), "the approval transitions to queued");
        assert!(
            !token.approve_close(),
            "a repeated approved close cannot enqueue a second destroy"
        );
        assert!(
            !token.admit_owner_destroy(),
            "the owner drop deduplicates against the queued destroy"
        );
        assert!(!token.begin_close_callback());
    }

    #[test]
    fn owner_drop_and_approved_close_dedupe_in_both_orders() {
        // Owner drop first, approved close second:
        let dropped_first = Rc::new(WindowLifetime::allocate());
        assert!(dropped_first.admit_owner_destroy());
        assert!(!dropped_first.begin_close_callback());
        assert!(!dropped_first.admit_owner_destroy());
        // Approved close first, owner drop second:
        let closed_first = Rc::new(WindowLifetime::allocate());
        assert!(closed_first.begin_close_callback());
        assert!(closed_first.approve_close());
        assert!(!closed_first.admit_owner_destroy());
    }

    #[test]
    fn a_dead_identity_is_terminal_for_close_admission() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.begin_close_callback());
        assert!(token.retire());
        assert!(!token.begin_close_callback());
        assert!(!token.approve_close());
        assert!(!token.admit_owner_destroy());
        token.cancel_close_callback();
        assert!(
            !token.begin_close_callback(),
            "cancel never revives a retired identity"
        );
    }

    /// The open transition: a `params.show` window created hidden requests
    /// one deferred show; the attach continuation consumes it exactly once,
    /// so a duplicate request (second enqueue) and a duplicate continuation
    /// (the WM_SHOWWINDOW fired by the show itself) can never show twice.
    #[test]
    fn deferred_show_is_requested_once_and_consumed_once() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.request_deferred_show(), "creation admits the request");
        assert!(
            !token.request_deferred_show(),
            "a duplicate request never schedules a second show"
        );
        assert!(token.deferred_show_pending());
        assert!(
            token.take_deferred_show(),
            "the continuation shows exactly once"
        );
        assert!(!token.deferred_show_pending());
        assert!(
            !token.take_deferred_show(),
            "a duplicate continuation never shows again"
        );
    }

    /// A `params.show=false` window never requests a deferred show, so the
    /// host's continuation finds nothing and the normal activation path
    /// owns the first show — deferred activation cannot show early.
    #[test]
    fn activation_without_a_deferred_request_never_shows() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(!token.deferred_show_pending());
        assert!(!token.take_deferred_show());
    }

    /// A retired identity never shows: retirement drops any pending request,
    /// and a leftover continuation finds neither a pending request nor a
    /// live identity. A dead identity cannot request either.
    #[test]
    fn a_retired_identity_is_never_shown_by_a_leftover_continuation() {
        let token = Rc::new(WindowLifetime::allocate());
        assert!(token.request_deferred_show());
        token.retire();
        assert!(
            !token.deferred_show_pending(),
            "retirement drops the pending show"
        );
        assert!(!token.take_deferred_show());
        assert!(!token.request_deferred_show());

        let dead_first = Rc::new(WindowLifetime::allocate());
        dead_first.retire();
        assert!(!dead_first.request_deferred_show());
    }

    /// The close phase owns the show admission (the production predicate the
    /// host's continuation calls): an approved close or an owner drop that
    /// beat the first host drive consumes and discards the pending show —
    /// the abandoned window is never displayed. A close that has not decided
    /// (callback running) or was canceled (back to `Idle`) keeps the
    /// legitimate show.
    #[test]
    fn a_destroy_queued_identity_never_shows_a_canceled_or_undecided_one_does() {
        // Owner drop before the first host drive:
        let dropped = Rc::new(WindowLifetime::allocate());
        assert!(dropped.request_deferred_show());
        assert!(dropped.admit_owner_destroy());
        assert!(
            !dropped.take_deferred_show(),
            "a destroy-queued identity is never shown"
        );
        assert!(
            !dropped.deferred_show_pending(),
            "the pending show is consumed and discarded with the request"
        );

        // Approved close before the first host drive:
        let approved = Rc::new(WindowLifetime::allocate());
        assert!(approved.request_deferred_show());
        assert!(approved.begin_close_callback());
        assert!(approved.approve_close());
        assert!(!approved.take_deferred_show());

        // A close whose callback is still running has decided nothing:
        let deciding = Rc::new(WindowLifetime::allocate());
        assert!(deciding.request_deferred_show());
        assert!(deciding.begin_close_callback());
        assert!(
            deciding.take_deferred_show(),
            "an undecided close does not block the legitimate show"
        );

        // A canceled close is back to Idle: the show stands.
        let canceled = Rc::new(WindowLifetime::allocate());
        assert!(canceled.request_deferred_show());
        assert!(canceled.begin_close_callback());
        canceled.cancel_close_callback();
        assert!(
            canceled.take_deferred_show(),
            "a canceled close leaves the legitimate show intact"
        );
    }
}
