//! Pure terminal-barrier state for the Windows AppWindow caption host: the
//! barrier phase rule plus the loop-owned retirement ledger. Deliberately
//! plain data — std only, no SDK, no Win32, no borrow of any host or window
//! object — so the phase/ledger/admission transitions are unit-testable
//! off-Windows. The caption host owns the one ledger instance, and every
//! admission decision reads the rule functions here; there is no duplicate
//! test-only state machine.

use std::collections::HashMap;

/// Barrier phases, held in the host's process-wide latch on the
/// foreground thread: `PHASE_OPEN` until the approved terminal exit;
/// `PHASE_TERMINAL` from just before the first SDK rundown/release
/// boundary; `PHASE_CLOSED` at final close admission. Never rewritten
/// backwards.
pub(crate) const PHASE_OPEN: u8 = 0;
pub(crate) const PHASE_TERMINAL: u8 = 1;
pub(crate) const PHASE_CLOSED: u8 = 2;

/// The one admission rule for the terminal barrier, shared by the enqueue
/// guard and these tests: live work is admitted only while the phase is
/// open, a retirement acknowledgment is still admitted during the
/// rundown/release phase, and nothing at all is admitted once admission
/// has closed for the final balance.
pub(crate) fn admits(phase: u8, retirement: bool) -> bool {
    match phase {
        PHASE_OPEN => true,
        PHASE_TERMINAL => retirement,
        _ => false,
    }
}

/// Whether native window creation is admitted: only while the phase is
/// open. `WM_NCCREATE` consults this at the actual native creation
/// boundary, so a window the terminal barrier could no longer register,
/// destroy or verify never comes to exist.
pub(crate) fn creation_admitted(phase: u8) -> bool {
    phase == PHASE_OPEN
}

/// The loop-owned retirement ledger: every native identity
/// registered at native creation and not yet acknowledged retired, plus
/// the record of unexpected live work observed during the terminal
/// phase. A rejected registration in particular never appears in the
/// outstanding set — which is exactly why the violation record exists:
/// with it, live work during rundown can never pass an empty-ledger
/// balance check.
#[derive(Default)]
pub(crate) struct TerminalLedger {
    /// Identities awaiting their matching `WM_NCDESTROY` cleanup, keyed
    /// by the immutable lifetime id, valued by the registering HWND.
    outstanding: HashMap<u64, isize>,
    /// Set when live work reached the terminal drain; verification can
    /// then never succeed.
    violated: bool,
}

impl TerminalLedger {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Records a native creation. Every window the host saw
    /// at `WM_NCCREATE` is accounted for here until its matching cleanup
    /// acknowledgment.
    pub(crate) fn register(&mut self, id: u64, hwnd_value: isize) {
        self.outstanding.insert(id, hwnd_value);
    }

    /// Retires the ONE matching identity: a duplicate or a dead
    /// identity's late cleanup can never remove another window's entry,
    /// because ids are globally unique and never reused. Returns true
    /// when THIS call retired an entry.
    pub(crate) fn retire(&mut self, id: u64) -> bool {
        self.outstanding.remove(&id).is_some()
    }

    /// Records unexpected live work observed during the terminal phase.
    pub(crate) fn note_violation(&mut self) {
        self.violated = true;
    }

    pub(crate) fn is_violated(&self) -> bool {
        self.violated
    }

    /// Whether the terminal barrier may pass: every registered identity
    /// retired and no terminal-phase live work recorded.
    pub(crate) fn is_balanced(&self) -> bool {
        self.outstanding.is_empty() && !self.violated
    }

    /// Outstanding identities, sorted for deterministic failure
    /// diagnostics.
    pub(crate) fn sorted_outstanding(&self) -> Vec<(u64, isize)> {
        let mut entries: Vec<(u64, isize)> = self
            .outstanding
            .iter()
            .map(|(id, hwnd)| (*id, *hwnd))
            .collect();
        entries.sort_unstable();
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_registration_then_matching_retirement_balances() {
        let mut ledger = TerminalLedger::new();
        ledger.register(1, 0x10);
        assert!(
            !ledger.is_balanced(),
            "a registered identity is outstanding"
        );
        assert!(ledger.retire(1), "the matching cleanup retires it");
        assert!(
            ledger.is_balanced(),
            "matching retirement completes the barrier"
        );
    }

    #[test]
    fn a_new_window_during_the_terminal_phase_cannot_balance() {
        let mut ledger = TerminalLedger::new();
        ledger.register(1, 0x10);
        ledger.retire(1);
        assert!(ledger.is_balanced());
        // Rundown phase: native creation is refused at the WM_NCCREATE
        // boundary, live work is not admitted, and a registration that
        // still reaches the terminal drain blocks the balance — it can
        // never pass as an empty ledger.
        assert!(!creation_admitted(PHASE_TERMINAL));
        assert!(!admits(PHASE_TERMINAL, false));
        assert!(admits(PHASE_TERMINAL, true), "retirement stays admitted");
        ledger.register(2, 0x20);
        assert!(
            !ledger.is_balanced(),
            "a rundown registration blocks the balance"
        );
        ledger.retire(2);
        assert!(ledger.is_balanced());
        ledger.note_violation();
        assert!(
            !ledger.is_balanced(),
            "recorded terminal live work fails the barrier even with an empty ledger"
        );
        assert!(ledger.is_violated());
        // Close admission: nothing at all is admitted for the balance.
        assert!(!admits(PHASE_CLOSED, true));
        assert!(!creation_admitted(PHASE_CLOSED));
    }

    #[test]
    fn cleanup_only_retires_its_own_identity() {
        let mut ledger = TerminalLedger::new();
        ledger.register(1, 0x10);
        ledger.register(2, 0x10); // replacement allocated at the same HWND value
        assert!(
            ledger.retire(2),
            "the replacement's cleanup retires the replacement"
        );
        assert!(!ledger.retire(2), "a duplicate cleanup is a no-op");
        assert!(
            !ledger.retire(3),
            "a never-registered identity retires nothing"
        );
        assert!(
            !ledger.is_balanced(),
            "identity 1 stays outstanding at the reused HWND"
        );
        assert!(ledger.retire(1));
        assert!(ledger.is_balanced());
    }

    #[test]
    fn retirement_completes_the_barrier_during_the_terminal_phase() {
        let mut ledger = TerminalLedger::new();
        ledger.register(7, 0x70);
        assert!(
            admits(PHASE_TERMINAL, true),
            "cleanup enqueued during rundown is admitted"
        );
        assert!(ledger.retire(7));
        assert!(
            ledger.is_balanced() && !ledger.is_violated(),
            "matching retirement during rundown lets the barrier pass"
        );
    }

    #[test]
    fn outstanding_diagnostics_are_sorted_by_identity() {
        let mut ledger = TerminalLedger::new();
        ledger.register(3, 0x30);
        ledger.register(1, 0x10);
        assert_eq!(ledger.sorted_outstanding(), vec![(1, 0x10), (3, 0x30)]);
    }
}
