use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::{DistanceMetric, ModelType, NormalizationType};

/// Embedding model descriptor.
///
/// Describes the embedding model used to generate embeddings, including model identity,
/// configuration, and distance metric for similarity calculations.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EmbeddingModel<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the embedding model.
    #[builder(setter(into))]
    pub id: String,

    /// Version of the embedding model.
    #[builder(setter(into))]
    pub version: String,

    /// Type of the embedding model.
    #[serde(rename = "type")]
    pub type_: ModelType,

    /// Dimensionality of the embedding space (e.g., 384 for sentence-transformers/all-MiniLM-L6-v2).
    pub dimension: i32,

    /// Distance metric used for similarity calculations (e.g., cosine, euclidean, dot product).
    pub metric: DistanceMetric,

    /// Identifier of the embedding space (e.g., "openai-text-embedding-3-small").
    #[builder(setter(into))]
    pub embedding_space_id: String,

    /// Optional normalization type applied to the embeddings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub normalization: Option<NormalizationType>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl EmbeddingModel {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EmbeddingModelBuilder {
        EmbeddingModelBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_model_creation() {
        let model = EmbeddingModel::builder()
            .id("model-001")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("openai-text-embedding-3-small")
            .build()
            .unwrap();

        assert_eq!(model.id, "model-001");
        assert_eq!(model.version, "1.0");
        assert_eq!(model.type_, ModelType::Encoder);
        assert_eq!(model.dimension, 384);
        assert_eq!(model.metric, DistanceMetric::Cosine);
        assert_eq!(model.embedding_space_id, "openai-text-embedding-3-small");
        assert!(model.normalization.is_none());
    }

    #[test]
    fn test_embedding_model_serialization() {
        let model = EmbeddingModel::builder()
            .id("model-002")
            .version("2.0")
            .type_(ModelType::Llm)
            .dimension(512)
            .metric(DistanceMetric::L2)
            .embedding_space_id("clip-vision-b32")
            .normalization(Some(NormalizationType::L2Norm))
            .build()
            .unwrap();

        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("\"id\":\"model-002\""));
        assert!(json.contains("\"version\":\"2.0\""));
        assert!(json.contains("\"type\":"));
        assert!(json.contains("\"dimension\":512"));
        assert!(json.contains("\"embedding_space_id\":\"clip-vision-b32\""));
        assert!(json.contains("\"normalization\":"));
    }

    #[test]
    fn test_embedding_model_type_field_serialization() {
        let model = EmbeddingModel::builder()
            .id("model-003")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("test-space")
            .build()
            .unwrap();

        let json = serde_json::to_string(&model).unwrap();
        // Verify that type_ field serializes as "type" (not "type_")
        assert!(json.contains("\"type\":"));
        assert!(!json.contains("\"type_\":"));
    }

    #[test]
    fn test_embedding_model_deserialization() {
        let json = r#"{"id":"model-004","version":"1.5","type":"encoder","dimension":768,"metric":"l2","embedding_space_id":"ada-001"}"#;
        let model: EmbeddingModel = serde_json::from_str(json).unwrap();

        assert_eq!(model.id, "model-004");
        assert_eq!(model.version, "1.5");
        assert_eq!(model.dimension, 768);
        assert_eq!(model.embedding_space_id, "ada-001");
    }

    #[test]
    fn test_embedding_model_roundtrip() {
        let model = EmbeddingModel::builder()
            .id("model-005")
            .version("3.0")
            .type_(ModelType::Slm)
            .dimension(1024)
            .metric(DistanceMetric::Dot)
            .embedding_space_id("large-model")
            .normalization(Some(NormalizationType::L2Norm))
            .build()
            .unwrap();

        let json = serde_json::to_string(&model).unwrap();
        let parsed: EmbeddingModel = serde_json::from_str(&json).unwrap();
        assert_eq!(model, parsed);
    }

    #[test]
    fn test_embedding_model_default() {
        let model = EmbeddingModel::builder()
            .id("model-006")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("default-space")
            .build()
            .unwrap();

        assert_eq!(model.id, "model-006");
        assert!(model.normalization.is_none());
        assert!(model.ext.is_none());
    }
}
