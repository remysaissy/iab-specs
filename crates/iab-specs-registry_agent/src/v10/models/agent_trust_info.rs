use crate::v10::enums::TrustLevel;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Lightweight trust information about an agent.
///
/// Provides quick trust assessment without full registration details.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct AgentTrustInfo {
    /// URL of the agent.
    #[builder(setter(into))]
    pub agent_url: String,

    /// Whether the agent is registered in any registry.
    pub is_registered: bool,

    /// Current trust level.
    pub trust_level: TrustLevel,

    /// Registry ID if registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub registry_id: Option<String>,
}

impl AgentTrustInfo {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AgentTrustInfoBuilder {
        AgentTrustInfoBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_trust_info_creation() {
        let info = AgentTrustInfo::builder()
            .agent_url("https://agent.example.com")
            .is_registered(true)
            .trust_level(TrustLevel::Verified)
            .registry_id("reg-001")
            .build()
            .unwrap();

        assert_eq!(info.agent_url, "https://agent.example.com");
        assert!(info.is_registered);
        assert_eq!(info.trust_level, TrustLevel::Verified);
        assert_eq!(info.registry_id.as_deref(), Some("reg-001"));
    }

    #[test]
    fn test_agent_trust_info_serialization() {
        let info = AgentTrustInfo::builder()
            .agent_url("https://agent.example.com")
            .is_registered(true)
            .trust_level(TrustLevel::Registered)
            .build()
            .unwrap();

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"agent_url\":\"https://agent.example.com\""));
        assert!(json.contains("\"is_registered\":true"));
        assert!(json.contains("\"trust_level\":\"registered\""));
        assert!(!json.contains("registry_id"));
    }

    #[test]
    fn test_agent_trust_info_deserialization() {
        let json = r#"{
            "agent_url": "https://agent.example.com",
            "is_registered": false,
            "trust_level": "unknown",
            "registry_id": null
        }"#;

        let info: AgentTrustInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.agent_url, "https://agent.example.com");
        assert!(!info.is_registered);
        assert_eq!(info.trust_level, TrustLevel::Unknown);
        assert!(info.registry_id.is_none());
    }

    #[test]
    fn test_agent_trust_info_roundtrip() {
        let info = AgentTrustInfo::builder()
            .agent_url("https://roundtrip.example.com")
            .is_registered(true)
            .trust_level(TrustLevel::Preferred)
            .registry_id("reg-rt")
            .build()
            .unwrap();

        let json = serde_json::to_string(&info).unwrap();
        let parsed: AgentTrustInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }

    #[test]
    fn test_agent_trust_info_default() {
        let info = AgentTrustInfo::default();
        assert_eq!(info.agent_url, "");
        assert!(!info.is_registered);
        assert_eq!(info.trust_level, TrustLevel::Unknown);
        assert!(info.registry_id.is_none());
    }

    #[test]
    fn test_trust_level_serializes_as_snake_case() {
        let info = AgentTrustInfo::builder()
            .trust_level(TrustLevel::Preferred)
            .build()
            .unwrap();

        let json = serde_json::to_string(&info).unwrap();
        assert!(
            json.contains("\"trust_level\":\"preferred\""),
            "TrustLevel should serialize as snake_case string, got: {}",
            json
        );
    }
}
