//! OpenRTB Native 1.2 Response Objects
//!
//! This module implements response-side objects for the OpenRTB Native Ads 1.2 specification.

use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// OpenRTB Native 1.2 Response
///
/// Root object for native ad response containing asset responses and tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::{NativeResponse, AssetResponse, Link, TitleResponse};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let response = NativeResponse::builder()
///     .ver("1.2")
///     .assets(vec![
///         AssetResponse::builder()
///             .id(1)
///             .required(Some(1))
///             .title(Some(TitleResponse::builder()
///                 .text("Amazing Product".to_string())
///                 .build()?))
///             .build()?
///     ])
///     .link(Link::builder()
///         .url("https://example.com/product".to_string())
///         .build()?)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NativeResponse<Ext: Extension = crate::DefaultExt> {
    /// Version of the Native Markup in use.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ver: Option<String>,

    /// Array of asset response objects.
    /// **Required field** (unless using assetsurl or dcourl)
    #[builder(default, setter(into))]
    pub assets: Vec<AssetResponse<Ext>>,

    /// Link object for default click destination and tracking.
    /// **Required field**
    #[builder(setter(into))]
    pub link: Link<Ext>,

    /// Array of impression tracking URLs.
    /// **Deprecated** - Use eventtrackers instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub imptrackers: Option<Vec<String>>,

    /// JavaScript impression tracker.
    /// **Deprecated** - Use eventtrackers instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub jstracker: Option<String>,

    /// Array of event tracker response objects.
    /// Preferred method for event tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub eventtrackers: Option<Vec<EventTrackerResponse<Ext>>>,

    /// URL to privacy/AdChoices link.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub privacy: Option<String>,

    /// URL where assets JSON is hosted.
    /// Alternative to inline assets array.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub assetsurl: Option<String>,

    /// URL for dynamic creative optimization (DCO).
    /// Beta feature for dynamic creative retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dcourl: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl NativeResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NativeResponseBuilder {
        NativeResponseBuilder::create_empty()
    }
}

/// Native Asset Response
///
/// Response for a single asset in the native ad.
/// Must contain exactly one of: title, img, video, or data.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::{AssetResponse, ImageResponse};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let asset = AssetResponse::builder()
///     .id(2)
///     .required(Some(1))
///     .img(Some(ImageResponse::builder()
///         .url("https://example.com/image.jpg".to_string())
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
pub struct AssetResponse<Ext: Extension = crate::DefaultExt> {
    /// Unique asset ID, matching the request asset ID.
    /// **Required field**
    #[builder(setter(into))]
    pub id: i32,

    /// Asset required flag from request (0=optional, 1=required).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub required: Option<i32>,

    /// Title response object. Mutually exclusive with img, video, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<TitleResponse<Ext>>,

    /// Image response object. Mutually exclusive with title, video, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub img: Option<ImageResponse<Ext>>,

    /// Video response object. Mutually exclusive with title, img, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub video: Option<VideoResponse<Ext>>,

    /// Data response object. Mutually exclusive with title, img, video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<DataResponse<Ext>>,

    /// Link object for asset-specific click destination.
    /// Overrides default link in NativeResponse.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub link: Option<Link<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AssetResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssetResponseBuilder {
        AssetResponseBuilder::create_empty()
    }
}

/// Title Asset Response
///
/// Response for a title text asset.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::TitleResponse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let title = TitleResponse::builder()
///     .text("Amazing Product - Buy Now!".to_string())
///     .len(Some(27))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct TitleResponse<Ext: Extension = crate::DefaultExt> {
    /// The title text.
    /// **Required field**
    #[builder(setter(into))]
    pub text: String,

    /// Length of the title text. Optional for response validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub len: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl TitleResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TitleResponseBuilder {
        TitleResponseBuilder::create_empty()
    }
}

/// Image Asset Response
///
/// Response for an image asset.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::ImageResponse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let image = ImageResponse::builder()
///     .url("https://cdn.example.com/product-image.jpg".to_string())
///     .w(Some(1200))
///     .h(Some(627))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ImageResponse<Ext: Extension = crate::DefaultExt> {
    /// URL of the image asset.
    /// **Required field**
    #[builder(setter(into))]
    pub url: String,

    /// Width of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Height of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ImageResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImageResponseBuilder {
        ImageResponseBuilder::create_empty()
    }
}

/// Video Asset Response
///
/// Response for a video asset.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::VideoResponse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let video = VideoResponse::builder()
///     .vasttag("<VAST>...</VAST>".to_string())
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct VideoResponse<Ext: Extension = crate::DefaultExt> {
    /// VAST XML markup for the video.
    /// **Required field**
    #[builder(setter(into))]
    pub vasttag: String,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl VideoResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoResponseBuilder {
        VideoResponseBuilder::create_empty()
    }
}

/// Data Asset Response
///
/// Response for a data element.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::DataResponse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = DataResponse::builder()
///     .value("High quality product with excellent reviews".to_string())
///     .type_(Some(2))  // Descriptive text
///     .label(Some("Description".to_string()))
///     .len(Some(46))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DataResponse<Ext: Extension = crate::DefaultExt> {
    /// The data value.
    /// **Required field**
    #[builder(setter(into))]
    pub value: String,

    /// Type of data element.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    #[builder(default)]
    pub type_: Option<i32>,

    /// Optional formatted string name of the data type to be displayed.
    /// For example: "Sponsored By", "Price", "Rating", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label: Option<String>,

    /// Length of the data value. Optional for response validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub len: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl DataResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataResponseBuilder {
        DataResponseBuilder::create_empty()
    }
}

/// Link Object
///
/// Defines the click destination and tracking for the native ad or individual asset.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::Link;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let link = Link::builder()
///     .url("https://example.com/product?utm_source=native".to_string())
///     .clicktrackers(Some(vec![
///         "https://tracker1.com/click".to_string(),
///         "https://tracker2.com/click".to_string(),
///     ]))
///     .fallback(Some("https://example.com/".to_string()))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Link<Ext: Extension = crate::DefaultExt> {
    /// Landing URL for the clickable link.
    /// **Required field**
    #[builder(setter(into))]
    pub url: String,

    /// Array of third-party click tracker URLs.
    /// Fired in addition to url when user clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clicktrackers: Option<Vec<String>>,

    /// Fallback URL for deep-link scenarios.
    /// Used when primary URL fails (e.g., app not installed).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fallback: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Link {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> LinkBuilder {
        LinkBuilder::create_empty()
    }
}

/// Event Tracker Response
///
/// Response for event tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_openrtb_native::v12::EventTrackerResponse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tracker = EventTrackerResponse::builder()
///     .event(1)  // Impression
///     .method(1)  // Image pixel
///     .url("https://tracker.example.com/imp?id=123")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EventTrackerResponse<Ext: Extension = crate::DefaultExt> {
    /// Type of event being tracked.
    /// **Required field**
    /// Refer to `EventType` enumeration:
    /// - 1 = Impression
    /// - 2 = Viewable impression (MRC definition)
    /// - 3 = Click
    #[builder(setter(into))]
    pub event: i32,

    /// Method of tracking.
    /// **Required field**
    /// Refer to `EventTrackingMethod` enumeration:
    /// - 1 = Image pixel (1x1)
    /// - 2 = JavaScript
    #[builder(setter(into))]
    pub method: i32,

    /// URL of the tracking pixel or JavaScript tag.
    /// Optional per OpenRTB Native 1.2 specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub url: Option<String>,

    /// Optional custom data for the tracker.
    /// Can contain macros for dynamic substitution.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub customdata: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl EventTrackerResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EventTrackerResponseBuilder {
        EventTrackerResponseBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_response_creation() {
        let response = NativeResponse::builder()
            .ver("1.2")
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(response.ver, Some("1.2".to_string()));
        assert_eq!(response.link.url, "https://example.com");
    }

    #[test]
    fn test_native_response_serialization() {
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains(r#""url":"https://example.com""#));
    }

    #[test]
    fn test_native_response_deserialization() {
        let json = r#"{"assets":[],"link":{"url":"https://example.com"}}"#;
        let response: NativeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.link.url, "https://example.com");
    }

    #[test]
    fn test_asset_response_with_title() {
        let asset = AssetResponse::builder()
            .id(1)
            .required(Some(1))
            .title(Some(
                TitleResponse::builder()
                    .text("Amazing Product".to_string())
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(asset.id, 1);
        assert!(asset.title.is_some());
        assert_eq!(asset.title.as_ref().unwrap().text, "Amazing Product");
    }

    #[test]
    fn test_asset_response_with_image() {
        let asset = AssetResponse::builder()
            .id(2)
            .img(Some(
                ImageResponse::builder()
                    .url("https://example.com/image.jpg".to_string())
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
        assert_eq!(img.url, "https://example.com/image.jpg");
        assert_eq!(img.w, Some(1200));
        assert_eq!(img.h, Some(627));
    }

    #[test]
    fn test_title_response_creation() {
        let title = TitleResponse::builder()
            .text("Test Title".to_string())
            .len(Some(10))
            .build()
            .unwrap();

        assert_eq!(title.text, "Test Title");
        assert_eq!(title.len, Some(10));
    }

    #[test]
    fn test_image_response_creation() {
        let image = ImageResponse::builder()
            .url("https://example.com/img.jpg".to_string())
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(image.url, "https://example.com/img.jpg");
        assert_eq!(image.w, Some(300));
        assert_eq!(image.h, Some(250));
    }

    #[test]
    fn test_video_response_creation() {
        let video = VideoResponse::builder()
            .vasttag("<VAST>...</VAST>".to_string())
            .build()
            .unwrap();

        assert_eq!(video.vasttag, "<VAST>...</VAST>");
    }

    #[test]
    fn test_data_response_creation() {
        let data = DataResponse::builder()
            .value("Product description".to_string())
            .len(Some(19))
            .build()
            .unwrap();

        assert_eq!(data.value, "Product description");
        assert_eq!(data.len, Some(19));
    }

    #[test]
    fn test_link_with_trackers() {
        let link = Link::builder()
            .url("https://example.com/product".to_string())
            .clicktrackers(Some(vec![
                "https://tracker1.com".to_string(),
                "https://tracker2.com".to_string(),
            ]))
            .fallback(Some("https://example.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(link.url, "https://example.com/product");
        assert!(link.clicktrackers.is_some());
        assert_eq!(link.clicktrackers.as_ref().unwrap().len(), 2);
        assert_eq!(link.fallback, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_event_tracker_response_creation() {
        let tracker = EventTrackerResponse::builder()
            .event(1)
            .method(1)
            .url("https://tracker.example.com/imp")
            .build()
            .unwrap();

        assert_eq!(tracker.event, 1);
        assert_eq!(tracker.method, 1);
        assert_eq!(
            tracker.url,
            Some("https://tracker.example.com/imp".to_string())
        );
    }

    #[test]
    fn test_complete_native_response() {
        let response = NativeResponse::builder()
            .ver("1.2")
            .assets(vec![
                AssetResponse::builder()
                    .id(1)
                    .title(Some(
                        TitleResponse::builder()
                            .text("Amazing Product".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                AssetResponse::builder()
                    .id(2)
                    .img(Some(
                        ImageResponse::builder()
                            .url("https://example.com/img.jpg".to_string())
                            .w(Some(1200))
                            .h(Some(627))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                AssetResponse::builder()
                    .id(3)
                    .data(Some(
                        DataResponse::builder()
                            .value("Great product!".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .link(
                Link::builder()
                    .url("https://example.com/product".to_string())
                    .clicktrackers(Some(vec!["https://tracker.com".to_string()]))
                    .build()
                    .unwrap(),
            )
            .eventtrackers(Some(vec![
                EventTrackerResponse::builder()
                    .event(1)
                    .method(1)
                    .url("https://tracker.com/imp".to_string())
                    .build()
                    .unwrap(),
            ]))
            .privacy(Some("https://example.com/privacy".to_string()))
            .build()
            .unwrap();

        assert_eq!(response.assets.len(), 3);
        assert!(response.eventtrackers.is_some());
        assert_eq!(
            response.privacy,
            Some("https://example.com/privacy".to_string())
        );
    }

    #[test]
    fn test_native_response_roundtrip() {
        let original = NativeResponse::builder()
            .ver("1.2")
            .assets(vec![
                AssetResponse::builder()
                    .id(1)
                    .title(Some(
                        TitleResponse::builder()
                            .text("Test".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NativeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ver, original.ver);
        assert_eq!(parsed.assets.len(), original.assets.len());
        assert_eq!(parsed.link.url, original.link.url);
    }

    #[test]
    fn test_deprecated_tracking_fields() {
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .imptrackers(Some(vec!["https://imp1.com".to_string()]))
            .jstracker(Some("<script>...</script>".to_string()))
            .build()
            .unwrap();

        assert!(response.imptrackers.is_some());
        assert!(response.jstracker.is_some());
    }

    // === Negative Tests and Edge Cases ===

    #[test]
    fn test_optional_response_fields_omitted_in_json() {
        // Test that None fields are not serialized
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();

        // Optional fields should not appear in JSON when None
        assert!(!json.contains("\"ver\""));
        assert!(!json.contains("\"imptrackers\""));
        assert!(!json.contains("\"jstracker\""));
        assert!(!json.contains("\"privacy\""));
        assert!(!json.contains("\"assetsurl\""));
        assert!(!json.contains("\"dcourl\""));
    }

    #[test]
    fn test_data_response_type_field_serialization() {
        // Test that DataResponse "type_" field serializes as "type" in JSON
        let data = DataResponse::builder()
            .value("test value".to_string())
            .type_(Some(2))
            .build()
            .unwrap();

        let json = serde_json::to_string(&data).unwrap();

        // Should use "type" in JSON, not "type_"
        assert!(json.contains(r#""type":2"#));
        assert!(!json.contains("type_"));
    }

    #[test]
    fn test_data_response_with_all_fields() {
        // Test DataResponse with newly added type_ and label fields
        let data = DataResponse::builder()
            .value("$99.99".to_string())
            .type_(Some(6)) // Price
            .label(Some("Price".to_string()))
            .len(Some(6))
            .build()
            .unwrap();

        assert_eq!(data.value, "$99.99");
        assert_eq!(data.type_, Some(6));
        assert_eq!(data.label, Some("Price".to_string()));
        assert_eq!(data.len, Some(6));
    }

    #[test]
    fn test_event_tracker_response_optional_url() {
        // Test that EventTrackerResponse.url is now optional
        let tracker = EventTrackerResponse::builder()
            .event(1)
            .method(2) // JavaScript
            .build()
            .unwrap();

        assert_eq!(tracker.event, 1);
        assert_eq!(tracker.method, 2);
        assert!(tracker.url.is_none());
    }

    #[test]
    fn test_event_tracker_response_with_url() {
        // Test EventTrackerResponse with URL provided
        let tracker = EventTrackerResponse::builder()
            .event(1)
            .method(1)
            .url("https://tracker.com/imp")
            .build()
            .unwrap();

        assert_eq!(tracker.url, Some("https://tracker.com/imp".to_string()));
    }

    #[test]
    fn test_minimal_native_response() {
        // Test minimal valid response (only required fields)
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert!(response.ver.is_none());
        assert_eq!(response.assets.len(), 0);
        assert_eq!(response.link.url, "https://example.com");
    }

    #[test]
    fn test_minimal_asset_response() {
        // Test minimal AssetResponse (only required field)
        let asset = AssetResponse::builder().id(1).build().unwrap();

        assert_eq!(asset.id, 1);
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
        assert!(asset.link.is_none());
    }

    #[test]
    fn test_asset_response_with_multiple_types() {
        // Test that Rust allows creating asset responses with multiple types
        // (spec violation, but type system doesn't prevent it)
        let asset = AssetResponse::builder()
            .id(1)
            .title(Some(
                TitleResponse::builder()
                    .text("Title".to_string())
                    .build()
                    .unwrap(),
            ))
            .img(Some(
                ImageResponse::builder()
                    .url("https://example.com/img.jpg".to_string())
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        // Both fields are present (violates spec mutual exclusivity)
        assert!(asset.title.is_some());
        assert!(asset.img.is_some());
    }

    #[test]
    fn test_minimal_title_response() {
        // Test minimal TitleResponse (only required field)
        let title = TitleResponse::builder()
            .text("Amazing Product".to_string())
            .build()
            .unwrap();

        assert_eq!(title.text, "Amazing Product");
        assert!(title.len.is_none());
    }

    #[test]
    fn test_minimal_image_response() {
        // Test minimal ImageResponse (only required field)
        let image = ImageResponse::builder()
            .url("https://cdn.example.com/image.jpg".to_string())
            .build()
            .unwrap();

        assert_eq!(image.url, "https://cdn.example.com/image.jpg");
        assert!(image.w.is_none());
        assert!(image.h.is_none());
    }

    #[test]
    fn test_minimal_video_response() {
        // Test minimal VideoResponse (only required field)
        let video = VideoResponse::builder()
            .vasttag("<VAST>...</VAST>".to_string())
            .build()
            .unwrap();

        assert_eq!(video.vasttag, "<VAST>...</VAST>");
    }

    #[test]
    fn test_minimal_data_response() {
        // Test minimal DataResponse (only required field)
        let data = DataResponse::builder()
            .value("High quality product".to_string())
            .build()
            .unwrap();

        assert_eq!(data.value, "High quality product");
        assert!(data.type_.is_none());
        assert!(data.label.is_none());
        assert!(data.len.is_none());
    }

    #[test]
    fn test_minimal_link() {
        // Test minimal Link (only required field)
        let link = Link::builder()
            .url("https://example.com".to_string())
            .build()
            .unwrap();

        assert_eq!(link.url, "https://example.com");
        assert!(link.clicktrackers.is_none());
        assert!(link.fallback.is_none());
    }

    #[test]
    fn test_link_with_multiple_click_trackers() {
        // Test Link with multiple click tracking URLs
        let link = Link::builder()
            .url("https://example.com/product".to_string())
            .clicktrackers(Some(vec![
                "https://tracker1.com/click".to_string(),
                "https://tracker2.com/click".to_string(),
                "https://tracker3.com/click".to_string(),
            ]))
            .build()
            .unwrap();

        let trackers = link.clicktrackers.unwrap();
        assert_eq!(trackers.len(), 3);
    }

    #[test]
    fn test_link_with_fallback() {
        // Test Link with fallback URL for deep-link scenarios
        let link = Link::builder()
            .url("myapp://product/123".to_string())
            .fallback(Some("https://example.com/product/123".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            link.fallback,
            Some("https://example.com/product/123".to_string())
        );
    }

    #[test]
    fn test_response_deserialization_with_unknown_fields() {
        // Test that deserialization ignores unknown fields
        let json = r#"{
            "assets": [],
            "link": {"url": "https://example.com"},
            "unknown_field": "should be ignored",
            "another_unknown": 123
        }"#;

        let result: Result<NativeResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.link.url, "https://example.com");
    }

    #[test]
    fn test_response_deserialization_with_null_fields() {
        // Test that explicit null values are handled correctly
        let json = r#"{
            "ver": null,
            "assets": [],
            "link": {"url": "https://example.com"},
            "privacy": null
        }"#;

        let result: Result<NativeResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.ver.is_none());
        assert!(response.privacy.is_none());
    }

    #[test]
    fn test_empty_assets_array_response() {
        // Test response with empty assets array (valid with assetsurl/dcourl)
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .assetsurl(Some("https://example.com/assets.json".to_string()))
            .build()
            .unwrap();

        assert_eq!(response.assets.len(), 0);
        assert!(response.assetsurl.is_some());
    }

    #[test]
    fn test_dco_url_response() {
        // Test response with DCO URL (Beta feature)
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .dcourl(Some("https://dco.example.com/creative".to_string()))
            .build()
            .unwrap();

        assert!(response.dcourl.is_some());
        assert_eq!(response.dcourl.unwrap(), "https://dco.example.com/creative");
    }

    #[test]
    fn test_asset_response_with_specific_link() {
        // Test AssetResponse with asset-specific link override
        let asset = AssetResponse::builder()
            .id(1)
            .title(Some(
                TitleResponse::builder()
                    .text("Product".to_string())
                    .build()
                    .unwrap(),
            ))
            .link(Some(
                Link::builder()
                    .url("https://specific-link.com".to_string())
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert!(asset.link.is_some());
        assert_eq!(
            asset.link.as_ref().unwrap().url,
            "https://specific-link.com"
        );
    }

    #[test]
    fn test_event_tracker_response_with_custom_data() {
        // Test EventTrackerResponse with customdata field
        let tracker = EventTrackerResponse::builder()
            .event(1)
            .method(1)
            .url("https://tracker.com/imp")
            .customdata(Some("macro=%%AUCTION_ID%%".to_string()))
            .build()
            .unwrap();

        assert!(tracker.customdata.is_some());
        assert!(tracker.customdata.as_ref().unwrap().contains("macro"));
    }

    #[test]
    fn test_all_data_asset_types_responses() {
        // Test all 12 NativeDataAssetType values in responses
        for data_type in 1..=12 {
            let data = DataResponse::builder()
                .value("test".to_string())
                .type_(Some(data_type))
                .build()
                .unwrap();

            assert_eq!(data.type_, Some(data_type));
        }
    }

    #[test]
    fn test_privacy_url() {
        // Test privacy/AdChoices URL field
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .privacy(Some("https://example.com/privacy-policy".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            response.privacy,
            Some("https://example.com/privacy-policy".to_string())
        );
    }

    // === Default Value Tests ===

    #[test]
    fn test_native_response_default() {
        let response: NativeResponse = NativeResponse::default();
        assert!(response.ver.is_none());
        assert!(response.assets.is_empty());
        assert_eq!(response.link.url, "");
        assert!(response.imptrackers.is_none());
        assert!(response.jstracker.is_none());
        assert!(response.eventtrackers.is_none());
        assert!(response.privacy.is_none());
        assert!(response.assetsurl.is_none());
        assert!(response.dcourl.is_none());
        assert!(response.ext.is_none());
    }

    #[test]
    fn test_asset_response_default() {
        let asset: AssetResponse = AssetResponse::default();
        assert_eq!(asset.id, 0);
        assert!(asset.required.is_none());
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.video.is_none());
        assert!(asset.data.is_none());
        assert!(asset.link.is_none());
        assert!(asset.ext.is_none());
    }

    #[test]
    fn test_title_response_default() {
        let title: TitleResponse = TitleResponse::default();
        assert_eq!(title.text, "");
        assert!(title.len.is_none());
        assert!(title.ext.is_none());
    }

    #[test]
    fn test_image_response_default() {
        let image: ImageResponse = ImageResponse::default();
        assert_eq!(image.url, "");
        assert!(image.w.is_none());
        assert!(image.h.is_none());
        assert!(image.ext.is_none());
    }

    #[test]
    fn test_video_response_default() {
        let video: VideoResponse = VideoResponse::default();
        assert_eq!(video.vasttag, "");
        assert!(video.ext.is_none());
    }

    #[test]
    fn test_data_response_default() {
        let data: DataResponse = DataResponse::default();
        assert_eq!(data.value, "");
        assert!(data.type_.is_none());
        assert!(data.label.is_none());
        assert!(data.len.is_none());
        assert!(data.ext.is_none());
    }

    #[test]
    fn test_link_default() {
        let link: Link = Link::default();
        assert_eq!(link.url, "");
        assert!(link.clicktrackers.is_none());
        assert!(link.fallback.is_none());
        assert!(link.ext.is_none());
    }

    #[test]
    fn test_event_tracker_response_default() {
        let tracker: EventTrackerResponse = EventTrackerResponse::default();
        assert_eq!(tracker.event, 0);
        assert_eq!(tracker.method, 0);
        assert!(tracker.url.is_none());
        assert!(tracker.customdata.is_none());
        assert!(tracker.ext.is_none());
    }

    // === Serde Roundtrip Tests ===

    #[test]
    fn test_title_response_serde_roundtrip() {
        let original = TitleResponse::builder()
            .text("Buy Now - Limited Offer!".to_string())
            .len(Some(23))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: TitleResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_image_response_serde_roundtrip() {
        let original = ImageResponse::builder()
            .url("https://cdn.example.com/hero.jpg".to_string())
            .w(Some(1200))
            .h(Some(627))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: ImageResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_video_response_serde_roundtrip() {
        let original = VideoResponse::builder()
            .vasttag("<VAST version=\"4.0\"><Ad></Ad></VAST>".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: VideoResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_data_response_serde_roundtrip() {
        let original = DataResponse::builder()
            .value("$49.99".to_string())
            .type_(Some(6))
            .label(Some("Price".to_string()))
            .len(Some(6))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: DataResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_link_serde_roundtrip() {
        let original = Link::builder()
            .url("https://example.com/product?ref=native".to_string())
            .clicktrackers(Some(vec![
                "https://tracker1.com/click?id=abc".to_string(),
                "https://tracker2.com/click?id=def".to_string(),
            ]))
            .fallback(Some("https://example.com/fallback".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Link = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_event_tracker_response_serde_roundtrip() {
        let original = EventTrackerResponse::builder()
            .event(1)
            .method(1)
            .url("https://tracker.example.com/imp?id=abc")
            .customdata(Some("campaign=summer2026".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: EventTrackerResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_asset_response_with_title_serde_roundtrip() {
        let original = AssetResponse::builder()
            .id(1)
            .required(Some(1))
            .title(Some(
                TitleResponse::builder()
                    .text("Great Product".to_string())
                    .len(Some(13))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: AssetResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_asset_response_with_img_serde_roundtrip() {
        let original = AssetResponse::builder()
            .id(2)
            .img(Some(
                ImageResponse::builder()
                    .url("https://cdn.example.com/banner.png".to_string())
                    .w(Some(728))
                    .h(Some(90))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: AssetResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_asset_response_with_video_serde_roundtrip() {
        let original = AssetResponse::builder()
            .id(3)
            .video(Some(
                VideoResponse::builder()
                    .vasttag("<VAST version=\"3.0\"><Ad><InLine></InLine></Ad></VAST>".to_string())
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: AssetResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_asset_response_with_data_serde_roundtrip() {
        let original = AssetResponse::builder()
            .id(4)
            .data(Some(
                DataResponse::builder()
                    .value("Acme Corp".to_string())
                    .type_(Some(1))
                    .label(Some("Sponsored".to_string()))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: AssetResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_native_response_full_serde_roundtrip() {
        let original = NativeResponse::builder()
            .ver("1.2")
            .assets(vec![
                AssetResponse::builder()
                    .id(1)
                    .required(Some(1))
                    .title(Some(
                        TitleResponse::builder()
                            .text("Summer Sale".to_string())
                            .len(Some(11))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                AssetResponse::builder()
                    .id(2)
                    .img(Some(
                        ImageResponse::builder()
                            .url("https://cdn.example.com/sale.jpg".to_string())
                            .w(Some(1200))
                            .h(Some(627))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .link(
                Link::builder()
                    .url("https://example.com/sale".to_string())
                    .clicktrackers(Some(vec!["https://click.tracker.com/c".to_string()]))
                    .fallback(Some("https://example.com".to_string()))
                    .build()
                    .unwrap(),
            )
            .imptrackers(Some(vec!["https://imp.tracker.com/i".to_string()]))
            .jstracker(Some("<script>track();</script>".to_string()))
            .eventtrackers(Some(vec![
                EventTrackerResponse::builder()
                    .event(1)
                    .method(1)
                    .url("https://evt.tracker.com/imp")
                    .customdata(Some("cd=123".to_string()))
                    .build()
                    .unwrap(),
            ]))
            .privacy(Some("https://example.com/privacy".to_string()))
            .assetsurl(Some("https://example.com/assets.json".to_string()))
            .dcourl(Some("https://dco.example.com/creative".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NativeResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    // === Full Realistic JSON Deserialization ===

    #[test]
    fn test_realistic_dsp_native_response_deserialization() {
        let json = r#"{
            "ver": "1.2",
            "assets": [
                {
                    "id": 1,
                    "required": 1,
                    "title": {
                        "text": "Summer Collection - 50% Off Everything",
                        "len": 40
                    }
                },
                {
                    "id": 2,
                    "required": 1,
                    "img": {
                        "url": "https://cdn.advertiser.com/campaigns/summer2026/hero-1200x627.jpg",
                        "w": 1200,
                        "h": 627
                    }
                },
                {
                    "id": 3,
                    "img": {
                        "url": "https://cdn.advertiser.com/brand/logo-128x128.png",
                        "w": 128,
                        "h": 128
                    }
                },
                {
                    "id": 4,
                    "required": 1,
                    "data": {
                        "value": "Shop the hottest summer styles at unbeatable prices. Free shipping on orders over $50.",
                        "type": 2,
                        "label": "Description",
                        "len": 85
                    }
                },
                {
                    "id": 5,
                    "data": {
                        "value": "FashionBrand",
                        "type": 1,
                        "label": "Sponsored"
                    }
                },
                {
                    "id": 6,
                    "data": {
                        "value": "Shop Now",
                        "type": 12
                    }
                },
                {
                    "id": 7,
                    "data": {
                        "value": "4.8",
                        "type": 3
                    }
                }
            ],
            "link": {
                "url": "https://www.fashionbrand.com/summer-sale?utm_source=native&utm_medium=display&utm_campaign=summer2026",
                "clicktrackers": [
                    "https://tracking.dsp.com/click?cid=camp-789&creative=cr-456&ts=${AUCTION_PRICE}",
                    "https://thirdparty.measurement.com/click?ref=dsp123"
                ],
                "fallback": "https://www.fashionbrand.com/"
            },
            "eventtrackers": [
                {
                    "event": 1,
                    "method": 1,
                    "url": "https://tracking.dsp.com/imp?cid=camp-789&creative=cr-456&price=${AUCTION_PRICE}"
                },
                {
                    "event": 2,
                    "method": 1,
                    "url": "https://viewability.measurement.com/view?ref=dsp123"
                },
                {
                    "event": 1,
                    "method": 2,
                    "url": "https://tracking.dsp.com/js/tracker.js",
                    "customdata": "{\"campaignId\":\"camp-789\",\"creativeId\":\"cr-456\"}"
                }
            ],
            "privacy": "https://www.fashionbrand.com/privacy-policy"
        }"#;

        let response: NativeResponse = serde_json::from_str(json).unwrap();

        // Verify top-level fields
        assert_eq!(response.ver, Some("1.2".to_string()));
        assert_eq!(response.assets.len(), 7);
        assert_eq!(
            response.privacy,
            Some("https://www.fashionbrand.com/privacy-policy".to_string())
        );

        // Verify title asset
        let title_asset = &response.assets[0];
        assert_eq!(title_asset.id, 1);
        assert_eq!(title_asset.required, Some(1));
        let title = title_asset.title.as_ref().unwrap();
        assert_eq!(title.text, "Summer Collection - 50% Off Everything");
        assert_eq!(title.len, Some(40));

        // Verify main image asset
        let img_asset = &response.assets[1];
        assert_eq!(img_asset.id, 2);
        let img = img_asset.img.as_ref().unwrap();
        assert_eq!(img.w, Some(1200));
        assert_eq!(img.h, Some(627));

        // Verify data asset with type and label
        let data_asset = &response.assets[3];
        assert_eq!(data_asset.id, 4);
        let data = data_asset.data.as_ref().unwrap();
        assert_eq!(data.type_, Some(2));
        assert_eq!(data.label, Some("Description".to_string()));
        assert_eq!(data.len, Some(85));

        // Verify CTA data asset
        let cta_asset = &response.assets[5];
        let cta_data = cta_asset.data.as_ref().unwrap();
        assert_eq!(cta_data.value, "Shop Now");
        assert_eq!(cta_data.type_, Some(12));

        // Verify link
        assert!(response.link.url.contains("utm_source=native"));
        let trackers = response.link.clicktrackers.as_ref().unwrap();
        assert_eq!(trackers.len(), 2);
        assert_eq!(
            response.link.fallback,
            Some("https://www.fashionbrand.com/".to_string())
        );

        // Verify event trackers
        let evt = response.eventtrackers.as_ref().unwrap();
        assert_eq!(evt.len(), 3);
        assert_eq!(evt[0].event, 1);
        assert_eq!(evt[0].method, 1);
        assert_eq!(evt[1].event, 2); // Viewable impression
        assert_eq!(evt[2].method, 2); // JavaScript tracker
        assert!(evt[2].customdata.is_some());
    }

    // === Edge Case Tests ===

    #[test]
    fn test_event_tracker_customdata_serde_roundtrip() {
        let original = EventTrackerResponse::builder()
            .event(3)
            .method(2)
            .url("https://tracker.com/click")
            .customdata(Some(
                "{\"macro\":\"%%CLICK_ID%%\",\"ts\":1234567890}".to_string(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: EventTrackerResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.customdata, original.customdata);
    }

    #[test]
    fn test_data_response_label_serde_roundtrip() {
        let original = DataResponse::builder()
            .value("Acme Corp".to_string())
            .label(Some("Sponsored By".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: DataResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.label, original.label);
    }

    #[test]
    fn test_data_response_type_field_json_roundtrip() {
        // Verify "type" in JSON maps to type_ in Rust and back
        let json = r#"{"value":"test","type":5}"#;
        let parsed: DataResponse = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.type_, Some(5));

        let serialized = serde_json::to_string(&parsed).unwrap();
        assert!(serialized.contains(r#""type":5"#));
        assert!(!serialized.contains("type_"));

        let re_parsed: DataResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(re_parsed.type_, Some(5));
    }

    #[test]
    fn test_title_response_len_serde_roundtrip() {
        let original = TitleResponse::builder()
            .text("Short".to_string())
            .len(Some(5))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: TitleResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len, Some(5));
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_link_fallback_serde_roundtrip() {
        let original = Link::builder()
            .url("myapp://deep/link/path".to_string())
            .fallback(Some("https://example.com/web/fallback".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Link = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.fallback, original.fallback);
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_link_with_empty_clicktrackers_array() {
        let link = Link::builder()
            .url("https://example.com".to_string())
            .clicktrackers(Some(vec![]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains(r#""clicktrackers":[]"#));

        let parsed: Link = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.clicktrackers, Some(vec![]));
    }

    #[test]
    fn test_asset_response_with_video_type() {
        let asset = AssetResponse::builder()
            .id(10)
            .required(Some(1))
            .video(Some(
                VideoResponse::builder()
                    .vasttag(
                        "<VAST version=\"4.0\"><Ad><Wrapper></Wrapper></Ad></VAST>".to_string(),
                    )
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(asset.id, 10);
        assert!(asset.video.is_some());
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.data.is_none());

        let json = serde_json::to_string(&asset).unwrap();
        let parsed: AssetResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, asset);
    }

    #[test]
    fn test_native_response_deprecated_and_new_tracking() {
        // Test response with both deprecated (imptrackers/jstracker) AND new (eventtrackers)
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .imptrackers(Some(vec![
                "https://legacy.tracker.com/imp1".to_string(),
                "https://legacy.tracker.com/imp2".to_string(),
            ]))
            .jstracker(Some("<script>legacyTrack();</script>".to_string()))
            .eventtrackers(Some(vec![
                EventTrackerResponse::builder()
                    .event(1)
                    .method(1)
                    .url("https://modern.tracker.com/imp")
                    .build()
                    .unwrap(),
                EventTrackerResponse::builder()
                    .event(2)
                    .method(2)
                    .url("https://modern.tracker.com/viewable.js")
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        // Both deprecated and modern tracking present
        assert!(response.imptrackers.is_some());
        assert_eq!(response.imptrackers.as_ref().unwrap().len(), 2);
        assert!(response.jstracker.is_some());
        assert!(response.eventtrackers.is_some());
        assert_eq!(response.eventtrackers.as_ref().unwrap().len(), 2);

        // Serde roundtrip preserves both
        let json = serde_json::to_string(&response).unwrap();
        let parsed: NativeResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, response);
    }

    #[test]
    fn test_empty_string_required_fields() {
        // url, text, vasttag, value are "required" but empty strings are valid in Rust
        let title = TitleResponse::builder()
            .text("".to_string())
            .build()
            .unwrap();
        assert_eq!(title.text, "");

        let image = ImageResponse::builder()
            .url("".to_string())
            .build()
            .unwrap();
        assert_eq!(image.url, "");

        let video = VideoResponse::builder()
            .vasttag("".to_string())
            .build()
            .unwrap();
        assert_eq!(video.vasttag, "");

        let data = DataResponse::builder()
            .value("".to_string())
            .build()
            .unwrap();
        assert_eq!(data.value, "");

        let link = Link::builder().url("".to_string()).build().unwrap();
        assert_eq!(link.url, "");
    }

    #[test]
    fn test_unicode_in_title_response_text() {
        let title = TitleResponse::builder()
            .text("🔥 Soldes d'été — Économisez 50% 🎉".to_string())
            .build()
            .unwrap();

        assert_eq!(title.text, "🔥 Soldes d'été — Économisez 50% 🎉");

        // Verify serde roundtrip preserves unicode
        let json = serde_json::to_string(&title).unwrap();
        let parsed: TitleResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.text, title.text);
    }
}
