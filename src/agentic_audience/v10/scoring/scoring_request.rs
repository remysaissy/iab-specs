use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Request to score embeddings against campaigns.
///
/// Contains a collection of embeddings to be scored against one or more campaigns,
/// optionally filtered by campaign IDs.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ScoringRequest<Ext: Extension = crate::DefaultExt> {
    /// Collection of embeddings to be scored (required).
    #[serde(default)]
    #[builder(default)]
    pub embeddings: Vec<super::super::models::Embedding<Ext>>,

    /// Optional list of campaign IDs to filter scoring against.
    /// If omitted, all campaigns are scored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub campaign_ids: Option<Vec<String>>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ScoringRequest {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ScoringRequestBuilder {
        ScoringRequestBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::enums::EmbeddingType;
    use super::*;

    #[test]
    fn test_scoring_request_creation() {
        let emb = super::super::super::models::Embedding::builder()
            .id("emb-001")
            .type_(EmbeddingType::IdentityBehavioral)
            .dimension(128)
            .vector(Some(vec![0.1, 0.2]))
            .build()
            .unwrap();

        let request = ScoringRequest::builder()
            .embeddings(vec![emb])
            .campaign_ids(Some(vec!["camp-001".to_string()]))
            .build()
            .unwrap();

        assert_eq!(request.embeddings.len(), 1);
        assert_eq!(request.campaign_ids, Some(vec!["camp-001".to_string()]));
        assert!(request.ext.is_none());
    }

    #[test]
    fn test_scoring_request_serialization() {
        let emb = super::super::super::models::Embedding::builder()
            .id("emb-002")
            .type_(EmbeddingType::ContextContent)
            .dimension(64)
            .vector(Some(vec![0.5]))
            .build()
            .unwrap();

        let request = ScoringRequest::builder()
            .embeddings(vec![emb])
            .campaign_ids(Some(vec!["camp-001".to_string(), "camp-002".to_string()]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"embeddings\""));
        assert!(json.contains("\"campaign_ids\""));
        assert!(json.contains("\"camp-001\""));
        assert!(json.contains("\"camp-002\""));
    }

    #[test]
    fn test_scoring_request_deserialization() {
        let json = r#"{
            "embeddings": [
                {
                    "id": "emb-003",
                    "type": "identity_behavioral",
                    "dimension": 32,
                    "vector": [0.1, 0.2, 0.3]
                }
            ],
            "campaign_ids": ["camp-003"]
        }"#;

        let request: ScoringRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.embeddings.len(), 1);
        assert_eq!(request.campaign_ids, Some(vec!["camp-003".to_string()]));
    }

    #[test]
    fn test_scoring_request_roundtrip() {
        let emb1 = super::super::super::models::Embedding::builder()
            .id("emb-004")
            .type_(EmbeddingType::CreativeVisual)
            .dimension(256)
            .vector(Some(vec![0.7, 0.8, 0.9]))
            .build()
            .unwrap();

        let emb2 = super::super::super::models::Embedding::builder()
            .id("emb-005")
            .type_(EmbeddingType::InventoryPublisher)
            .dimension(256)
            .vector(Some(vec![0.2, 0.3]))
            .build()
            .unwrap();

        let request = ScoringRequest::builder()
            .embeddings(vec![emb1, emb2])
            .campaign_ids(Some(vec!["camp-004".to_string()]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: ScoringRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, parsed);
    }

    #[test]
    fn test_scoring_request_default() {
        let request = ScoringRequest::builder().build().unwrap();

        assert!(request.embeddings.is_empty());
        assert!(request.campaign_ids.is_none());
        assert!(request.ext.is_none());
    }

    #[test]
    fn test_scoring_request_empty_embeddings_accepted() {
        // Spec: empty embeddings array is accepted at builder level
        let request = ScoringRequest::builder()
            .embeddings(vec![])
            .campaign_ids(Some(vec!["camp-001".to_string()]))
            .build()
            .unwrap();
        assert!(request.embeddings.is_empty());
    }
}
