//! OpenRTB Native 1.2 Request Objects
//!
//! This module implements request-side objects for the OpenRTB Native Ads 1.2 specification.

use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// OpenRTB Native 1.2 Request
///
/// Root object for native ad request specification conforming to the
/// Dynamic Native Ads API specification.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::{NativeRequest, Asset, Title};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let request = NativeRequest::builder()
///     .ver("1.2")
///     .context(Some(1))  // Content-centric
///     .plcmttype(Some(1)) // In-feed
///     .assets(vec![
///         Asset::builder()
///             .id(1)
///             .required(Some(1))
///             .title(Some(Title::builder()
///                 .len(90)
///                 .build()?))
///             .build()?
///     ])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NativeRequest<Ext: Extension = serde_json::Value> {
    /// Version of the Native Markup. Highly recommended.
    /// Default: "1.2"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ver: Option<String>,

    /// Context type. Recommended.
    /// Refer to `DisplayContextType` enumeration:
    /// - 1 = Content-centric (e.g., newsfeed, article)
    /// - 2 = Social-centric (e.g., social network feed)
    /// - 3 = Product context (e.g., product details, reviews)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub context: Option<i32>,

    /// Context subtype. Optional finer-grained context classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub contextsubtype: Option<i32>,

    /// Placement type. Recommended.
    /// Refer to `DisplayPlacementType` enumeration:
    /// - 1 = In-feed (e.g., newsfeed, content stream)
    /// - 2 = Sidebar
    /// - 3 = Interstitial/Overlay
    /// - 4 = Floating
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub plcmttype: Option<i32>,

    /// Placement count. Number of identical placements in the feed/stream.
    /// Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub plcmtcnt: Option<i32>,

    /// Sequence number. 0 for first ad, 1+ for subsequent ads in feed.
    /// Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seq: Option<i32>,

    /// Array of asset objects representing the native ad elements.
    /// **Required field**
    #[builder(default, setter(into))]
    pub assets: Vec<Asset<Ext>>,

    /// Asset URL support. 0=no, 1=yes.
    /// Indicates support for returning asset objects via URL rather than inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aurlsupport: Option<i32>,

    /// DCO URL support. 0=no, 1=yes.
    /// Beta feature for dynamic creative optimization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub durlsupport: Option<i32>,

    /// Event trackers. Array of event tracker objects.
    /// Preferred method for tracking over deprecated imptrackers/jstracker.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub eventtrackers: Option<Vec<EventTracker<Ext>>>,

    /// Privacy/AdChoices support flag. 0=no, 1=yes. Recommended.
    /// Indicates whether to provide link to privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub privacy: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl NativeRequest {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NativeRequestBuilder {
        NativeRequestBuilder::create_empty()
    }
}

/// Native Asset Request
///
/// Represents a single asset in the native ad request.
/// Each asset must contain exactly one of: title, img, video, or data.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::{Asset, Image};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let asset = Asset::builder()
///     .id(2)
///     .required(Some(1))
///     .img(Some(Image::builder()
///         .type_(Some(3)) // Main image
///         .w(Some(1200))
///         .h(Some(627))
///         .build()?))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Asset<Ext: Extension = serde_json::Value> {
    /// Unique asset ID within the request.
    /// Used to map response assets to request assets.
    /// **Required field**
    #[builder(setter(into))]
    pub id: i32,

    /// Asset required flag. 0=optional, 1=required.
    /// If required=1 and asset is not provided in response, bid may be rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub required: Option<i32>,

    /// Title object. Mutually exclusive with img, video, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<Title<Ext>>,

    /// Image object. Mutually exclusive with title, video, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub img: Option<Image<Ext>>,

    /// Video object. Mutually exclusive with title, img, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub video: Option<Video<Ext>>,

    /// Data object. Mutually exclusive with title, img, video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<Data<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Asset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssetBuilder {
        AssetBuilder::create_empty()
    }
}

/// Title Asset Request
///
/// Specifies a title text asset for the native ad.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::Title;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let title = Title::builder()
///     .len(90)  // Maximum 90 characters
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Title<Ext: Extension = serde_json::Value> {
    /// Maximum length of the title text in characters.
    /// Recommended lengths: 25, 90, or 140 characters.
    /// **Required field**
    #[builder(setter(into))]
    pub len: i32,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Title {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TitleBuilder {
        TitleBuilder::create_empty()
    }
}

/// Image Asset Request
///
/// Specifies an image asset for the native ad.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::Image;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let image = Image::builder()
///     .type_(Some(3))  // Main image
///     .w(Some(1200))
///     .h(Some(627))
///     .wmin(Some(600))
///     .hmin(Some(314))
///     .wmax(Some(1920))
///     .hmax(Some(1080))
///     .mimes(Some(vec!["image/jpeg".to_string(), "image/png".to_string()]))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Image<Ext: Extension = serde_json::Value> {
    /// Type of image asset.
    /// Refer to `NativeImageAssetType` enumeration:
    /// - 1 = Icon (typically small, square)
    /// - 2 = Logo
    /// - 3 = Main (large creative image)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    #[builder(default)]
    pub type_: Option<i32>,

    /// Width of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Height of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Minimum width of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wmin: Option<i32>,

    /// Minimum height of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hmin: Option<i32>,

    /// Maximum width of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wmax: Option<i32>,

    /// Maximum height of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hmax: Option<i32>,

    /// Whitelist of content MIME types supported.
    /// Common types: "image/jpeg", "image/png", "image/gif"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mimes: Option<Vec<String>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Image {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImageBuilder {
        ImageBuilder::create_empty()
    }
}

/// Video Asset Request
///
/// Specifies a video asset for the native ad.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::Video;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let video = Video::builder()
///     .mimes(vec!["video/mp4".to_string(), "video/webm".to_string()])
///     .minduration(Some(5))
///     .maxduration(Some(30))
///     .protocols(Some(vec![2, 3, 5, 6]))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Video<Ext: Extension = serde_json::Value> {
    /// Whitelist of content MIME types supported.
    /// **Required field**
    #[builder(default, setter(into))]
    pub mimes: Vec<String>,

    /// Minimum video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minduration: Option<i32>,

    /// Maximum video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxduration: Option<i32>,

    /// Array of supported video protocols.
    /// Refer to AdCOM `Protocol` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub protocols: Option<Vec<i32>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Video {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoBuilder {
        VideoBuilder::create_empty()
    }
}

/// Data Asset Request
///
/// Specifies a data element for the native ad (e.g., sponsored by, description, rating).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::Data;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = Data::builder()
///     .type_(2)  // Description
///     .len(Some(140))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Data<Ext: Extension = serde_json::Value> {
    /// Type of data element.
    /// **Required field**
    /// Refer to `NativeDataAssetType` enumeration:
    /// - 1 = Sponsored by message
    /// - 2 = Descriptive text
    /// - 3 = Rating (e.g., "4.5 stars")
    /// - 4 = Number of likes
    /// - 5 = Number of downloads
    /// - 6 = Product price
    /// - 7 = Sale price (discounted)
    /// - 8 = Phone number
    /// - 9 = Address
    /// - 10 = Additional descriptive text
    /// - 11 = Display URL
    /// - 12 = Call to action text
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub type_: i32,

    /// Maximum length of the data text in characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub len: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Data {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataBuilder {
        DataBuilder::create_empty()
    }
}

/// Event Tracker Request
///
/// Specifies event tracking requirements for the native ad.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::native::v12::EventTracker;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tracker = EventTracker::builder()
///     .event(1)  // Impression
///     .methods(vec![1, 2])  // Image pixel and JavaScript
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EventTracker<Ext: Extension = serde_json::Value> {
    /// Type of event to track.
    /// **Required field**
    /// Refer to `EventType` enumeration:
    /// - 1 = Impression
    /// - 2 = Viewable impression (MRC definition)
    /// - 3 = Click
    #[builder(setter(into))]
    pub event: i32,

    /// Array of tracking methods supported/required.
    /// **Required field**
    /// Refer to `EventTrackingMethod` enumeration:
    /// - 1 = Image pixel (1x1)
    /// - 2 = JavaScript
    #[builder(default, setter(into))]
    pub methods: Vec<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl EventTracker {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EventTrackerBuilder {
        EventTrackerBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_request_creation() {
        let request = NativeRequest::builder()
            .ver("1.2")
            .context(Some(1))
            .plcmttype(Some(1))
            .assets(vec![])
            .build()
            .unwrap();

        assert_eq!(request.ver, Some("1.2".to_string()));
        assert_eq!(request.context, Some(1));
        assert_eq!(request.plcmttype, Some(1));
    }

    #[test]
    fn test_native_request_serialization() {
        let request = NativeRequest::builder()
            .ver("1.2")
            .assets(vec![])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""ver":"1.2""#));
    }

    #[test]
    fn test_native_request_deserialization() {
        let json = r#"{"ver":"1.2","assets":[]}"#;
        let request: NativeRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.ver, Some("1.2".to_string()));
        assert_eq!(request.assets.len(), 0);
    }

    #[test]
    fn test_asset_with_title() {
        let asset = Asset::builder()
            .id(1)
            .required(Some(1))
            .title(Some(Title::builder().len(90).build().unwrap()))
            .build()
            .unwrap();

        assert_eq!(asset.id, 1);
        assert!(asset.title.is_some());
        assert_eq!(asset.title.as_ref().unwrap().len, 90);
    }

    #[test]
    fn test_asset_with_image() {
        let asset = Asset::builder()
            .id(2)
            .img(Some(
                Image::builder()
                    .type_(Some(3))
                    .w(Some(1200))
                    .h(Some(627))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(asset.id, 2);
        assert!(asset.img.is_some());
        let img = asset.img.as_ref().unwrap();
        assert_eq!(img.type_, Some(3));
        assert_eq!(img.w, Some(1200));
        assert_eq!(img.h, Some(627));
    }

    #[test]
    fn test_asset_serialization() {
        let asset = Asset::builder()
            .id(1)
            .title(Some(Title::builder().len(25).build().unwrap()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&asset).unwrap();
        assert!(json.contains(r#""id":1"#));
        assert!(json.contains(r#""len":25"#));
    }

    #[test]
    fn test_title_creation() {
        let title = Title::builder().len(90).build().unwrap();
        assert_eq!(title.len, 90);
    }

    #[test]
    fn test_image_with_dimensions() {
        let image = Image::builder()
            .type_(Some(1))
            .w(Some(300))
            .h(Some(250))
            .wmin(Some(200))
            .hmin(Some(150))
            .build()
            .unwrap();

        assert_eq!(image.type_, Some(1));
        assert_eq!(image.w, Some(300));
        assert_eq!(image.h, Some(250));
        assert_eq!(image.wmin, Some(200));
        assert_eq!(image.hmin, Some(150));
    }

    #[test]
    fn test_image_with_mimes() {
        let image = Image::builder()
            .mimes(Some(vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
            ]))
            .build()
            .unwrap();

        assert!(image.mimes.is_some());
        let mimes = image.mimes.as_ref().unwrap();
        assert_eq!(mimes.len(), 2);
        assert!(mimes.contains(&"image/jpeg".to_string()));
    }

    #[test]
    fn test_video_creation() {
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minduration(Some(5))
            .maxduration(Some(30))
            .build()
            .unwrap();

        assert_eq!(video.mimes.len(), 1);
        assert_eq!(video.minduration, Some(5));
        assert_eq!(video.maxduration, Some(30));
    }

    #[test]
    fn test_data_creation() {
        let data = Data::builder().type_(2).len(Some(140)).build().unwrap();

        assert_eq!(data.type_, 2);
        assert_eq!(data.len, Some(140));
    }

    #[test]
    fn test_event_tracker_creation() {
        let tracker = EventTracker::builder()
            .event(1)
            .methods(vec![1, 2])
            .build()
            .unwrap();

        assert_eq!(tracker.event, 1);
        assert_eq!(tracker.methods.len(), 2);
        assert_eq!(tracker.methods[0], 1);
        assert_eq!(tracker.methods[1], 2);
    }

    #[test]
    fn test_complete_native_request() {
        let request = NativeRequest::builder()
            .ver("1.2")
            .context(Some(1))
            .plcmttype(Some(1))
            .plcmtcnt(Some(1))
            .seq(Some(0))
            .assets(vec![
                Asset::builder()
                    .id(1)
                    .required(Some(1))
                    .title(Some(Title::builder().len(90).build().unwrap()))
                    .build()
                    .unwrap(),
                Asset::builder()
                    .id(2)
                    .img(Some(
                        Image::builder()
                            .type_(Some(3))
                            .w(Some(1200))
                            .h(Some(627))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                Asset::builder()
                    .id(3)
                    .data(Some(
                        Data::builder().type_(2).len(Some(140)).build().unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .eventtrackers(Some(vec![
                EventTracker::builder()
                    .event(1)
                    .methods(vec![1, 2])
                    .build()
                    .unwrap(),
            ]))
            .privacy(Some(1))
            .build()
            .unwrap();

        assert_eq!(request.assets.len(), 3);
        assert!(request.eventtrackers.is_some());
        assert_eq!(request.privacy, Some(1));
    }

    #[test]
    fn test_native_request_roundtrip() {
        let original = NativeRequest::builder()
            .ver("1.2")
            .context(Some(1))
            .plcmttype(Some(1))
            .assets(vec![
                Asset::builder()
                    .id(1)
                    .title(Some(Title::builder().len(90).build().unwrap()))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NativeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ver, original.ver);
        assert_eq!(parsed.context, original.context);
        assert_eq!(parsed.assets.len(), original.assets.len());
    }

    // === Negative Tests and Edge Cases ===

    #[test]
    fn test_optional_fields_omitted_in_json() {
        // Test that None fields are not serialized
        let request = NativeRequest::builder().assets(vec![]).build().unwrap();

        let json = serde_json::to_string(&request).unwrap();

        // Optional fields should not appear in JSON when None
        assert!(!json.contains("\"ver\""));
        assert!(!json.contains("\"context\""));
        assert!(!json.contains("\"plcmttype\""));
        assert!(!json.contains("\"privacy\""));
    }

    #[test]
    fn test_data_type_field_serialization() {
        // Test that "type_" field serializes as "type" in JSON
        let data = Data::builder().type_(2).build().unwrap();

        let json = serde_json::to_string(&data).unwrap();

        // Should use "type" in JSON, not "type_"
        assert!(json.contains(r#""type":2"#));
        assert!(!json.contains("type_"));
    }

    #[test]
    fn test_image_type_field_serialization() {
        // Test that Image "type_" field serializes as "type" in JSON
        let image = Image::builder().type_(Some(3)).build().unwrap();

        let json = serde_json::to_string(&image).unwrap();

        // Should use "type" in JSON, not "type_"
        assert!(json.contains(r#""type":3"#));
        assert!(!json.contains("type_"));
    }

    #[test]
    fn test_empty_assets_array() {
        // Test that empty assets array is valid
        let request = NativeRequest::builder()
            .ver("1.2")
            .assets(vec![])
            .build()
            .unwrap();

        assert_eq!(request.assets.len(), 0);

        // Should serialize and deserialize correctly
        let json = serde_json::to_string(&request).unwrap();
        let parsed: NativeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.assets.len(), 0);
    }

    #[test]
    fn test_asset_with_no_asset_type() {
        // Test asset with only id (no title/img/video/data)
        // This is valid at the type level but violates spec semantics
        let asset = Asset::builder().id(1).build().unwrap();

        assert_eq!(asset.id, 1);
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
    }

    #[test]
    fn test_asset_with_multiple_types() {
        // Test that Rust allows creating assets with multiple types
        // (spec violation, but type system doesn't prevent it)
        let asset = Asset::builder()
            .id(1)
            .title(Some(Title::builder().len(25).build().unwrap()))
            .img(Some(Image::builder().type_(Some(3)).build().unwrap()))
            .build()
            .unwrap();

        // Both fields are present (violates spec mutual exclusivity)
        assert!(asset.title.is_some());
        assert!(asset.img.is_some());
    }

    #[test]
    fn test_minimal_native_request() {
        // Test minimal valid request (only required fields)
        let request = NativeRequest::builder().assets(vec![]).build().unwrap();

        assert!(request.ver.is_none());
        assert!(request.context.is_none());
        assert_eq!(request.assets.len(), 0);
    }

    #[test]
    fn test_minimal_title() {
        // Test minimal Title (only required field)
        let title = Title::builder().len(90).build().unwrap();

        assert_eq!(title.len, 90);
        assert!(title.ext.is_none());
    }

    #[test]
    fn test_minimal_image() {
        // Test Image with no fields (all optional)
        let image = Image::builder().build().unwrap();

        assert!(image.type_.is_none());
        assert!(image.w.is_none());
        assert!(image.h.is_none());
        assert!(image.mimes.is_none());
    }

    #[test]
    fn test_image_with_max_dimensions() {
        // Test new wmax/hmax fields
        let image = Image::builder()
            .wmin(Some(600))
            .hmin(Some(314))
            .wmax(Some(1920))
            .hmax(Some(1080))
            .build()
            .unwrap();

        assert_eq!(image.wmin, Some(600));
        assert_eq!(image.hmin, Some(314));
        assert_eq!(image.wmax, Some(1920));
        assert_eq!(image.hmax, Some(1080));
    }

    #[test]
    fn test_minimal_video() {
        // Test Video with only required field
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .build()
            .unwrap();

        assert_eq!(video.mimes.len(), 1);
        assert!(video.minduration.is_none());
        assert!(video.maxduration.is_none());
    }

    #[test]
    fn test_minimal_data() {
        // Test Data with only required field
        let data = Data::builder().type_(2).build().unwrap();

        assert_eq!(data.type_, 2);
        assert!(data.len.is_none());
    }

    #[test]
    fn test_minimal_event_tracker() {
        // Test EventTracker with only required fields
        let tracker = EventTracker::builder()
            .event(1)
            .methods(vec![1])
            .build()
            .unwrap();

        assert_eq!(tracker.event, 1);
        assert_eq!(tracker.methods.len(), 1);
    }

    #[test]
    fn test_deserialization_with_unknown_fields() {
        // Test that deserialization ignores unknown fields
        let json = r#"{
            "ver": "1.2",
            "assets": [],
            "unknown_field": "should be ignored",
            "another_unknown": 123
        }"#;

        let result: Result<NativeRequest, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.ver, Some("1.2".to_string()));
    }

    #[test]
    fn test_deserialization_with_null_optional_fields() {
        // Test that explicit null values are handled correctly
        let json = r#"{
            "ver": null,
            "context": null,
            "assets": []
        }"#;

        let result: Result<NativeRequest, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert!(request.ver.is_none());
        assert!(request.context.is_none());
    }

    #[test]
    fn test_all_context_types() {
        // Test all valid DisplayContextType values (1-3)
        for context_type in 1..=3 {
            let request = NativeRequest::builder()
                .context(Some(context_type))
                .assets(vec![])
                .build()
                .unwrap();

            assert_eq!(request.context, Some(context_type));
        }
    }

    #[test]
    fn test_all_placement_types() {
        // Test all valid DisplayPlacementType values (1-4)
        for placement_type in 1..=4 {
            let request = NativeRequest::builder()
                .plcmttype(Some(placement_type))
                .assets(vec![])
                .build()
                .unwrap();

            assert_eq!(request.plcmttype, Some(placement_type));
        }
    }

    #[test]
    fn test_multi_placement_request() {
        // Test plcmtcnt > 1 for multiple identical placements
        let request = NativeRequest::builder()
            .plcmtcnt(Some(5))
            .seq(Some(2))
            .assets(vec![])
            .build()
            .unwrap();

        assert_eq!(request.plcmtcnt, Some(5));
        assert_eq!(request.seq, Some(2));
    }

    #[test]
    fn test_dco_flags() {
        // Test DCO support flags
        let request = NativeRequest::builder()
            .aurlsupport(Some(1))
            .durlsupport(Some(1))
            .assets(vec![])
            .build()
            .unwrap();

        assert_eq!(request.aurlsupport, Some(1));
        assert_eq!(request.durlsupport, Some(1));
    }

    // === Phase 2.1: Integer-as-Enum Field Validation Tests ===

    #[test]
    fn test_context_with_invalid_value() {
        // DisplayContextType valid values are 1-3
        // Test that invalid values currently pass
        let json = r#"{"context":99,"assets":[]}"#;
        let result: Result<NativeRequest, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Invalid context value 99 currently passes");
        assert_eq!(result.unwrap().context, Some(99));
        // TODO: DisplayContextType should be validated (valid range: 1-3)
    }

    #[test]
    fn test_context_with_zero() {
        // Test context with zero (invalid for DisplayContextType)
        let json = r#"{"context":0,"assets":[]}"#;
        let result: Result<NativeRequest, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Zero context value currently passes");
        assert_eq!(result.unwrap().context, Some(0));
        // Document: Zero is not a valid DisplayContextType value
    }

    #[test]
    fn test_contextsubtype_with_invalid_value() {
        // Test invalid contextsubtype
        let json = r#"{"contextsubtype":999,"assets":[]}"#;
        let result: Result<NativeRequest, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Invalid contextsubtype currently passes");
        assert_eq!(result.unwrap().contextsubtype, Some(999));
        // TODO: Consider validation for contextsubtype based on context value
    }

    #[test]
    fn test_plcmttype_with_invalid_value() {
        // DisplayPlacementType valid values are 1-4
        // Test that invalid values currently pass
        let json = r#"{"plcmttype":99,"assets":[]}"#;
        let result: Result<NativeRequest, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Invalid plcmttype value 99 currently passes"
        );
        assert_eq!(result.unwrap().plcmttype, Some(99));
        // TODO: DisplayPlacementType should be validated (valid range: 1-4)
    }

    #[test]
    fn test_plcmttype_with_zero() {
        // Test plcmttype with zero (invalid)
        let json = r#"{"plcmttype":0,"assets":[]}"#;
        let result: Result<NativeRequest, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Zero plcmttype value currently passes");
        assert_eq!(result.unwrap().plcmttype, Some(0));
        // Document: Zero is not a valid DisplayPlacementType value
    }

    #[test]
    fn test_negative_enum_values() {
        // Test negative values in native enum fields
        let json = r#"{"context":-1,"plcmttype":-1,"assets":[]}"#;
        let result: Result<NativeRequest, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Negative enum values currently pass");
        let request = result.unwrap();
        assert_eq!(request.context, Some(-1));
        assert_eq!(request.plcmttype, Some(-1));
        // Document: Negative values are invalid for DisplayContextType and DisplayPlacementType
    }

    #[test]
    fn test_valid_context_plcmttype_combinations() {
        // Test all valid combinations of context and plcmttype
        for context in 1..=3 {
            for plcmttype in 1..=4 {
                let request = NativeRequest::builder()
                    .context(Some(context))
                    .plcmttype(Some(plcmttype))
                    .assets(vec![])
                    .build()
                    .unwrap();

                assert_eq!(request.context, Some(context));
                assert_eq!(request.plcmttype, Some(plcmttype));

                // Verify serialization roundtrip
                let json = serde_json::to_string(&request).unwrap();
                let deserialized: NativeRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(request.context, deserialized.context);
                assert_eq!(request.plcmttype, deserialized.plcmttype);
            }
        }
    }

    // === Phase 2.2: Mutually Exclusive Field Tests ===

    #[test]
    fn test_asset_with_no_type_fields() {
        // Per spec: Asset must have exactly ONE of title/img/video/data
        // Test that asset with NO type fields currently passes
        let asset = Asset::builder().id(1).build();

        assert!(asset.is_ok(), "Asset with no type fields currently passes");
        let asset = asset.unwrap();
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
        // TODO: Should be rejected - asset must have exactly one type field
    }

    #[test]
    fn test_asset_with_multiple_type_fields_title_and_img() {
        // Test that asset with BOTH title AND img currently passes
        let title = Title::builder().len(90).build().unwrap();
        let img = Image::builder().build().unwrap();

        let asset = Asset::builder()
            .id(1)
            .title(Some(title))
            .img(Some(img))
            .build();

        assert!(
            asset.is_ok(),
            "Asset with both title and img currently passes"
        );
        let asset = asset.unwrap();
        assert!(asset.title.is_some());
        assert!(asset.img.is_some());
        // TODO: Should be rejected - violates mutually exclusive constraint
    }

    #[test]
    fn test_asset_with_multiple_type_fields_all_four() {
        // Test that asset with ALL four type fields currently passes
        let title = Title::builder().len(90).build().unwrap();
        let img = Image::builder().build().unwrap();
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .build()
            .unwrap();
        let data = Data::builder().type_(1).build().unwrap();

        let asset = Asset::builder()
            .id(1)
            .title(Some(title))
            .img(Some(img))
            .video(Some(video))
            .data(Some(data))
            .build();

        assert!(
            asset.is_ok(),
            "Asset with all four type fields currently passes"
        );
        let asset = asset.unwrap();
        assert!(asset.title.is_some());
        assert!(asset.img.is_some());
        assert!(asset.video.is_some());
        assert!(asset.data.is_some());
        // TODO: Should be rejected - can only have ONE type field
    }

    #[test]
    fn test_asset_with_title_only() {
        // Valid: exactly one asset type
        let title = Title::builder().len(90).build().unwrap();

        let asset = Asset::builder().id(1).title(Some(title)).build().unwrap();

        assert!(asset.title.is_some());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
    }

    #[test]
    fn test_asset_with_img_only() {
        // Valid: exactly one asset type
        let img = Image::builder().build().unwrap();

        let asset = Asset::builder().id(1).img(Some(img)).build().unwrap();

        assert!(asset.title.is_none());
        assert!(asset.img.is_some());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
    }

    #[test]
    fn test_asset_with_video_only() {
        // Valid: exactly one asset type
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .build()
            .unwrap();

        let asset = Asset::builder().id(1).video(Some(video)).build().unwrap();

        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_some());
        assert!(asset.data.is_none());
    }

    #[test]
    fn test_asset_with_data_only() {
        // Valid: exactly one asset type
        let data = Data::builder().type_(1).build().unwrap();

        let asset = Asset::builder().id(1).data(Some(data)).build().unwrap();

        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_some());
    }

    #[test]
    fn test_asset_deserialization_with_multiple_types() {
        // Test deserialization of asset with multiple type fields
        let json = r#"{
            "id": 1,
            "title": {"len": 90},
            "img": {"w": 300, "h": 250}
        }"#;

        let result: Result<Asset, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Asset with multiple types in JSON currently deserializes"
        );
        let asset = result.unwrap();
        assert!(asset.title.is_some());
        assert!(asset.img.is_some());
        // TODO: Deserialization should reject mutually exclusive fields
    }

    #[test]
    fn test_asset_deserialization_with_no_types() {
        // Test deserialization of asset without any type fields
        let json = r#"{"id": 1}"#;

        let result: Result<Asset, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Asset with no types currently deserializes");
        let asset = result.unwrap();
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
        // TODO: Should require at least one type field
    }
}
