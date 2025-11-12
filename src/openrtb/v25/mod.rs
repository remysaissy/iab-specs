/// OpenRTB 2.5 Protocol Implementation
///
/// This module implements the complete OpenRTB 2.5 specification as defined by the IAB.
///
/// OpenRTB 2.5 is the foundation protocol for real-time bidding in digital advertising,
/// defining the communication standard between supply-side platforms (SSPs) and
/// demand-side platforms (DSPs).
///
/// ## Example: Creating a Bid Request
///
/// ```rust
/// use iab_specs::openrtb::v25::{BidRequest, Imp, Banner};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let request = BidRequest::builder()
///     .id("bid-request-123".to_string())
///     .imp(vec![
///         Imp::builder()
///             .id("imp1".to_string())
///             .banner(Some(Banner::builder()
///                 .w(Some(728))
///                 .h(Some(90))
///                 .build()
///                 .unwrap()
///             ))
///             .bidfloor(1.50)
///             .bidfloorcur("USD".to_string())
///             .build()
///             .unwrap()
///     ])
///     .tmax(Some(100)) // 100ms timeout
///     .build()
///     .unwrap();
///
/// // Serialize to JSON
/// let json = serde_json::to_string(&request)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Example: Parsing a Bid Response
///
/// ```rust
/// use iab_specs::openrtb::v25::BidResponse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let json = r#"{
///     "id": "bid-request-123",
///     "bidid": "bid-789",
///     "seatbid": [{
///         "bid": [{
///             "id": "1",
///             "impid": "imp1",
///             "price": 2.50,
///             "adid": "ad-456",
///             "adomain": ["advertiser.com"],
///             "crid": "creative-123"
///         }]
///     }]
/// }"#;
///
/// let response: BidResponse = serde_json::from_str(json)?;
/// assert_eq!(response.id, "bid-request-123");
/// # Ok(())
/// # }
/// ```
///
/// ## AdCOM Integration
///
/// OpenRTB 2.5 uses AdCOM (Advertising Common Object Model) for common domain objects.
///
/// ```
/// #[cfg(feature = "adcom")]
/// {
/// // Import from AdCOM:
/// use iab_specs::adcom::enums::{AuctionType, DeviceType};
/// # let _ = AuctionType::FirstPrice;
/// }
/// ```
///
/// ## Reference
///
/// OpenRTB 2.5 Specification:
/// <https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf>
// Core bid objects (Phase 2, Commit 3)
pub mod bid;
pub mod request;
pub mod response;
pub mod seat_bid;

// Impression and media objects (Phase 2, Commit 4)
pub mod audio;
pub mod banner;
pub mod format;
pub mod imp;
pub mod native;
pub mod video;

// Context objects (Phase 2, Commit 5)
pub mod app;
pub mod content;
pub mod producer;
pub mod publisher;
pub mod site;

// User and device objects (Phase 2, Commit 6)
pub mod device;
pub mod geo;
pub mod user;

// Regulatory and source objects (Phase 2, Commit 7)
pub mod regs;
pub mod source;

// Re-export core bid types for convenient access
pub use bid::Bid;
pub use request::BidRequest;
pub use response::BidResponse;
pub use seat_bid::SeatBid;

// Re-export impression and media types
pub use audio::Audio;
pub use banner::Banner;
pub use format::Format;
pub use imp::Imp;
pub use native::Native;
pub use video::Video;

// Re-export context types
pub use crate::adcom::context::Data;
pub use crate::adcom::context::Segment;
pub use app::App;
pub use content::Content;
pub use producer::Producer;
pub use publisher::Publisher;
pub use site::Site;

// Re-export user and device types
pub use device::Device;
pub use geo::Geo;
pub use user::User;

// Re-export regulatory and source types
pub use regs::Regs;
pub use source::Source;
