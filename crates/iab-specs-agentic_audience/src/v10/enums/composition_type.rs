use serde::{Deserialize, Serialize};

/// Composition type defining how audience segments are combined and structured.
///
/// This enum specifies the structural organization of audience segments:
/// - `Atomic`: Single, indivisible segment or signal.
/// - `Composite`: Multiple atomic segments combined with logical operators.
/// - `Graph`: Network-based relationships between segments (social, influence).
/// - `CrossSignalFusion`: Multiple signal types fused through machine learning.
/// - `Hierarchical`: Nested segments with parent-child relationships.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum CompositionType {
    /// Single, indivisible segment or signal.
    #[default]
    Atomic,
    /// Multiple atomic segments combined with logical operators (AND, OR, NOT).
    Composite,
    /// Network-based relationships between segments (social graph, influence networks).
    Graph,
    /// Multiple signal types fused through machine learning or statistical methods.
    CrossSignalFusion,
    /// Nested segments with parent-child hierarchical relationships.
    Hierarchical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("atomic", CompositionType::Atomic),
            ("composite", CompositionType::Composite),
            ("graph", CompositionType::Graph),
            ("cross_signal_fusion", CompositionType::CrossSignalFusion),
            ("hierarchical", CompositionType::Hierarchical),
        ];

        for (json_str, expected) in values {
            let result: CompositionType = serde_json::from_str(&format!("\"{}\"", json_str))
                .expect(&format!("Failed to deserialize: {}", json_str));
            assert_eq!(
                result, expected,
                "Failed for value: {} (expected {:?}, got {:?})",
                json_str, expected, result
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_composition_type\"";
        let result: Result<CompositionType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            CompositionType::Atomic,
            CompositionType::Composite,
            CompositionType::Graph,
            CompositionType::CrossSignalFusion,
            CompositionType::Hierarchical,
        ];

        for original in values {
            let json = serde_json::to_string(&original)
                .expect(&format!("Failed to serialize: {:?}", original));
            let deserialized: CompositionType =
                serde_json::from_str(&json).expect(&format!("Failed to deserialize: {}", json));
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = CompositionType::default();
        assert_eq!(default, CompositionType::Atomic, "Default should be Atomic");
    }

    #[test]
    fn test_integer_value_rejected() {
        // Spec: Agentic Audience v1.0 — enums are string-serialized, integers must be rejected
        let result: Result<CompositionType, _> = serde_json::from_str("42");
        assert!(result.is_err(), "Integer value should be rejected");
    }
}
