//! OpenRTB Native Ads 1.2 Specification
//!
//! This module implements the complete OpenRTB Native Ads 1.2 specification
//! for trading native advertising formats across platforms.
//!
//! # Overview
//!
//! Native advertising allows ads to match the form and function of the platform
//! on which they appear. OpenRTB Native 1.2 defines standardized request and
//! response objects for trading these ad formats programmatically.
//!
//! # Key Features
//!
//! - **Asset-Based Composition**: Build native ads from individual assets (title, image, data)
//! - **Event Tracking**: Comprehensive event tracking with impression, viewability, and click support
//! - **Multi-Placement**: Support for multiple identical placements in feeds
//! - **DCO Support**: Dynamic Creative Optimization via URL-based asset delivery
//! - **AdCOM Integration**: Uses existing AdCOM enumerations for consistency
//!
//! # Architecture
//!
//! The module is organized into two main components:
//!
//! - [`request`] - Request-side objects (NativeRequest, Asset, Title, Image, Video, Data, EventTracker)
//! - [`response`] - Response-side objects (NativeResponse, AssetResponse, Link, EventTrackerResponse, etc.)
//!
//! # Quick Start
//!
//! ## Creating a Native Ad Request
//!
//! ```rust
//! use iab_specs::openrtb::native::v12::{NativeRequest, Asset, Title, Image, Data};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let request = NativeRequest::builder()
//!     .ver("1.2")
//!     .context(Some(1))      // Content-centric
//!     .plcmttype(Some(1))    // In-feed
//!     .assets(vec![
//!         // Title asset
//!         Asset::builder()
//!             .id(1)
//!             .required(Some(1))
//!             .title(Some(Title::builder()
//!                 .len(90)
//!                 .build()?))
//!             .build()?,
//!         // Main image asset
//!         Asset::builder()
//!             .id(2)
//!             .img(Some(Image::builder()
//!                 .type_(Some(3))   // Main image
//!                 .w(Some(1200))
//!                 .h(Some(627))
//!                 .build()?))
//!             .build()?,
//!         // Description data asset
//!         Asset::builder()
//!             .id(3)
//!             .data(Some(Data::builder()
//!                 .type_(2)         // Description
//!                 .len(Some(140))
//!                 .build()?))
//!             .build()?,
//!     ])
//!     .build()?;
//!
//! // Serialize to JSON for embedding in OpenRTB request
//! let json = serde_json::to_string(&request)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Parsing a Native Ad Response
//!
//! ```rust
//! use iab_specs::openrtb::native::v12::{
//!     NativeResponse, AssetResponse, Link, TitleResponse, ImageResponse, DataResponse
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let response = NativeResponse::builder()
//!     .ver("1.2")
//!     .assets(vec![
//!         // Title response
//!         AssetResponse::builder()
//!             .id(1)
//!             .title(Some(TitleResponse::builder()
//!                 .text("Amazing Product - Limited Offer!".to_string())
//!                 .build()?))
//!             .build()?,
//!         // Image response
//!         AssetResponse::builder()
//!             .id(2)
//!             .img(Some(ImageResponse::builder()
//!                 .url("https://cdn.example.com/product.jpg".to_string())
//!                 .w(Some(1200))
//!                 .h(Some(627))
//!                 .build()?))
//!             .build()?,
//!         // Description response
//!         AssetResponse::builder()
//!             .id(3)
//!             .data(Some(DataResponse::builder()
//!                 .value("High-quality product with excellent reviews".to_string())
//!                 .build()?))
//!             .build()?,
//!     ])
//!     .link(Link::builder()
//!         .url("https://example.com/product?utm_source=native".to_string())
//!         .clicktrackers(Some(vec![
//!             "https://tracker1.com/click".to_string(),
//!         ]))
//!         .build()?)
//!     .build()?;
//!
//! // Access assets
//! for asset in &response.assets {
//!     if let Some(title) = &asset.title {
//!         println!("Title: {}", title.text);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Integration with OpenRTB 2.5
//!
//! ```rust,ignore
//! use iab_specs::openrtb::v25::{BidRequest, Imp, Native};
//! use iab_specs::openrtb::native::v12::NativeRequest;
//!
//! // Create native request
//! let native_req = NativeRequest::builder()
//!     .ver(Some("1.2".to_string()))
//!     .assets(/* ... */)
//!     .build()?;
//!
//! // Serialize to JSON string
//! let native_json = serde_json::to_string(&native_req)?;
//!
//! // Embed in OpenRTB bid request
//! let bid_request = BidRequest::builder()
//!     .id("req-123".to_string())
//!     .imp(vec![
//!         Imp::builder()
//!             .id("imp1".to_string())
//!             .native(Some(Native::builder()
//!                 .request(native_json)
//!                 .ver(Some("1.2".to_string()))
//!                 .build()?))
//!             .build()?
//!     ])
//!     .build()?;
//! ```
//!
//! # Important Constraints and Validation
//!
//! ## Asset Mutual Exclusivity
//!
//! Each [`Asset`] object **must contain exactly one** of the following:
//! - [`title`](Asset::title) - Title asset specification
//! - [`img`](Asset::img) - Image asset specification
//! - [`video`](Asset::video) - Video asset specification
//! - [`data`](Asset::data) - Data asset specification
//!
//! Including multiple asset types in a single Asset object violates the specification.
//!
//! ## Required Fields
//!
//! The following fields are **required** by the specification:
//!
//! ### Request Objects
//! - [`NativeRequest::assets`] - Must contain at least one asset
//! - [`Asset::id`] - Unique identifier within request
//! - [`Title::len`] - Maximum title length
//! - [`Video::mimes`] - Supported MIME types
//! - [`Data::type_`] - Data asset type
//! - [`EventTracker::event`] - Event type
//! - [`EventTracker::methods`] - Tracking methods
//!
//! ### Response Objects
//! - [`NativeResponse::link`] - Default click destination
//! - [`AssetResponse::id`] - Maps to request asset ID
//! - [`TitleResponse::text`] - Title text content
//! - [`ImageResponse::url`] - Image URL
//! - [`VideoResponse::vasttag`] - VAST XML markup
//! - [`DataResponse::value`] - Data value content
//! - [`Link::url`] - Landing URL
//! - [`EventTrackerResponse::event`] - Event type
//! - [`EventTrackerResponse::method`] - Tracking method
//!
//! ## Deprecated Fields
//!
//! The following fields are **deprecated** as of OpenRTB Native 1.2:
//!
//! - [`NativeResponse::imptrackers`] - Use [`eventtrackers`](NativeResponse::eventtrackers) instead
//! - [`NativeResponse::jstracker`] - Use [`eventtrackers`](NativeResponse::eventtrackers) instead
//!
//! Both deprecated and new tracking methods are supported for backward compatibility.
//!
//! ## Recommended Fields
//!
//! The specification **recommends** including:
//! - [`NativeRequest::context`] - Ad placement context
//! - [`NativeRequest::plcmttype`] - Placement type
//! - [`NativeRequest::privacy`] - Privacy/AdChoices support flag
//! - [`Image::type_`] - Image asset type
//!
//! # Extension Support
//!
//! All objects support custom extensions via the generic `Ext` parameter.
//! By default, extensions use `serde_json::Value` for flexible, untyped extensions:
//!
//! ```rust
//! use iab_specs::openrtb::native::v12::NativeRequest;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let request = NativeRequest::builder()
//!     .ver("1.2")
//!     .assets(vec![])
//!     .ext(Some(Box::new(serde_json::json!({
//!         "custom_field": "custom_value",
//!         "vendor_id": 12345
//!     }))))
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Specification Reference
//!
//! This implementation follows the [OpenRTB Native Ads 1.2 specification](https://github.com/InteractiveAdvertisingBureau/Native-Ads/blob/main/OpenRTB-Native-Ads-Specification-Final-1.2.md)
//! published by the IAB in March 2017.
//!
//! # AdCOM Enumerations
//!
//! This module uses enumerations from the AdCOM specification:
//!
//! - [`DisplayContextType`](crate::adcom::enums::DisplayContextType) - Context types (content, social, product)
//! - [`DisplayPlacementType`](crate::adcom::enums::DisplayPlacementType) - Placement types (feed, sidebar, etc.)
//! - [`NativeDataAssetType`](crate::adcom::enums::NativeDataAssetType) - Data asset types
//! - [`NativeImageAssetType`](crate::adcom::enums::NativeImageAssetType) - Image asset types
//! - [`EventType`](crate::adcom::enums::EventType) - Event types for tracking
//! - [`EventTrackingMethod`](crate::adcom::enums::EventTrackingMethod) - Tracking methods

pub mod request;
pub mod response;

// Re-export request objects
pub use request::{
    Asset, AssetBuilder, Data, DataBuilder, EventTracker, EventTrackerBuilder, Image, ImageBuilder,
    NativeRequest, NativeRequestBuilder, Title, TitleBuilder, Video, VideoBuilder,
};

// Re-export response objects
pub use response::{
    AssetResponse, AssetResponseBuilder, DataResponse, DataResponseBuilder, EventTrackerResponse,
    EventTrackerResponseBuilder, ImageResponse, ImageResponseBuilder, Link, LinkBuilder,
    NativeResponse, NativeResponseBuilder, TitleResponse, TitleResponseBuilder, VideoResponse,
    VideoResponseBuilder,
};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_request_response_cycle() {
        // Create a complete native ad request
        let request = NativeRequest::builder()
            .ver("1.2")
            .context(Some(1)) // Content-centric
            .plcmttype(Some(1)) // In-feed
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
                            .type_(Some(3)) // Main image
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
                        Data::builder()
                            .type_(2) // Description
                            .len(Some(140))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .eventtrackers(Some(vec![
                EventTracker::builder()
                    .event(1) // Impression
                    .methods(vec![1, 2]) // Image pixel and JavaScript
                    .build()
                    .unwrap(),
            ]))
            .privacy(Some(1))
            .build()
            .unwrap();

        // Serialize request
        let request_json = serde_json::to_string(&request).unwrap();
        assert!(request_json.contains("\"ver\":\"1.2\""));
        assert!(request_json.contains("\"assets\""));

        // Create matching response
        let response = NativeResponse::builder()
            .ver("1.2")
            .assets(vec![
                AssetResponse::builder()
                    .id(1)
                    .required(Some(1))
                    .title(Some(
                        TitleResponse::builder()
                            .text("Amazing Product - Buy Now!".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                AssetResponse::builder()
                    .id(2)
                    .img(Some(
                        ImageResponse::builder()
                            .url("https://cdn.example.com/product.jpg".to_string())
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
                            .value("High-quality product with great reviews".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .link(
                Link::builder()
                    .url("https://example.com/product".to_string())
                    .clicktrackers(Some(vec!["https://tracker.com/click".to_string()]))
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

        // Serialize response
        let response_json = serde_json::to_string(&response).unwrap();
        assert!(response_json.contains("\"ver\":\"1.2\""));
        assert!(response_json.contains("\"link\""));

        // Verify response matches request
        assert_eq!(request.assets.len(), response.assets.len());
        for (req_asset, resp_asset) in request.assets.iter().zip(response.assets.iter()) {
            assert_eq!(req_asset.id, resp_asset.id);
        }
    }

    #[test]
    fn test_multi_placement_request() {
        let request = NativeRequest::builder()
            .ver("1.2")
            .context(Some(1))
            .plcmttype(Some(1))
            .plcmtcnt(Some(3)) // Multiple placements
            .seq(Some(0))
            .assets(vec![
                Asset::builder()
                    .id(1)
                    .title(Some(Title::builder().len(90).build().unwrap()))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(request.plcmtcnt, Some(3));

        let json = serde_json::to_string(&request).unwrap();
        let parsed: NativeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.plcmtcnt, Some(3));
    }

    #[test]
    fn test_dco_url_support() {
        let request = NativeRequest::builder()
            .ver("1.2")
            .assets(vec![])
            .aurlsupport(Some(1))
            .durlsupport(Some(1))
            .build()
            .unwrap();

        assert_eq!(request.aurlsupport, Some(1));
        assert_eq!(request.durlsupport, Some(1));

        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .assetsurl(Some("https://cdn.example.com/assets.json".to_string()))
            .dcourl(Some("https://dco.example.com/creative".to_string()))
            .build()
            .unwrap();

        assert!(response.assetsurl.is_some());
        assert!(response.dcourl.is_some());
    }

    #[test]
    fn test_deprecated_tracking_compatibility() {
        // Test that deprecated imptrackers/jstracker still work
        let response = NativeResponse::builder()
            .assets(vec![])
            .link(
                Link::builder()
                    .url("https://example.com".to_string())
                    .build()
                    .unwrap(),
            )
            .imptrackers(Some(vec![
                "https://imp1.com".to_string(),
                "https://imp2.com".to_string(),
            ]))
            .jstracker(Some("<script>console.log('tracked');</script>".to_string()))
            .build()
            .unwrap();

        assert!(response.imptrackers.is_some());
        assert_eq!(response.imptrackers.as_ref().unwrap().len(), 2);
        assert!(response.jstracker.is_some());

        // Verify serialization
        let json = serde_json::to_string(&response).unwrap();
        let parsed: NativeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.imptrackers, response.imptrackers);
        assert_eq!(parsed.jstracker, response.jstracker);
    }

    #[test]
    fn test_video_asset_request_response() {
        let request = NativeRequest::builder()
            .ver("1.2")
            .assets(vec![
                Asset::builder()
                    .id(1)
                    .video(Some(
                        Video::builder()
                            .mimes(vec!["video/mp4".to_string(), "video/webm".to_string()])
                            .minduration(Some(5))
                            .maxduration(Some(30))
                            .protocols(Some(vec![2, 3, 5, 6]))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let video = request.assets[0].video.as_ref().unwrap();
        assert_eq!(video.mimes.len(), 2);
        assert_eq!(video.minduration, Some(5));
        assert_eq!(video.maxduration, Some(30));

        let response = NativeResponse::builder()
            .assets(vec![
                AssetResponse::builder()
                    .id(1)
                    .video(Some(
                        VideoResponse::builder()
                            .vasttag("<VAST version=\"3.0\">...</VAST>".to_string())
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

        let vast = &response.assets[0].video.as_ref().unwrap().vasttag;
        assert!(vast.contains("VAST"));
    }

    #[test]
    fn test_all_data_asset_types() {
        let data_types = vec![
            (1, "Sponsored"),
            (2, "Description"),
            (3, "Rating"),
            (4, "Likes"),
            (5, "Downloads"),
            (6, "Price"),
            (7, "SalePrice"),
            (8, "Phone"),
            (9, "Address"),
            (10, "Description2"),
            (11, "DisplayUrl"),
            (12, "CallToAction"),
        ];

        for (type_id, _name) in data_types {
            let asset = Asset::builder()
                .id(type_id)
                .data(Some(
                    Data::builder()
                        .type_(type_id)
                        .len(Some(100))
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap();

            assert_eq!(asset.data.as_ref().unwrap().type_, type_id);
        }
    }

    #[test]
    fn test_all_image_asset_types() {
        let image_types = vec![(1, "Icon"), (2, "Logo"), (3, "Main")];

        for (type_id, _name) in image_types {
            let asset = Asset::builder()
                .id(type_id)
                .img(Some(
                    Image::builder()
                        .type_(Some(type_id))
                        .w(Some(300))
                        .h(Some(250))
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap();

            assert_eq!(asset.img.as_ref().unwrap().type_, Some(type_id));
        }
    }

    #[test]
    fn test_asset_specific_link() {
        let response = NativeResponse::builder()
            .assets(vec![
                AssetResponse::builder()
                    .id(1)
                    .title(Some(
                        TitleResponse::builder()
                            .text("Title".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .link(Some(
                        Link::builder()
                            .url("https://specific-asset-link.com".to_string())
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .link(
                Link::builder()
                    .url("https://default-link.com".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        // Verify asset-specific link overrides default
        let asset_link = response.assets[0].link.as_ref().unwrap();
        assert_eq!(asset_link.url, "https://specific-asset-link.com");
        assert_eq!(response.link.url, "https://default-link.com");
    }

    #[test]
    fn test_roundtrip_with_extensions() {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
        struct CustomExt {
            custom_field: String,
        }

        let request = NativeRequestBuilder::default()
            .ver("1.2")
            .assets(vec![])
            .ext(Some(Box::new(CustomExt {
                custom_field: "custom_value".to_string(),
            })))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: NativeRequest<CustomExt> = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ext.as_ref().unwrap().custom_field, "custom_value");
    }
}
