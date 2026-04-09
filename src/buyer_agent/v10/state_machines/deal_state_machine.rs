use crate::buyer_agent::v10::enums::DealStatus;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub const VALID_DEAL_TRANSITIONS: &[(DealStatus, DealStatus)] = &[
    (DealStatus::Quoted, DealStatus::Negotiating),
    (DealStatus::Quoted, DealStatus::Accepted),
    (DealStatus::Quoted, DealStatus::Rejected),
    (DealStatus::Quoted, DealStatus::Expired),
    (DealStatus::Quoted, DealStatus::Cancelled),
    (DealStatus::Negotiating, DealStatus::Accepted),
    (DealStatus::Negotiating, DealStatus::Rejected),
    (DealStatus::Negotiating, DealStatus::Cancelled),
    (DealStatus::Accepted, DealStatus::Booking),
    (DealStatus::Accepted, DealStatus::Cancelled),
    (DealStatus::Booking, DealStatus::Booked),
    (DealStatus::Booking, DealStatus::Failed),
    (DealStatus::Booking, DealStatus::Cancelled),
    (DealStatus::Booked, DealStatus::Delivering),
    (DealStatus::Booked, DealStatus::Cancelled),
    (DealStatus::Delivering, DealStatus::Completed),
    (DealStatus::Delivering, DealStatus::MakegoodPending),
    (DealStatus::Delivering, DealStatus::PartiallyCanceled),
    (DealStatus::Delivering, DealStatus::Cancelled),
    (DealStatus::MakegoodPending, DealStatus::Delivering),
    (DealStatus::MakegoodPending, DealStatus::Cancelled),
    (DealStatus::PartiallyCanceled, DealStatus::Cancelled),
    (DealStatus::Failed, DealStatus::Cancelled),
];

pub fn can_transition_deal(from: &DealStatus, to: &DealStatus) -> bool {
    VALID_DEAL_TRANSITIONS
        .iter()
        .any(|(valid_from, valid_to)| valid_from == from && valid_to == to)
}

pub fn valid_deal_transitions_from(state: &DealStatus) -> Vec<DealStatus> {
    VALID_DEAL_TRANSITIONS
        .iter()
        .filter_map(|(from, to)| if from == state { Some(*to) } else { None })
        .collect()
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct DealTransition {
    pub from: DealStatus,

    pub to: DealStatus,

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

impl DealTransition {
    pub fn builder() -> DealTransitionBuilder {
        DealTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_valid_deal_transition_returns_true() {
        for (from, to) in VALID_DEAL_TRANSITIONS.iter().copied() {
            assert!(
                can_transition_deal(&from, &to),
                "expected transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_invalid_deal_transition_for_each_state_returns_false() {
        let invalid_transitions = [
            (DealStatus::Quoted, DealStatus::Completed),
            (DealStatus::Negotiating, DealStatus::Booking),
            (DealStatus::Accepted, DealStatus::Completed),
            (DealStatus::Booking, DealStatus::Delivering),
            (DealStatus::Booked, DealStatus::Completed),
            (DealStatus::Delivering, DealStatus::Accepted),
            (DealStatus::Completed, DealStatus::Quoted),
            (DealStatus::Cancelled, DealStatus::Quoted),
            (DealStatus::Rejected, DealStatus::Quoted),
            (DealStatus::Expired, DealStatus::Quoted),
            (DealStatus::Failed, DealStatus::Completed),
            (DealStatus::MakegoodPending, DealStatus::Completed),
            (DealStatus::PartiallyCanceled, DealStatus::Delivering),
        ];

        for (from, to) in invalid_transitions {
            assert!(
                !can_transition_deal(&from, &to),
                "expected transition {:?} -> {:?} to be invalid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_valid_deal_transitions_from_non_terminal_states() {
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Quoted),
            vec![
                DealStatus::Negotiating,
                DealStatus::Accepted,
                DealStatus::Rejected,
                DealStatus::Expired,
                DealStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Negotiating),
            vec![
                DealStatus::Accepted,
                DealStatus::Rejected,
                DealStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Accepted),
            vec![DealStatus::Booking, DealStatus::Cancelled]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Booking),
            vec![
                DealStatus::Booked,
                DealStatus::Failed,
                DealStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Booked),
            vec![DealStatus::Delivering, DealStatus::Cancelled]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Delivering),
            vec![
                DealStatus::Completed,
                DealStatus::MakegoodPending,
                DealStatus::PartiallyCanceled,
                DealStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::Failed),
            vec![DealStatus::Cancelled]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::MakegoodPending),
            vec![DealStatus::Delivering, DealStatus::Cancelled]
        );
        assert_eq!(
            valid_deal_transitions_from(&DealStatus::PartiallyCanceled),
            vec![DealStatus::Cancelled]
        );
    }

    #[test]
    fn test_terminal_states_have_no_outgoing_transitions() {
        for terminal_state in [
            DealStatus::Completed,
            DealStatus::Cancelled,
            DealStatus::Rejected,
            DealStatus::Expired,
        ] {
            assert!(valid_deal_transitions_from(&terminal_state).is_empty());
        }
    }

    #[test]
    fn test_deal_lifecycle_happy_path() {
        let happy_path = [
            (DealStatus::Quoted, DealStatus::Negotiating),
            (DealStatus::Negotiating, DealStatus::Accepted),
            (DealStatus::Accepted, DealStatus::Booking),
            (DealStatus::Booking, DealStatus::Booked),
            (DealStatus::Booked, DealStatus::Delivering),
            (DealStatus::Delivering, DealStatus::Completed),
        ];

        for (from, to) in happy_path {
            assert!(
                can_transition_deal(&from, &to),
                "expected happy path transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_makegood_pending_transitions() {
        assert!(can_transition_deal(
            &DealStatus::MakegoodPending,
            &DealStatus::Delivering,
        ));
        assert!(!can_transition_deal(
            &DealStatus::MakegoodPending,
            &DealStatus::Completed,
        ));
    }

    #[test]
    fn test_partially_canceled_transitions() {
        assert!(can_transition_deal(
            &DealStatus::PartiallyCanceled,
            &DealStatus::Cancelled,
        ));
        assert!(!can_transition_deal(
            &DealStatus::PartiallyCanceled,
            &DealStatus::Delivering,
        ));
    }

    #[test]
    fn test_deal_transition_builder_and_serde_roundtrip() {
        let original = DealTransition::builder()
            .from(DealStatus::Negotiating)
            .to(DealStatus::Accepted)
            .timestamp("2026-04-01T12:00:00Z")
            .reason("Buyer accepted final rate")
            .actor("buyer-123")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: DealTransition = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed, original);
        assert_eq!(parsed.from, DealStatus::Negotiating);
        assert_eq!(parsed.to, DealStatus::Accepted);
    }

    /// Buyer Agent 1.0 § Deal State Machine — exhaustive 13×13 transition matrix
    #[test]
    fn test_exhaustive_13x13_deal_transition_matrix() {
        let all_states = [
            DealStatus::Quoted,
            DealStatus::Negotiating,
            DealStatus::Accepted,
            DealStatus::Booking,
            DealStatus::Booked,
            DealStatus::Delivering,
            DealStatus::Completed,
            DealStatus::Cancelled,
            DealStatus::Rejected,
            DealStatus::Expired,
            DealStatus::Failed,
            DealStatus::MakegoodPending,
            DealStatus::PartiallyCanceled,
        ];

        for from in &all_states {
            for to in &all_states {
                let expected = VALID_DEAL_TRANSITIONS.contains(&(*from, *to));
                assert_eq!(
                    can_transition_deal(from, to),
                    expected,
                    "Mismatch for {:?} -> {:?}: expected {}, got {}",
                    from,
                    to,
                    expected,
                    !expected
                );
            }
        }
    }

    /// Buyer Agent 1.0 § Deal State Machine — self-transitions are never valid
    #[test]
    fn test_self_transition_deal_always_rejected() {
        let all_states = [
            DealStatus::Quoted,
            DealStatus::Negotiating,
            DealStatus::Accepted,
            DealStatus::Booking,
            DealStatus::Booked,
            DealStatus::Delivering,
            DealStatus::Completed,
            DealStatus::Cancelled,
            DealStatus::Rejected,
            DealStatus::Expired,
            DealStatus::Failed,
            DealStatus::MakegoodPending,
            DealStatus::PartiallyCanceled,
        ];

        for state in &all_states {
            assert!(
                !can_transition_deal(state, state),
                "Self-transition for {:?} should be rejected",
                state
            );
        }
    }

    /// Buyer Agent 1.0 § DealTransition — Default trait produces valid zero-state
    #[test]
    fn test_deal_transition_record_default() {
        let t: DealTransition = DealTransition::default();
        assert_eq!(t.from, DealStatus::Quoted);
        assert_eq!(t.to, DealStatus::Quoted);
        assert!(t.timestamp.is_none());
        assert!(t.reason.is_none());
        assert!(t.actor.is_none());
    }

    /// Buyer Agent 1.0 § DealTransition — optional fields skipped when None
    #[test]
    fn test_deal_transition_optional_fields_none() {
        let t = DealTransition::builder()
            .from(DealStatus::Quoted)
            .to(DealStatus::Negotiating)
            .build()
            .unwrap();

        assert!(t.timestamp.is_none());
        assert!(t.reason.is_none());
        assert!(t.actor.is_none());

        let json = serde_json::to_string(&t).unwrap();
        assert!(!json.contains("timestamp"));
        assert!(!json.contains("reason"));
        assert!(!json.contains("actor"));
    }
}
