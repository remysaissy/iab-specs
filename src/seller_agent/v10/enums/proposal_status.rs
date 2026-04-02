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
}
