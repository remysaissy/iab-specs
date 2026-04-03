use serde::{Deserialize, Serialize};

/// Type of agent registered in the registry.
///
/// Classifies agents by their role in the advertising ecosystem.
/// All serialization uses snake_case format (e.g., `"data_provider"` for `DataProvider`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    /// Demand-Side Platform agent.
    #[default]
    Dsp,
    /// Supply-Side Platform agent.
    Ssp,
    /// Customer Data Platform agent.
    Cdp,
    /// Data provider agent.
    DataProvider,
    /// Creative management agent.
    Creative,
    /// Measurement and analytics agent.
    Measurement,
    /// Verification and brand safety agent.
    Verification,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            (AgentType::Dsp, "dsp"),
            (AgentType::Ssp, "ssp"),
            (AgentType::Cdp, "cdp"),
            (AgentType::DataProvider, "data_provider"),
            (AgentType::Creative, "creative"),
            (AgentType::Measurement, "measurement"),
            (AgentType::Verification, "verification"),
        ];

        for (variant, expected) in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            let expected_json = format!("\"{}\"", expected);
            assert_eq!(
                serialized, expected_json,
                "Expected {:?} to serialize as {}, got {}",
                variant, expected_json, serialized
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent\"";
        let result: Result<AgentType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid agent type should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            AgentType::Dsp,
            AgentType::Ssp,
            AgentType::Cdp,
            AgentType::DataProvider,
            AgentType::Creative,
            AgentType::Measurement,
            AgentType::Verification,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: AgentType =
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
        let default = AgentType::default();
        assert_eq!(default, AgentType::Dsp, "Default should be Dsp");
    }
}
