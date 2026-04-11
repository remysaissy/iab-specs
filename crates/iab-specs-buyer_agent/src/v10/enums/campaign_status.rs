use serde::{Deserialize, Serialize};

/// The current state of a Campaign in the Buyer Agent workflow.
///
/// Campaigns progress through various states from initialization through completion or failure.
/// All serialization uses snake_case format (e.g., `"brief_received"` for `BriefReceived`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    /// Campaign has been initialized but not yet started.
    #[default]
    Initialized,

    /// Campaign brief has been received and processed.
    BriefReceived,

    /// Budget has been allocated to the campaign.
    BudgetAllocated,

    /// Campaign is in research phase, analyzing opportunities.
    Researching,

    /// Campaign is awaiting approval to proceed to execution.
    AwaitingApproval,

    /// Campaign is actively executing bookings.
    ExecutingBookings,

    /// Campaign has successfully completed all activities.
    Completed,

    /// Campaign failed to complete as planned.
    Failed,

    /// Campaign was cancelled before completion.
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            CampaignStatus::Initialized,
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated,
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings,
            CampaignStatus::Completed,
            CampaignStatus::Failed,
            CampaignStatus::Cancelled,
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
        let result: Result<CampaignStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            CampaignStatus::Initialized,
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated,
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings,
            CampaignStatus::Completed,
            CampaignStatus::Failed,
            CampaignStatus::Cancelled,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: CampaignStatus =
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
        let default = CampaignStatus::default();
        assert_eq!(
            default,
            CampaignStatus::Initialized,
            "Default should be Initialized"
        );
    }

    /// Buyer Agent 1.0 § CampaignStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = CampaignStatus::BriefReceived;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, CampaignStatus::BriefReceived);
    }

    /// Buyer Agent 1.0 § CampaignStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(CampaignStatus::Initialized);
        set.insert(CampaignStatus::BriefReceived);
        set.insert(CampaignStatus::BudgetAllocated);
        set.insert(CampaignStatus::Researching);
        set.insert(CampaignStatus::AwaitingApproval);
        set.insert(CampaignStatus::ExecutingBookings);
        set.insert(CampaignStatus::Completed);
        set.insert(CampaignStatus::Failed);
        set.insert(CampaignStatus::Cancelled);

        assert_eq!(set.len(), 9);
        assert!(set.contains(&CampaignStatus::Initialized));
        assert!(set.contains(&CampaignStatus::Failed));
    }

    /// Buyer Agent 1.0 § CampaignStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(CampaignStatus::Initialized, CampaignStatus::BriefReceived);
        assert_ne!(
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated
        );
        assert_ne!(CampaignStatus::BudgetAllocated, CampaignStatus::Researching);
        assert_ne!(
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval
        );
        assert_ne!(
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings
        );
        assert_ne!(CampaignStatus::ExecutingBookings, CampaignStatus::Completed);
        assert_ne!(CampaignStatus::Completed, CampaignStatus::Failed);
        assert_ne!(CampaignStatus::Failed, CampaignStatus::Cancelled);
    }

    /// Buyer Agent 1.0 § CampaignStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"BriefReceived\"", "\"Initialized\""];

        for example in &pascal_case_examples {
            let result: Result<CampaignStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Buyer Agent 1.0 § CampaignStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (CampaignStatus::Initialized, "\"initialized\""),
            (CampaignStatus::BriefReceived, "\"brief_received\""),
            (CampaignStatus::BudgetAllocated, "\"budget_allocated\""),
            (CampaignStatus::Researching, "\"researching\""),
            (CampaignStatus::AwaitingApproval, "\"awaiting_approval\""),
            (CampaignStatus::ExecutingBookings, "\"executing_bookings\""),
            (CampaignStatus::Completed, "\"completed\""),
            (CampaignStatus::Failed, "\"failed\""),
            (CampaignStatus::Cancelled, "\"cancelled\""),
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
