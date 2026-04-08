use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Individual campaign score result.
///
/// Represents the computed score of an embedding against a specific campaign,
/// including optional percentile information for relative ranking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct CampaignScore<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the campaign (required).
    #[builder(setter(into))]
    pub campaign_id: String,

    /// The computed score value, typically normalized to [0.0, 1.0] range (required).
    pub score: f64,

    /// Optional percentile ranking of this score among all campaigns (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub percentile: Option<f64>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl CampaignScore {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CampaignScoreBuilder {
        CampaignScoreBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_score_creation() {
        let score = CampaignScore::builder()
            .campaign_id("camp-001")
            .score(0.85)
            .build()
            .unwrap();

        assert_eq!(score.campaign_id, "camp-001");
        assert_eq!(score.score, 0.85);
        assert!(score.percentile.is_none());
        assert!(score.ext.is_none());
    }

    #[test]
    fn test_campaign_score_serialization() {
        let score = CampaignScore::builder()
            .campaign_id("camp-002")
            .score(0.92)
            .percentile(Some(0.95))
            .build()
            .unwrap();

        let json = serde_json::to_string(&score).unwrap();
        assert!(json.contains("\"campaign_id\":\"camp-002\""));
        assert!(json.contains("0.92"));
        assert!(json.contains("\"percentile\":0.95"));
    }

    #[test]
    fn test_campaign_score_deserialization() {
        let json = r#"{"campaign_id":"camp-003","score":0.75,"percentile":0.80}"#;
        let score: CampaignScore = serde_json::from_str(json).unwrap();

        assert_eq!(score.campaign_id, "camp-003");
        assert_eq!(score.score, 0.75);
        assert_eq!(score.percentile, Some(0.80));
        assert!(score.ext.is_none());
    }

    #[test]
    fn test_campaign_score_roundtrip() {
        let score = CampaignScore::builder()
            .campaign_id("camp-004")
            .score(0.62)
            .percentile(Some(0.70))
            .build()
            .unwrap();

        let json = serde_json::to_string(&score).unwrap();
        let parsed: CampaignScore = serde_json::from_str(&json).unwrap();
        assert_eq!(score, parsed);
    }

    #[test]
    fn test_campaign_score_default() {
        let score = CampaignScore::builder()
            .campaign_id("camp-default")
            .score(0.5)
            .build()
            .unwrap();

        assert_eq!(score.campaign_id, "camp-default");
        assert_eq!(score.score, 0.5);
        assert!(score.percentile.is_none());
        assert!(score.ext.is_none());
    }

    #[test]
    fn test_campaign_score_malformed_json_rejected() {
        // Spec: score field must be numeric (f64)
        let json = r#"{"campaign_id": "camp-001", "score": "high"}"#;
        let result: Result<CampaignScore, _> = serde_json::from_str(json);
        assert!(result.is_err(), "String score should be rejected");
    }
}
