use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::{CompositionType, EmbeddingType, TemporalScope};

/// A single embedding vector with metadata for the Agentic Audience protocol.
///
/// Represents a dense vector embedding produced by an embedding model, along with
/// metadata describing its type, temporal scope, and composition strategy.
///
/// Note: `vector` and `quantized_b64` are mutually exclusive — provide one or the other.
/// This is not enforced at the type level to allow flexibility; consumers should validate
/// at runtime.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Embedding<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the embedding (REQUIRED).
    #[builder(setter(into))]
    pub id: String,

    /// Classification of the embedding (REQUIRED).
    #[serde(rename = "type")]
    pub type_: EmbeddingType,

    /// Dimensionality of the embedding vector (REQUIRED).
    pub dimension: i32,

    /// Full-precision embedding vector.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub vector: Option<Vec<f32>>,

    /// Base64-encoded quantized vector representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub quantized_b64: Option<String>,

    /// Arbitrary metadata as a JSON blob.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metadata: Option<serde_json::Value>,

    /// Time-to-live in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ttl: Option<i64>,

    /// Creation timestamp in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub created_at: Option<String>,

    /// Temporal scope of the embedding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub temporal_scope: Option<TemporalScope>,

    /// Composition strategy used to produce this embedding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub composition: Option<CompositionType>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Embedding {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EmbeddingBuilder {
        EmbeddingBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let embedding = Embedding::builder()
            .id("emb-001")
            .type_(EmbeddingType::ContextContent)
            .dimension(5)
            .vector(Some(vec![0.1, 0.2, 0.3, 0.4, 0.5]))
            .build()
            .unwrap();

        assert_eq!(embedding.id, "emb-001");
        assert_eq!(embedding.type_, EmbeddingType::ContextContent);
        assert_eq!(embedding.dimension, 5);
        assert_eq!(embedding.vector, Some(vec![0.1, 0.2, 0.3, 0.4, 0.5]));
        assert!(embedding.quantized_b64.is_none());
        assert!(embedding.metadata.is_none());
        assert!(embedding.ttl.is_none());
        assert!(embedding.created_at.is_none());
        assert!(embedding.temporal_scope.is_none());
        assert!(embedding.composition.is_none());
        assert!(embedding.ext.is_none());
    }

    #[test]
    fn test_embedding_with_full_precision_vector() {
        let vector: Vec<f32> = (0..384).map(|i| i as f32 * 0.001).collect();

        let embedding = Embedding::builder()
            .id("emb-fp")
            .type_(EmbeddingType::IdentityBehavioral)
            .dimension(384)
            .vector(Some(vector.clone()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&embedding).unwrap();
        assert!(json.contains("\"vector\""));

        // Verify the vector field is a JSON array
        let parsed_value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed_value["vector"].is_array());
        assert_eq!(parsed_value["vector"].as_array().unwrap().len(), 384);

        // Deserialize back and check float values match
        let deserialized: Embedding = serde_json::from_str(&json).unwrap();
        let deserialized_vector = deserialized.vector.unwrap();
        assert_eq!(deserialized_vector.len(), 384);
        for (i, val) in deserialized_vector.iter().enumerate() {
            assert!(
                (val - vector[i]).abs() < f32::EPSILON,
                "Mismatch at index {}: {} != {}",
                i,
                val,
                vector[i]
            );
        }
    }

    #[test]
    fn test_embedding_with_quantized_base64() {
        let embedding = Embedding::builder()
            .id("emb-q8")
            .type_(EmbeddingType::CreativeVisual)
            .dimension(128)
            .quantized_b64("SGVsbG8gV29ybGQ=")
            .build()
            .unwrap();

        assert!(embedding.vector.is_none());
        assert_eq!(
            embedding.quantized_b64,
            Some("SGVsbG8gV29ybGQ=".to_string())
        );

        // Serialize/deserialize roundtrip
        let json = serde_json::to_string(&embedding).unwrap();
        assert!(json.contains("\"quantized_b64\""));
        assert!(!json.contains("\"vector\""));

        let deserialized: Embedding = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.quantized_b64, embedding.quantized_b64);
        assert!(deserialized.vector.is_none());
    }

    #[test]
    fn test_embedding_serialization() {
        let embedding = Embedding::builder()
            .id("emb-ser")
            .type_(EmbeddingType::InventoryPublisher)
            .dimension(64)
            .vector(Some(vec![1.0, 2.0, 3.0]))
            .ttl(Some(3600))
            .build()
            .unwrap();

        let json = serde_json::to_string(&embedding).unwrap();

        // Verify type_ serializes as "type"
        assert!(json.contains("\"type\":"));
        assert!(!json.contains("\"type_\":"));

        // Check expected keys
        assert!(json.contains("\"id\":"));
        assert!(json.contains("\"dimension\":"));
        assert!(json.contains("\"vector\":"));
        assert!(json.contains("\"ttl\":"));

        // Optional fields not set should be absent
        assert!(!json.contains("\"quantized_b64\":"));
        assert!(!json.contains("\"metadata\":"));
        assert!(!json.contains("\"created_at\":"));
        assert!(!json.contains("\"temporal_scope\":"));
        assert!(!json.contains("\"composition\":"));
        assert!(!json.contains("\"ext\":"));
    }

    #[test]
    fn test_embedding_deserialization() {
        let json = r#"{
            "id": "emb-deser",
            "type": "context_content",
            "dimension": 3,
            "vector": [0.5, 0.6, 0.7]
        }"#;

        let embedding: Embedding = serde_json::from_str(json).unwrap();
        assert_eq!(embedding.id, "emb-deser");
        assert_eq!(embedding.type_, EmbeddingType::ContextContent);
        assert_eq!(embedding.dimension, 3);
        assert_eq!(embedding.vector, Some(vec![0.5, 0.6, 0.7]));
        assert!(embedding.quantized_b64.is_none());
        assert!(embedding.metadata.is_none());
    }

    #[test]
    fn test_embedding_roundtrip() {
        let embedding = Embedding::builder()
            .id("emb-rt")
            .type_(EmbeddingType::ReinforcementConversion)
            .dimension(8)
            .vector(Some(vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]))
            .metadata(Some(
                serde_json::json!({"source": "page_content", "version": 2}),
            ))
            .ttl(Some(7200))
            .created_at("2026-04-01T12:00:00Z")
            .temporal_scope(Some(TemporalScope::Session))
            .composition(Some(CompositionType::Composite))
            .build()
            .unwrap();

        let json = serde_json::to_string(&embedding).unwrap();
        let parsed: Embedding = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, embedding.id);
        assert_eq!(parsed.type_, embedding.type_);
        assert_eq!(parsed.dimension, embedding.dimension);
        assert_eq!(parsed.vector, embedding.vector);
        assert_eq!(parsed.quantized_b64, embedding.quantized_b64);
        assert_eq!(parsed.metadata, embedding.metadata);
        assert_eq!(parsed.ttl, embedding.ttl);
        assert_eq!(parsed.created_at, embedding.created_at);
        assert_eq!(parsed.temporal_scope, embedding.temporal_scope);
        assert_eq!(parsed.composition, embedding.composition);
        assert_eq!(parsed.ext, embedding.ext);
    }

    #[test]
    fn test_embedding_default() {
        let embedding = Embedding::builder().build().unwrap();

        assert_eq!(embedding.id, "");
        assert_eq!(embedding.type_, EmbeddingType::default());
        assert_eq!(embedding.dimension, 0);
        assert!(embedding.vector.is_none());
        assert!(embedding.quantized_b64.is_none());
        assert!(embedding.metadata.is_none());
        assert!(embedding.ttl.is_none());
        assert!(embedding.created_at.is_none());
        assert!(embedding.temporal_scope.is_none());
        assert!(embedding.composition.is_none());
        assert!(embedding.ext.is_none());
    }

    #[test]
    fn test_embedding_with_metadata() {
        let metadata = serde_json::json!({"source": "page_content"});

        let embedding = Embedding::builder()
            .id("emb-meta")
            .type_(EmbeddingType::ContextContent)
            .dimension(128)
            .metadata(Some(metadata.clone()))
            .build()
            .unwrap();

        assert_eq!(embedding.metadata, Some(metadata.clone()));

        // Verify roundtrip preserves metadata
        let json = serde_json::to_string(&embedding).unwrap();
        let parsed: Embedding = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.metadata, Some(metadata));
    }
}
