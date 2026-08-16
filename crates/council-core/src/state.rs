use thiserror::Error;

use crate::model::DebateState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebateEvent {
    PreflightPassed,
    SnapshotStarted,
    SnapshotReady,
    SnapshotReviewRequired,
    SnapshotReviewApproved,
    SnapshotReviewRejected,
    SnapshotReviewInvalidated,
    OpeningStarted,
    OpeningComplete,
    IndependentOpeningComplete,
    CrossExaminationStarted,
    CrossExaminationComplete,
    FinalPositionsStarted,
    FinalPositionsComplete,
    TargetedRoundRequested,
    HumanDecisionRecorded,
    Compile,
    Export,
    Pause,
    Resume,
    Cancel,
    Fail,
    SafetyAbort,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StateTransitionError {
    #[error("event {event:?} is not valid from state {state:?}")]
    Invalid {
        state: DebateState,
        event: DebateEvent,
    },
}

#[derive(Debug, Clone)]
pub struct DebateStateMachine {
    state: DebateState,
    history: Vec<(DebateState, DebateEvent, DebateState)>,
    targeted_rounds: u8,
}

impl DebateStateMachine {
    pub fn new(state: DebateState) -> Self {
        Self {
            state,
            history: Vec::new(),
            targeted_rounds: 0,
        }
    }

    pub fn state(&self) -> &DebateState {
        &self.state
    }

    pub fn history(&self) -> &[(DebateState, DebateEvent, DebateState)] {
        &self.history
    }

    pub fn transition(&mut self, event: DebateEvent) -> Result<DebateState, StateTransitionError> {
        if matches!(&event, DebateEvent::TargetedRoundRequested) && self.targeted_rounds >= 1 {
            return Err(StateTransitionError::Invalid {
                state: self.state.clone(),
                event,
            });
        }
        let next =
            next_state(&self.state, &event).ok_or_else(|| StateTransitionError::Invalid {
                state: self.state.clone(),
                event: event.clone(),
            })?;
        let previous = self.state.clone();
        self.state = next.clone();
        if matches!(&event, DebateEvent::TargetedRoundRequested) {
            self.targeted_rounds += 1;
        }
        self.history.push((previous, event, next.clone()));
        Ok(next)
    }
}

fn next_state(state: &DebateState, event: &DebateEvent) -> Option<DebateState> {
    use DebateEvent::*;
    use DebateState::*;
    match (state, event) {
        (Draft, PreflightPassed) => Some(Preflight),
        (Preflight, SnapshotStarted) => Some(Snapshotting),
        (Snapshotting, SnapshotReady) => Some(Ready),
        (Snapshotting, DebateEvent::SnapshotReviewRequired) => {
            Some(DebateState::SnapshotReviewRequired)
        }
        (DebateState::SnapshotReviewRequired, DebateEvent::SnapshotReviewApproved) => Some(Ready),
        (DebateState::SnapshotReviewRequired, DebateEvent::SnapshotReviewRejected) => {
            Some(DebateState::SafetyAbort)
        }
        (Ready, DebateEvent::SnapshotReviewInvalidated) => {
            Some(DebateState::SnapshotReviewRequired)
        }
        (CrossExamination, DebateEvent::SnapshotReviewInvalidated) => {
            Some(DebateState::SnapshotReviewRequired)
        }
        (FinalPositions, DebateEvent::SnapshotReviewInvalidated) => {
            Some(DebateState::SnapshotReviewRequired)
        }
        (Ready, OpeningStarted) => Some(Opening),
        (Ready, CrossExaminationStarted) => Some(CrossExamination),
        (Ready, FinalPositionsStarted) => Some(FinalPositions),
        (Opening, OpeningComplete) => Some(CrossExamination),
        (Opening, IndependentOpeningComplete) => Some(AwaitingHumanDecision),
        (CrossExamination, CrossExaminationStarted) => Some(CrossExamination),
        (CrossExamination, CrossExaminationComplete) => Some(FinalPositions),
        (FinalPositions, FinalPositionsStarted) => Some(FinalPositions),
        (FinalPositions, FinalPositionsComplete) => Some(AwaitingHumanDecision),
        (AwaitingHumanDecision, TargetedRoundRequested) => Some(CrossExamination),
        (AwaitingHumanDecision, HumanDecisionRecorded) => Some(Decided),
        (Decided, Compile) => Some(Compiled),
        (Compiled, Export) => Some(Exported),
        (
            Draft
            | Preflight
            | Snapshotting
            | DebateState::SnapshotReviewRequired
            | Ready
            | Opening
            | CrossExamination
            | FinalPositions
            | AwaitingHumanDecision,
            Pause,
        ) => Some(Paused),
        (Paused, Resume) => Some(Ready),
        (
            Draft
            | Preflight
            | Snapshotting
            | DebateState::SnapshotReviewRequired
            | Ready
            | Opening
            | CrossExamination
            | FinalPositions
            | AwaitingHumanDecision,
            Cancel,
        ) => Some(Cancelled),
        (
            Draft
            | Preflight
            | Snapshotting
            | DebateState::SnapshotReviewRequired
            | Ready
            | Opening
            | CrossExamination
            | FinalPositions
            | AwaitingHumanDecision,
            Fail,
        ) => Some(Failed),
        (
            Draft
            | Preflight
            | Snapshotting
            | Ready
            | Opening
            | CrossExamination
            | FinalPositions
            | AwaitingHumanDecision,
            DebateEvent::SafetyAbort,
        ) => Some(DebateState::SafetyAbort),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_deterministic_happy_path() {
        let mut machine = DebateStateMachine::new(DebateState::Draft);
        for event in [
            DebateEvent::PreflightPassed,
            DebateEvent::SnapshotStarted,
            DebateEvent::SnapshotReady,
            DebateEvent::OpeningStarted,
            DebateEvent::OpeningComplete,
            DebateEvent::CrossExaminationComplete,
            DebateEvent::FinalPositionsComplete,
            DebateEvent::HumanDecisionRecorded,
            DebateEvent::Compile,
            DebateEvent::Export,
        ] {
            machine.transition(event).unwrap();
        }
        assert_eq!(*machine.state(), DebateState::Exported);
    }

    #[test]
    fn rejects_skipping_preflight() {
        let mut machine = DebateStateMachine::new(DebateState::Draft);
        assert!(machine.transition(DebateEvent::OpeningStarted).is_err());
    }

    #[test]
    fn human_can_open_a_targeted_round_after_final_positions() {
        let mut machine = DebateStateMachine::new(DebateState::AwaitingHumanDecision);
        assert_eq!(
            machine
                .transition(DebateEvent::TargetedRoundRequested)
                .unwrap(),
            DebateState::CrossExamination
        );
        assert!(
            machine
                .transition(DebateEvent::TargetedRoundRequested)
                .is_err()
        );
    }

    #[test]
    fn independent_opening_stops_at_human_gate() {
        let mut machine = DebateStateMachine::new(DebateState::Ready);
        machine.transition(DebateEvent::OpeningStarted).unwrap();
        assert_eq!(
            machine
                .transition(DebateEvent::IndependentOpeningComplete)
                .unwrap(),
            DebateState::AwaitingHumanDecision
        );
    }

    #[test]
    fn snapshot_review_is_a_persisted_gate_not_a_safety_dead_end() {
        let mut machine = DebateStateMachine::new(DebateState::Draft);
        machine.transition(DebateEvent::PreflightPassed).unwrap();
        machine.transition(DebateEvent::SnapshotStarted).unwrap();
        assert_eq!(
            machine
                .transition(DebateEvent::SnapshotReviewRequired)
                .unwrap(),
            DebateState::SnapshotReviewRequired
        );
        assert_eq!(
            machine
                .transition(DebateEvent::SnapshotReviewApproved)
                .unwrap(),
            DebateState::Ready
        );
    }

    #[test]
    fn stale_snapshot_review_returns_to_review_and_rejection_aborts() {
        let mut machine = DebateStateMachine::new(DebateState::Ready);
        assert_eq!(
            machine
                .transition(DebateEvent::SnapshotReviewInvalidated)
                .unwrap(),
            DebateState::SnapshotReviewRequired
        );
        assert_eq!(
            DebateStateMachine::new(DebateState::SnapshotReviewRequired)
                .transition(DebateEvent::SnapshotReviewRejected)
                .unwrap(),
            DebateState::SafetyAbort
        );
        for state in [DebateState::CrossExamination, DebateState::FinalPositions] {
            let mut machine = DebateStateMachine::new(state);
            assert_eq!(
                machine
                    .transition(DebateEvent::SnapshotReviewInvalidated)
                    .unwrap(),
                DebateState::SnapshotReviewRequired
            );
            assert_eq!(
                machine
                    .transition(DebateEvent::SnapshotReviewApproved)
                    .unwrap(),
                DebateState::Ready
            );
        }
    }

    #[test]
    fn review_required_state_can_be_cancelled_but_not_resumed_around_the_gate() {
        let mut machine = DebateStateMachine::new(DebateState::SnapshotReviewRequired);
        assert!(machine.transition(DebateEvent::Resume).is_err());
        assert_eq!(
            machine.transition(DebateEvent::Cancel).unwrap(),
            DebateState::Cancelled
        );
    }
}
