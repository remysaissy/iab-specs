use crate::agentic_direct::v21::a2a::AgentCard;
use crate::registry_agent::v10::enums::{TrustLevel, VerificationStatus};
use crate::registry_agent::v10::models::registry_source::RegistrySource;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A registered agent in the registry.
///
/// Wraps an [`AgentCard`] with registration metadata, trust level, and verification status.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct RegisteredAgent<Ext: Extension = crate::DefaultExt> {
    /// The agent's card with capabilities and skills.
    pub agent_card: AgentCard<Ext>,

    /// Unique registry identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub registry_id: Option<String>,

    /// When the agent was registered (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub registered_at: Option<String>,

    /// Current trust level.
    pub trust_level: TrustLevel,

    /// Current verification status.
    pub verification_status: VerificationStatus,

    /// Source registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<RegistrySource>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl RegisteredAgent {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RegisteredAgentBuilder {
        RegisteredAgentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registered_agent_creation() {
        let agent = RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("Test Agent")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://agent.example.com")
                    .build()
                    .unwrap(),
            )
            .trust_level(TrustLevel::Registered)
            .verification_status(VerificationStatus::Verified)
            .registry_id("reg-001")
            .build()
            .unwrap();

        assert_eq!(agent.agent_card.name, "Test Agent");
        assert_eq!(agent.trust_level, TrustLevel::Registered);
        assert_eq!(agent.verification_status, VerificationStatus::Verified);
        assert_eq!(agent.registry_id.as_deref(), Some("reg-001"));
    }

    #[test]
    fn test_registered_agent_serialization() {
        let agent = RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("Serialize Agent")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://serialize.example.com")
                    .build()
                    .unwrap(),
            )
            .trust_level(TrustLevel::Verified)
            .verification_status(VerificationStatus::Verified)
            .build()
            .unwrap();

        let json = serde_json::to_string(&agent).unwrap();
        assert!(json.contains("\"agent_card\""));
        assert!(json.contains("\"trust_level\":\"verified\""));
        assert!(json.contains("\"verification_status\":\"verified\""));
        // Nested AgentCard uses camelCase
        assert!(json.contains("\"protocolVersion\":\"0.3.0\""));
    }

    #[test]
    fn test_registered_agent_deserialization() {
        let json = r#"{
            "agent_card": {
                "name": "Deser Agent",
                "version": "2.0.0",
                "protocolVersion": "0.3.0",
                "url": "https://deser.example.com"
            },
            "trust_level": "registered",
            "verification_status": "pending"
        }"#;

        let agent: RegisteredAgent = serde_json::from_str(json).unwrap();
        assert_eq!(agent.agent_card.name, "Deser Agent");
        assert_eq!(agent.agent_card.version, "2.0.0");
        assert_eq!(agent.trust_level, TrustLevel::Registered);
        assert_eq!(agent.verification_status, VerificationStatus::Pending);
        assert!(agent.registry_id.is_none());
    }

    #[test]
    fn test_registered_agent_roundtrip() {
        let agent = RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("Roundtrip Agent")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://roundtrip.example.com")
                    .build()
                    .unwrap(),
            )
            .trust_level(TrustLevel::Preferred)
            .verification_status(VerificationStatus::Verified)
            .registry_id("reg-rt")
            .registered_at("2026-01-01T00:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&agent).unwrap();
        let parsed: RegisteredAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(agent, parsed);
    }

    #[test]
    fn test_registered_agent_default() {
        let agent = RegisteredAgent::<crate::DefaultExt>::default();
        assert_eq!(agent.agent_card, AgentCard::default());
        assert_eq!(agent.trust_level, TrustLevel::Unknown);
        assert_eq!(agent.verification_status, VerificationStatus::Unverified);
        assert!(agent.registry_id.is_none());
        assert!(agent.registered_at.is_none());
        assert!(agent.source.is_none());
        assert!(agent.ext.is_none());
    }

    #[test]
    fn test_registered_agent_with_source() {
        let agent = RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("Sourced Agent")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://sourced.example.com")
                    .build()
                    .unwrap(),
            )
            .trust_level(TrustLevel::Verified)
            .verification_status(VerificationStatus::Verified)
            .source(Some(
                RegistrySource::builder()
                    .name("IAB Registry")
                    .url("https://registry.iab.com")
                    .last_verified_at("2026-03-01T12:00:00Z")
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert!(agent.source.is_some());
        let source = agent.source.as_ref().unwrap();
        assert_eq!(source.name, "IAB Registry");
        assert_eq!(source.url, "https://registry.iab.com");

        let json = serde_json::to_string(&agent).unwrap();
        assert!(json.contains("\"source\""));
        assert!(json.contains("\"IAB Registry\""));

        let parsed: RegisteredAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(agent, parsed);
    }

    #[test]
    fn test_registered_agent_without_optional_fields() {
        // Spec: RegisteredAgent — minimal construction with only required defaults
        let agent = RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("Minimal Agent")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://minimal.example.com")
                    .build()
                    .unwrap(),
            )
            .trust_level(TrustLevel::Unknown)
            .verification_status(VerificationStatus::Unverified)
            .build()
            .unwrap();

        let json = serde_json::to_string(&agent).unwrap();
        assert!(!json.contains("\"registry_id\""));
        assert!(!json.contains("\"registered_at\""));
        assert!(!json.contains("\"source\""));
        assert!(!json.contains("\"ext\""));

        let parsed: RegisteredAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(agent, parsed);
    }
}
