use serde::{Deserialize, Serialize};

/// Status of an assignment in the Agentic Direct workflow.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentStatus {
    /// Assignment is in draft state.
    #[default]
    Draft,
    /// Assignment is active.
    Active,
    /// Assignment has been paused.
    Paused,
    /// Assignment has been completed.
    Completed,
    /// Assignment was cancelled.
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("draft", AssignmentStatus::Draft),
            ("active", AssignmentStatus::Active),
            ("paused", AssignmentStatus::Paused),
            ("completed", AssignmentStatus::Completed),
            ("cancelled", AssignmentStatus::Cancelled),
        ];

        for (json_str, expected) in values {
            let result: AssignmentStatus =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<AssignmentStatus, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            AssignmentStatus::Draft,
            AssignmentStatus::Active,
            AssignmentStatus::Paused,
            AssignmentStatus::Completed,
            AssignmentStatus::Cancelled,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: AssignmentStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = AssignmentStatus::default();
        assert_eq!(default, AssignmentStatus::Draft, "Default should be Draft");
    }
}
