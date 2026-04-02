use serde::{Deserialize, Serialize};

/// The negotiation strategy type for deal negotiations.
///
/// Strategy types define the approach taken during price and term negotiations with buyers.
/// All serialization uses snake_case format (e.g., `"collaborative"` for `Collaborative`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum NegotiationStrategyType {
    /// Aggressive negotiation strategy aiming for maximum value.
    Aggressive,

    /// Standard negotiation strategy balancing firmness and flexibility.
    #[default]
    Standard,

    /// Collaborative negotiation strategy focused on win-win outcomes.
    Collaborative,

    /// Premium negotiation strategy for high-value deals.
    Premium,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            NegotiationStrategyType::Aggressive,
            NegotiationStrategyType::Standard,
            NegotiationStrategyType::Collaborative,
            NegotiationStrategyType::Premium,
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
        let json = "\"nonexistent_strategy\"";
        let result: Result<NegotiationStrategyType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid strategy should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            NegotiationStrategyType::Aggressive,
            NegotiationStrategyType::Standard,
            NegotiationStrategyType::Collaborative,
            NegotiationStrategyType::Premium,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: NegotiationStrategyType =
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
        let default = NegotiationStrategyType::default();
        assert_eq!(
            default,
            NegotiationStrategyType::Standard,
            "Default should be Standard"
        );
    }
}
