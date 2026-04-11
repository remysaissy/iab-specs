use crate::v10::enums::SellerOrderStatus;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub const VALID_SELLER_ORDER_TRANSITIONS: &[(SellerOrderStatus, SellerOrderStatus)] = &[
    (SellerOrderStatus::Draft, SellerOrderStatus::Submitted),
    (SellerOrderStatus::Draft, SellerOrderStatus::Cancelled),
    (
        SellerOrderStatus::Submitted,
        SellerOrderStatus::PendingApproval,
    ),
    (SellerOrderStatus::Submitted, SellerOrderStatus::Rejected),
    (SellerOrderStatus::Submitted, SellerOrderStatus::Cancelled),
    (
        SellerOrderStatus::PendingApproval,
        SellerOrderStatus::Approved,
    ),
    (
        SellerOrderStatus::PendingApproval,
        SellerOrderStatus::Rejected,
    ),
    (
        SellerOrderStatus::PendingApproval,
        SellerOrderStatus::Cancelled,
    ),
    (SellerOrderStatus::Approved, SellerOrderStatus::InProgress),
    (SellerOrderStatus::Approved, SellerOrderStatus::Cancelled),
    (SellerOrderStatus::Rejected, SellerOrderStatus::Cancelled),
    (SellerOrderStatus::InProgress, SellerOrderStatus::Syncing),
    (SellerOrderStatus::InProgress, SellerOrderStatus::Paused),
    (SellerOrderStatus::InProgress, SellerOrderStatus::Cancelled),
    (SellerOrderStatus::InProgress, SellerOrderStatus::Expired),
    (SellerOrderStatus::Syncing, SellerOrderStatus::Booked),
    (SellerOrderStatus::Syncing, SellerOrderStatus::Failed),
    (SellerOrderStatus::Syncing, SellerOrderStatus::Cancelled),
    (SellerOrderStatus::Booked, SellerOrderStatus::InProgress),
    (SellerOrderStatus::Booked, SellerOrderStatus::Paused),
    (SellerOrderStatus::Booked, SellerOrderStatus::Completed),
    (SellerOrderStatus::Booked, SellerOrderStatus::Cancelled),
    (SellerOrderStatus::Paused, SellerOrderStatus::InProgress),
    (SellerOrderStatus::Paused, SellerOrderStatus::Cancelled),
    (SellerOrderStatus::Failed, SellerOrderStatus::InProgress),
    (SellerOrderStatus::Failed, SellerOrderStatus::Cancelled),
];

pub fn can_transition_seller_order(from: &SellerOrderStatus, to: &SellerOrderStatus) -> bool {
    VALID_SELLER_ORDER_TRANSITIONS
        .iter()
        .any(|(valid_from, valid_to)| valid_from == from && valid_to == to)
}

pub fn valid_seller_order_transitions_from(state: &SellerOrderStatus) -> Vec<SellerOrderStatus> {
    VALID_SELLER_ORDER_TRANSITIONS
        .iter()
        .filter_map(|(from, to)| if from == state { Some(*to) } else { None })
        .collect()
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct SellerOrderTransition {
    pub from: SellerOrderStatus,

    pub to: SellerOrderStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub actor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub audit_note: Option<String>,
}

impl SellerOrderTransition {
    pub fn builder() -> SellerOrderTransitionBuilder {
        SellerOrderTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_valid_seller_order_transition_returns_true() {
        for (from, to) in VALID_SELLER_ORDER_TRANSITIONS.iter().copied() {
            assert!(
                can_transition_seller_order(&from, &to),
                "expected transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_invalid_seller_order_transition_for_each_state_returns_false() {
        let invalid_transitions = [
            (SellerOrderStatus::Draft, SellerOrderStatus::Completed),
            (SellerOrderStatus::Submitted, SellerOrderStatus::Booked),
            (
                SellerOrderStatus::PendingApproval,
                SellerOrderStatus::Completed,
            ),
            (SellerOrderStatus::Approved, SellerOrderStatus::Completed),
            (SellerOrderStatus::Rejected, SellerOrderStatus::Approved),
            (SellerOrderStatus::InProgress, SellerOrderStatus::Completed),
            (SellerOrderStatus::Syncing, SellerOrderStatus::Completed),
            (SellerOrderStatus::Booked, SellerOrderStatus::Syncing),
            (SellerOrderStatus::Paused, SellerOrderStatus::Completed),
            (SellerOrderStatus::Completed, SellerOrderStatus::Draft),
            (SellerOrderStatus::Failed, SellerOrderStatus::Completed),
            (SellerOrderStatus::Cancelled, SellerOrderStatus::Draft),
            (SellerOrderStatus::Expired, SellerOrderStatus::Draft),
        ];

        for (from, to) in invalid_transitions {
            assert!(
                !can_transition_seller_order(&from, &to),
                "expected transition {:?} -> {:?} to be invalid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_valid_seller_order_transitions_from_non_terminal_states() {
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Draft),
            vec![SellerOrderStatus::Submitted, SellerOrderStatus::Cancelled,]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Submitted),
            vec![
                SellerOrderStatus::PendingApproval,
                SellerOrderStatus::Rejected,
                SellerOrderStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::PendingApproval),
            vec![
                SellerOrderStatus::Approved,
                SellerOrderStatus::Rejected,
                SellerOrderStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Approved),
            vec![SellerOrderStatus::InProgress, SellerOrderStatus::Cancelled,]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Rejected),
            vec![SellerOrderStatus::Cancelled]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::InProgress),
            vec![
                SellerOrderStatus::Syncing,
                SellerOrderStatus::Paused,
                SellerOrderStatus::Cancelled,
                SellerOrderStatus::Expired,
            ]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Syncing),
            vec![
                SellerOrderStatus::Booked,
                SellerOrderStatus::Failed,
                SellerOrderStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Booked),
            vec![
                SellerOrderStatus::InProgress,
                SellerOrderStatus::Paused,
                SellerOrderStatus::Completed,
                SellerOrderStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Paused),
            vec![SellerOrderStatus::InProgress, SellerOrderStatus::Cancelled,]
        );
        assert_eq!(
            valid_seller_order_transitions_from(&SellerOrderStatus::Failed),
            vec![SellerOrderStatus::InProgress, SellerOrderStatus::Cancelled,]
        );
    }

    #[test]
    fn test_terminal_states_have_no_outgoing_transitions() {
        for terminal_state in [
            SellerOrderStatus::Completed,
            SellerOrderStatus::Cancelled,
            SellerOrderStatus::Expired,
        ] {
            assert!(
                valid_seller_order_transitions_from(&terminal_state).is_empty(),
                "expected {:?} to have no outgoing transitions",
                terminal_state
            );
        }
    }

    #[test]
    fn test_seller_order_lifecycle_happy_path() {
        let happy_path = [
            (SellerOrderStatus::Draft, SellerOrderStatus::Submitted),
            (
                SellerOrderStatus::Submitted,
                SellerOrderStatus::PendingApproval,
            ),
            (
                SellerOrderStatus::PendingApproval,
                SellerOrderStatus::Approved,
            ),
            (SellerOrderStatus::Approved, SellerOrderStatus::InProgress),
            (SellerOrderStatus::InProgress, SellerOrderStatus::Syncing),
            (SellerOrderStatus::Syncing, SellerOrderStatus::Booked),
            (SellerOrderStatus::Booked, SellerOrderStatus::Completed),
        ];

        for (from, to) in happy_path {
            assert!(
                can_transition_seller_order(&from, &to),
                "expected happy path transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_retry_after_sync_failure() {
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Syncing,
            &SellerOrderStatus::Failed,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Failed,
            &SellerOrderStatus::InProgress,
        ));
        assert!(!can_transition_seller_order(
            &SellerOrderStatus::Failed,
            &SellerOrderStatus::Completed,
        ));
    }

    #[test]
    fn test_seller_order_transition_builder_and_serde_roundtrip() {
        let original = SellerOrderTransition::builder()
            .from(SellerOrderStatus::Submitted)
            .to(SellerOrderStatus::PendingApproval)
            .timestamp("2026-04-01T12:00:00Z")
            .reason("Order submitted for publisher review")
            .actor("seller-agent-001")
            .audit_note("Auto-submitted after brief validation")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: SellerOrderTransition = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed, original);
        assert_eq!(parsed.from, SellerOrderStatus::Submitted);
        assert_eq!(parsed.to, SellerOrderStatus::PendingApproval);
    }

    #[test]
    fn test_pause_resume_cycle() {
        assert!(can_transition_seller_order(
            &SellerOrderStatus::InProgress,
            &SellerOrderStatus::Paused,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Paused,
            &SellerOrderStatus::InProgress,
        ));
    }

    /// Seller Agent 1.0 § SellerOrder State Machine — exhaustive 13×13 transition matrix
    #[test]
    fn test_exhaustive_13x13_seller_order_transition_matrix() {
        let all_states = [
            SellerOrderStatus::Draft,
            SellerOrderStatus::Submitted,
            SellerOrderStatus::PendingApproval,
            SellerOrderStatus::Approved,
            SellerOrderStatus::Rejected,
            SellerOrderStatus::InProgress,
            SellerOrderStatus::Syncing,
            SellerOrderStatus::Booked,
            SellerOrderStatus::Paused,
            SellerOrderStatus::Completed,
            SellerOrderStatus::Failed,
            SellerOrderStatus::Cancelled,
            SellerOrderStatus::Expired,
        ];

        for from in &all_states {
            for to in &all_states {
                let expected = VALID_SELLER_ORDER_TRANSITIONS.contains(&(*from, *to));
                assert_eq!(
                    can_transition_seller_order(from, to),
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

    /// Seller Agent 1.0 § SellerOrder State Machine — self-transitions are never valid
    #[test]
    fn test_self_transition_seller_order_always_rejected() {
        let all_states = [
            SellerOrderStatus::Draft,
            SellerOrderStatus::Submitted,
            SellerOrderStatus::PendingApproval,
            SellerOrderStatus::Approved,
            SellerOrderStatus::Rejected,
            SellerOrderStatus::InProgress,
            SellerOrderStatus::Syncing,
            SellerOrderStatus::Booked,
            SellerOrderStatus::Paused,
            SellerOrderStatus::Completed,
            SellerOrderStatus::Failed,
            SellerOrderStatus::Cancelled,
            SellerOrderStatus::Expired,
        ];

        for state in &all_states {
            assert!(
                !can_transition_seller_order(state, state),
                "Self-transition for {:?} should be rejected",
                state
            );
        }
    }

    /// Seller Agent 1.0 § SellerOrder State Machine — Default trait produces valid zero-state
    #[test]
    fn test_seller_order_transition_record_default() {
        let t = SellerOrderTransition::default();
        assert_eq!(t.from, SellerOrderStatus::Draft);
        assert_eq!(t.to, SellerOrderStatus::Draft);
        assert!(t.timestamp.is_none());
        assert!(t.reason.is_none());
        assert!(t.actor.is_none());
        assert!(t.audit_note.is_none());
    }

    /// Seller Agent 1.0 § SellerOrder State Machine — optional fields skipped when None
    #[test]
    fn test_seller_order_transition_optional_fields_none() {
        let t = SellerOrderTransition::builder()
            .from(SellerOrderStatus::Submitted)
            .to(SellerOrderStatus::PendingApproval)
            .build()
            .unwrap();

        assert!(t.timestamp.is_none());
        assert!(t.reason.is_none());
        assert!(t.actor.is_none());
        assert!(t.audit_note.is_none());

        let json = serde_json::to_string(&t).unwrap();
        assert!(!json.contains("timestamp"));
        assert!(!json.contains("reason"));
        assert!(!json.contains("actor"));
        assert!(!json.contains("audit_note"));
    }

    /// Seller Agent 1.0 § SellerOrder State Machine — Booked pause/resume full cycle
    #[test]
    fn test_booked_pause_resume_cycle() {
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Booked,
            &SellerOrderStatus::Paused,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Paused,
            &SellerOrderStatus::InProgress,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::InProgress,
            &SellerOrderStatus::Syncing,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Syncing,
            &SellerOrderStatus::Booked,
        ));
    }

    /// Seller Agent 1.0 § SellerOrder State Machine — Failed retry full recovery cycle
    #[test]
    fn test_failed_retry_full_cycle() {
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Failed,
            &SellerOrderStatus::InProgress,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::InProgress,
            &SellerOrderStatus::Syncing,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Syncing,
            &SellerOrderStatus::Booked,
        ));
        assert!(can_transition_seller_order(
            &SellerOrderStatus::Booked,
            &SellerOrderStatus::Completed,
        ));
    }
}
