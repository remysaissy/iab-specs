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
}
