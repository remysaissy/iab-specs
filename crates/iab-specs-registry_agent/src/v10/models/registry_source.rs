use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Source registry where an agent is registered.
///
/// Tracks which registry verified the agent and when.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct RegistrySource {
    /// Name of the registry.
    #[builder(setter(into))]
    pub name: String,

    /// URL of the registry.
    #[builder(setter(into))]
    pub url: String,

    /// When the agent was last verified by this registry (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub last_verified_at: Option<String>,
}

impl RegistrySource {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RegistrySourceBuilder {
        RegistrySourceBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_source_creation() {
        let source = RegistrySource::builder()
            .name("IAB Registry")
            .url("https://registry.iab.com")
            .last_verified_at("2026-01-15T10:30:00Z")
            .build()
            .unwrap();

        assert_eq!(source.name, "IAB Registry");
        assert_eq!(source.url, "https://registry.iab.com");
        assert_eq!(
            source.last_verified_at.as_deref(),
            Some("2026-01-15T10:30:00Z")
        );
    }

    #[test]
    fn test_registry_source_serialization() {
        let source = RegistrySource::builder()
            .name("IAB Registry")
            .url("https://registry.iab.com")
            .last_verified_at("2026-01-15T10:30:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"name\":\"IAB Registry\""));
        assert!(json.contains("\"url\":\"https://registry.iab.com\""));
        assert!(json.contains("\"last_verified_at\":\"2026-01-15T10:30:00Z\""));
    }

    #[test]
    fn test_registry_source_deserialization() {
        let json = r#"{
            "name": "IAB Registry",
            "url": "https://registry.iab.com",
            "last_verified_at": "2026-01-15T10:30:00Z"
        }"#;

        let source: RegistrySource = serde_json::from_str(json).unwrap();
        assert_eq!(source.name, "IAB Registry");
        assert_eq!(source.url, "https://registry.iab.com");
        assert_eq!(
            source.last_verified_at.as_deref(),
            Some("2026-01-15T10:30:00Z")
        );
    }

    #[test]
    fn test_registry_source_roundtrip() {
        let source = RegistrySource::builder()
            .name("Test Registry")
            .url("https://test.registry.com")
            .build()
            .unwrap();

        let json = serde_json::to_string(&source).unwrap();
        // last_verified_at should be omitted when None
        assert!(!json.contains("last_verified_at"));

        let parsed: RegistrySource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, parsed);
    }

    #[test]
    fn test_registry_source_default() {
        let source = RegistrySource::default();
        assert_eq!(source.name, "");
        assert_eq!(source.url, "");
        assert!(source.last_verified_at.is_none());
    }
}
