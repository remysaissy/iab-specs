use crate::Extension;
/// OpenRTB 2.5 Banner Ad Objects
///
/// This module implements Banner and Format objects for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Format object representing an allowed banner size (OpenRTB 2.5 Section 3.2.10)
///
/// A `Format` object represents either an allowed size (width/height) or Flex Ad
/// parameters (ratio-based sizing) for banner impressions.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Format;
///
/// // Fixed size format
/// let format = Format::builder()
///     .w(Some(300))
///     .h(Some(250))
///     .build()
///     .unwrap();
///
/// // Flex Ad format with ratio
/// let flex_format = Format::builder()
///     .wratio(Some(16))
///     .hratio(Some(9))
///     .wmin(Some(300))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Format<Ext: Extension = serde_json::Value> {
    /// Width in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Height in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Relative width when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wratio: Option<i32>,

    /// Relative height when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hratio: Option<i32>,

    /// Minimum width in device-independent pixels (DIPS) at which the ad will be
    /// displayed when size is expressed as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wmin: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Format {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> FormatBuilder {
        FormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_fixed_size() {
        let format = Format::builder().w(Some(300)).h(Some(250)).build().unwrap();

        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
    }

    #[test]
    fn test_format_flex_ad() {
        let format = Format::builder()
            .wratio(Some(16))
            .hratio(Some(9))
            .wmin(Some(300))
            .build()
            .unwrap();

        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
        assert_eq!(format.wmin, Some(300));
    }
}
