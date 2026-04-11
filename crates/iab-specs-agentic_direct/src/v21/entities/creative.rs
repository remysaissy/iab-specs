use super::super::enums::CreativeStatus;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Creative entity.
///
/// Represents an advertising creative with format, dimensions, and creative content information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Creative<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Creative name (required).
    #[builder(setter(into))]
    pub name: String,

    /// Account identifier (required).
    #[builder(setter(into))]
    pub account_id: String,

    /// Current status of the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub status: Option<CreativeStatus>,

    /// Format of the creative (e.g., "display", "video", "native").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ad_format: Option<String>,

    /// Click-through URL for the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub click_url: Option<String>,

    /// HTML markup content of the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub markup: Option<String>,

    /// Width of the creative in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<i32>,

    /// Height of the creative in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<i32>,

    /// Duration of the creative in seconds (for video).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub duration: Option<i32>,

    /// MIME type of the creative content.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub mime_type: Option<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Creative {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CreativeBuilder {
        CreativeBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creative_creation() {
        let creative = Creative::builder()
            .name("Display Banner")
            .account_id("acc-123")
            .build()
            .unwrap();

        assert_eq!(creative.name, "Display Banner");
        assert_eq!(creative.account_id, "acc-123");
        assert!(creative.id.is_none());
        assert!(creative.status.is_none());
        assert!(creative.markup.is_none());
    }

    #[test]
    fn test_creative_serialization() {
        let creative = Creative::builder()
            .name("Video Creative")
            .account_id("acc-456")
            .ad_format("video")
            .width(Some(1920))
            .height(Some(1080))
            .duration(Some(30))
            .mime_type("video/mp4")
            .build()
            .unwrap();

        let json = serde_json::to_string(&creative).unwrap();
        assert!(json.contains("\"name\":\"Video Creative\""));
        assert!(json.contains("\"account_id\":\"acc-456\""));
        assert!(json.contains("\"ad_format\":\"video\""));
        assert!(json.contains("\"width\":1920"));
        assert!(json.contains("\"height\":1080"));
        assert!(json.contains("\"duration\":30"));
        assert!(json.contains("\"mime_type\":\"video/mp4\""));
    }

    #[test]
    fn test_creative_deserialization() {
        let json = r#"{"name":"Native Creative","account_id":"acc-789","ad_format":"native","click_url":"https://example.com"}"#;
        let creative: Creative = serde_json::from_str(json).unwrap();

        assert_eq!(creative.name, "Native Creative");
        assert_eq!(creative.account_id, "acc-789");
        assert_eq!(creative.ad_format, Some("native".to_string()));
        assert_eq!(creative.click_url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_creative_roundtrip() {
        let creative = Creative::builder()
            .id("cre-999")
            .name("Roundtrip Creative")
            .account_id("acc-111")
            .ad_format("display")
            .click_url("https://example.com/click")
            .width(Some(300))
            .height(Some(250))
            .build()
            .unwrap();

        let json = serde_json::to_string(&creative).unwrap();
        let parsed: Creative = serde_json::from_str(&json).unwrap();
        assert_eq!(creative, parsed);
    }

    #[test]
    fn test_creative_with_html_markup() {
        let html_markup = "<div><h1>Special Offer</h1><p>50% Off Today!</p></div>";

        let creative = Creative::builder()
            .name("HTML Creative")
            .account_id("acc-222")
            .ad_format("display")
            .markup(html_markup)
            .width(Some(300))
            .height(Some(250))
            .build()
            .unwrap();

        assert_eq!(creative.markup, Some(html_markup.to_string()));

        let json = serde_json::to_string(&creative).unwrap();
        let parsed: Creative = serde_json::from_str(&json).unwrap();
        assert_eq!(creative.markup, parsed.markup);
    }
}
