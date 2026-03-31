use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::{ProtocolType, SecuritySchemeType, SkillInputMode, TransportType};

/// Agent capabilities for the A2A Protocol.
///
/// Describes optional capabilities an agent supports.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilities {
    /// Whether the agent supports push notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub push_notifications: Option<bool>,

    /// Whether the agent supports streaming responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub streaming: Option<bool>,

    /// Whether the agent supports MCP integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mcp_integration: Option<bool>,
}

impl AgentCapabilities {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AgentCapabilitiesBuilder {
        AgentCapabilitiesBuilder::create_empty()
    }
}

/// Additional interface for agent communication.
///
/// Describes an alternative protocol/transport endpoint.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(rename_all = "camelCase")]
pub struct AgentInterface {
    /// Protocol type for this interface.
    pub protocol: ProtocolType,

    /// Protocol version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub version: Option<String>,

    /// Transport type for this interface.
    pub transport: TransportType,

    /// URL endpoint for this interface.
    #[builder(setter(into))]
    pub url: String,
}

impl AgentInterface {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AgentInterfaceBuilder {
        AgentInterfaceBuilder::create_empty()
    }
}

/// Security scheme for agent authentication.
///
/// Describes an authentication mechanism supported by the agent.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    /// Type of security scheme.
    #[serde(rename = "type")]
    pub type_: SecuritySchemeType,

    /// Description of the security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// OAuth2 flows configuration (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flows: Option<serde_json::Value>,
}

impl SecurityScheme {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SecuritySchemeBuilder {
        SecuritySchemeBuilder::create_empty()
    }
}

/// Skill offered by an agent.
///
/// Represents a specific capability or function an agent can perform.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
#[serde(rename_all = "camelCase")]
pub struct Skill<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the skill.
    #[builder(setter(into))]
    pub id: String,

    /// Human-readable name of the skill.
    #[builder(setter(into))]
    pub name: String,

    /// Description of what the skill does.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// Tags for categorizing the skill.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub tags: Vec<String>,

    /// Example inputs or prompts for the skill.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub examples: Vec<String>,

    /// Supported input modes for the skill.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub input_modes: Vec<SkillInputMode>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Skill {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SkillBuilder {
        SkillBuilder::create_empty()
    }
}

/// A2A Protocol Agent Card.
///
/// Describes an agent's identity, capabilities, skills, and communication interfaces.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
#[serde(rename_all = "camelCase")]
pub struct AgentCard<Ext: Extension = crate::DefaultExt> {
    /// Name of the agent.
    #[builder(setter(into))]
    pub name: String,

    /// Description of the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// Version of the agent.
    #[builder(setter(into))]
    pub version: String,

    /// A2A protocol version supported.
    #[builder(setter(into))]
    pub protocol_version: String,

    /// URL endpoint for the agent.
    #[builder(setter(into))]
    pub url: String,

    /// Skills offered by the agent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub skills: Vec<Skill<Ext>>,

    /// Agent capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub capabilities: Option<AgentCapabilities>,

    /// Additional communication interfaces.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub additional_interfaces: Vec<AgentInterface>,

    /// Security schemes supported by the agent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub security_schemes: Vec<SecurityScheme>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AgentCard {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AgentCardBuilder {
        AgentCardBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_card_creation() {
        let card = AgentCard::builder()
            .name("Test Agent")
            .version("1.0.0")
            .protocol_version("0.3.0")
            .url("https://agent.example.com")
            .build()
            .unwrap();

        assert_eq!(card.name, "Test Agent");
        assert_eq!(card.version, "1.0.0");
        assert_eq!(card.protocol_version, "0.3.0");
        assert_eq!(card.url, "https://agent.example.com");
        assert!(card.description.is_none());
        assert!(card.skills.is_empty());
        assert!(card.capabilities.is_none());
        assert!(card.additional_interfaces.is_empty());
        assert!(card.security_schemes.is_empty());
    }

    #[test]
    fn test_agent_card_roundtrip() {
        let card = AgentCard::builder()
            .name("Roundtrip Agent")
            .version("2.0.0")
            .protocol_version("0.3.0")
            .url("https://roundtrip.example.com")
            .description("A test agent for roundtrip")
            .build()
            .unwrap();

        let json = serde_json::to_string(&card).unwrap();
        // Verify camelCase serialization
        assert!(
            json.contains("\"protocolVersion\""),
            "Expected camelCase 'protocolVersion' in JSON, got: {}",
            json
        );
        assert!(
            !json.contains("\"protocol_version\""),
            "Should NOT contain snake_case 'protocol_version' in JSON"
        );

        let parsed: AgentCard = serde_json::from_str(&json).unwrap();
        assert_eq!(card, parsed);
    }

    #[test]
    fn test_agent_card_with_skills() {
        let skills = vec![
            Skill::builder()
                .id("skill-1")
                .name("Text Analysis")
                .description("Analyze text content")
                .tags(vec!["nlp".to_string(), "analysis".to_string()])
                .build()
                .unwrap(),
            Skill::builder()
                .id("skill-2")
                .name("Image Generation")
                .description("Generate images from text")
                .input_modes(vec![SkillInputMode::Text])
                .build()
                .unwrap(),
            Skill::builder()
                .id("skill-3")
                .name("Data Processing")
                .examples(vec!["Process CSV data".to_string()])
                .input_modes(vec![SkillInputMode::File, SkillInputMode::Data])
                .build()
                .unwrap(),
        ];

        let card = AgentCard::builder()
            .name("Multi-Skill Agent")
            .version("1.0.0")
            .protocol_version("0.3.0")
            .url("https://multi.example.com")
            .skills(skills)
            .build()
            .unwrap();

        assert_eq!(card.skills.len(), 3);
        assert_eq!(card.skills[0].id, "skill-1");
        assert_eq!(card.skills[1].id, "skill-2");
        assert_eq!(card.skills[2].id, "skill-3");
        assert_eq!(card.skills[0].tags.len(), 2);
        assert_eq!(card.skills[2].input_modes.len(), 2);
    }

    #[test]
    fn test_skill_roundtrip() {
        let skill = Skill::builder()
            .id("s1")
            .name("Test Skill")
            .description("A skill for testing")
            .tags(vec!["test".to_string()])
            .examples(vec!["example input".to_string()])
            .input_modes(vec![SkillInputMode::Text, SkillInputMode::File])
            .build()
            .unwrap();

        let json = serde_json::to_string(&skill).unwrap();
        // Verify camelCase for inputModes
        assert!(
            json.contains("\"inputModes\""),
            "Expected camelCase 'inputModes' in JSON, got: {}",
            json
        );

        let parsed: Skill = serde_json::from_str(&json).unwrap();
        assert_eq!(skill, parsed);
    }

    #[test]
    fn test_capabilities_roundtrip() {
        let caps = AgentCapabilities::builder()
            .push_notifications(Some(true))
            .streaming(Some(false))
            .mcp_integration(Some(true))
            .build()
            .unwrap();

        let json = serde_json::to_string(&caps).unwrap();
        // Verify camelCase
        assert!(
            json.contains("\"pushNotifications\""),
            "Expected camelCase 'pushNotifications' in JSON, got: {}",
            json
        );
        assert!(
            json.contains("\"mcpIntegration\""),
            "Expected camelCase 'mcpIntegration' in JSON, got: {}",
            json
        );

        let parsed: AgentCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(caps, parsed);
    }

    #[test]
    fn test_interface_roundtrip() {
        let iface = AgentInterface::builder()
            .protocol(ProtocolType::Mcp)
            .version("1.0")
            .transport(TransportType::Sse)
            .url("https://mcp.example.com/sse")
            .build()
            .unwrap();

        let json = serde_json::to_string(&iface).unwrap();
        let parsed: AgentInterface = serde_json::from_str(&json).unwrap();
        assert_eq!(iface, parsed);
    }

    #[test]
    fn test_security_scheme_roundtrip() {
        let scheme = SecurityScheme::builder()
            .type_(SecuritySchemeType::OAuth2)
            .description("OAuth2 authentication")
            .flows(Some(serde_json::json!({
                "authorizationCode": {
                    "authorizationUrl": "https://auth.example.com/authorize",
                    "tokenUrl": "https://auth.example.com/token"
                }
            })))
            .build()
            .unwrap();

        let json = serde_json::to_string(&scheme).unwrap();
        // Verify type is serialized as "type" not "type_"
        assert!(
            json.contains("\"type\""),
            "Expected 'type' key in JSON, got: {}",
            json
        );

        let parsed: SecurityScheme = serde_json::from_str(&json).unwrap();
        assert_eq!(scheme, parsed);
    }
}
