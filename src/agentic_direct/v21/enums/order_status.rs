use serde::{Deserialize, Serialize};

/// Status of an order in the Agentic Direct workflow.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// Order is in draft state, not yet submitted for approval.
    #[default]
    Draft,
    /// Order is pending review by the counterparty.
    PendingReview,
    /// Order has been approved and is ready for execution.
    Approved,
    /// Order is currently being executed.
    InProgress,
    /// Order has been paused temporarily.
    Paused,
    /// Order has been completed successfully.
    Completed,
    /// Order was cancelled.
    Cancelled,
    /// Order was rejected by the counterparty.
    Rejected,
    /// Order has expired.
    Expired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("draft", OrderStatus::Draft),
            ("pending_review", OrderStatus::PendingReview),
            ("approved", OrderStatus::Approved),
            ("in_progress", OrderStatus::InProgress),
            ("paused", OrderStatus::Paused),
            ("completed", OrderStatus::Completed),
            ("cancelled", OrderStatus::Cancelled),
            ("rejected", OrderStatus::Rejected),
            ("expired", OrderStatus::Expired),
        ];

        for (json_str, expected) in values {
            let result: OrderStatus = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<OrderStatus, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            OrderStatus::Draft,
            OrderStatus::PendingReview,
            OrderStatus::Approved,
            OrderStatus::InProgress,
            OrderStatus::Paused,
            OrderStatus::Completed,
            OrderStatus::Cancelled,
            OrderStatus::Rejected,
            OrderStatus::Expired,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: OrderStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = OrderStatus::default();
        assert_eq!(default, OrderStatus::Draft, "Default should be Draft");
    }

    #[test]
    fn test_case_sensitive_deserialization() {
        // Spec: Agentic Direct 2.1 — snake_case serialization is mandatory
        let invalid = ["\"Draft\"", "\"PENDING_REVIEW\""];
        for json in &invalid {
            assert!(
                serde_json::from_str::<OrderStatus>(json).is_err(),
                "{} should be rejected",
                json
            );
        }
    }
}
