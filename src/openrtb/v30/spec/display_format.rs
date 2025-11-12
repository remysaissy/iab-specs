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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DisplayFormat<Ext: Extension = serde_json::Value> {
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
}
