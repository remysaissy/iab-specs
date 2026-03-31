use serde::{Deserialize, Serialize};

/// Status of a creative asset.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum CreativeStatus {
    /// Creative is in draft state.
    #[default]
    Draft,
    /// Creative is pending approval.
    PendingApproval,
    /// Creative has been approved.
    Approved,
    /// Creative was rejected.
    Rejected,
    /// Creative is actively running.
    Active,
    /// Creative has been paused.
    Paused,
    /// Creative has been archived.
    Archived,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("draft", CreativeStatus::Draft),
            ("pending_approval", CreativeStatus::PendingApproval),
            ("approved", CreativeStatus::Approved),
            ("rejected", CreativeStatus::Rejected),
            ("active", CreativeStatus::Active),
            ("paused", CreativeStatus::Paused),
            ("archived", CreativeStatus::Archived),
        ];

        for (json_str, expected) in values {
            let result: CreativeStatus =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<CreativeStatus, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            CreativeStatus::Draft,
            CreativeStatus::PendingApproval,
            CreativeStatus::Approved,
            CreativeStatus::Rejected,
            CreativeStatus::Active,
            CreativeStatus::Paused,
            CreativeStatus::Archived,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CreativeStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = CreativeStatus::default();
        assert_eq!(default, CreativeStatus::Draft, "Default should be Draft");
    }
}
