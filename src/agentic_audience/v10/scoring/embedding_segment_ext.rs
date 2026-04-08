use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::{DistanceMetric, EmbeddingType};

/// OpenRTB extension type for embedding transport in bid requests.
///
/// This type is used as `user.data.segment.ext` to carry embedding vectors through the OpenRTB bid stream.
/// It enables demand-side platforms to receive semantic embeddings associated with audience segments,
/// facilitating embedding-based audience targeting and matching strategies.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EmbeddingSegmentExt<Ext: Extension = crate::DefaultExt> {
    /// Protocol version (required, e.g., "1.0").
    ///
    /// Identifies the version of this extension specification.
    #[builder(setter(into))]
    pub ver: String,

    /// Embedding vector (required).
    ///
    /// The vector representation of the segment, typically in 32-bit floating point format.
    /// Common dimensions range from 64 to 768 depending on the model.
    #[serde(default)]
    #[builder(default)]
    pub vector: Vec<f32>,

    /// Embedding model identifier (required).
    ///
    /// The name or identifier of the model used to generate this embedding.
    /// Examples: "minilm-l6-v2", "mpnet-base-v2", "intfloat-multilingual-e5-small"
    #[builder(setter(into))]
    pub model: String,

    /// Embedding dimensionality (required).
    ///
    /// The number of elements in the embedding vector. Common values: 64, 384, 768, 1536.
    pub dimension: i32,

    /// Embedding type classification (required).
    ///
    /// Describes the semantic category of this embedding (e.g., identity, contextual, reinforcement).
    /// See [`EmbeddingType`] for valid values.
    #[serde(rename = "type")]
    pub type_: EmbeddingType,

    /// Distance metric for vector comparison (optional).
    ///
    /// The distance metric to use when comparing this embedding to other vectors.
    /// If omitted, cosine similarity is typically assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metric: Option<DistanceMetric>,

    /// Time-to-live in seconds (optional).
    ///
    /// The number of seconds this embedding is valid. After this period, the embedding
    /// should be refreshed. If omitted, the embedding is valid indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ttl: Option<i64>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl EmbeddingSegmentExt {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EmbeddingSegmentExtBuilder {
        EmbeddingSegmentExtBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_ext_creation() {
        let ext = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .vector(vec![0.15, -0.22, 0.08])
            .model("minilm-l6-v2")
            .dimension(384)
            .type_(EmbeddingType::ContextContent)
            .build()
            .unwrap();

        assert_eq!(ext.ver, "1.0");
        assert_eq!(ext.vector, vec![0.15, -0.22, 0.08]);
        assert_eq!(ext.model, "minilm-l6-v2");
        assert_eq!(ext.dimension, 384);
        assert_eq!(ext.type_, EmbeddingType::ContextContent);
        assert_eq!(ext.metric, None);
        assert_eq!(ext.ttl, None);
    }

    #[test]
    fn test_segment_ext_serialization() {
        let ext = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .vector(vec![0.15, -0.22, 0.08])
            .model("minilm-l6-v2")
            .dimension(384)
            .type_(EmbeddingType::ContextContent)
            .build()
            .unwrap();

        let json = serde_json::to_value(&ext).unwrap();
        let obj = json.as_object().unwrap();

        // Verify required fields are present
        assert!(obj.contains_key("ver"));
        assert!(obj.contains_key("vector"));
        assert!(obj.contains_key("model"));
        assert!(obj.contains_key("dimension"));
        assert!(obj.contains_key("type"));

        // Verify type_ serializes as "type" not "type_"
        assert_eq!(json["type"], "context_content");

        // Verify optional fields are omitted
        assert!(!obj.contains_key("metric"));
        assert!(!obj.contains_key("ttl"));
    }

    #[test]
    fn test_segment_ext_deserialization() {
        let json_str = r#"{
            "ver": "1.0",
            "vector": [0.15, -0.22, 0.08],
            "model": "minilm-l6-v2",
            "dimension": 384,
            "type": "context_content"
        }"#;

        let ext: EmbeddingSegmentExt = serde_json::from_str(json_str).unwrap();

        assert_eq!(ext.ver, "1.0");
        assert_eq!(ext.vector, vec![0.15, -0.22, 0.08]);
        assert_eq!(ext.model, "minilm-l6-v2");
        assert_eq!(ext.dimension, 384);
        assert_eq!(ext.type_, EmbeddingType::ContextContent);
    }

    #[test]
    fn test_segment_ext_roundtrip() {
        let original = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .vector(vec![0.1, 0.2, 0.3, 0.4, 0.5])
            .model("mpnet-base-v2")
            .dimension(768)
            .type_(EmbeddingType::IdentityBehavioral)
            .metric(Some(DistanceMetric::Cosine))
            .ttl(Some(3600))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EmbeddingSegmentExt = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
        assert_eq!(deserialized.metric, Some(DistanceMetric::Cosine));
        assert_eq!(deserialized.ttl, Some(3600));
    }

    #[test]
    fn test_segment_ext_default() {
        let ext = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .model("test-model")
            .type_(EmbeddingType::ContextContent)
            .dimension(384)
            .build()
            .unwrap();

        // vec field defaults to empty
        assert!(ext.vector.is_empty());
        // optional fields default to None
        assert_eq!(ext.metric, None);
        assert_eq!(ext.ttl, None);
        assert_eq!(ext.ext, None);
    }

    #[test]
    fn test_segment_ext_matches_spec_format() {
        let ext = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .vector(vec![0.15, -0.22, 0.08])
            .model("minilm-l6-v2")
            .dimension(384)
            .type_(EmbeddingType::ContextContent)
            .build()
            .unwrap();

        let json = serde_json::to_string(&ext).unwrap();

        // Verify keys in JSON match specification order
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = parsed.as_object().unwrap();

        assert_eq!(obj.get("ver").unwrap(), "1.0");
        assert_eq!(obj.get("model").unwrap(), "minilm-l6-v2");
        assert_eq!(obj.get("dimension").unwrap(), 384);
        assert_eq!(obj.get("type").unwrap(), "context_content");

        let vector = obj.get("vector").unwrap().as_array().unwrap();
        assert_eq!(vector.len(), 3);
        assert!((vector[0].as_f64().unwrap() - 0.15).abs() < 0.0001);
        assert!((vector[1].as_f64().unwrap() - (-0.22)).abs() < 0.0001);
        assert!((vector[2].as_f64().unwrap() - 0.08).abs() < 0.0001);
    }

    #[test]
    fn test_segment_ext_vector_dimension_mismatch() {
        // Spec: vector.len() vs dimension mismatch is not enforced at builder level
        let ext = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .vector(vec![0.1, 0.2])
            .model("test-model")
            .dimension(384)
            .type_(EmbeddingType::ContextContent)
            .build()
            .unwrap();
        assert_eq!(ext.dimension, 384);
        assert_eq!(ext.vector.len(), 2);
    }

    #[test]
    fn test_segment_ext_malformed_json_rejected() {
        // Spec: required fields must be present with correct types
        let json = r#"{"ver": "1.0", "vector": "not_an_array"}"#;
        let result: Result<EmbeddingSegmentExt, _> = serde_json::from_str(json);
        assert!(result.is_err(), "String vector should be rejected");
    }
}
