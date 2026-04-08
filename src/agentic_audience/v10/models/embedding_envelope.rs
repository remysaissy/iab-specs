use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::embedding::Embedding;
use super::embedding_context::EmbeddingContext;
use super::embedding_model::EmbeddingModel;

/// Top-level transport container for embeddings.
///
/// Wraps a model descriptor, optional contextual metadata, and a vector of embeddings.
/// This is the primary struct for embedding envelope serialization and deserialization in the
/// Agentic Audience protocol.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EmbeddingEnvelope<Ext: Extension = crate::DefaultExt> {
    /// The embedding model descriptor.
    pub model: EmbeddingModel<Ext>,

    /// Optional contextual metadata about the embeddings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub context: Option<EmbeddingContext<Ext>>,

    /// Vector of embeddings produced by the model.
    #[serde(default)]
    #[builder(default)]
    pub embeddings: Vec<Embedding<Ext>>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl EmbeddingEnvelope {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EmbeddingEnvelopeBuilder {
        EmbeddingEnvelopeBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agentic_audience::v10::enums::{DistanceMetric, EmbeddingType, ModelType};

    #[test]
    fn test_envelope_creation() {
        let model = EmbeddingModel::builder()
            .id("model-001")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("test-space")
            .build()
            .unwrap();

        let context = EmbeddingContext::builder()
            .url("https://example.com/article")
            .page_title("Test Article")
            .language("en")
            .build()
            .unwrap();

        let embedding1 = Embedding::builder()
            .id("emb-1")
            .type_(EmbeddingType::ContextContent)
            .dimension(384)
            .vector(Some(vec![0.1; 384]))
            .build()
            .unwrap();

        let embedding2 = Embedding::builder()
            .id("emb-2")
            .type_(EmbeddingType::IdentityBehavioral)
            .dimension(384)
            .vector(Some(vec![0.2; 384]))
            .build()
            .unwrap();

        let envelope = EmbeddingEnvelope::builder()
            .model(model.clone())
            .context(Some(context.clone()))
            .embeddings(vec![embedding1, embedding2])
            .build()
            .unwrap();

        assert_eq!(envelope.model, model);
        assert_eq!(envelope.context, Some(context));
        assert_eq!(envelope.embeddings.len(), 2);
        assert!(envelope.ext.is_none());
    }

    #[test]
    fn test_envelope_serialization() {
        let model = EmbeddingModel::builder()
            .id("model-002")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("test-space")
            .build()
            .unwrap();

        let embedding = Embedding::builder()
            .id("emb-1")
            .type_(EmbeddingType::ContextContent)
            .dimension(384)
            .vector(Some(vec![0.1; 384]))
            .build()
            .unwrap();

        let envelope = EmbeddingEnvelope::builder()
            .model(model)
            .embeddings(vec![embedding])
            .build()
            .unwrap();

        let json = serde_json::to_string(&envelope).unwrap();

        // Verify top-level keys
        assert!(json.contains("\"model\":"));
        assert!(json.contains("\"embeddings\":"));
        // context should be omitted when None
        assert!(!json.contains("\"context\":"));
    }

    #[test]
    fn test_envelope_deserialization() {
        let json = r#"{
            "model": {
                "id": "model-003",
                "version": "1.0",
                "type": "encoder",
                "dimension": 384,
                "metric": "cosine",
                "embedding_space_id": "test-space"
            },
            "context": {
                "url": "https://example.com",
                "page_title": "Example",
                "language": "en"
            },
            "embeddings": [
                {
                    "id": "emb-1",
                    "type": "context_content",
                    "dimension": 384,
                    "vector": [0.1, 0.2, 0.3]
                }
            ]
        }"#;

        let envelope: EmbeddingEnvelope = serde_json::from_str(json).unwrap();

        assert_eq!(envelope.model.id, "model-003");
        assert_eq!(envelope.model.dimension, 384);
        assert!(envelope.context.is_some());
        assert_eq!(envelope.embeddings.len(), 1);
        assert_eq!(envelope.embeddings[0].id, "emb-1");
    }

    #[test]
    fn test_envelope_roundtrip() {
        let model = EmbeddingModel::builder()
            .id("model-004")
            .version("2.0")
            .type_(ModelType::Llm)
            .dimension(768)
            .metric(DistanceMetric::L2)
            .embedding_space_id("advanced-space")
            .build()
            .unwrap();

        let context = EmbeddingContext::builder()
            .url("https://example.com/news")
            .page_title("Breaking News")
            .keywords(vec!["tech".to_string(), "ai".to_string()])
            .language("en")
            .build()
            .unwrap();

        let embedding1 = Embedding::builder()
            .id("emb-1")
            .type_(EmbeddingType::ContextContent)
            .dimension(768)
            .vector(Some(vec![0.1; 768]))
            .build()
            .unwrap();

        let embedding2 = Embedding::builder()
            .id("emb-2")
            .type_(EmbeddingType::IdentityBehavioral)
            .dimension(768)
            .vector(Some(vec![0.2; 768]))
            .build()
            .unwrap();

        let embedding3 = Embedding::builder()
            .id("emb-3")
            .type_(EmbeddingType::ReinforcementConversion)
            .dimension(768)
            .vector(Some(vec![0.3; 768]))
            .build()
            .unwrap();

        let original = EmbeddingEnvelope::builder()
            .model(model)
            .context(Some(context))
            .embeddings(vec![embedding1, embedding2, embedding3])
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: EmbeddingEnvelope = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.model, original.model);
        assert_eq!(parsed.context, original.context);
        assert_eq!(parsed.embeddings.len(), original.embeddings.len());
        assert_eq!(parsed.embeddings[0].id, "emb-1");
        assert_eq!(parsed.embeddings[1].id, "emb-2");
        assert_eq!(parsed.embeddings[2].id, "emb-3");
    }

    #[test]
    fn test_envelope_default() {
        let envelope = EmbeddingEnvelope::builder().build().unwrap();

        // model has defaults (empty string, 0, etc.)
        assert_eq!(envelope.model.id, "");
        assert_eq!(envelope.model.dimension, 0);
        assert!(envelope.context.is_none());
        assert!(envelope.embeddings.is_empty());
        assert!(envelope.ext.is_none());
    }

    #[test]
    fn test_envelope_without_context() {
        let model = EmbeddingModel::builder()
            .id("model-005")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("test-space")
            .build()
            .unwrap();

        let embedding = Embedding::builder()
            .id("emb-1")
            .type_(EmbeddingType::ContextContent)
            .dimension(384)
            .vector(Some(vec![0.1; 384]))
            .build()
            .unwrap();

        let envelope = EmbeddingEnvelope::builder()
            .model(model)
            .embeddings(vec![embedding])
            .build()
            .unwrap();

        let json = serde_json::to_string(&envelope).unwrap();

        // Verify context is omitted from JSON when None
        assert!(!json.contains("\"context\":"));

        // Verify it deserializes correctly without context
        let parsed: EmbeddingEnvelope = serde_json::from_str(&json).unwrap();
        assert!(parsed.context.is_none());
        assert_eq!(parsed.embeddings.len(), 1);
    }

    #[test]
    fn test_envelope_empty_embeddings_accepted() {
        // Spec: empty embeddings array is accepted at builder level
        let model = EmbeddingModel::builder()
            .id("m")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("s")
            .build()
            .unwrap();
        let envelope = EmbeddingEnvelope::builder()
            .model(model)
            .embeddings(vec![])
            .build()
            .unwrap();
        assert!(envelope.embeddings.is_empty());
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed: EmbeddingEnvelope = serde_json::from_str(&json).unwrap();
        assert!(parsed.embeddings.is_empty());
    }

    #[test]
    fn test_envelope_malformed_json_rejected() {
        // Spec: nested required fields must be present
        let json = r#"{"model": "not_an_object", "embeddings": []}"#;
        let result: Result<EmbeddingEnvelope, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid model object should be rejected");
    }
}
