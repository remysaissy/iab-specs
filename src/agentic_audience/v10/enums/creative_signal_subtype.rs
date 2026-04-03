use serde::{Deserialize, Serialize};

/// Subtypes of creative signals based on creative attributes and performance.
///
/// Creative signals indicate user affinity or response patterns to specific creative elements,
/// formats, messaging styles, and creative performance metrics across impression history.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum CreativeSignalSubtype {
    /// Signals based on visual elements and design attributes of creatives.
    #[default]
    Visual,
    /// Signals from textual content, messaging, and copy variations.
    Textual,
    /// Signals from multi-modal creatives combining visual, textual, and audio elements.
    Multimodal,
    /// Signals from creative performance metrics including click-through and view rates.
    CreativePerformance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("visual", CreativeSignalSubtype::Visual),
            ("textual", CreativeSignalSubtype::Textual),
            ("multimodal", CreativeSignalSubtype::Multimodal),
            (
                "creative_performance",
                CreativeSignalSubtype::CreativePerformance,
            ),
        ];

        for (json_str, expected) in values {
            let result: CreativeSignalSubtype =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<CreativeSignalSubtype, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            CreativeSignalSubtype::Visual,
            CreativeSignalSubtype::Textual,
            CreativeSignalSubtype::Multimodal,
            CreativeSignalSubtype::CreativePerformance,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CreativeSignalSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = CreativeSignalSubtype::default();
        assert_eq!(
            default,
            CreativeSignalSubtype::Visual,
            "Default should be Visual"
        );
    }
}
