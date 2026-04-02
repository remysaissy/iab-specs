use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Placement entity.
///
/// Represents the placement details for a line item, including the ad unit code and
/// optional targeting overrides.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Placement<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Line identifier this placement belongs to (required).
    #[builder(setter(into))]
    pub line_id: String,

    /// Ad unit code or placement key used by the publisher (required).
    #[builder(setter(into))]
    pub ad_unit_code: String,

    /// Arbitrary JSON payload for placement-specific targeting overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub overrides: Option<serde_json::Value>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Placement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PlacementBuilder {
        PlacementBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placement_creation() {
        let placement = Placement::builder()
            .line_id("line-001")
            .ad_unit_code("homepage_top")
            .build()
            .unwrap();

        assert_eq!(placement.line_id, "line-001");
        assert_eq!(placement.ad_unit_code, "homepage_top");
        assert!(placement.id.is_none());
        assert!(placement.overrides.is_none());
        assert!(placement.ext.is_none());
    }

    #[test]
    fn test_placement_serialization() {
        let placement = Placement::builder()
            .id("plc-123")
            .line_id("line-002")
            .ad_unit_code("article_mid")
            .overrides(Some(serde_json::json!({"geo": ["US"], "device": "mobile"})))
            .build()
            .unwrap();

        let json = serde_json::to_string(&placement).unwrap();
        assert!(json.contains("\"id\":\"plc-123\""));
        assert!(json.contains("\"line_id\":\"line-002\""));
        assert!(json.contains("\"ad_unit_code\":\"article_mid\""));
        assert!(json.contains("\"overrides\":{\"device\":\"mobile\",\"geo\":[\"US\"]}"));
    }

    #[test]
    fn test_placement_deserialization() {
        let json = r#"{"line_id":"line-003","ad_unit_code":"video_preroll","overrides":{"bid_modifier":1.2,"deal_ids":["deal-1","deal-2"]}}"#;
        let placement: Placement = serde_json::from_str(json).unwrap();

        assert_eq!(placement.line_id, "line-003");
        assert_eq!(placement.ad_unit_code, "video_preroll");
        assert_eq!(placement.overrides.as_ref().unwrap()["bid_modifier"], 1.2);
        assert_eq!(
            placement.overrides.as_ref().unwrap()["deal_ids"][0],
            "deal-1"
        );
        assert_eq!(
            placement.overrides.as_ref().unwrap()["deal_ids"][1],
            "deal-2"
        );
    }

    #[test]
    fn test_placement_roundtrip() {
        let placement = Placement::builder()
            .id("plc-999")
            .line_id("line-004")
            .ad_unit_code("sidebar_rectangle")
            .overrides(Some(
                serde_json::json!({"floor": 3.25, "viewability": "high"}),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&placement).unwrap();
        let parsed: Placement = serde_json::from_str(&json).unwrap();
        assert_eq!(placement, parsed);
    }

    #[test]
    fn test_placement_with_overrides_json() {
        let overrides = serde_json::json!({
            "targeting": {
                "sections": ["sports", "news"],
                "audience": {"segments": ["fans", "subscribers"]}
            },
            "caps": {
                "per_user": 2
            }
        });

        let placement = Placement::builder()
            .line_id("line-005")
            .ad_unit_code("native_feed")
            .overrides(Some(overrides.clone()))
            .build()
            .unwrap();

        assert_eq!(placement.overrides, Some(overrides));

        let json = serde_json::to_string(&placement).unwrap();
        let parsed: Placement = serde_json::from_str(&json).unwrap();
        assert_eq!(placement.overrides, parsed.overrides);
    }

    #[test]
    fn test_placement_malformed_json_rejected() {
        let json = r#"not valid json at all"#;
        let result: Result<Placement, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#"{"line_id": 12345, "ad_unit_code": true}"#;
        let result: Result<Placement, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Wrong field types should fail");
    }
}
