use serde::{Deserialize, Serialize};

/// The approval status of a booking or strategy within the Buyer Agent workflow.
///
/// Approval statuses track the review and acceptance state of campaigns and bookings.
/// All serialization uses snake_case format (e.g., `"pending"` for `Pending`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    /// Awaiting approval or review.
    #[default]
    Pending,

    /// Has been approved and is authorized to proceed.
    Approved,

    /// Has been rejected and cannot proceed.
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            ApprovalStatus::Pending,
            ApprovalStatus::Approved,
            ApprovalStatus::Rejected,
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
        let result: Result<ApprovalStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            ApprovalStatus::Pending,
            ApprovalStatus::Approved,
            ApprovalStatus::Rejected,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: ApprovalStatus =
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
        let default = ApprovalStatus::default();
        assert_eq!(
            default,
            ApprovalStatus::Pending,
            "Default should be Pending"
        );
    }

    /// Buyer Agent 1.0 § ApprovalStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = ApprovalStatus::Approved;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, ApprovalStatus::Approved);
    }

    /// Buyer Agent 1.0 § ApprovalStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ApprovalStatus::Pending);
        set.insert(ApprovalStatus::Approved);
        set.insert(ApprovalStatus::Rejected);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&ApprovalStatus::Pending));
        assert!(set.contains(&ApprovalStatus::Rejected));
    }

    /// Buyer Agent 1.0 § ApprovalStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(ApprovalStatus::Pending, ApprovalStatus::Approved);
        assert_ne!(ApprovalStatus::Approved, ApprovalStatus::Rejected);
        assert_ne!(ApprovalStatus::Rejected, ApprovalStatus::Pending);
    }

    /// Buyer Agent 1.0 § ApprovalStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Approved\"", "\"Pending\""];

        for example in &pascal_case_examples {
            let result: Result<ApprovalStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Buyer Agent 1.0 § ApprovalStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (ApprovalStatus::Pending, "\"pending\""),
            (ApprovalStatus::Approved, "\"approved\""),
            (ApprovalStatus::Rejected, "\"rejected\""),
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
