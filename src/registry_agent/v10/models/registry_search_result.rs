use crate::Extension;
use crate::registry_agent::v10::models::registered_agent::RegisteredAgent;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Search results from the agent registry.
///
/// Contains matching agents with pagination metadata.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type. Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct RegistrySearchResult<Ext: Extension = crate::DefaultExt> {
    /// Matching registered agents.
    #[serde(default)]
    #[builder(default)]
    pub agents: Vec<RegisteredAgent<Ext>>,

    /// Total count of matching agents (may be more than returned).
    pub total_count: i64,

    /// Whether more results are available.
    pub has_more: bool,

    /// Extension object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl RegistrySearchResult {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RegistrySearchResultBuilder {
        RegistrySearchResultBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agentic_direct::v21::a2a::AgentCard;
    use crate::registry_agent::v10::enums::{TrustLevel, VerificationStatus};

    fn sample_agent() -> RegisteredAgent {
        RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("Test DSP Agent")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://agent.example.com")
                    .build()
                    .unwrap(),
            )
            .registry_id("agent-001")
            .trust_level(TrustLevel::Verified)
            .verification_status(VerificationStatus::Verified)
            .build()
            .unwrap()
    }

    #[test]
    fn test_search_result_creation() {
        let agent = sample_agent();
        let result = RegistrySearchResult::builder()
            .agents(vec![agent.clone()])
            .total_count(42)
            .has_more(true)
            .build()
            .unwrap();

        assert_eq!(result.agents.len(), 1);
        assert_eq!(result.agents[0].agent_card.name, "Test DSP Agent");
        assert_eq!(result.agents[0].registry_id.as_deref(), Some("agent-001"));
        assert_eq!(result.total_count, 42);
        assert!(result.has_more);
        assert!(result.ext.is_none());
    }

    #[test]
    fn test_search_result_serialization() {
        let result = RegistrySearchResult::builder()
            .agents(vec![sample_agent()])
            .total_count(100)
            .has_more(true)
            .build()
            .unwrap();

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"total_count\":100"));
        assert!(json.contains("\"has_more\":true"));
        assert!(json.contains("\"agent_card\""));
        assert!(json.contains("\"name\":\"Test DSP Agent\""));
    }

    #[test]
    fn test_search_result_deserialization() {
        let json = r#"{
            "agents": [{
                "agent_card": {
                    "name": "Creative Agent",
                    "version": "1.0.0",
                    "protocolVersion": "0.3.0",
                    "url": "https://creative.example.com"
                },
                "trust_level": "preferred",
                "verification_status": "verified"
            }],
            "total_count": 1,
            "has_more": false
        }"#;

        let result: RegistrySearchResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.agents.len(), 1);
        assert_eq!(result.agents[0].agent_card.name, "Creative Agent");
        assert_eq!(result.agents[0].trust_level, TrustLevel::Preferred);
        assert_eq!(result.total_count, 1);
        assert!(!result.has_more);
    }

    #[test]
    fn test_search_result_roundtrip() {
        let original = RegistrySearchResult::builder()
            .agents(vec![
                sample_agent(),
                RegisteredAgent::builder()
                    .agent_card(
                        AgentCard::builder()
                            .name("SSP Agent")
                            .version("2.0.0")
                            .protocol_version("0.3.0")
                            .url("https://ssp.example.com")
                            .build()
                            .unwrap(),
                    )
                    .registry_id("agent-003")
                    .trust_level(TrustLevel::Registered)
                    .verification_status(VerificationStatus::Pending)
                    .build()
                    .unwrap(),
            ])
            .total_count(2)
            .has_more(false)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RegistrySearchResult = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_search_result_default() {
        let result: RegistrySearchResult = RegistrySearchResult::default();

        assert!(result.agents.is_empty());
        assert_eq!(result.total_count, 0);
        assert!(!result.has_more);
        assert!(result.ext.is_none());
    }

    #[test]
    fn test_search_result_empty_agents_roundtrip() {
        // Spec: RegistrySearchResult — empty result set
        let result = RegistrySearchResult::builder()
            .agents(vec![])
            .total_count(0)
            .has_more(false)
            .build()
            .unwrap();

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"agents\":[]"));
        assert!(json.contains("\"total_count\":0"));
        assert!(json.contains("\"has_more\":false"));

        let parsed: RegistrySearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, parsed);
    }
}
