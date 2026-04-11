//! # Agentic Audience v1.0 (Draft)
//!
//! ⚠️ **Draft Specification**: Based on Agentic Audience v1.0 Draft. Breaking changes may occur.
//!
//! Data models for the embedding exchange protocol, including:
//! - Signal taxonomy enums and embedding type classification
//! - Embedding envelope for transport (model, context, embeddings)
//! - Campaign scoring types (head, request, response, score)
//! - OpenRTB bid stream extension (`EmbeddingSegmentExt`)
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Signal taxonomy enumerations and embedding type classification
//! - [`models`] - Embedding envelope, model, context, and embedding types
//! - [`scoring`] - Campaign scoring head, request, response, score, and OpenRTB extension
//!
//! # Quick Start
//!
//! ## Creating an Embedding Envelope
//!
//! ```rust
//! use iab_specs_agentic_audience::v10::{
//!     EmbeddingEnvelope, EmbeddingModel, EmbeddingContext, Embedding,
//!     ModelType, DistanceMetric, EmbeddingType,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an embedding envelope with model, context, and embeddings
//! let envelope = EmbeddingEnvelope::builder()
//!     .model(EmbeddingModel::builder()
//!         .id("minilm-l6-v2")
//!         .version("1.0")
//!         .type_(ModelType::Encoder)
//!         .dimension(384)
//!         .metric(DistanceMetric::Cosine)
//!         .embedding_space_id("sentence-transformers/all-MiniLM-L6-v2")
//!         .build()?)
//!     .context(Some(EmbeddingContext::builder()
//!         .url("https://example.com/article")
//!         .page_title("AI in Advertising")
//!         .language("en")
//!         .keywords(vec!["ai".to_string(), "advertising".to_string()])
//!         .build()?))
//!     .embeddings(vec![
//!         Embedding::builder()
//!             .id("emb-001")
//!             .type_(EmbeddingType::ContextContent)
//!             .dimension(384)
//!             .vector(Some(vec![0.1; 384]))
//!             .build()?,
//!     ])
//!     .build()?;
//!
//! // Serialize to JSON
//! let json = serde_json::to_string_pretty(&envelope)?;
//! assert!(json.contains("\"minilm-l6-v2\""));
//! # Ok(())
//! # }
//! ```
//!
//! ## Scoring Embeddings Against Campaigns
//!
//! ```rust
//! use iab_specs_agentic_audience::v10::{
//!     CampaignHead, ScoringRequest, ScoringResponse, CampaignScore,
//!     Embedding, EmbeddingType,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a campaign head with scoring weights
//! let head = CampaignHead::builder()
//!     .campaign_id("camp-summer-2026")
//!     .head_weights(vec![0.5, 0.3, 0.2])
//!     .dimension(384)
//!     .model_id("minilm-l6-v2")
//!     .build()?;
//!
//! // Create a scoring request with embeddings
//! let request = ScoringRequest::builder()
//!     .embeddings(vec![
//!         Embedding::builder()
//!             .id("emb-001")
//!             .type_(EmbeddingType::IdentityBehavioral)
//!             .dimension(384)
//!             .vector(Some(vec![0.1; 384]))
//!             .build()?,
//!     ])
//!     .campaign_ids(Some(vec!["camp-summer-2026".to_string()]))
//!     .build()?;
//!
//! // Create a scoring response with results
//! let response = ScoringResponse::builder()
//!     .scores(vec![
//!         CampaignScore::builder()
//!             .campaign_id("camp-summer-2026")
//!             .score(0.87)
//!             .percentile(Some(0.92))
//!             .build()?,
//!     ])
//!     .build()?;
//!
//! assert_eq!(response.scores[0].score, 0.87);
//! # Ok(())
//! # }
//! ```
//!
//! ## OpenRTB Bid Stream Extension
//!
//! ```rust
//! use iab_specs_agentic_audience::v10::{
//!     EmbeddingSegmentExt, EmbeddingType, DistanceMetric,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an OpenRTB segment extension for embedding transport
//! let segment_ext = EmbeddingSegmentExt::builder()
//!     .ver("1.0")
//!     .vector(vec![0.1, 0.2, 0.3, 0.4])
//!     .model("minilm-l6-v2")
//!     .dimension(4)
//!     .type_(EmbeddingType::ContextContent)
//!     .metric(Some(DistanceMetric::Cosine))
//!     .ttl(Some(3600))
//!     .build()?;
//!
//! // Serialize — this goes into user.data.segment.ext in OpenRTB
//! let json = serde_json::to_string_pretty(&segment_ext)?;
//! assert!(json.contains("\"ver\": \"1.0\""));
//! # Ok(())
//! # }
//! ```
//!
//! # Specification Reference
//!
//! This implementation follows the [Agentic Audience v1.0 Draft](https://github.com/IABTechLab/agentic-audiences)
//! specification published by IAB Tech Lab.

pub mod enums;
pub mod models;
pub mod scoring;

// Re-export all public types for ergonomic access
pub use enums::*;
pub use models::*;
pub use scoring::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_data_model_workflow() {
        // 1. Create an EmbeddingEnvelope with model + context + 2 embeddings
        let envelope = EmbeddingEnvelope::builder()
            .model(
                EmbeddingModel::builder()
                    .id("minilm-l6-v2")
                    .version("1.0")
                    .type_(ModelType::Encoder)
                    .dimension(384)
                    .metric(DistanceMetric::Cosine)
                    .embedding_space_id("sentence-transformers/all-MiniLM-L6-v2")
                    .normalization(Some(NormalizationType::L2Norm))
                    .build()
                    .unwrap(),
            )
            .context(Some(
                EmbeddingContext::builder()
                    .url("https://example.com/article")
                    .page_title("AI in Advertising Technology")
                    .language("en")
                    .keywords(vec![
                        "ai".to_string(),
                        "advertising".to_string(),
                        "embeddings".to_string(),
                    ])
                    .content_hash("sha256:abc123def456")
                    .build()
                    .unwrap(),
            ))
            .embeddings(vec![
                Embedding::builder()
                    .id("emb-001")
                    .type_(EmbeddingType::ContextContent)
                    .dimension(384)
                    .vector(Some(vec![0.1; 384]))
                    .temporal_scope(Some(TemporalScope::Session))
                    .composition(Some(CompositionType::Atomic))
                    .ttl(Some(3600))
                    .created_at("2026-04-01T12:00:00Z")
                    .build()
                    .unwrap(),
                Embedding::builder()
                    .id("emb-002")
                    .type_(EmbeddingType::IdentityBehavioral)
                    .dimension(384)
                    .vector(Some(vec![0.2; 384]))
                    .temporal_scope(Some(TemporalScope::Persistent))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(envelope.model.id, "minilm-l6-v2");
        assert_eq!(envelope.model.dimension, 384);
        assert!(envelope.context.is_some());
        assert_eq!(envelope.embeddings.len(), 2);

        // 2. Create a ScoringRequest referencing those embeddings
        let scoring_request = ScoringRequest::builder()
            .embeddings(envelope.embeddings.clone())
            .campaign_ids(Some(vec![
                "camp-summer-2026".to_string(),
                "camp-holiday-2026".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(scoring_request.embeddings.len(), 2);
        assert_eq!(scoring_request.campaign_ids.as_ref().unwrap().len(), 2);

        // 3. Create a ScoringResponse with mock scores
        let scoring_response = ScoringResponse::builder()
            .scores(vec![
                CampaignScore::builder()
                    .campaign_id("camp-summer-2026")
                    .score(0.87)
                    .percentile(Some(0.92))
                    .build()
                    .unwrap(),
                CampaignScore::builder()
                    .campaign_id("camp-holiday-2026")
                    .score(0.65)
                    .percentile(Some(0.71))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(scoring_response.scores.len(), 2);
        assert_eq!(scoring_response.scores[0].score, 0.87);
        assert_eq!(scoring_response.scores[1].campaign_id, "camp-holiday-2026");

        // 4. Serialize/deserialize ALL, verify roundtrip
        let envelope_json = serde_json::to_string_pretty(&envelope).unwrap();
        let parsed_envelope: EmbeddingEnvelope = serde_json::from_str(&envelope_json).unwrap();
        assert_eq!(envelope, parsed_envelope);

        let request_json = serde_json::to_string_pretty(&scoring_request).unwrap();
        let parsed_request: ScoringRequest = serde_json::from_str(&request_json).unwrap();
        assert_eq!(scoring_request, parsed_request);

        let response_json = serde_json::to_string_pretty(&scoring_response).unwrap();
        let parsed_response: ScoringResponse = serde_json::from_str(&response_json).unwrap();
        assert_eq!(scoring_response, parsed_response);
    }

    #[test]
    fn test_openrtb_extension() {
        // Create an EmbeddingSegmentExt matching the spec example
        let segment_ext = EmbeddingSegmentExt::builder()
            .ver("1.0")
            .vector(vec![0.12, -0.34, 0.56, 0.78])
            .model("minilm-l6-v2")
            .dimension(4)
            .type_(EmbeddingType::ContextContent)
            .metric(Some(DistanceMetric::Cosine))
            .ttl(Some(3600))
            .build()
            .unwrap();

        assert_eq!(segment_ext.ver, "1.0");
        assert_eq!(segment_ext.vector.len(), 4);
        assert_eq!(segment_ext.model, "minilm-l6-v2");
        assert_eq!(segment_ext.dimension, 4);
        assert_eq!(segment_ext.type_, EmbeddingType::ContextContent);
        assert_eq!(segment_ext.metric, Some(DistanceMetric::Cosine));
        assert_eq!(segment_ext.ttl, Some(3600));

        // Verify JSON format matches expected structure
        let json = serde_json::to_string_pretty(&segment_ext).unwrap();
        assert!(json.contains("\"ver\": \"1.0\""));
        assert!(json.contains("\"model\": \"minilm-l6-v2\""));
        assert!(json.contains("\"type\": \"context_content\""));
        assert!(json.contains("\"metric\": \"cosine\""));
        assert!(json.contains("\"ttl\": 3600"));

        // Verify roundtrip
        let parsed: EmbeddingSegmentExt = serde_json::from_str(&json).unwrap();
        assert_eq!(segment_ext, parsed);
    }

    #[test]
    fn test_all_24_embedding_types() {
        let all_variants = [
            EmbeddingType::IdentityPii,
            EmbeddingType::IdentityBehavioral,
            EmbeddingType::IdentityDemographic,
            EmbeddingType::IdentityGraph,
            EmbeddingType::ContextContent,
            EmbeddingType::ContextTemporal,
            EmbeddingType::ContextGeospatial,
            EmbeddingType::ContextDevice,
            EmbeddingType::ContextSession,
            EmbeddingType::ReinforcementEngagement,
            EmbeddingType::ReinforcementConversion,
            EmbeddingType::ReinforcementAttribution,
            EmbeddingType::ReinforcementFeedback,
            EmbeddingType::CreativeVisual,
            EmbeddingType::CreativeTextual,
            EmbeddingType::CreativeMultimodal,
            EmbeddingType::CreativePerformance,
            EmbeddingType::InventoryPublisher,
            EmbeddingType::InventoryPlacement,
            EmbeddingType::InventoryAudience,
            EmbeddingType::QuerySearch,
            EmbeddingType::QueryBuyerIntent,
            EmbeddingType::QuerySellerOffer,
            EmbeddingType::Capi,
        ];

        assert_eq!(
            all_variants.len(),
            24,
            "Expected exactly 24 EmbeddingType variants"
        );

        for variant in &all_variants {
            // Serialize to JSON
            let json = serde_json::to_string(variant).unwrap();
            // Deserialize back
            let parsed: EmbeddingType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                variant, &parsed,
                "Roundtrip failed for {:?} (serialized as {})",
                variant, json
            );

            // Verify it can be used in an Embedding struct
            let emb = Embedding::builder()
                .id(format!("emb-{}", json.trim_matches('"')))
                .type_(*variant)
                .dimension(64)
                .vector(Some(vec![0.1; 64]))
                .build()
                .unwrap();

            let emb_json = serde_json::to_string(&emb).unwrap();
            let parsed_emb: Embedding = serde_json::from_str(&emb_json).unwrap();
            assert_eq!(emb, parsed_emb);
        }
    }

    #[test]
    fn test_envelope_with_scoring_workflow() {
        // Full end-to-end: create envelope → scoring request → scoring response

        // Step 1: Publisher creates an embedding envelope
        let envelope = EmbeddingEnvelope::builder()
            .model(
                EmbeddingModel::builder()
                    .id("mpnet-base-v2")
                    .version("2.0")
                    .type_(ModelType::Encoder)
                    .dimension(768)
                    .metric(DistanceMetric::Dot)
                    .embedding_space_id("sentence-transformers/all-mpnet-base-v2")
                    .build()
                    .unwrap(),
            )
            .embeddings(vec![
                Embedding::builder()
                    .id("emb-page-context")
                    .type_(EmbeddingType::ContextContent)
                    .dimension(768)
                    .vector(Some(vec![0.05; 768]))
                    .temporal_scope(Some(TemporalScope::RealTime))
                    .composition(Some(CompositionType::Composite))
                    .build()
                    .unwrap(),
                Embedding::builder()
                    .id("emb-user-behavior")
                    .type_(EmbeddingType::IdentityBehavioral)
                    .dimension(768)
                    .vector(Some(vec![0.03; 768]))
                    .temporal_scope(Some(TemporalScope::Persistent))
                    .composition(Some(CompositionType::Atomic))
                    .build()
                    .unwrap(),
                Embedding::builder()
                    .id("emb-engagement")
                    .type_(EmbeddingType::ReinforcementEngagement)
                    .dimension(768)
                    .vector(Some(vec![0.07; 768]))
                    .temporal_scope(Some(TemporalScope::Retrospective))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        // Step 2: DSP creates a scoring request from the envelope's embeddings
        let scoring_request = ScoringRequest::builder()
            .embeddings(envelope.embeddings.clone())
            .campaign_ids(Some(vec![
                "camp-auto-001".to_string(),
                "camp-tech-002".to_string(),
                "camp-finance-003".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(scoring_request.embeddings.len(), 3);
        assert_eq!(scoring_request.embeddings[0].id, envelope.embeddings[0].id);

        // Step 3: Scoring service returns campaign scores
        let scoring_response = ScoringResponse::builder()
            .scores(vec![
                CampaignScore::builder()
                    .campaign_id("camp-auto-001")
                    .score(0.92)
                    .percentile(Some(0.97))
                    .build()
                    .unwrap(),
                CampaignScore::builder()
                    .campaign_id("camp-tech-002")
                    .score(0.78)
                    .percentile(Some(0.83))
                    .build()
                    .unwrap(),
                CampaignScore::builder()
                    .campaign_id("camp-finance-003")
                    .score(0.45)
                    .percentile(Some(0.52))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(scoring_response.scores.len(), 3);
        // Best match is the auto campaign
        let best = scoring_response
            .scores
            .iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
            .unwrap();
        assert_eq!(best.campaign_id, "camp-auto-001");
        assert_eq!(best.score, 0.92);

        // Step 4: Verify full roundtrip serialization for all objects
        let env_json = serde_json::to_string_pretty(&envelope).unwrap();
        let parsed_env: EmbeddingEnvelope = serde_json::from_str(&env_json).unwrap();
        assert_eq!(envelope, parsed_env);

        let req_json = serde_json::to_string_pretty(&scoring_request).unwrap();
        let parsed_req: ScoringRequest = serde_json::from_str(&req_json).unwrap();
        assert_eq!(scoring_request, parsed_req);

        let resp_json = serde_json::to_string_pretty(&scoring_response).unwrap();
        let parsed_resp: ScoringResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(scoring_response, parsed_resp);

        // Step 5: Also verify CampaignHead can be created for this model
        let head = CampaignHead::builder()
            .campaign_id("camp-auto-001")
            .head_weights(vec![0.4, 0.35, 0.25])
            .dimension(768)
            .model_id("mpnet-base-v2")
            .created_at("2026-04-01T00:00:00Z")
            .build()
            .unwrap();

        let head_json = serde_json::to_string_pretty(&head).unwrap();
        let parsed_head: CampaignHead = serde_json::from_str(&head_json).unwrap();
        assert_eq!(head, parsed_head);
        assert_eq!(head.dimension, envelope.model.dimension);
    }
}
