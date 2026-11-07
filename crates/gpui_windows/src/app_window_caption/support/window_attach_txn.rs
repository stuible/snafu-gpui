//! Pure attach-transaction rules for the Windows AppWindow caption host: the
//! per-stage attempt journal, the forward mutation pass, the staged
//! restoration pass, and the association-gate classifier. Deliberately plain
//! data — std only, no SDK, no Win32, no borrow of any host or window object —
//! so the journaling order (a stage is journaled as attempted BEFORE its
//! mutating call, because an HRESULT failure never implies "no native
//! effect") and the retirement rule between stages are unit-testable
//! off-Windows. The caption host's session build and rollback drive their SDK
//! calls through the closures injected here; there is no duplicate test-only
//! state machine.

/// One mutating stage of a caption attach, in attach order. Restoration
/// attempts the same stages in reverse. The two region kinds are separate
/// stages so a partial second-kind failure rolls back exactly the kinds
/// that were mutated — each keeps its own captured original. The two
/// button-background colors are separate stages for the same reason: each
/// carries its own nullable original (unset default or prior color), so a
/// partial failure restores exactly the property that was written.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Stage {
    ExtendContent,
    Height,
    Theme,
    ButtonBackground,
    ButtonInactiveBackground,
    Regions,
    PassthroughRegions,
}

/// All stages in attach order.
pub(crate) const STAGES: [Stage; 7] = [
    Stage::ExtendContent,
    Stage::Height,
    Stage::Theme,
    Stage::ButtonBackground,
    Stage::ButtonInactiveBackground,
    Stage::Regions,
    Stage::PassthroughRegions,
];

/// The result of one injected stage call.
#[derive(Debug)]
pub(crate) enum StageOutcome<E> {
    /// The stage's SDK call succeeded.
    Applied,
    /// The identity retired at a boundary inside the stage (for example
    /// between a QI and its setter): the setter was skipped and nothing
    /// was mutated by this call, but the stage stays journaled as
    /// attempted — restoring an unmutated stage re-applies the original
    /// value, which is safe, while skipping a needed restore is not.
    Retired,
    /// The stage's SDK call failed; the native state may still have been
    /// partially changed, so the stage stays journaled as attempted.
    Failed(E),
}

/// The aggregated outcome of a forward attach pass.
#[derive(Debug)]
pub(crate) enum TransactionOutcome<E> {
    /// Every stage ran successfully.
    Complete,
    /// The identity retired before or during the pass: remaining stages
    /// were skipped without ever being journaled, and nothing is rolled
    /// back (a dead HWND is never restored).
    Retired,
    /// The first failing stage, with its error. Stages after it never
    /// ran; the attempted ones stay journaled for the rollback.
    Failed { stage: Stage, error: E },
}

/// The attempt journal: one bit per stage, set before the stage's
/// mutating call and cleared only after that stage is verifiably
/// restored. After a complete restoration the journal is empty; a
/// non-empty journal IS the unfinished state a failed rollback retains.
#[derive(Default)]
pub(crate) struct StageJournal {
    attempted: [bool; STAGES.len()],
}

impl StageJournal {
    pub(crate) fn is_attempted(&self, stage: Stage) -> bool {
        self.attempted[stage as usize]
    }

    /// Journals a stage as attempted. `run_stages` calls this BEFORE the
    /// stage's mutating call; tests use it to seed a journal.
    pub(crate) fn mark_attempted(&mut self, stage: Stage) {
        self.attempted[stage as usize] = true;
    }

    /// Clears one stage after its verified restoration.
    pub(crate) fn clear(&mut self, stage: Stage) {
        self.attempted[stage as usize] = false;
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.attempted.iter().all(|attempted| !attempted)
    }

    /// The journaled stages in restoration order (reverse attach order).
    pub(crate) fn restore_order(&self) -> Vec<Stage> {
        STAGES
            .iter()
            .rev()
            .copied()
            .filter(|stage| self.is_attempted(*stage))
            .collect()
    }
}

/// Forward attach pass: runs `stages` in order. Before each stage the
/// injected `alive` check runs (a retired identity ends the pass with the
/// stage skipped and never journaled); the stage is journaled as
/// attempted before its call; after an applied stage the alive check runs
/// again, so retirement on any boundary is observed before the next stage
/// begins. A stage may also report its own internal retirement — the
/// check between its QI and its setter — through
/// [`StageOutcome::Retired`].
pub(crate) fn run_stages<E>(
    stages: &[Stage],
    journal: &mut StageJournal,
    alive: impl Fn() -> bool,
    mut stage_call: impl FnMut(Stage) -> StageOutcome<E>,
) -> TransactionOutcome<E> {
    for &stage in stages {
        if !alive() {
            return TransactionOutcome::Retired;
        }
        journal.mark_attempted(stage);
        match stage_call(stage) {
            StageOutcome::Applied => {
                if !alive() {
                    return TransactionOutcome::Retired;
                }
            }
            StageOutcome::Retired => return TransactionOutcome::Retired,
            StageOutcome::Failed(error) => return TransactionOutcome::Failed { stage, error },
        }
    }
    TransactionOutcome::Complete
}

/// The report of a staged restoration pass over a journal.
#[derive(Debug)]
pub(crate) struct RestoreReport<E> {
    /// Stages whose restoration failed while the identity stayed live;
    /// they remain journaled as unfinished.
    pub(crate) failures: Vec<(Stage, E)>,
    /// True when the identity retired mid-pass: remaining stages were
    /// skipped and stay journaled, but a dead HWND is never restored
    /// again — the retired cleanup releases them instead.
    pub(crate) retired: bool,
}

/// Restoration pass: attempts every journaled stage in reverse attach
/// order, independently — one stage's failure never blocks the others
/// while the identity stays alive — and clears each stage it restored.
/// A retired identity stops the pass; it never restores further.
pub(crate) fn run_restore<E>(
    journal: &mut StageJournal,
    alive: impl Fn() -> bool,
    mut stage_call: impl FnMut(Stage) -> StageOutcome<E>,
) -> RestoreReport<E> {
    let mut failures = Vec::new();
    for stage in journal.restore_order() {
        if !alive() {
            return RestoreReport {
                failures,
                retired: true,
            };
        }
        match stage_call(stage) {
            StageOutcome::Applied => {
                journal.clear(stage);
                if !alive() {
                    return RestoreReport {
                        failures,
                        retired: true,
                    };
                }
            }
            StageOutcome::Retired => {
                return RestoreReport {
                    failures,
                    retired: true,
                };
            }
            StageOutcome::Failed(error) => failures.push((stage, error)),
        }
    }
    RestoreReport {
        failures,
        retired: false,
    }
}

/// The association gate's three answers, classified from the raw ABI pair
/// (HRESULT, out-pointer null-ness) of the generated `IAppWindow3`
/// `DispatcherQueue` accessor. Only a success HRESULT with a null out
/// proves the AppWindow unassociated; a success with a non-null out is an
/// associated queue; every failing HRESULT — including a failed QI for
/// `IAppWindow3` itself, which the caller also classifies as
/// [`AssociationState::CheckFailed`] — is a failed gate, never an assumed
/// pass.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AssociationState {
    Unassociated,
    Associated,
    CheckFailed,
}

pub(crate) fn classify_association(hr: i32, out_is_null: bool) -> AssociationState {
    if hr != 0 {
        AssociationState::CheckFailed
    } else if out_is_null {
        AssociationState::Unassociated
    } else {
        AssociationState::Associated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn association_error_non_null_and_null_are_distinguished() {
        assert_eq!(
            classify_association(0, true),
            AssociationState::Unassociated,
            "success HRESULT with a null out is the one valid unassociated answer"
        );
        assert_eq!(
            classify_association(0, false),
            AssociationState::Associated,
            "a success with a non-null out is a real association, never passed"
        );
        assert_eq!(
            classify_association(0x8000_4015u32 as i32, true),
            AssociationState::CheckFailed,
            "an accessor HRESULT error fails the gate closed, even with a null out"
        );
        assert_eq!(
            classify_association(0x8000_4015u32 as i32, false),
            AssociationState::CheckFailed
        );
    }

    #[test]
    fn a_stage_is_journaled_attempted_before_its_call_and_retained_on_failure() {
        let mut journal = StageJournal::default();
        let mut called = Vec::new();
        let outcome = run_stages(
            &STAGES,
            &mut journal,
            || true,
            |stage| {
                called.push(stage);
                if stage == Stage::Height {
                    StageOutcome::Failed("setter failed".to_string())
                } else {
                    StageOutcome::Applied
                }
            },
        );
        assert_eq!(
            called,
            vec![Stage::ExtendContent, Stage::Height],
            "the pass stops at the first failure; later stages never run"
        );
        match outcome {
            TransactionOutcome::Failed { stage, error } => {
                assert_eq!(stage, Stage::Height);
                assert_eq!(error, "setter failed");
            }
            _ => panic!("a failed stage must surface as Failed"),
        }
        assert!(journal.is_attempted(Stage::ExtendContent));
        assert!(
            journal.is_attempted(Stage::Height),
            "the failed stage stays journaled: the HRESULT cannot prove the \
             native state is unchanged"
        );
        assert!(!journal.is_attempted(Stage::Theme));
        assert!(!journal.is_attempted(Stage::Regions));
        assert_eq!(
            journal.restore_order(),
            vec![Stage::Height, Stage::ExtendContent],
            "restoration runs the attempted stages in reverse"
        );
    }

    #[test]
    fn retirement_between_stages_skips_the_next_setter_untouched() {
        let mut journal = StageJournal::default();
        let mut called = Vec::new();
        let alive = std::cell::Cell::new(true);
        let outcome: TransactionOutcome<String> = run_stages(
            &STAGES,
            &mut journal,
            || alive.get(),
            |stage| {
                called.push(stage);
                if stage == Stage::ExtendContent {
                    // the identity retires after the first stage's call
                    alive.set(false);
                }
                StageOutcome::Applied
            },
        );
        assert!(
            matches!(outcome, TransactionOutcome::Retired),
            "retirement on the boundary after an applied stage ends the pass"
        );
        assert_eq!(
            called,
            vec![Stage::ExtendContent],
            "the next stage's call is skipped"
        );
        assert!(
            journal.is_attempted(Stage::ExtendContent) && !journal.is_attempted(Stage::Height),
            "a skipped stage is never journaled — there is nothing to restore for it"
        );
    }

    #[test]
    fn retirement_between_a_qi_and_its_setter_skips_the_setter() {
        let mut journal = StageJournal::default();
        let mut setter_invoked = false;
        // The stage closure simulates the production shape: its QI
        // succeeded, but the identity retired before the setter ran.
        let alive_after_qi = false;
        let outcome: TransactionOutcome<String> = run_stages(
            &[Stage::Theme],
            &mut journal,
            || true,
            |_stage| {
                if alive_after_qi {
                    setter_invoked = true;
                    StageOutcome::Applied
                } else {
                    StageOutcome::Retired
                }
            },
        );
        assert!(
            !setter_invoked,
            "the setter is skipped when the identity retired between the QI and the setter"
        );
        assert!(matches!(outcome, TransactionOutcome::Retired));
        assert!(
            journal.is_attempted(Stage::Theme),
            "an entered stage stays journaled: restoring it re-applies the \
             original value, which is safe even if the setter never ran"
        );
    }

    #[test]
    fn restoration_attempts_independent_stages_and_retains_only_failures() {
        let mut journal = StageJournal::default();
        for stage in STAGES {
            journal.mark_attempted(stage);
        }
        let report = run_restore(
            &mut journal,
            || true,
            |stage| match stage {
                Stage::Regions => StageOutcome::Failed("clear failed".to_string()),
                Stage::Theme => StageOutcome::Applied,
                Stage::Height => StageOutcome::Failed("height restore failed".to_string()),
                Stage::ExtendContent => StageOutcome::Applied,
                Stage::ButtonBackground => StageOutcome::Applied,
                Stage::ButtonInactiveBackground => StageOutcome::Applied,
                Stage::PassthroughRegions => StageOutcome::Applied,
            },
        );
        assert!(!report.retired);
        assert_eq!(
            report.failures,
            vec![
                (Stage::Regions, "clear failed".to_string()),
                (Stage::Height, "height restore failed".to_string())
            ],
            "one stage's failure never blocks the others"
        );
        assert!(
            !journal.is_attempted(Stage::Theme)
                && !journal.is_attempted(Stage::ExtendContent)
                && !journal.is_attempted(Stage::PassthroughRegions),
            "restored stages are cleared"
        );
        assert!(
            journal.is_attempted(Stage::Regions) && journal.is_attempted(Stage::Height),
            "only the unfinished stages stay journaled"
        );
    }

    #[test]
    fn a_second_kind_failure_after_the_first_applied_rolls_both_back() {
        // The region pass shape: Caption applies, then Passthrough fails.
        // Both stay journaled (the first is re-restored even though its
        // setter succeeded — an HRESULT never proves the native state),
        // and the restore runs the kinds in reverse attach order.
        let mut journal = StageJournal::default();
        let mut calls = Vec::new();
        let outcome: TransactionOutcome<String> = run_stages(
            &[Stage::Regions, Stage::PassthroughRegions],
            &mut journal,
            || true,
            |stage| {
                calls.push(stage);
                match stage {
                    Stage::Regions => StageOutcome::Applied,
                    Stage::PassthroughRegions => {
                        StageOutcome::Failed("passthrough setter failed".to_string())
                    }
                    other => StageOutcome::Failed(format!("unexpected {other:?}")),
                }
            },
        );
        match outcome {
            TransactionOutcome::Failed { stage, error } => {
                assert_eq!(stage, Stage::PassthroughRegions);
                assert_eq!(error, "passthrough setter failed");
            }
            _ => panic!("the second kind's failure must surface as Failed"),
        }
        assert_eq!(calls, vec![Stage::Regions, Stage::PassthroughRegions]);
        assert!(journal.is_attempted(Stage::Regions));
        assert!(journal.is_attempted(Stage::PassthroughRegions));

        let mut restored = Vec::new();
        let report: RestoreReport<String> = run_restore(
            &mut journal,
            || true,
            |stage| {
                restored.push(stage);
                StageOutcome::Applied
            },
        );
        assert!(!report.retired && report.failures.is_empty());
        assert_eq!(
            restored,
            vec![Stage::PassthroughRegions, Stage::Regions],
            "rollback re-applies the captured originals per kind, second kind first"
        );
        assert!(
            journal.is_empty(),
            "a clean two-kind rollback retains nothing"
        );
    }

    #[test]
    fn restoration_stops_on_death_and_retains_the_remaining_stages() {
        let mut journal = StageJournal::default();
        for stage in STAGES {
            journal.mark_attempted(stage);
        }
        let mut called = Vec::new();
        let alive = std::cell::Cell::new(true);
        let report: RestoreReport<String> = run_restore(
            &mut journal,
            || alive.get(),
            |stage| {
                called.push(stage);
                if stage == Stage::Theme {
                    alive.set(false);
                }
                StageOutcome::Applied
            },
        );
        assert!(report.retired);
        assert_eq!(
            called,
            vec![
                Stage::PassthroughRegions,
                Stage::Regions,
                Stage::ButtonInactiveBackground,
                Stage::ButtonBackground,
                Stage::Theme,
            ],
            "the pass stops at the first retirement; no later stage runs"
        );
        assert!(report.failures.is_empty());
        assert!(
            !journal.is_attempted(Stage::Regions) && !journal.is_attempted(Stage::Theme),
            "stages restored before death stay cleared"
        );
        assert!(
            !journal.is_attempted(Stage::ButtonBackground)
                && !journal.is_attempted(Stage::ButtonInactiveBackground),
            "the button stages restored before death stay cleared too"
        );
        assert!(
            journal.is_attempted(Stage::Height) && journal.is_attempted(Stage::ExtendContent),
            "the skipped stages stay journaled; the retired cleanup releases them"
        );
    }

    #[test]
    fn a_complete_restoration_empties_the_journal() {
        let mut journal = StageJournal::default();
        for stage in STAGES {
            journal.mark_attempted(stage);
        }
        let report: RestoreReport<String> =
            run_restore(&mut journal, || true, |_| StageOutcome::Applied);
        assert!(!report.retired && report.failures.is_empty());
        assert!(journal.is_empty(), "nothing is retained after full restore");
    }
}
