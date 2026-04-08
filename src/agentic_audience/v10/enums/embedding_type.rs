use serde::{Deserialize, Serialize};

/// Embedding type combining signal type and subtype into a flat enumeration.
///
/// This enum represents specific combinations of signal types and subtypes for embedding
/// semantic information about audience signals. For example, `IdentityPii` combines the
/// "Identity" signal type with "Pii" subtype for direct identifiers like email or phone.
///
/// Each variant serializes to snake_case format (e.g., `IdentityPii` → `"identity_pii"`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingType {
    /// Identity-based PII signals (direct identifiers like email, phone, SSO).
    #[default]
    IdentityPii,
    /// Identity-based behavioral signals derived from user behavior patterns.
    IdentityBehavioral,
    /// Identity-based demographic signals (age, gender, income).
    IdentityDemographic,
    /// Identity-based graph signals from social and professional networks.
    IdentityGraph,
    /// Context-based content signals about the environment and media.
    ContextContent,
    /// Context-based temporal signals (time of day, season, trend).
    ContextTemporal,
    /// Context-based geospatial signals (location, geography, proximity).
    ContextGeospatial,
    /// Context-based device signals (device type, OS, version).
    ContextDevice,
    /// Context-based session signals (session duration, interaction patterns).
    ContextSession,
    /// Reinforcement signals from engagement metrics and interaction data.
    ReinforcementEngagement,
    /// Reinforcement signals from conversion tracking and attribution.
    ReinforcementConversion,
    /// Reinforcement signals from multi-touch attribution models.
    ReinforcementAttribution,
    /// Reinforcement signals from user feedback and satisfaction metrics.
    ReinforcementFeedback,
    /// Creative-based visual signals from image and video analysis.
    CreativeVisual,
    /// Creative-based textual signals from copy, headline, and description analysis.
    CreativeTextual,
    /// Creative-based multimodal signals combining visual, audio, and text.
    CreativeMultimodal,
    /// Creative-based performance signals from historical campaign results.
    CreativePerformance,
    /// Inventory signals from publisher information.
    InventoryPublisher,
    /// Inventory signals from placement and format characteristics.
    InventoryPlacement,
    /// Inventory signals from audience composition and targeting.
    InventoryAudience,
    /// Query-based search signals from search terms and intent.
    QuerySearch,
    /// Query-based buyer intent signals from purchase behavior.
    QueryBuyerIntent,
    /// Query-based seller offer signals from supply-side indicators.
    QuerySellerOffer,
    /// Conversions API signals for tracking conversion events.
    Capi,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("identity_pii", EmbeddingType::IdentityPii),
            ("identity_behavioral", EmbeddingType::IdentityBehavioral),
            ("identity_demographic", EmbeddingType::IdentityDemographic),
            ("identity_graph", EmbeddingType::IdentityGraph),
            ("context_content", EmbeddingType::ContextContent),
            ("context_temporal", EmbeddingType::ContextTemporal),
            ("context_geospatial", EmbeddingType::ContextGeospatial),
            ("context_device", EmbeddingType::ContextDevice),
            ("context_session", EmbeddingType::ContextSession),
            (
                "reinforcement_engagement",
                EmbeddingType::ReinforcementEngagement,
            ),
            (
                "reinforcement_conversion",
                EmbeddingType::ReinforcementConversion,
            ),
            (
                "reinforcement_attribution",
                EmbeddingType::ReinforcementAttribution,
            ),
            (
                "reinforcement_feedback",
                EmbeddingType::ReinforcementFeedback,
            ),
            ("creative_visual", EmbeddingType::CreativeVisual),
            ("creative_textual", EmbeddingType::CreativeTextual),
            ("creative_multimodal", EmbeddingType::CreativeMultimodal),
            ("creative_performance", EmbeddingType::CreativePerformance),
            ("inventory_publisher", EmbeddingType::InventoryPublisher),
            ("inventory_placement", EmbeddingType::InventoryPlacement),
            ("inventory_audience", EmbeddingType::InventoryAudience),
            ("query_search", EmbeddingType::QuerySearch),
            ("query_buyer_intent", EmbeddingType::QueryBuyerIntent),
            ("query_seller_offer", EmbeddingType::QuerySellerOffer),
            ("capi", EmbeddingType::Capi),
        ];

        for (json_str, expected) in values {
            let result: EmbeddingType = serde_json::from_str(&format!("\"{}\"", json_str))
                .expect(&format!("Failed to deserialize: {}", json_str));
            assert_eq!(
                result, expected,
                "Failed for value: {} (expected {:?}, got {:?})",
                json_str, expected, result
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_embedding_type\"";
        let result: Result<EmbeddingType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
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

        for original in values {
            let json = serde_json::to_string(&original)
                .expect(&format!("Failed to serialize: {:?}", original));
            let deserialized: EmbeddingType =
                serde_json::from_str(&json).expect(&format!("Failed to deserialize: {}", json));
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = EmbeddingType::default();
        assert_eq!(
            default,
            EmbeddingType::IdentityPii,
            "Default should be IdentityPii"
        );
    }

    #[test]
    fn test_integer_value_rejected() {
        // Spec: Agentic Audience v1.0 — enums are string-serialized, integers must be rejected
        let result: Result<EmbeddingType, _> = serde_json::from_str("42");
        assert!(result.is_err(), "Integer value should be rejected");
    }
}
