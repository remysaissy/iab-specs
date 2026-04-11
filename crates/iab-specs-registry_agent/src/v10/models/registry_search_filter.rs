use crate::Extension;
use crate::v10::enums::{AgentType, TrustLevel};
use derive_builder::Builder;
use iab_specs_agentic_direct::v21::enums::ProtocolType;
use serde::{Deserialize, Serialize};

/// Filter criteria for searching the agent registry.
///
/// Allows filtering agents by various attributes including skills,
/// protocols, trust levels, and agent types.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type. Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct RegistrySearchFilter<Ext: Extension = crate::DefaultExt> {
    /// Free-text search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub query: Option<String>,

    /// Filter by skill tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub skill_tags: Vec<String>,

    /// Filter by supported protocol types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub protocol_types: Vec<ProtocolType>,

    /// Filter by trust levels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub trust_levels: Vec<TrustLevel>,

    /// Filter by agent types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub agent_types: Vec<AgentType>,

    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_results: Option<i32>,

    /// Extension object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl RegistrySearchFilter {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RegistrySearchFilterBuilder {
        RegistrySearchFilterBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filter_creation() {
        let filter = RegistrySearchFilter::builder()
            .query("media buying agent")
            .trust_levels(vec![TrustLevel::Verified, TrustLevel::Preferred])
            .agent_types(vec![AgentType::Dsp, AgentType::Ssp])
            .max_results(Some(10))
            .build()
            .unwrap();

        assert_eq!(filter.query.as_deref(), Some("media buying agent"));
        assert_eq!(filter.trust_levels.len(), 2);
        assert_eq!(filter.trust_levels[0], TrustLevel::Verified);
        assert_eq!(filter.trust_levels[1], TrustLevel::Preferred);
        assert_eq!(filter.agent_types.len(), 2);
        assert_eq!(filter.agent_types[0], AgentType::Dsp);
        assert_eq!(filter.agent_types[1], AgentType::Ssp);
        assert_eq!(filter.max_results, Some(10));
        assert!(filter.skill_tags.is_empty());
        assert!(filter.protocol_types.is_empty());
        assert!(filter.ext.is_none());
    }

    #[test]
    fn test_search_filter_serialization() {
        let filter = RegistrySearchFilter::builder()
            .query("data provider")
            .skill_tags(vec!["audience".to_string(), "targeting".to_string()])
            .protocol_types(vec![ProtocolType::JsonRpc, ProtocolType::Rest])
            .trust_levels(vec![TrustLevel::Verified])
            .agent_types(vec![AgentType::DataProvider])
            .max_results(Some(25))
            .build()
            .unwrap();

        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"query\":\"data provider\""));
        assert!(json.contains("\"skill_tags\":[\"audience\",\"targeting\"]"));
        assert!(json.contains("\"json_rpc\""));
        assert!(json.contains("\"rest\""));
        assert!(json.contains("\"verified\""));
        assert!(json.contains("\"data_provider\""));
        assert!(json.contains("\"max_results\":25"));
    }

    #[test]
    fn test_search_filter_deserialization() {
        let json = r#"{
            "query": "creative agent",
            "skill_tags": ["banner", "video"],
            "protocol_types": ["mcp"],
            "trust_levels": ["preferred"],
            "agent_types": ["creative"],
            "max_results": 5
        }"#;

        let filter: RegistrySearchFilter = serde_json::from_str(json).unwrap();
        assert_eq!(filter.query.as_deref(), Some("creative agent"));
        assert_eq!(filter.skill_tags, vec!["banner", "video"]);
        assert_eq!(filter.protocol_types, vec![ProtocolType::Mcp]);
        assert_eq!(filter.trust_levels, vec![TrustLevel::Preferred]);
        assert_eq!(filter.agent_types, vec![AgentType::Creative]);
        assert_eq!(filter.max_results, Some(5));
    }

    #[test]
    fn test_search_filter_roundtrip() {
        let original = RegistrySearchFilter::builder()
            .query("full roundtrip test")
            .skill_tags(vec!["programmatic".to_string(), "measurement".to_string()])
            .protocol_types(vec![
                ProtocolType::JsonRpc,
                ProtocolType::Mcp,
                ProtocolType::Rest,
            ])
            .trust_levels(vec![
                TrustLevel::Registered,
                TrustLevel::Verified,
                TrustLevel::Preferred,
            ])
            .agent_types(vec![
                AgentType::Dsp,
                AgentType::Measurement,
                AgentType::Verification,
            ])
            .max_results(Some(50))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RegistrySearchFilter = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_search_filter_default() {
        let filter = RegistrySearchFilter::<crate::DefaultExt>::default();

        assert!(filter.query.is_none());
        assert!(filter.skill_tags.is_empty());
        assert!(filter.protocol_types.is_empty());
        assert!(filter.trust_levels.is_empty());
        assert!(filter.agent_types.is_empty());
        assert!(filter.max_results.is_none());
        assert!(filter.ext.is_none());
    }

    #[test]
    fn test_search_filter_empty_vecs_not_serialized() {
        let filter = RegistrySearchFilter::builder()
            .query("only query")
            .build()
            .unwrap();

        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"query\":\"only query\""));
        assert!(!json.contains("skill_tags"));
        assert!(!json.contains("protocol_types"));
        assert!(!json.contains("trust_levels"));
        assert!(!json.contains("agent_types"));
        assert!(!json.contains("max_results"));
        assert!(!json.contains("ext"));
    }
}
