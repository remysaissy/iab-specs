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

    /// Seller Agent 1.0 § SellerOrderStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = SellerOrderStatus::Draft;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, SellerOrderStatus::Draft);
    }

    /// Seller Agent 1.0 § SellerOrderStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(SellerOrderStatus::Draft);
        set.insert(SellerOrderStatus::Submitted);
        set.insert(SellerOrderStatus::PendingApproval);
        set.insert(SellerOrderStatus::Approved);
        set.insert(SellerOrderStatus::Rejected);
        set.insert(SellerOrderStatus::InProgress);
        set.insert(SellerOrderStatus::Syncing);
        set.insert(SellerOrderStatus::Booked);
        set.insert(SellerOrderStatus::Paused);
        set.insert(SellerOrderStatus::Completed);
        set.insert(SellerOrderStatus::Failed);
        set.insert(SellerOrderStatus::Cancelled);
        set.insert(SellerOrderStatus::Expired);

        assert_eq!(set.len(), 13);
        assert!(set.contains(&SellerOrderStatus::Draft));
        assert!(set.contains(&SellerOrderStatus::Expired));
    }

    /// Seller Agent 1.0 § SellerOrderStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(SellerOrderStatus::Draft, SellerOrderStatus::Submitted);
        assert_ne!(
            SellerOrderStatus::Submitted,
            SellerOrderStatus::PendingApproval
        );
        assert_ne!(
            SellerOrderStatus::PendingApproval,
            SellerOrderStatus::Approved
        );
        assert_ne!(SellerOrderStatus::Approved, SellerOrderStatus::Rejected);
        assert_ne!(SellerOrderStatus::Rejected, SellerOrderStatus::InProgress);
        assert_ne!(SellerOrderStatus::InProgress, SellerOrderStatus::Syncing);
        assert_ne!(SellerOrderStatus::Syncing, SellerOrderStatus::Booked);
        assert_ne!(SellerOrderStatus::Booked, SellerOrderStatus::Paused);
        assert_ne!(SellerOrderStatus::Paused, SellerOrderStatus::Completed);
        assert_ne!(SellerOrderStatus::Completed, SellerOrderStatus::Failed);
        assert_ne!(SellerOrderStatus::Failed, SellerOrderStatus::Cancelled);
        assert_ne!(SellerOrderStatus::Cancelled, SellerOrderStatus::Expired);
    }

    /// Seller Agent 1.0 § SellerOrderStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Draft\"", "\"PendingApproval\""];

        for example in &pascal_case_examples {
            let result: Result<SellerOrderStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § SellerOrderStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (SellerOrderStatus::Draft, "\"draft\""),
            (SellerOrderStatus::Submitted, "\"submitted\""),
            (SellerOrderStatus::PendingApproval, "\"pending_approval\""),
            (SellerOrderStatus::Approved, "\"approved\""),
            (SellerOrderStatus::Rejected, "\"rejected\""),
            (SellerOrderStatus::InProgress, "\"in_progress\""),
            (SellerOrderStatus::Syncing, "\"syncing\""),
            (SellerOrderStatus::Booked, "\"booked\""),
            (SellerOrderStatus::Paused, "\"paused\""),
            (SellerOrderStatus::Completed, "\"completed\""),
            (SellerOrderStatus::Failed, "\"failed\""),
            (SellerOrderStatus::Cancelled, "\"cancelled\""),
            (SellerOrderStatus::Expired, "\"expired\""),
        ];

        for (variant, expected_json) in &expected {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(
                &json, expected_json,
                "Mismatch for {:?}: got {}, expected {}",
                variant, json, expected_json
            );
        }
    }
}
