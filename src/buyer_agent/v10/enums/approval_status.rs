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
}
