use super::Format;
use crate::Extension;

/// OpenRTB 2.5 Banner Ad Objects
///
/// This module implements Banner and Format objects for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Banner ad impression (OpenRTB 2.5 Section 3.2.6)
///
/// A `Banner` object represents a banner, expandable, or in-banner video impression.
/// It describes the ad creative dimensions, supported formats, and restrictions.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{Banner, Format};
///
/// let banner = Banner::builder()
///     .format(Some(vec![
///         Format::builder().w(Some(300)).h(Some(250)).build().unwrap(),
///         Format::builder().w(Some(728)).h(Some(90)).build().unwrap(),
///     ]))
///     .w(Some(300))
///     .h(Some(250))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Banner<Ext: Extension = serde_json::Value> {
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
    pub ext: Option<Box<Ext>>,
}

impl Banner {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BannerBuilder {
        BannerBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_creation() {
        let banner = Banner::builder()
            .format(Some(vec![
                Format::builder().w(Some(300)).h(Some(250)).build().unwrap(),
                Format::builder().w(Some(728)).h(Some(90)).build().unwrap(),
            ]))
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(banner.w, Some(300));
        assert_eq!(banner.h, Some(250));
        assert_eq!(banner.format.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_banner_serialization() {
        let banner = Banner::builder().w(Some(300)).h(Some(250)).build().unwrap();

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
    fn test_banner_with_blocked_types_and_attrs() {
        let banner = Banner::builder()
            .w(Some(300))
            .h(Some(250))
            .btype(Some(vec![1, 4]))
            .battr(Some(vec![1, 2, 3]))
            .build()
            .unwrap();

        assert_eq!(banner.btype.as_ref().unwrap().len(), 2);
        assert_eq!(banner.battr.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_banner_with_api_frameworks() {
        let banner = Banner::builder()
            .w(Some(300))
            .h(Some(250))
            .api(Some(vec![3, 5, 6]))
            .build()
            .unwrap();

        assert_eq!(banner.api.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_banner_with_mimes() {
        let banner = Banner::builder()
            .w(Some(300))
            .h(Some(250))
            .mimes(Some(vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(banner.mimes.as_ref().unwrap().len(), 2);
        assert!(
            banner
                .mimes
                .as_ref()
                .unwrap()
                .contains(&"image/jpeg".to_string())
        );
    }

    // === Phase 2.1: Integer-as-Enum Field Validation Tests ===

    #[test]
    fn test_btype_with_valid_banner_ad_type_values() {
        // BannerAdType enum valid values are 1-4
        // 1=XhtmlTextAd, 2=XhtmlBannerAd, 3=JavaScriptAd, 4=IFrame
        let banner = Banner::builder()
            .btype(Some(vec![1, 2, 3, 4]))
            .build()
            .unwrap();

        assert_eq!(banner.btype.as_ref().unwrap(), &vec![1, 2, 3, 4]);

        // Verify serialization roundtrip
        let json = serde_json::to_string(&banner).unwrap();
        let deserialized: Banner = serde_json::from_str(&json).unwrap();
        assert_eq!(banner, deserialized);
    }

    #[test]
    fn test_btype_with_invalid_values() {
        // Test that invalid BannerAdType values (e.g., 99) currently pass
        let json = r#"{"btype":[99]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        // Currently no validation - invalid values pass
        assert!(result.is_ok(), "Invalid btype value 99 currently passes");

        let banner = result.unwrap();
        assert_eq!(banner.btype, Some(vec![99]));
        // TODO: Consider adding validation to reject invalid BannerAdType values
    }

    #[test]
    fn test_btype_with_mixed_valid_invalid_values() {
        // Test mix of valid and invalid values
        let json = r#"{"btype":[1, 99, 3, 200]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Mixed valid/invalid btype values currently pass"
        );
        let banner = result.unwrap();
        assert_eq!(banner.btype, Some(vec![1, 99, 3, 200]));
        // TODO: Should mixed valid/invalid values be rejected?
    }

    #[test]
    fn test_battr_with_valid_creative_attribute_values() {
        // CreativeAttribute enum valid values are 1-17
        let valid_values: Vec<i32> = (1..=17).collect();
        let banner = Banner::builder()
            .battr(Some(valid_values.clone()))
            .build()
            .unwrap();

        assert_eq!(banner.battr.as_ref().unwrap(), &valid_values);
    }

    #[test]
    fn test_battr_with_invalid_values() {
        // Test invalid CreativeAttribute values
        let json = r#"{"battr":[99, 200]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Invalid battr values currently pass");
        assert_eq!(result.unwrap().battr, Some(vec![99, 200]));
        // TODO: Consider validation for CreativeAttribute range (1-17)
    }

    #[test]
    fn test_pos_with_valid_ad_position_values() {
        // AdPosition enum valid values are 0-7
        // 0=Unknown, 1=AboveTheFold, 2=MayNotBeVisible, 3=BelowTheFold,
        // 4=Header, 5=Footer, 6=Sidebar, 7=FullScreen
        for pos_value in 0..=7 {
            let banner = Banner::builder().pos(Some(pos_value)).build().unwrap();

            assert_eq!(banner.pos, Some(pos_value));

            // Verify serialization roundtrip
            let json = serde_json::to_string(&banner).unwrap();
            let deserialized: Banner = serde_json::from_str(&json).unwrap();
            assert_eq!(banner.pos, deserialized.pos);
        }
    }

    #[test]
    fn test_pos_with_invalid_value() {
        // Test invalid AdPosition value
        let json = r#"{"pos":99}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Invalid pos value 99 currently passes");
        assert_eq!(result.unwrap().pos, Some(99));
        // TODO: AdPosition should be validated (valid range: 0-7)
    }

    #[test]
    fn test_expdir_with_valid_expandable_direction_values() {
        // ExpandableDirection enum valid values are 1-5
        // 1=Left, 2=Right, 3=Up, 4=Down, 5=FullScreen
        let banner = Banner::builder()
            .expdir(Some(vec![1, 2, 3, 4, 5]))
            .build()
            .unwrap();

        assert_eq!(banner.expdir, Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_expdir_with_invalid_values() {
        // Test invalid ExpandableDirection values
        let json = r#"{"expdir":[0, 99]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Invalid expdir values currently pass");
        assert_eq!(result.unwrap().expdir, Some(vec![0, 99]));
        // TODO: ExpandableDirection should be validated (valid range: 1-5)
    }

    #[test]
    fn test_api_with_valid_api_framework_values() {
        // ApiFramework enum valid values are 1-9
        // 1=Vpaid1, 2=Vpaid2, 3=Mraid1, 4=Ormma, 5=Mraid2,
        // 6=Mraid3, 7=Omid1, 8=Simid1, 9=Simid1_1
        let banner = Banner::builder()
            .api(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]))
            .build()
            .unwrap();

        assert_eq!(banner.api, Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn test_api_with_invalid_values() {
        // Test invalid ApiFramework values
        let json = r#"{"api":[0, 10, 99]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Invalid api values currently pass");
        assert_eq!(result.unwrap().api, Some(vec![0, 10, 99]));
        // TODO: ApiFramework should be validated (valid range: 1-9)
    }

    #[test]
    fn test_zero_in_integer_enum_fields() {
        // Test that zero values (often invalid for enums starting at 1) currently pass
        let json = r#"{"btype":[0],"expdir":[0],"api":[0]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Zero values in enum fields currently pass");
        let banner = result.unwrap();
        assert_eq!(banner.btype, Some(vec![0]));
        assert_eq!(banner.expdir, Some(vec![0]));
        assert_eq!(banner.api, Some(vec![0]));
        // Document: Zero is invalid for BannerAdType, ExpandableDirection, and ApiFramework
    }

    #[test]
    fn test_negative_values_in_integer_enum_fields() {
        // Test that negative values currently pass
        let json = r#"{"btype":[-1],"pos":-1,"api":[-5]}"#;
        let result: Result<Banner, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Negative values in enum fields currently pass"
        );
        let banner = result.unwrap();
        assert_eq!(banner.btype, Some(vec![-1]));
        assert_eq!(banner.pos, Some(-1));
        assert_eq!(banner.api, Some(vec![-5]));
        // Document: Negative values are invalid for all these enum types
    }
}
