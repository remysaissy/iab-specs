use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::LineStatus;

pub const VALID_LINE_TRANSITIONS: &[(LineStatus, LineStatus)] = &[
    (LineStatus::Draft, LineStatus::PendingReview),
    (LineStatus::Draft, LineStatus::Cancelled),
    (LineStatus::PendingReview, LineStatus::Reserved),
    (LineStatus::PendingReview, LineStatus::Rejected),
    (LineStatus::PendingReview, LineStatus::Cancelled),
    (LineStatus::Reserved, LineStatus::Booked),
    (LineStatus::Reserved, LineStatus::Cancelled),
    (LineStatus::Booked, LineStatus::InProgress),
    (LineStatus::Booked, LineStatus::Cancelled),
    (LineStatus::InProgress, LineStatus::Paused),
    (LineStatus::InProgress, LineStatus::Completed),
    (LineStatus::InProgress, LineStatus::Cancelled),
    (LineStatus::Paused, LineStatus::InProgress),
    (LineStatus::Paused, LineStatus::Cancelled),
    (LineStatus::Rejected, LineStatus::Cancelled),
];

pub fn can_transition_line(from: &LineStatus, to: &LineStatus) -> bool {
    VALID_LINE_TRANSITIONS
        .iter()
        .any(|(f, t)| f == from && t == to)
}

pub fn valid_line_transitions_from(state: &LineStatus) -> Vec<LineStatus> {
    VALID_LINE_TRANSITIONS
        .iter()
        .filter(|(f, _)| f == state)
        .map(|(_, t)| *t)
        .collect()
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct LineTransition {
    pub from: LineStatus,
    pub to: LineStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub actor: Option<String>,
}

impl LineTransition {
    pub fn builder() -> LineTransitionBuilder {
        LineTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_transitions() {
        let valid_pairs = vec![
            (LineStatus::Draft, LineStatus::PendingReview),
            (LineStatus::Draft, LineStatus::Cancelled),
            (LineStatus::PendingReview, LineStatus::Reserved),
            (LineStatus::PendingReview, LineStatus::Rejected),
            (LineStatus::PendingReview, LineStatus::Cancelled),
            (LineStatus::Reserved, LineStatus::Booked),
            (LineStatus::Reserved, LineStatus::Cancelled),
            (LineStatus::Booked, LineStatus::InProgress),
            (LineStatus::Booked, LineStatus::Cancelled),
            (LineStatus::InProgress, LineStatus::Paused),
            (LineStatus::InProgress, LineStatus::Completed),
            (LineStatus::InProgress, LineStatus::Cancelled),
            (LineStatus::Paused, LineStatus::InProgress),
            (LineStatus::Paused, LineStatus::Cancelled),
            (LineStatus::Rejected, LineStatus::Cancelled),
        ];

        for (from, to) in &valid_pairs {
            assert!(
                can_transition_line(from, to),
                "Expected valid transition from {:?} to {:?}",
                from,
                to
            );
        }

        assert_eq!(valid_pairs.len(), VALID_LINE_TRANSITIONS.len());
    }

    #[test]
    fn test_invalid_transition_per_state() {
        // Draft cannot go directly to InProgress
        assert!(!can_transition_line(
            &LineStatus::Draft,
            &LineStatus::InProgress
        ));

        // PendingReview cannot go to InProgress
        assert!(!can_transition_line(
            &LineStatus::PendingReview,
            &LineStatus::InProgress
        ));

        // Reserved cannot go to Draft
        assert!(!can_transition_line(
            &LineStatus::Reserved,
            &LineStatus::Draft
        ));

        // Booked cannot go to Draft
        assert!(!can_transition_line(
            &LineStatus::Booked,
            &LineStatus::Draft
        ));

        // InProgress cannot go to Draft
        assert!(!can_transition_line(
            &LineStatus::InProgress,
            &LineStatus::Draft
        ));

        // Paused cannot go to Completed
        assert!(!can_transition_line(
            &LineStatus::Paused,
            &LineStatus::Completed
        ));

        // Rejected cannot go to Reserved
        assert!(!can_transition_line(
            &LineStatus::Rejected,
            &LineStatus::Reserved
        ));
    }

    #[test]
    fn test_terminal_states_have_no_transitions() {
        let terminal_states = vec![LineStatus::Completed, LineStatus::Cancelled];

        for state in &terminal_states {
            let transitions = valid_line_transitions_from(state);
            assert!(
                transitions.is_empty(),
                "Terminal state {:?} should have no outgoing transitions, but found: {:?}",
                state,
                transitions
            );
        }

        let rejected_transitions = valid_line_transitions_from(&LineStatus::Rejected);
        assert_eq!(rejected_transitions, vec![LineStatus::Cancelled]);
    }

    #[test]
    fn test_happy_path() {
        let path = vec![
            LineStatus::Draft,
            LineStatus::PendingReview,
            LineStatus::Reserved,
            LineStatus::Booked,
            LineStatus::InProgress,
            LineStatus::Completed,
        ];

        for window in path.windows(2) {
            assert!(
                can_transition_line(&window[0], &window[1]),
                "Happy path transition from {:?} to {:?} should be valid",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn test_pause_resume() {
        assert!(can_transition_line(
            &LineStatus::InProgress,
            &LineStatus::Paused
        ));
        assert!(can_transition_line(
            &LineStatus::Paused,
            &LineStatus::InProgress
        ));
    }

    #[test]
    fn test_transition_struct_roundtrip() {
        let transition = LineTransition::builder()
            .from(LineStatus::Draft)
            .to(LineStatus::PendingReview)
            .timestamp("2025-03-31T12:00:00Z")
            .reason("Ready for review")
            .actor("user-001")
            .build()
            .unwrap();

        let json = serde_json::to_string(&transition).unwrap();
        let parsed: LineTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(transition, parsed);
    }
}
