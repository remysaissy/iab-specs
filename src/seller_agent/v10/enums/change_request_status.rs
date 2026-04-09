use serde::{Deserialize, Serialize};

/// The current status of a change request in the workflow.
///
/// This enum tracks the progression of a change request from submission to completion.
/// All serialization uses snake_case format (e.g., `"pending"` for `Pending`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChangeRequestStatus {
    /// Change request has been submitted and awaits review.
    #[default]
    Pending,

    /// Change request has been reviewed and approved.
    Approved,

    /// Change request has been reviewed and rejected.
    Rejected,

    /// Approved change has been applied to the order.
    Applied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            ChangeRequestStatus::Pending,
            ChangeRequestStatus::Approved,
            ChangeRequestStatus::Rejected,
            ChangeRequestStatus::Applied,
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
        let result: Result<ChangeRequestStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            ChangeRequestStatus::Pending,
            ChangeRequestStatus::Approved,
            ChangeRequestStatus::Rejected,
            ChangeRequestStatus::Applied,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: ChangeRequestStatus =
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
        let default = ChangeRequestStatus::default();
        assert_eq!(
            default,
            ChangeRequestStatus::Pending,
            "Default should be Pending"
        );
    }

    /// Seller Agent 1.0 § ChangeRequestStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = ChangeRequestStatus::Pending;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, ChangeRequestStatus::Pending);
    }

    /// Seller Agent 1.0 § ChangeRequestStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ChangeRequestStatus::Pending);
        set.insert(ChangeRequestStatus::Approved);
        set.insert(ChangeRequestStatus::Rejected);
        set.insert(ChangeRequestStatus::Applied);

        assert_eq!(set.len(), 4);
        assert!(set.contains(&ChangeRequestStatus::Pending));
        assert!(set.contains(&ChangeRequestStatus::Applied));
    }

    /// Seller Agent 1.0 § ChangeRequestStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(ChangeRequestStatus::Pending, ChangeRequestStatus::Approved);
        assert_ne!(ChangeRequestStatus::Approved, ChangeRequestStatus::Rejected);
        assert_ne!(ChangeRequestStatus::Rejected, ChangeRequestStatus::Applied);
    }

    /// Seller Agent 1.0 § ChangeRequestStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Pending\"", "\"Approved\""];

        for example in &pascal_case_examples {
            let result: Result<ChangeRequestStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § ChangeRequestStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (ChangeRequestStatus::Pending, "\"pending\""),
            (ChangeRequestStatus::Approved, "\"approved\""),
            (ChangeRequestStatus::Rejected, "\"rejected\""),
            (ChangeRequestStatus::Applied, "\"applied\""),
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
