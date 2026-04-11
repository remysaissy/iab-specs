use crate::Extension;
/// OpenRTB 3.0 Display Placement Specification
///
/// This module implements the DisplayPlacement object for display ad placements.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DisplayFormat object for supported display creative formats
///
/// Specifies a supported display creative format with dimensions and other properties.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DisplayFormat<Ext: Extension = crate::DefaultExt> {
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
    pub ext: Option<Box<Ext>>,
}

impl DisplayFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayFormatBuilder {
        DisplayFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec: Object: DisplayFormat — verifies wratio and hratio fields for flexible aspect ratio ads
    #[test]
    fn test_display_format_with_aspect_ratio() {
        let format = DisplayFormat::builder()
            .wratio(Some(16))
            .hratio(Some(9))
            .build()
            .unwrap();

        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
    }

    // Spec: Object: DisplayFormat — verifies expdir field for expandable ad directions
    #[test]
    fn test_display_format_expandable() {
        let format = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .expdir(Some(vec![1, 2, 3, 4])) // All directions
            .build()
            .unwrap();

        assert_eq!(format.expdir.as_ref().unwrap().len(), 4);
    }

    // Spec: Object: DisplayFormat — verifies default() produces all None fields
    #[test]
    fn test_display_format_default() {
        let format: DisplayFormat = DisplayFormat::default();
        assert_eq!(format.w, None);
        assert_eq!(format.h, None);
        assert_eq!(format.wratio, None);
        assert_eq!(format.hratio, None);
        assert_eq!(format.expdir, None);
        assert!(format.ext.is_none());
    }

    // Spec: Object: DisplayFormat — verifies fixed dimensions w=300, h=250
    #[test]
    fn test_display_format_fixed_dimensions() {
        let format = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
    }

    // Spec: Object: DisplayFormat — verifies serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_display_format_roundtrip() {
        let format = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .wratio(Some(16))
            .hratio(Some(9))
            .expdir(Some(vec![1, 2]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&format).unwrap();
        let deserialized: DisplayFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(format, deserialized);
    }

    // Spec: Object: DisplayFormat — verifies JSON serialization includes dimension field values
    #[test]
    fn test_display_format_serialization() {
        let format = DisplayFormat::builder()
            .w(Some(728))
            .h(Some(90))
            .build()
            .unwrap();

        let json = serde_json::to_string(&format).unwrap();
        assert!(json.contains("\"w\":728"));
        assert!(json.contains("\"h\":90"));
    }

    // Spec: Object: DisplayFormat — verifies JSON deserialization from a JSON string
    #[test]
    fn test_display_format_deserialization() {
        let json = r#"{"w":300,"h":250,"wratio":4,"hratio":3}"#;

        let format: DisplayFormat = serde_json::from_str(json).unwrap();
        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
        assert_eq!(format.wratio, Some(4));
        assert_eq!(format.hratio, Some(3));
    }

    // Spec: Object: DisplayFormat — verifies empty format serializes to empty JSON object
    #[test]
    fn test_display_format_optional_fields_not_in_json() {
        let format: DisplayFormat = DisplayFormat::default();
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "{}");
    }
}
