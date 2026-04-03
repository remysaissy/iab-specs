use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// UCPModelDescriptor provides metadata about the model that produced UCP embeddings.
///
/// This descriptor identifies the embedding model version, dimensionality, and other
/// characteristics necessary for consumers to understand and utilize the embeddings.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::UCPModelDescriptor;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let descriptor = UCPModelDescriptor::builder()
///     .model_id("sentence-transformers/all-mpnet-base-v2")
///     .version("1.0.0")
///     .dimension(768)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct UCPModelDescriptor<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the embedding model (REQUIRED).
    #[builder(setter(into))]
    pub model_id: String,

    /// Version of the embedding model (REQUIRED).
    #[builder(setter(into))]
    pub version: String,

    /// Dimensionality of the embedding vectors produced by this model (REQUIRED).
    #[builder(default)]
    pub dimension: i32,

    /// Extension object for model descriptor-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl UCPModelDescriptor {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> UCPModelDescriptorBuilder {
        UCPModelDescriptorBuilder::create_empty()
    }
}

/// UCPConsent represents privacy and consent information for UCP embedding data.
///
/// Consent records track user permissions and expiration for embedding-based targeting,
/// ensuring compliance with privacy regulations and user preferences.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::UCPConsent;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let consent = UCPConsent::builder()
///     .purpose("audience-targeting")
///     .granted(true)
///     .expires_at("2026-12-31T23:59:59Z")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct UCPConsent<Ext: Extension = crate::DefaultExt> {
    /// Purpose of the embedding data (e.g., "audience-targeting", "frequency-capping") (REQUIRED).
    #[builder(setter(into))]
    pub purpose: String,

    /// Whether consent has been granted for this purpose (REQUIRED).
    #[builder(default)]
    pub granted: bool,

    /// ISO 8601 timestamp when this consent expires (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub expires_at: Option<String>,

    /// Extension object for consent-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl UCPConsent {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> UCPConsentBuilder {
        UCPConsentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== UCPModelDescriptor Tests ==========

    #[test]
    fn test_ucp_model_descriptor_minimal() {
        let descriptor = UCPModelDescriptor::builder()
            .model_id("bert-base-uncased")
            .version("1.0.0")
            .dimension(768)
            .build()
            .unwrap();

        assert_eq!(descriptor.model_id, "bert-base-uncased");
        assert_eq!(descriptor.version, "1.0.0");
        assert_eq!(descriptor.dimension, 768);
        assert!(descriptor.ext.is_none());
    }

    #[test]
    fn test_ucp_model_descriptor_full() {
        let descriptor = UCPModelDescriptor::builder()
            .model_id("sentence-transformers/all-mpnet-base-v2")
            .version("2.1.5")
            .dimension(768)
            .build()
            .unwrap();

        assert_eq!(
            descriptor.model_id,
            "sentence-transformers/all-mpnet-base-v2"
        );
        assert_eq!(descriptor.version, "2.1.5");
        assert_eq!(descriptor.dimension, 768);
    }

    #[test]
    fn test_ucp_model_descriptor_serialization() {
        let descriptor = UCPModelDescriptor::builder()
            .model_id("clip-vit-large-patch14")
            .version("1.5.0")
            .dimension(768)
            .build()
            .unwrap();

        let json = serde_json::to_string(&descriptor).unwrap();
        assert!(json.contains("\"model_id\":\"clip-vit-large-patch14\""));
        assert!(json.contains("\"version\":\"1.5.0\""));
        assert!(json.contains("\"dimension\":768"));
    }

    #[test]
    fn test_ucp_model_descriptor_deserialization() {
        let json = r#"{
            "model_id": "universal-sentence-encoder",
            "version": "4.0.1",
            "dimension": 512
        }"#;

        let descriptor: UCPModelDescriptor = serde_json::from_str(json).unwrap();
        assert_eq!(descriptor.model_id, "universal-sentence-encoder");
        assert_eq!(descriptor.version, "4.0.1");
        assert_eq!(descriptor.dimension, 512);
        assert!(descriptor.ext.is_none());
    }

    #[test]
    fn test_ucp_model_descriptor_roundtrip() {
        let original = UCPModelDescriptor::builder()
            .model_id("minilm-l6-v2")
            .version("1.2.3")
            .dimension(384)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: UCPModelDescriptor = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(parsed.model_id, "minilm-l6-v2");
        assert_eq!(parsed.version, "1.2.3");
        assert_eq!(parsed.dimension, 384);
    }

    // ========== UCPConsent Tests ==========

    #[test]
    fn test_ucp_consent_minimal() {
        let consent = UCPConsent::builder()
            .purpose("audience-targeting")
            .build()
            .unwrap();

        assert_eq!(consent.purpose, "audience-targeting");
        assert!(!consent.granted); // defaults to false
        assert!(consent.expires_at.is_none());
        assert!(consent.ext.is_none());
    }

    #[test]
    fn test_ucp_consent_full() {
        let consent = UCPConsent::builder()
            .purpose("frequency-capping")
            .granted(true)
            .expires_at("2026-12-31T23:59:59Z")
            .build()
            .unwrap();

        assert_eq!(consent.purpose, "frequency-capping");
        assert!(consent.granted);
        assert_eq!(consent.expires_at, Some("2026-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_ucp_consent_serialization() {
        let consent = UCPConsent::builder()
            .purpose("data-enrichment")
            .granted(true)
            .expires_at("2027-06-30T00:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&consent).unwrap();
        assert!(json.contains("\"purpose\":\"data-enrichment\""));
        assert!(json.contains("\"granted\":true"));
        assert!(json.contains("\"expires_at\":\"2027-06-30T00:00:00Z\""));
    }

    #[test]
    fn test_ucp_consent_deserialization() {
        let json = r#"{
            "purpose": "audience-targeting",
            "granted": false,
            "expires_at": "2026-03-15T12:30:00Z"
        }"#;

        let consent: UCPConsent = serde_json::from_str(json).unwrap();
        assert_eq!(consent.purpose, "audience-targeting");
        assert!(!consent.granted);
        assert_eq!(consent.expires_at, Some("2026-03-15T12:30:00Z".to_string()));
    }

    #[test]
    fn test_ucp_consent_roundtrip() {
        let original = UCPConsent::builder()
            .purpose("analytics")
            .granted(true)
            .expires_at("2025-12-31T23:59:59Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: UCPConsent = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(parsed.purpose, "analytics");
        assert!(parsed.granted);
        assert_eq!(parsed.expires_at, Some("2025-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_ucp_consent_skip_serializing_none() {
        let consent = UCPConsent::builder()
            .purpose("retargeting")
            .granted(false)
            .build()
            .unwrap();

        let json = serde_json::to_string(&consent).unwrap();
        assert!(json.contains("\"purpose\":\"retargeting\""));
        assert!(json.contains("\"granted\":false"));
        assert!(!json.contains("expires_at"));
    }
}
