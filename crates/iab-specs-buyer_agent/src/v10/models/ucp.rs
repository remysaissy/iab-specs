use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// UCPEmbedding represents a Universal Common Platform (UCP) vector embedding with metadata.
///
/// Embeddings encode audience characteristics, behavioral signals, or content attributes
/// as a dense vector suitable for similarity-based matching and targeting.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::UCPEmbedding;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let embedding = UCPEmbedding::builder()
///     .vector(vec![0.1, 0.2, 0.3])
///     .model_descriptor("bert-base-uncased-v1")
///     .dimension(3)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct UCPEmbedding<Ext: Extension = crate::DefaultExt> {
    /// The embedding vector as a list of floating-point values (REQUIRED).
    #[serde(default)]
    #[builder(default)]
    pub vector: Vec<f32>,

    /// Human-readable descriptor identifying the model that produced this embedding (REQUIRED).
    #[builder(setter(into))]
    pub model_descriptor: String,

    /// Dimensionality of the embedding vector (REQUIRED).
    #[builder(default)]
    pub dimension: i32,

    /// Consent status for this embedding (e.g., "opt-in", "opt-out", "unknown").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub consent: Option<String>,

    /// Time-to-live in seconds indicating when this embedding expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ttl: Option<i64>,

    /// Extension object for embedding-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl UCPEmbedding {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> UCPEmbeddingBuilder {
        UCPEmbeddingBuilder::create_empty()
    }
}

/// AudiencePlan represents a targeted audience strategy with embedding-based querying and coverage estimates.
///
/// Audience plans define how embeddings are used to target audiences, including coverage estimates,
/// targeting criteria, and performance predictions.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::AudiencePlan;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let plan = AudiencePlan::builder()
///     .query_embedding(vec![0.1, 0.2, 0.3])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AudiencePlan<Ext: Extension = crate::DefaultExt> {
    /// The query embedding vector for audience matching (REQUIRED).
    #[serde(default)]
    #[builder(default)]
    pub query_embedding: Vec<f32>,

    /// Coverage estimates as an arbitrary JSON blob (e.g., reach, frequency, CPM).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub coverage_estimates: Option<serde_json::Value>,

    /// Targeting criteria as an arbitrary JSON blob (e.g., demographics, interests, behaviors).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub targeting_criteria: Option<serde_json::Value>,

    /// Extension object for plan-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AudiencePlan {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AudiencePlanBuilder {
        AudiencePlanBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== UCPEmbedding Tests ==========

    #[test]
    fn test_ucp_embedding_minimal() {
        let embedding = UCPEmbedding::builder()
            .vector(vec![0.1, 0.2, 0.3])
            .model_descriptor("bert-base-uncased")
            .dimension(3)
            .build()
            .unwrap();

        assert_eq!(embedding.vector, vec![0.1, 0.2, 0.3]);
        assert_eq!(embedding.model_descriptor, "bert-base-uncased");
        assert_eq!(embedding.dimension, 3);
        assert!(embedding.consent.is_none());
        assert!(embedding.ttl.is_none());
        assert!(embedding.ext.is_none());
    }

    #[test]
    fn test_ucp_embedding_full() {
        let embedding = UCPEmbedding::builder()
            .vector(vec![0.1, 0.2, 0.3, 0.4, 0.5])
            .model_descriptor("sentence-transformers/all-mpnet-base-v2")
            .dimension(5)
            .consent("opt-in")
            .ttl(Some(86400))
            .build()
            .unwrap();

        assert_eq!(embedding.vector.len(), 5);
        assert_eq!(embedding.dimension, 5);
        assert_eq!(embedding.consent, Some("opt-in".to_string()));
        assert_eq!(embedding.ttl, Some(86400));
    }

    #[test]
    fn test_ucp_embedding_384_dimensional_roundtrip() {
        // Test with 384-dimensional vector for floating-point precision
        let vector: Vec<f32> = (0..384).map(|i| (i as f32) * 0.001).collect();
        let embedding = UCPEmbedding::builder()
            .vector(vector.clone())
            .model_descriptor("sentence-transformers/all-mpnet-base-v2")
            .dimension(384)
            .build()
            .unwrap();

        // Verify roundtrip preserves all values
        let json = serde_json::to_string(&embedding).unwrap();
        let parsed: UCPEmbedding = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.vector.len(), 384);
        assert_eq!(parsed.dimension, 384);
        assert_eq!(
            parsed.model_descriptor,
            "sentence-transformers/all-mpnet-base-v2"
        );

        // Verify floating-point precision
        for (i, (original, parsed_val)) in vector.iter().zip(parsed.vector.iter()).enumerate() {
            assert!(
                (original - parsed_val).abs() < 1e-6,
                "Mismatch at index {}: {} vs {}",
                i,
                original,
                parsed_val
            );
        }
    }

    #[test]
    fn test_ucp_embedding_serialization() {
        let embedding = UCPEmbedding::builder()
            .vector(vec![0.5, 0.6, 0.7])
            .model_descriptor("gpt2-embedding")
            .dimension(3)
            .consent("opt-out")
            .ttl(Some(3600))
            .build()
            .unwrap();

        let json = serde_json::to_string(&embedding).unwrap();
        assert!(json.contains("\"vector\":[0.5,0.6,0.7]"));
        assert!(json.contains("\"model_descriptor\":\"gpt2-embedding\""));
        assert!(json.contains("\"dimension\":3"));
        assert!(json.contains("\"consent\":\"opt-out\""));
        assert!(json.contains("\"ttl\":3600"));
    }

    #[test]
    fn test_ucp_embedding_deserialization() {
        let json = r#"{
            "vector": [0.1, 0.2, 0.3],
            "model_descriptor": "clip-vit-base-patch32",
            "dimension": 3,
            "consent": "unknown",
            "ttl": 7200
        }"#;

        let embedding: UCPEmbedding = serde_json::from_str(json).unwrap();
        assert_eq!(embedding.vector, vec![0.1, 0.2, 0.3]);
        assert_eq!(embedding.model_descriptor, "clip-vit-base-patch32");
        assert_eq!(embedding.dimension, 3);
        assert_eq!(embedding.consent, Some("unknown".to_string()));
        assert_eq!(embedding.ttl, Some(7200));
    }

    #[test]
    fn test_ucp_embedding_empty_vector() {
        let embedding = UCPEmbedding::builder()
            .vector(vec![])
            .model_descriptor("empty-model")
            .dimension(0)
            .build()
            .unwrap();

        assert_eq!(embedding.vector.len(), 0);
        assert_eq!(embedding.dimension, 0);
    }

    // ========== AudiencePlan Tests ==========

    #[test]
    fn test_audience_plan_minimal() {
        let plan = AudiencePlan::builder()
            .query_embedding(vec![0.1, 0.2, 0.3])
            .build()
            .unwrap();

        assert_eq!(plan.query_embedding, vec![0.1, 0.2, 0.3]);
        assert!(plan.coverage_estimates.is_none());
        assert!(plan.targeting_criteria.is_none());
        assert!(plan.ext.is_none());
    }

    #[test]
    fn test_audience_plan_full() {
        let plan = AudiencePlan::builder()
            .query_embedding(vec![0.1, 0.2, 0.3, 0.4, 0.5])
            .coverage_estimates(Some(serde_json::json!({
                "reach": 1000000,
                "frequency": 5,
                "cpm": 2.50
            })))
            .targeting_criteria(Some(serde_json::json!({
                "age_range": "25-54",
                "interests": ["tech", "finance"],
                "device": "mobile"
            })))
            .build()
            .unwrap();

        assert_eq!(plan.query_embedding.len(), 5);
        assert!(plan.coverage_estimates.is_some());
        assert!(plan.targeting_criteria.is_some());
    }

    #[test]
    fn test_audience_plan_coverage_estimates_json() {
        let coverage = serde_json::json!({
            "reach": 500000,
            "frequency": 3,
            "cpm": 1.75,
            "estimated_impressions": 1500000
        });

        let plan = AudiencePlan::builder()
            .query_embedding(vec![0.1, 0.2])
            .coverage_estimates(Some(coverage.clone()))
            .build()
            .unwrap();

        assert_eq!(plan.coverage_estimates, Some(coverage.clone()));

        // Verify roundtrip preserves JSON blob
        let json = serde_json::to_string(&plan).unwrap();
        let parsed: AudiencePlan = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.coverage_estimates, Some(coverage));
    }

    #[test]
    fn test_audience_plan_targeting_criteria_json() {
        let criteria = serde_json::json!({
            "age_range": "18-34",
            "location": "US",
            "interests": ["sports", "entertainment", "technology"],
            "behaviors": {
                "e_commerce_shoppers": true,
                "video_viewers": true
            }
        });

        let plan = AudiencePlan::builder()
            .query_embedding(vec![0.2, 0.3])
            .targeting_criteria(Some(criteria.clone()))
            .build()
            .unwrap();

        assert_eq!(plan.targeting_criteria, Some(criteria.clone()));

        // Verify roundtrip preserves JSON blob
        let json = serde_json::to_string(&plan).unwrap();
        let parsed: AudiencePlan = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.targeting_criteria, Some(criteria));
    }

    #[test]
    fn test_audience_plan_serialization() {
        let plan = AudiencePlan::builder()
            .query_embedding(vec![0.5, 0.6])
            .coverage_estimates(Some(serde_json::json!({"reach": 250000})))
            .targeting_criteria(Some(serde_json::json!({"age": "25-54"})))
            .build()
            .unwrap();

        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("\"query_embedding\":[0.5,0.6]"));
        assert!(json.contains("\"coverage_estimates\":{\"reach\":250000}"));
        assert!(json.contains("\"targeting_criteria\":{\"age\":\"25-54\"}"));
    }

    #[test]
    fn test_audience_plan_deserialization() {
        let json = r#"{
            "query_embedding": [0.1, 0.2, 0.3],
            "coverage_estimates": {"reach": 750000, "cpm": 3.00},
            "targeting_criteria": {"gender": "all", "income": "high"}
        }"#;

        let plan: AudiencePlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.query_embedding, vec![0.1, 0.2, 0.3]);
        assert!(plan.coverage_estimates.is_some());
        assert!(plan.targeting_criteria.is_some());
    }

    #[test]
    fn test_audience_plan_384_dimensional_query_embedding() {
        // Test with 384-dimensional query embedding
        let query: Vec<f32> = (0..384).map(|i| (i as f32) * 0.001).collect();
        let plan = AudiencePlan::builder()
            .query_embedding(query.clone())
            .coverage_estimates(Some(serde_json::json!({"reach": 2000000})))
            .build()
            .unwrap();

        // Verify roundtrip preserves all values
        let json = serde_json::to_string(&plan).unwrap();
        let parsed: AudiencePlan = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.query_embedding.len(), 384);
        for (i, (original, parsed_val)) in
            query.iter().zip(parsed.query_embedding.iter()).enumerate()
        {
            assert!(
                (original - parsed_val).abs() < 1e-6,
                "Mismatch at index {}: {} vs {}",
                i,
                original,
                parsed_val
            );
        }
    }

    #[test]
    fn test_ucp_embedding_default_trait() {
        let emb: UCPEmbedding = UCPEmbedding::default();
        assert!(emb.vector.is_empty());
        assert_eq!(emb.model_descriptor, "");
        assert_eq!(emb.dimension, 0);
        assert!(emb.consent.is_none());
        assert!(emb.ttl.is_none());
        assert!(emb.ext.is_none());
    }

    #[test]
    fn test_ucp_embedding_dimension_mismatch_accepted() {
        let emb = UCPEmbedding::builder()
            .vector(vec![0.1, 0.2, 0.3])
            .model_descriptor("test-model")
            .dimension(384)
            .build()
            .unwrap();
        assert_eq!(emb.vector.len(), 3);
        assert_eq!(emb.dimension, 384);
    }

    #[test]
    fn test_ucp_embedding_negative_ttl() {
        let emb = UCPEmbedding::builder()
            .vector(vec![0.1])
            .model_descriptor("test")
            .dimension(1)
            .ttl(Some(-1))
            .build()
            .unwrap();
        assert_eq!(emb.ttl, Some(-1));
    }

    #[test]
    fn test_audience_plan_default_trait() {
        let plan: AudiencePlan = AudiencePlan::default();
        assert!(plan.query_embedding.is_empty());
        assert!(plan.coverage_estimates.is_none());
        assert!(plan.targeting_criteria.is_none());
        assert!(plan.ext.is_none());
    }

    #[test]
    fn test_audience_plan_empty_embedding_accepted() {
        let plan = AudiencePlan::builder()
            .query_embedding(vec![])
            .build()
            .unwrap();
        assert!(plan.query_embedding.is_empty());
    }

    #[test]
    fn test_ucp_embedding_with_json_extension() {
        let emb = UCPEmbeddingBuilder::<serde_json::Value>::default()
            .vector(vec![0.5])
            .model_descriptor("ext-model".to_string())
            .dimension(1)
            .ext(Some(Box::new(serde_json::json!({"provider": "openai"}))))
            .build()
            .unwrap();

        assert!(emb.ext.is_some());
        assert_eq!(emb.ext.as_ref().unwrap()["provider"], "openai");

        let json = serde_json::to_string(&emb).unwrap();
        let parsed: UCPEmbedding<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ext.as_ref().unwrap()["provider"], "openai");
    }
}
