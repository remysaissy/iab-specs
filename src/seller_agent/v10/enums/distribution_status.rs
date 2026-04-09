use serde::{Deserialize, Serialize};

/// The current status of a distribution event in the execution workflow.
///
/// This enum tracks the progression of sending campaign details to external parties.
/// All serialization uses snake_case format (e.g., `"pending"` for `Pending`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum DistributionStatus {
    /// Distribution is pending and awaits execution.
    #[default]
    Pending,

    /// Distribution has been sent to the recipient.
    Sent,

    /// Distribution has been confirmed as received.
    Confirmed,

    /// Distribution was rejected by the recipient.
    Rejected,

    /// Distribution validity has expired.
    Expired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            DistributionStatus::Pending,
            DistributionStatus::Sent,
            DistributionStatus::Confirmed,
            DistributionStatus::Rejected,
            DistributionStatus::Expired,
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
        let result: Result<DistributionStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            DistributionStatus::Pending,
            DistributionStatus::Sent,
            DistributionStatus::Confirmed,
            DistributionStatus::Rejected,
            DistributionStatus::Expired,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: DistributionStatus =
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
        let default = DistributionStatus::default();
        assert_eq!(
            default,
            DistributionStatus::Pending,
            "Default should be Pending"
        );
    }

    /// Seller Agent 1.0 § DistributionStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = DistributionStatus::Pending;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, DistributionStatus::Pending);
    }

    /// Seller Agent 1.0 § DistributionStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DistributionStatus::Pending);
        set.insert(DistributionStatus::Sent);
        set.insert(DistributionStatus::Confirmed);
        set.insert(DistributionStatus::Rejected);
        set.insert(DistributionStatus::Expired);

        assert_eq!(set.len(), 5);
        assert!(set.contains(&DistributionStatus::Pending));
        assert!(set.contains(&DistributionStatus::Expired));
    }

    /// Seller Agent 1.0 § DistributionStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(DistributionStatus::Pending, DistributionStatus::Sent);
        assert_ne!(DistributionStatus::Sent, DistributionStatus::Confirmed);
        assert_ne!(DistributionStatus::Confirmed, DistributionStatus::Rejected);
        assert_ne!(DistributionStatus::Rejected, DistributionStatus::Expired);
    }

    /// Seller Agent 1.0 § DistributionStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Pending\"", "\"Confirmed\""];

        for example in &pascal_case_examples {
            let result: Result<DistributionStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § DistributionStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (DistributionStatus::Pending, "\"pending\""),
            (DistributionStatus::Sent, "\"sent\""),
            (DistributionStatus::Confirmed, "\"confirmed\""),
            (DistributionStatus::Rejected, "\"rejected\""),
            (DistributionStatus::Expired, "\"expired\""),
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
