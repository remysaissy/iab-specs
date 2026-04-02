use serde::{Deserialize, Serialize};

/// The current lifecycle status of a seller order.
///
/// This enum defines the complete state machine for an order from creation through completion
/// or cancellation. All serialization uses snake_case format (e.g., `"draft"` for `Draft`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum SellerOrderStatus {
    /// Order is being created and is not yet submitted.
    #[default]
    Draft,

    /// Order has been submitted for review.
    Submitted,

    /// Order is awaiting approval from publisher or buyer.
    PendingApproval,

    /// Order has been approved and is ready for execution.
    Approved,

    /// Order was rejected during approval process.
    Rejected,

    /// Order is currently being executed or synced.
    InProgress,

    /// Order data is being synced to ad server.
    Syncing,

    /// Order has been booked in the ad server.
    Booked,

    /// Order has been paused and is not currently running.
    Paused,

    /// Order has completed all scheduled activity.
    Completed,

    /// Order failed to execute properly.
    Failed,

    /// Order was cancelled before completion.
    Cancelled,

    /// Order validity period has expired.
    Expired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
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

        for variant in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            assert!(
                serialized.starts_with('"') && serialized.ends_with('"'),
                "Serialized value {} should be a JSON string",
                serialized
            );
            let unquoted = &serialized[1..serialized.len() - 1];
            assert!(
                unquoted.chars().all(|c| c.is_lowercase() || c == '_'),
                "Serialized value {} should be snake_case",
                unquoted
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_status\"";
        let result: Result<SellerOrderStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
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

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: SellerOrderStatus =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(
                original, &deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = SellerOrderStatus::default();
        assert_eq!(default, SellerOrderStatus::Draft, "Default should be Draft");
    }
}
