use crate::Extension;
use crate::adcom::placement::{AudioPlacement, DisplayPlacement, VideoPlacement};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Placement Object (Section 4.1)
///
/// Abstract base for placement specifications defining ad slot characteristics.
/// Subtype objects include DisplayPlacement, VideoPlacement, and AudioPlacement.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Placement<Ext: Extension = serde_json::Value> {
    /// Placement identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Placement name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Placement description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// HTTPS only flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,

    /// Array of blocked advertiser categories using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,

    /// Taxonomy used for bcat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Array of blocked advertiser domains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddr: Option<Vec<String>>,

    /// Array of blocked creative attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    /// Workflow language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<String>>,

    /// Display placement details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<DisplayPlacement>>,

    /// Video placement details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<VideoPlacement>>,

    /// Audio placement details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<AudioPlacement>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn test_placement_builder() {
        let placement = Placement::builder()
            .id(Some("placement123".to_string()))
            .name(Some("Homepage Banner".to_string()))
            .desc(Some("Main banner placement".to_string()))
            .secure(Some(1))
            .build()
            .unwrap();

        assert_eq!(placement.id, Some("placement123".to_string()));
        assert_eq!(placement.name, Some("Homepage Banner".to_string()));
        assert_eq!(placement.desc, Some("Main banner placement".to_string()));
        assert_eq!(placement.secure, Some(1));
    }

    #[test]
    fn test_placement_default() {
        let placement = Placement::builder().build().unwrap();

        assert!(placement.id.is_none());
        assert!(placement.name.is_none());
        assert!(placement.desc.is_none());
        assert!(placement.secure.is_none());
        assert!(placement.display.is_none());
        assert!(placement.video.is_none());
        assert!(placement.audio.is_none());
    }

    #[test]
    fn test_placement_with_blocking_rules() {
        let placement = Placement::builder()
            .id(Some("p1".to_string()))
            .bcat(Some(vec!["IAB25".to_string(), "IAB26".to_string()]))
            .baddr(Some(vec!["advertiser.com".to_string()]))
            .battr(Some(vec![1, 2, 3]))
            .build()
            .unwrap();

        assert_eq!(
            placement.bcat,
            Some(vec!["IAB25".to_string(), "IAB26".to_string()])
        );
        assert_eq!(placement.baddr, Some(vec!["advertiser.com".to_string()]));
        assert_eq!(placement.battr, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_placement_with_display() {
        let display = DisplayPlacement::builder().pos(Some(1)).build().unwrap();

        let placement = Placement::builder()
            .id(Some("p2".to_string()))
            .display(Some(Box::new(display)))
            .build()
            .unwrap();

        assert!(placement.display.is_some());
        assert_eq!(placement.display.as_ref().unwrap().pos, Some(1));
    }

    #[test]
    fn test_placement_with_video() {
        let video = VideoPlacement::builder().ptype(Some(1)).build().unwrap();

        let placement = Placement::builder()
            .id(Some("p3".to_string()))
            .video(Some(Box::new(video)))
            .build()
            .unwrap();

        assert!(placement.video.is_some());
        assert_eq!(placement.video.as_ref().unwrap().ptype, Some(1));
    }

    #[test]
    fn test_placement_with_languages() {
        let placement = Placement::builder()
            .wlang(Some(vec![
                "en".to_string(),
                "es".to_string(),
                "fr".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(
            placement.wlang,
            Some(vec!["en".to_string(), "es".to_string(), "fr".to_string()])
        );
    }

    #[test]
    fn test_placement_serialization() {
        let placement = Placement::builder()
            .id(Some("p4".to_string()))
            .secure(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&placement).unwrap();
        assert!(json.contains("\"id\":\"p4\""));
        assert!(json.contains("\"secure\":1"));
    }

    #[test]
    fn test_placement_deserialization() {
        let json = r#"{"id":"p5","name":"Test Placement","secure":1}"#;
        let placement: Placement = serde_json::from_str(json).unwrap();

        assert_eq!(placement.id, Some("p5".to_string()));
        assert_eq!(placement.name, Some("Test Placement".to_string()));
        assert_eq!(placement.secure, Some(1));
    }
}
