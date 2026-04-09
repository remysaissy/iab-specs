use serde::{Deserialize, Serialize};

/// The current state of a Proposal in the Seller Agent workflow.
///
/// Proposals progress through various states from draft through acceptance or rejection.
/// All serialization uses snake_case format (e.g., `"under_review"` for `UnderReview`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    /// Proposal is in draft state and not yet submitted.
    #[default]
    Draft,

    /// Proposal has been submitted to the buyer.
    Submitted,

    /// Proposal is under review by the buyer.
    UnderReview,

    /// Buyer has made a counter-proposal.
    Countered,

    /// Proposal has been accepted.
    Accepted,

    /// Proposal has been rejected.
    Rejected,

    /// Proposal has expired and is no longer valid.
    Expired,

    /// Proposal was withdrawn by the seller.
    Withdrawn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            ProposalStatus::Draft,
            ProposalStatus::Submitted,
            ProposalStatus::UnderReview,
            ProposalStatus::Countered,
            ProposalStatus::Accepted,
            ProposalStatus::Rejected,
            ProposalStatus::Expired,
            ProposalStatus::Withdrawn,
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
        let result: Result<ProposalStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            ProposalStatus::Draft,
            ProposalStatus::Submitted,
            ProposalStatus::UnderReview,
            ProposalStatus::Countered,
            ProposalStatus::Accepted,
            ProposalStatus::Rejected,
            ProposalStatus::Expired,
            ProposalStatus::Withdrawn,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: ProposalStatus =
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
        let default = ProposalStatus::default();
        assert_eq!(default, ProposalStatus::Draft, "Default should be Draft");
    }

    /// Seller Agent 1.0 § ProposalStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = ProposalStatus::Draft;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, ProposalStatus::Draft);
    }

    /// Seller Agent 1.0 § ProposalStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ProposalStatus::Draft);
        set.insert(ProposalStatus::Submitted);
        set.insert(ProposalStatus::UnderReview);
        set.insert(ProposalStatus::Countered);
        set.insert(ProposalStatus::Accepted);
        set.insert(ProposalStatus::Rejected);
        set.insert(ProposalStatus::Expired);
        set.insert(ProposalStatus::Withdrawn);

        assert_eq!(set.len(), 8);
        assert!(set.contains(&ProposalStatus::Draft));
        assert!(set.contains(&ProposalStatus::Withdrawn));
    }

    /// Seller Agent 1.0 § ProposalStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(ProposalStatus::Draft, ProposalStatus::Submitted);
        assert_ne!(ProposalStatus::Submitted, ProposalStatus::UnderReview);
        assert_ne!(ProposalStatus::UnderReview, ProposalStatus::Countered);
        assert_ne!(ProposalStatus::Countered, ProposalStatus::Accepted);
        assert_ne!(ProposalStatus::Accepted, ProposalStatus::Rejected);
        assert_ne!(ProposalStatus::Rejected, ProposalStatus::Expired);
        assert_ne!(ProposalStatus::Expired, ProposalStatus::Withdrawn);
    }

    /// Seller Agent 1.0 § ProposalStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Draft\"", "\"UnderReview\""];

        for example in &pascal_case_examples {
            let result: Result<ProposalStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § ProposalStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (ProposalStatus::Draft, "\"draft\""),
            (ProposalStatus::Submitted, "\"submitted\""),
            (ProposalStatus::UnderReview, "\"under_review\""),
            (ProposalStatus::Countered, "\"countered\""),
            (ProposalStatus::Accepted, "\"accepted\""),
            (ProposalStatus::Rejected, "\"rejected\""),
            (ProposalStatus::Expired, "\"expired\""),
            (ProposalStatus::Withdrawn, "\"withdrawn\""),
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
