use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::campaign_score::CampaignScore;

/// Response containing campaign scores for embeddings.
///
/// Contains a collection of campaign scores computed in response to a scoring request.
/// Each score represents the affinity of an embedding against a specific campaign.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ScoringResponse<Ext: Extension = crate::DefaultExt> {
    /// Collection of campaign scores computed by the scoring model.
    #[serde(default)]
    #[builder(default)]
    pub scores: Vec<CampaignScore<Ext>>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ScoringResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ScoringResponseBuilder {
        ScoringResponseBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring_response_creation() {
        let score = CampaignScore::builder()
            .campaign_id("camp-001")
            .score(0.85)
            .build()
            .unwrap();

        let response = ScoringResponse::builder()
            .scores(vec![score])
            .build()
            .unwrap();

        assert_eq!(response.scores.len(), 1);
        assert_eq!(response.scores[0].campaign_id, "camp-001");
        assert_eq!(response.scores[0].score, 0.85);
        assert!(response.ext.is_none());
    }

    #[test]
    fn test_scoring_response_serialization() {
        let score1 = CampaignScore::builder()
            .campaign_id("camp-002")
            .score(0.92)
            .percentile(Some(0.95))
            .build()
            .unwrap();

        let score2 = CampaignScore::builder()
            .campaign_id("camp-003")
            .score(0.45)
            .build()
            .unwrap();

        let response = ScoringResponse::builder()
            .scores(vec![score1, score2])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"scores\""));
        assert!(json.contains("\"camp-002\""));
        assert!(json.contains("0.92"));
        assert!(json.contains("\"camp-003\""));
        assert!(json.contains("0.45"));
    }

    #[test]
    fn test_scoring_response_deserialization() {
        let json = r#"{
            "scores": [
                {
                    "campaign_id": "camp-004",
                    "score": 0.75,
                    "percentile": 0.80
                },
                {
                    "campaign_id": "camp-005",
                    "score": 0.55
                }
            ]
        }"#;

        let response: ScoringResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.scores.len(), 2);
        assert_eq!(response.scores[0].campaign_id, "camp-004");
        assert_eq!(response.scores[0].score, 0.75);
        assert_eq!(response.scores[0].percentile, Some(0.80));
        assert_eq!(response.scores[1].campaign_id, "camp-005");
    }

    #[test]
    fn test_scoring_response_roundtrip() {
        let score1 = CampaignScore::builder()
            .campaign_id("camp-006")
            .score(0.88)
            .percentile(Some(0.90))
            .build()
            .unwrap();

        let score2 = CampaignScore::builder()
            .campaign_id("camp-007")
            .score(0.62)
            .percentile(Some(0.70))
            .build()
            .unwrap();

        let response = ScoringResponse::builder()
            .scores(vec![score1, score2])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: ScoringResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response, parsed);
    }

    #[test]
    fn test_scoring_response_default() {
        let response = ScoringResponse::builder().build().unwrap();

        assert!(response.scores.is_empty());
        assert!(response.ext.is_none());
    }
}
