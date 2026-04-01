use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::OrderStatus;

pub const VALID_ORDER_TRANSITIONS: &[(OrderStatus, OrderStatus)] = &[
    (OrderStatus::Draft, OrderStatus::PendingReview),
    (OrderStatus::Draft, OrderStatus::Cancelled),
    (OrderStatus::PendingReview, OrderStatus::Approved),
    (OrderStatus::PendingReview, OrderStatus::Rejected),
    (OrderStatus::PendingReview, OrderStatus::Cancelled),
    (OrderStatus::Approved, OrderStatus::InProgress),
    (OrderStatus::Approved, OrderStatus::Cancelled),
    (OrderStatus::InProgress, OrderStatus::Paused),
    (OrderStatus::InProgress, OrderStatus::Completed),
    (OrderStatus::InProgress, OrderStatus::Cancelled),
    (OrderStatus::Paused, OrderStatus::InProgress),
    (OrderStatus::Paused, OrderStatus::Cancelled),
    (OrderStatus::Rejected, OrderStatus::Cancelled),
];

pub fn can_transition_order(from: &OrderStatus, to: &OrderStatus) -> bool {
    VALID_ORDER_TRANSITIONS
        .iter()
        .any(|(f, t)| f == from && t == to)
}

pub fn valid_order_transitions_from(state: &OrderStatus) -> Vec<OrderStatus> {
    VALID_ORDER_TRANSITIONS
        .iter()
        .filter(|(f, _)| f == state)
        .map(|(_, t)| *t)
        .collect()
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct OrderTransition {
    pub from: OrderStatus,
    pub to: OrderStatus,
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

impl OrderTransition {
    pub fn builder() -> OrderTransitionBuilder {
        OrderTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_transitions() {
        let valid_pairs = vec![
            (OrderStatus::Draft, OrderStatus::PendingReview),
            (OrderStatus::Draft, OrderStatus::Cancelled),
            (OrderStatus::PendingReview, OrderStatus::Approved),
            (OrderStatus::PendingReview, OrderStatus::Rejected),
            (OrderStatus::PendingReview, OrderStatus::Cancelled),
            (OrderStatus::Approved, OrderStatus::InProgress),
            (OrderStatus::Approved, OrderStatus::Cancelled),
            (OrderStatus::InProgress, OrderStatus::Paused),
            (OrderStatus::InProgress, OrderStatus::Completed),
            (OrderStatus::InProgress, OrderStatus::Cancelled),
            (OrderStatus::Paused, OrderStatus::InProgress),
            (OrderStatus::Paused, OrderStatus::Cancelled),
            (OrderStatus::Rejected, OrderStatus::Cancelled),
        ];

        for (from, to) in &valid_pairs {
            assert!(
                can_transition_order(from, to),
                "Expected valid transition from {:?} to {:?}",
                from,
                to
            );
        }

        assert_eq!(valid_pairs.len(), VALID_ORDER_TRANSITIONS.len());
    }

    #[test]
    fn test_invalid_transition_per_state() {
        // Draft cannot go directly to InProgress
        assert!(!can_transition_order(
            &OrderStatus::Draft,
            &OrderStatus::InProgress
        ));

        // PendingReview cannot go to InProgress
        assert!(!can_transition_order(
            &OrderStatus::PendingReview,
            &OrderStatus::InProgress
        ));

        // Approved cannot go to Draft
        assert!(!can_transition_order(
            &OrderStatus::Approved,
            &OrderStatus::Draft
        ));

        // InProgress cannot go to Draft
        assert!(!can_transition_order(
            &OrderStatus::InProgress,
            &OrderStatus::Draft
        ));

        // Paused cannot go to Completed
        assert!(!can_transition_order(
            &OrderStatus::Paused,
            &OrderStatus::Completed
        ));

        // Rejected cannot go to Approved
        assert!(!can_transition_order(
            &OrderStatus::Rejected,
            &OrderStatus::Approved
        ));
    }

    #[test]
    fn test_terminal_states_have_no_transitions() {
        let terminal_states = vec![
            OrderStatus::Completed,
            OrderStatus::Cancelled,
            OrderStatus::Expired,
        ];

        for state in &terminal_states {
            let transitions = valid_order_transitions_from(state);
            assert!(
                transitions.is_empty(),
                "Terminal state {:?} should have no outgoing transitions, but found: {:?}",
                state,
                transitions
            );
        }
    }

    #[test]
    fn test_happy_path() {
        let path = vec![
            OrderStatus::Draft,
            OrderStatus::PendingReview,
            OrderStatus::Approved,
            OrderStatus::InProgress,
            OrderStatus::Completed,
        ];

        for window in path.windows(2) {
            assert!(
                can_transition_order(&window[0], &window[1]),
                "Happy path transition from {:?} to {:?} should be valid",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn test_pause_resume() {
        assert!(can_transition_order(
            &OrderStatus::InProgress,
            &OrderStatus::Paused
        ));
        assert!(can_transition_order(
            &OrderStatus::Paused,
            &OrderStatus::InProgress
        ));
    }

    #[test]
    fn test_transition_struct_roundtrip() {
        let transition = OrderTransition::builder()
            .from(OrderStatus::Draft)
            .to(OrderStatus::PendingReview)
            .timestamp("2025-03-31T12:00:00Z")
            .reason("Ready for review")
            .actor("user-001")
            .build()
            .unwrap();

        let json = serde_json::to_string(&transition).unwrap();
        let parsed: OrderTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(transition, parsed);
    }
}
