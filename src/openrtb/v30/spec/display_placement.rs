use super::DisplayFormat;
use crate::Extension;
/// OpenRTB 3.0 Display Placement Specification
///
/// This module implements the DisplayPlacement object for display ad placements.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DisplayPlacement specification (AdCOM 1.0 Section 6.4)
///
/// The `DisplayPlacement` object describes a display ad placement including
/// dimensions, formats, and rendering capabilities.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `EventExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `NativeFmtExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, EventExt: Extension, NativeFmtExt: Extension",
    deserialize = "Ext: Extension, EventExt: Extension, NativeFmtExt: Extension"
))]
pub struct DisplayPlacement<
    Ext: Extension = serde_json::Value,
    EventExt: Extension = serde_json::Value,
    NativeFmtExt: Extension = serde_json::Value,
> {
    /// Placement position on screen:
    /// - 0 = Unknown
    /// - 1 = Above the fold
    /// - 2 = Below the fold
    /// - 3 = Header
    /// - 4 = Footer
    /// - 5 = Sidebar
    /// - 6 = Full screen
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pos: Option<i32>,

    /// Indicator for interstitial or full-screen placement:
    /// - 0 = no (default)
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub instl: Option<i32>,

    /// Indicator that the placement is in the top frame as opposed to an iframe:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub topframe: Option<i32>,

    /// Placement width in units specified by `unit`.
    /// Recommended for display placements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Placement height in units specified by `unit`.
    /// Recommended for display placements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Unit of measurement for `w` and `h`:
    /// - 1 = pixels (default)
    /// - 2 = percentage (viewport)
    /// - 3 = device independent pixels (DIPS)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub unit: Option<i32>,

    /// Indicator that the placement is private:
    /// - 0 = no (default)
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[serde(rename = "priv")]
    pub priv_: Option<i32>,

    /// Array of supported display formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub displayfmt: Option<Vec<DisplayFormat>>,

    /// Array of supported native ad format objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nativefmt: Option<Box<NativeFmtExt>>,

    /// Array of supported event tracking objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub event: Option<Vec<Box<EventExt>>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
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
    fn test_display_placement_creation() {
        let display = DisplayPlacement::builder()
            .pos(Some(1))
            .instl(Some(0))
            .topframe(Some(1))
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(display.pos, Some(1));
        assert_eq!(display.w, Some(300));
        assert_eq!(display.h, Some(250));
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
            .pos(Some(1))
            .displayfmt(Some(vec![format1, format2]))
            .build()
            .unwrap();

        assert_eq!(display.displayfmt.as_ref().unwrap().len(), 2);
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
    }

    #[test]
    fn test_display_placement_deserialization() {
        let json = r#"{
            "pos": 1,
            "instl": 0,
            "w": 300,
            "h": 250
        }"#;

        let display: DisplayPlacement = serde_json::from_str(json).unwrap();
        assert_eq!(display.pos, Some(1));
        assert_eq!(display.w, Some(300));
    }
}
