use serde::{Deserialize, Serialize};

/// Input mode for skill parameters in the A2A Protocol.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum SkillInputMode {
    /// Text-based input.
    #[default]
    Text,
    /// File-based input.
    File,
    /// Data structure-based input.
    Data,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("text", SkillInputMode::Text),
            ("file", SkillInputMode::File),
            ("data", SkillInputMode::Data),
        ];

        for (json_str, expected) in values {
            let result: SkillInputMode =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<SkillInputMode, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            SkillInputMode::Text,
            SkillInputMode::File,
            SkillInputMode::Data,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: SkillInputMode = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = SkillInputMode::default();
        assert_eq!(default, SkillInputMode::Text, "Default should be Text");
    }
}
