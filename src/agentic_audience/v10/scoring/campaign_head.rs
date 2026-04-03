use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Campaign-specific scoring head weights.
///
/// Represents the weights of a multi-head neural network scoring model for a specific campaign.
/// Each head may specialize in different aspects of audience-campaign affinity (e.g., engagement,
/// conversion, safety).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct CampaignHead<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the campaign (required).
    #[builder(setter(into))]
    pub campaign_id: String,

    /// Weights for each head in the scoring model (required).
    /// Typically a vector of floats representing the parameters of one attention head.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub head_weights: Vec<f32>,

    /// Dimensionality of the embedding space this head operates on (required).
    pub dimension: i32,

    /// Identifier of the embedding/scoring model used (required).
    #[builder(setter(into))]
    pub model_id: String,

    /// ISO 8601 timestamp when this head was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub created_at: Option<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl CampaignHead {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CampaignHeadBuilder {
        CampaignHeadBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_head_creation() {
        let head = CampaignHead::builder()
            .campaign_id("camp-001")
            .dimension(128)
            .model_id("model-v1")
            .head_weights(vec![0.1, 0.2, 0.3])
            .build()
            .unwrap();

        assert_eq!(head.campaign_id, "camp-001");
        assert_eq!(head.dimension, 128);
        assert_eq!(head.model_id, "model-v1");
        assert_eq!(head.head_weights, vec![0.1, 0.2, 0.3]);
        assert!(head.created_at.is_none());
        assert!(head.ext.is_none());
    }

    #[test]
    fn test_campaign_head_serialization() {
        let head = CampaignHead::builder()
            .campaign_id("camp-002")
            .dimension(256)
            .model_id("model-v2")
            .head_weights(vec![0.5, 0.6])
            .build()
            .unwrap();

        let json = serde_json::to_string(&head).unwrap();
        assert!(json.contains("\"campaign_id\":\"camp-002\""));
        assert!(json.contains("\"dimension\":256"));
        assert!(json.contains("\"model_id\":\"model-v2\""));
        assert!(json.contains("\"head_weights\":[0.5,0.6]"));
    }

    #[test]
    fn test_campaign_head_deserialization() {
        let json = r#"{"campaign_id":"camp-003","dimension":384,"model_id":"model-v3","head_weights":[0.1,0.2,0.3]}"#;
        let head: CampaignHead = serde_json::from_str(json).unwrap();

        assert_eq!(head.campaign_id, "camp-003");
        assert_eq!(head.dimension, 384);
        assert_eq!(head.model_id, "model-v3");
        assert_eq!(head.head_weights, vec![0.1, 0.2, 0.3]);
        assert!(head.created_at.is_none());
    }

    #[test]
    fn test_campaign_head_roundtrip() {
        let head = CampaignHead::builder()
            .campaign_id("camp-004")
            .dimension(512)
            .model_id("model-v4")
            .head_weights(vec![0.7, 0.8, 0.9, 1.0])
            .created_at("2026-04-03T12:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&head).unwrap();
        let parsed: CampaignHead = serde_json::from_str(&json).unwrap();
        assert_eq!(head, parsed);
    }

    #[test]
    fn test_campaign_head_384_dimension_roundtrip() {
        // Create a 384-dimensional head_weights vector (common for embeddings)
        let weights_384: Vec<f32> = (0..384).map(|i| i as f32 * 0.001).collect();

        let head = CampaignHead::builder()
            .campaign_id("camp-384")
            .dimension(384)
            .model_id("embedding-384")
            .head_weights(weights_384.clone())
            .build()
            .unwrap();

        // Serialize and verify vector is preserved
        let json = serde_json::to_string(&head).unwrap();
        let parsed: CampaignHead = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.head_weights.len(), 384);
        for (i, val) in parsed.head_weights.iter().enumerate() {
            assert!(
                (val - weights_384[i]).abs() < f32::EPSILON,
                "Mismatch at index {}: {} != {}",
                i,
                val,
                weights_384[i]
            );
        }
    }

    #[test]
    fn test_campaign_head_default() {
        let head = CampaignHead::builder()
            .campaign_id("camp-default")
            .dimension(128)
            .model_id("model-default")
            .build()
            .unwrap();

        assert_eq!(head.campaign_id, "camp-default");
        assert_eq!(head.dimension, 128);
        assert_eq!(head.model_id, "model-default");
        assert!(head.head_weights.is_empty());
        assert!(head.created_at.is_none());
        assert!(head.ext.is_none());
    }
}
