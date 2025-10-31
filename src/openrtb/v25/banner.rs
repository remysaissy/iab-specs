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
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Format;
///
/// // Fixed size format
/// let format = Format {
///     w: Some(300),
///     h: Some(250),
///     ..Default::default()
/// };
///
/// // Flex Ad format with ratio
/// let flex_format = Format {
///     wratio: Some(16),
///     hratio: Some(9),
///     wmin: Some(300),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Format {
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
    pub ext: Option<serde_json::Value>,
}

/// Banner ad impression (OpenRTB 2.5 Section 3.2.6)
///
/// A `Banner` object represents a banner, expandable, or in-banner video impression.
/// It describes the ad creative dimensions, supported formats, and restrictions.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{Banner, Format};
///
/// let banner = Banner {
///     format: Some(vec![
///         Format { w: Some(300), h: Some(250), ..Default::default() },
///         Format { w: Some(728), h: Some(90), ..Default::default() },
///     ]),
///     w: Some(300),
///     h: Some(250),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Banner {
    /// Array of format objects representing the banner sizes permitted.
    /// If none are specified, the system will use the w and h attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<Vec<Format>>,

    /// Exact width in device-independent pixels (DIPS).
    /// Recommended if no format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Exact height in device-independent pixels (DIPS).
    /// Recommended if no format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Blocked banner ad types.
    /// Refer to AdCOM `BannerAdType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub btype: Option<Vec<i32>>,

    /// Blocked creative attributes.
    /// Refer to AdCOM `CreativeAttribute` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub battr: Option<Vec<i32>>,

    /// Ad position on screen.
    /// Refer to AdCOM `AdPosition` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pos: Option<i32>,

    /// Content MIME types supported (e.g., "image/jpeg", "image/gif").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mimes: Option<Vec<String>>,

    /// Indicates if the banner is in the top frame (1) or an iframe (0).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub topframe: Option<i32>,

    /// Directions in which the banner may expand.
    /// Refer to AdCOM `ExpandableDirection` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub expdir: Option<Vec<i32>>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM `ApiFramework` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub api: Option<Vec<i32>>,

    /// Unique identifier for this banner object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Relevant only for Banner objects used with a Video object in an array of companion ads.
    /// Indicates the companion banner rendering mode:
    /// - 0 = concurrent (rendered with video)
    /// - 1 = end-card (rendered at video completion)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub vcm: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_fixed_size() {
        let format = Format {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };

        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
    }

    #[test]
    fn test_format_flex_ad() {
        let format = Format {
            wratio: Some(16),
            hratio: Some(9),
            wmin: Some(300),
            ..Default::default()
        };

        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
        assert_eq!(format.wmin, Some(300));
    }

    #[test]
    fn test_banner_creation() {
        let banner = Banner {
            format: Some(vec![
                Format {
                    w: Some(300),
                    h: Some(250),
                    ..Default::default()
                },
                Format {
                    w: Some(728),
                    h: Some(90),
                    ..Default::default()
                },
            ]),
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };

        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.format.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_banner_serialization() {
        let banner = Banner {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        };

        let json = serde_json::to_string(&banner).unwrap();
        assert!(json.contains("\"w\":300"));
        assert!(json.contains("\"h\":250"));
    }

    #[test]
    fn test_banner_deserialization() {
        let json = r#"{"w":300,"h":250}"#;
        let banner: Banner = serde_json::from_str(json).unwrap();

        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
    }

    #[test]
    fn test_banner_builder() {
        let banner = BannerBuilder::default()
            .w(Some(728))
            .h(Some(90))
            .pos(Some(1))
            .build()
            .unwrap();

        assert_eq!(banner.w, Some(728));
        assert_eq!(banner.h, Some(90));
        assert_eq!(banner.pos, Some(1));
    }

    #[test]
    fn test_banner_with_blocked_types_and_attrs() {
        let banner = Banner {
            w: Some(300),
            h: Some(250),
            btype: Some(vec![1, 4]),
            battr: Some(vec![1, 2, 3]),
            ..Default::default()
        };

        assert_eq!(banner.btype.as_ref().unwrap().len(), 2);
        assert_eq!(banner.battr.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_banner_with_api_frameworks() {
        let banner = Banner {
            w: Some(300),
            h: Some(250),
            api: Some(vec![3, 5, 6]),
            ..Default::default()
        };

        assert_eq!(banner.api.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_format_builder() {
        let format = FormatBuilder::default()
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
    }

    #[test]
    fn test_banner_with_mimes() {
        let banner = Banner {
            w: Some(300),
            h: Some(250),
            mimes: Some(vec!["image/jpeg".to_string(), "image/png".to_string()]),
            ..Default::default()
        };

        assert_eq!(banner.mimes.as_ref().unwrap().len(), 2);
        assert!(
            banner
                .mimes
                .as_ref()
                .unwrap()
                .contains(&"image/jpeg".to_string())
        );
    }
}
