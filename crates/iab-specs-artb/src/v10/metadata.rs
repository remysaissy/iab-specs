use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Response metadata describing the agent's API and model versions.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::Metadata;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let metadata = Metadata::builder()
///     .api_version("1.0")
///     .model_version("v0.10.0")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Metadata<Ext: Extension = crate::DefaultExt> {
    /// The ARTB API version the agent implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub api_version: Option<String>,

    /// The version of the agent's model or business logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub model_version: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Metadata {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let metadata = Metadata::builder()
            .api_version("1.0")
            .model_version("v0.10.0")
            .build()
            .unwrap();

        assert_eq!(metadata.api_version, Some("1.0".to_string()));
        assert_eq!(metadata.model_version, Some("v0.10.0".to_string()));
    }

    #[test]
    fn test_metadata_empty() {
        let metadata = Metadata::builder().build().unwrap();
        assert!(metadata.api_version.is_none());
        assert!(metadata.model_version.is_none());
    }

    #[test]
    fn test_metadata_serialization() {
        let metadata = Metadata::builder()
            .api_version("1.0")
            .model_version("v1.0.0")
            .build()
            .unwrap();

        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains("\"api_version\":\"1.0\""));
        assert!(json.contains("\"model_version\":\"v1.0.0\""));
    }

    #[test]
    fn test_metadata_deserialization() {
        let json = r#"{"api_version":"1.0","model_version":"v0.5.0"}"#;
        let metadata: Metadata = serde_json::from_str(json).unwrap();

        assert_eq!(metadata.api_version, Some("1.0".to_string()));
        assert_eq!(metadata.model_version, Some("v0.5.0".to_string()));
    }

    #[test]
    fn test_metadata_roundtrip() {
        let metadata = Metadata::builder()
            .api_version("1.0")
            .model_version("v2.0.0")
            .build()
            .unwrap();

        let json = serde_json::to_string(&metadata).unwrap();
        let parsed: Metadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata, parsed);
    }

    #[test]
    fn test_metadata_partial_fields() {
        let metadata = Metadata::builder().api_version("1.0").build().unwrap();

        assert_eq!(metadata.api_version, Some("1.0".to_string()));
        assert!(metadata.model_version.is_none());

        let json = serde_json::to_string(&metadata).unwrap();
        assert!(!json.contains("model_version"));
    }
}
