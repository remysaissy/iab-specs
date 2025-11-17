use crate::Extension;
use crate::adcom::placement::{DisplayFormat, EventSpec, NativeFormat};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DisplayPlacement Object (Section 4.2)
///
/// Placement details for display ad formats including banner and native.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DisplayPlacement<Ext: Extension = serde_json::Value> {
    /// Ad position on screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    /// Interstitial flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instl: Option<i32>,

    /// Top frame flag (1=iframe, 0=top frame)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topframe: Option<i32>,

    /// Array of iframe busters supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifrbust: Option<Vec<String>>,

    /// Click type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clktype: Option<i32>,

    /// AMPHTML creative support flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ampren: Option<i32>,

    /// Display placement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptype: Option<i32>,

    /// Display context type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,

    /// MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtypes permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<Vec<i32>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Array of display format specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayfmt: Option<Vec<DisplayFormat>>,

    /// Native format specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nativefmt: Option<Box<NativeFormat>>,

    /// Event tracking specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<EventSpec>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DisplayPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayPlacementBuilder {
        DisplayPlacementBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_placement_builder() {
        let display = DisplayPlacement::builder()
            .pos(Some(1))
            .w(Some(300))
            .h(Some(250))
            .topframe(Some(0))
            .build()
            .unwrap();

        assert_eq!(display.pos, Some(1));
        assert_eq!(display.w, Some(300));
        assert_eq!(display.h, Some(250));
        assert_eq!(display.topframe, Some(0));
    }

    #[test]
    fn test_display_placement_default() {
        let display = DisplayPlacement::builder().build().unwrap();

        assert!(display.pos.is_none());
        assert!(display.w.is_none());
        assert!(display.h.is_none());
        assert!(display.instl.is_none());
        assert!(display.displayfmt.is_none());
        assert!(display.nativefmt.is_none());
    }

    #[test]
    fn test_display_placement_interstitial() {
        let display = DisplayPlacement::builder()
            .instl(Some(1))
            .w(Some(320))
            .h(Some(480))
            .build()
            .unwrap();

        assert_eq!(display.instl, Some(1));
        assert_eq!(display.w, Some(320));
        assert_eq!(display.h, Some(480));
    }

    #[test]
    fn test_display_placement_with_mime_and_api() {
        let display = DisplayPlacement::builder()
            .mime(Some(vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
            ]))
            .api(Some(vec![1, 2, 5]))
            .ctype(Some(vec![1, 2]))
            .build()
            .unwrap();

        assert_eq!(
            display.mime,
            Some(vec!["image/jpeg".to_string(), "image/png".to_string()])
        );
        assert_eq!(display.api, Some(vec![1, 2, 5]));
        assert_eq!(display.ctype, Some(vec![1, 2]));
    }

    #[test]
    fn test_display_placement_with_amp() {
        let display = DisplayPlacement::builder().ampren(Some(1)).build().unwrap();

        assert_eq!(display.ampren, Some(1));
    }

    #[test]
    fn test_display_placement_with_formats() {
        let format1 = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        let format2 = DisplayFormat::builder()
            .w(Some(728))
            .h(Some(90))
            .build()
            .unwrap();

        let display = DisplayPlacement::builder()
            .displayfmt(Some(vec![format1, format2]))
            .build()
            .unwrap();

        assert!(display.displayfmt.is_some());
        assert_eq!(display.displayfmt.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_display_placement_serialization() {
        let display = DisplayPlacement::builder()
            .pos(Some(1))
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        let json = serde_json::to_string(&display).unwrap();
        assert!(json.contains("\"pos\":1"));
        assert!(json.contains("\"w\":300"));
        assert!(json.contains("\"h\":250"));
    }

    #[test]
    fn test_display_placement_deserialization() {
        let json = r#"{"pos":1,"w":300,"h":250,"instl":0}"#;
        let display: DisplayPlacement = serde_json::from_str(json).unwrap();

        assert_eq!(display.pos, Some(1));
        assert_eq!(display.w, Some(300));
        assert_eq!(display.h, Some(250));
        assert_eq!(display.instl, Some(0));
    }
}
