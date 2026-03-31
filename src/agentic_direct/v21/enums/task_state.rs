use serde::{Deserialize, Serialize};

/// State of an agentic task in the A2A Protocol.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    /// Task is currently being processed.
    #[default]
    Working,
    /// Task requires additional input from the caller.
    InputRequired,
    /// Task has been completed successfully.
    Completed,
    /// Task failed to complete.
    Failed,
    /// Task was cancelled by the caller.
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("working", TaskState::Working),
            ("input_required", TaskState::InputRequired),
            ("completed", TaskState::Completed),
            ("failed", TaskState::Failed),
            ("cancelled", TaskState::Cancelled),
        ];

        for (json_str, expected) in values {
            let result: TaskState = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<TaskState, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            TaskState::Working,
            TaskState::InputRequired,
            TaskState::Completed,
            TaskState::Failed,
            TaskState::Cancelled,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: TaskState = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = TaskState::default();
        assert_eq!(default, TaskState::Working, "Default should be Working");
    }
}
