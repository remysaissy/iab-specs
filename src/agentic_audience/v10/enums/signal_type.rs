use serde::{Deserialize, Serialize};

/// Primary signal type categories in the Agentic Audience framework.
///
/// Each signal type represents a distinct category of audience signals that can be
/// leveraged for targeting and segmentation in agent-driven advertising workflows.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum SignalType {
    /// Identity-based signals derived from direct user identification or verification.
    #[default]
    Identity,
    /// Contextual signals based on the content, environment, or session context.
    Contextual,
    /// Reinforcement signals from past user actions and engagement patterns.
    Reinforcement,
    /// Creative signals indicating user affinity or response to specific creative elements.
    Creative,
    /// Inventory signals related to available advertising inventory and placements.
    Inventory,
    /// Query intent signals derived from user search behavior and stated intent.
    QueryIntent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("identity", SignalType::Identity),
            ("contextual", SignalType::Contextual),
            ("reinforcement", SignalType::Reinforcement),
            ("creative", SignalType::Creative),
            ("inventory", SignalType::Inventory),
            ("query_intent", SignalType::QueryIntent),
        ];

        for (json_str, expected) in values {
            let result: SignalType = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<SignalType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            SignalType::Identity,
            SignalType::Contextual,
            SignalType::Reinforcement,
            SignalType::Creative,
            SignalType::Inventory,
            SignalType::QueryIntent,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: SignalType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = SignalType::default();
        assert_eq!(default, SignalType::Identity, "Default should be Identity");
    }
}
