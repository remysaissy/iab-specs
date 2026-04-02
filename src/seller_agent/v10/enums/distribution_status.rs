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
}
