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
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::spec::{DisplayPlacement, DisplayFormat};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let display = DisplayPlacement {
///     pos: Some(1), // Above the fold
///     instl: Some(0), // Not an interstitial
///     topframe: Some(1), // In the top frame
///     w: Some(300),
///     h: Some(250),
///     unit: Some(1), // Pixels
///     priv_: Some(0), // Not private
///     displayfmt: Some(vec![DisplayFormat {
///         w: Some(300),
///         h: Some(250),
///         ..Default::default()
///     }]),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct DisplayPlacement {
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
    pub nativefmt: Option<serde_json::Value>,

    /// Array of supported event tracking objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub event: Option<Vec<serde_json::Value>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

/// DisplayFormat object for supported display creative formats
///
/// Specifies a supported display creative format with dimensions and other properties.
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::spec::DisplayFormat;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let format = DisplayFormat {
///     w: Some(300),
///     h: Some(250),
///     wratio: Some(6),
///     hratio: Some(5),
///     expdir: Some(vec![1, 2, 3]),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct DisplayFormat {
    /// Width in units specified by the parent DisplayPlacement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Height in units specified by the parent DisplayPlacement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Relative width for flexible ads (aspect ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wratio: Option<i32>,

    /// Relative height for flexible ads (aspect ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hratio: Option<i32>,

    /// Directions in which the creative may expand:
    /// - 1 = Left
    /// - 2 = Right
    /// - 3 = Up
    /// - 4 = Down
    /// - 5 = Full screen
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub expdir: Option<Vec<i32>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_placement_creation() {
        let display = DisplayPlacement {
            pos: Some(1),
            instl: Some(0),
            topframe: Some(1),
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };

        assert_eq!(display.pos, Some(1));
        assert_eq!(display.w, Some(300));
        assert_eq!(display.h, Some(250));
    }

    #[test]
    fn test_display_placement_with_formats() {
        let display = DisplayPlacement {
            pos: Some(1),
            displayfmt: Some(vec![
                DisplayFormat {
                    w: Some(300),
                    h: Some(250),
                    ..Default::default()
                },
                DisplayFormat {
                    w: Some(728),
                    h: Some(90),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        assert_eq!(display.displayfmt.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_display_format_with_aspect_ratio() {
        let format = DisplayFormat {
            wratio: Some(16),
            hratio: Some(9),
            ..Default::default()
        };

        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
    }

    #[test]
    fn test_display_format_expandable() {
        let format = DisplayFormat {
            w: Some(300),
            h: Some(250),
            expdir: Some(vec![1, 2, 3, 4]), // All directions
            ..Default::default()
        };

        assert_eq!(format.expdir.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_display_placement_interstitial() {
        let display = DisplayPlacement {
            instl: Some(1),
            w: Some(320),
            h: Some(480),
            ..Default::default()
        };

        assert_eq!(display.instl, Some(1));
    }

    #[test]
    fn test_display_placement_serialization() {
        let display = DisplayPlacement {
            pos: Some(1),
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };

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

    #[test]
    fn test_display_placement_builder() {
        let display = DisplayPlacementBuilder::default()
            .pos(Some(1))
            .w(Some(728))
            .h(Some(90))
            .build()
            .unwrap();

        assert_eq!(display.pos, Some(1));
        assert_eq!(display.w, Some(728));
    }
}
