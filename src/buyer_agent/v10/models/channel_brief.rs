use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A ChannelBrief represents a per-channel breakdown of a campaign.
///
/// Channel briefs contain channel-specific objectives, budget allocation, and constraints
/// that drive execution within a single channel (e.g., "display", "video", "email").
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::ChannelBrief;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let channel = ChannelBrief::builder()
///     .channel("display")
///     .budget(25000.0)
///     .objectives(vec!["brand awareness".to_string(), "reach".to_string()])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ChannelBrief<Ext: Extension = crate::DefaultExt> {
    /// Channel identifier (e.g., "display", "video", "email") (REQUIRED).
    #[builder(setter(into))]
    pub channel: String,

    /// List of channel-specific objectives.
    #[serde(default)]
    #[builder(default)]
    pub objectives: Vec<String>,

    /// Channel budget allocation in currency units (REQUIRED).
    #[builder(default)]
    pub budget: f64,

    /// Channel-specific constraints as arbitrary JSON blob.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub constraints: Option<serde_json::Value>,

    /// Extension object for channel-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ChannelBrief {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ChannelBriefBuilder {
        ChannelBriefBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_minimal_channel_brief() {
        let brief = ChannelBrief::builder()
            .channel("display")
            .budget(10000.0)
            .build()
            .expect("Failed to build minimal ChannelBrief");

        assert_eq!(brief.channel, "display");
        assert_eq!(brief.budget, 10000.0);
        assert!(brief.objectives.is_empty());
        assert!(brief.constraints.is_none());
        assert!(brief.ext.is_none());
    }

    #[test]
    fn test_with_objectives() {
        let brief = ChannelBrief::builder()
            .channel("video")
            .budget(50000.0)
            .objectives(vec!["engagement".to_string(), "conversion".to_string()])
            .build()
            .expect("Failed to build ChannelBrief with objectives");

        assert_eq!(brief.channel, "video");
        assert_eq!(brief.budget, 50000.0);
        assert_eq!(brief.objectives.len(), 2);
        assert_eq!(brief.objectives[0], "engagement");
        assert_eq!(brief.objectives[1], "conversion");
    }

    #[test]
    fn test_with_constraints_json() {
        let constraints = json!({
            "max_impressions": 1000000,
            "frequency_cap": 3,
            "blacklist": ["competitor.com"]
        });

        let brief = ChannelBrief::builder()
            .channel("email")
            .budget(5000.0)
            .constraints(Some(constraints.clone()))
            .build()
            .expect("Failed to build ChannelBrief with constraints");

        assert_eq!(brief.channel, "email");
        assert_eq!(brief.budget, 5000.0);
        assert!(brief.constraints.is_some());
        let c = brief.constraints.unwrap();
        assert_eq!(c.get("max_impressions").unwrap(), 1000000);
        assert_eq!(c.get("frequency_cap").unwrap(), 3);
    }

    #[test]
    fn test_serialization() {
        let brief = ChannelBrief::builder()
            .channel("social")
            .budget(15000.0)
            .objectives(vec!["awareness".to_string()])
            .constraints(Some(json!({"platform": "facebook"})))
            .build()
            .expect("Failed to build ChannelBrief for serialization");

        let json_str = serde_json::to_string(&brief).expect("Failed to serialize ChannelBrief");

        let deserialized: ChannelBrief =
            serde_json::from_str(&json_str).expect("Failed to deserialize ChannelBrief");

        assert_eq!(brief, deserialized);
    }

    #[test]
    fn test_roundtrip() {
        let brief = ChannelBrief::builder()
            .channel("display")
            .budget(20000.0)
            .objectives(vec!["reach".to_string(), "frequency".to_string()])
            .constraints(Some(json!({
                "geo": "US",
                "device_types": ["mobile", "desktop"]
            })))
            .build()
            .expect("Failed to build ChannelBrief for roundtrip");

        // Serialize to JSON and back
        let json = serde_json::to_value(&brief).expect("Failed to serialize to value");
        let roundtripped: ChannelBrief =
            serde_json::from_value(json).expect("Failed to deserialize from value");

        assert_eq!(brief.channel, roundtripped.channel);
        assert_eq!(brief.budget, roundtripped.budget);
        assert_eq!(brief.objectives, roundtripped.objectives);
        assert_eq!(brief.constraints, roundtripped.constraints);
    }

    #[test]
    fn test_channel_brief_default_trait() {
        let brief: ChannelBrief = ChannelBrief::default();
        assert_eq!(brief.channel, "");
        assert_eq!(brief.budget, 0.0);
        assert!(brief.objectives.is_empty());
        assert!(brief.constraints.is_none());
        assert!(brief.ext.is_none());
    }

    #[test]
    fn test_channel_brief_zero_budget() {
        let brief = ChannelBrief::builder()
            .channel("test")
            .budget(0.0)
            .build()
            .unwrap();
        assert_eq!(brief.budget, 0.0);
    }

    #[test]
    fn test_channel_brief_deserialization_from_json_string() {
        let json_str = r#"{"channel":"audio","budget":5000.0,"objectives":["reach"]}"#;
        let brief: ChannelBrief = serde_json::from_str(json_str).unwrap();
        assert_eq!(brief.channel, "audio");
        assert_eq!(brief.budget, 5000.0);
        assert_eq!(brief.objectives, vec!["reach"]);
        assert!(brief.constraints.is_none());
    }

    #[test]
    fn test_channel_brief_with_json_extension() {
        let brief = ChannelBriefBuilder::<serde_json::Value>::default()
            .channel("ctv".to_string())
            .budget(20000.0)
            .ext(Some(Box::new(json!({"targeting": "household"}))))
            .build()
            .unwrap();

        assert!(brief.ext.is_some());
        assert_eq!(brief.ext.as_ref().unwrap()["targeting"], "household");

        let json_str = serde_json::to_string(&brief).unwrap();
        let parsed: ChannelBrief<serde_json::Value> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed.ext.as_ref().unwrap()["targeting"], "household");
    }
}
