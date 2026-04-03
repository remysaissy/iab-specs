use serde::{Deserialize, Serialize};

/// Type of machine learning model used for inference.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    /// Specialized encoder model for text encoding (default).
    #[default]
    Encoder,
    /// Large language model for general-purpose reasoning.
    Llm,
    /// Small language model for lightweight inference.
    Slm,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("encoder", ModelType::Encoder),
            ("llm", ModelType::Llm),
            ("slm", ModelType::Slm),
        ];

        for (json_str, expected) in values {
            let result: ModelType = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_model\"";
        let result: Result<ModelType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![ModelType::Encoder, ModelType::Llm, ModelType::Slm];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ModelType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = ModelType::default();
        assert_eq!(default, ModelType::Encoder, "Default should be Encoder");
    }
}
