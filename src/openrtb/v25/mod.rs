/// OpenRTB 2.5 Protocol Implementation
///
/// This module implements the OpenRTB 2.5 specification as defined by the IAB.
///
/// OpenRTB 2.5 is the foundation protocol for real-time bidding in digital advertising,
/// defining the communication standard between supply-side platforms (SSPs) and
/// demand-side platforms (DSPs).
///
/// ## AdCOM Integration
///
/// OpenRTB 2.5 uses AdCOM (Advertising Common Object Model) for common domain objects.
/// All AdCOM types (enumerations, etc.) are available through this module:
///
/// ```
/// use iab_specs::openrtb::v25::{AuctionType, DeviceType};
/// ```
///
/// ## Reference
///
/// OpenRTB 2.5 Specification:
/// <https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf>
///
/// ## Implementation Status
///
/// This module is under development. Core objects will be added in subsequent phases.
// Core bid objects (Phase 2, Commit 3)
pub mod bid;
pub mod request;
pub mod response;

// Impression and media objects (Phase 2, Commit 4)
pub mod audio;
pub mod banner;
pub mod imp;
pub mod native;
pub mod video;

// Context objects (Phase 2, Commit 5)
pub mod app;
pub mod content;
pub mod data;
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
pub use bid::{Bid, SeatBid};
pub use request::BidRequest;
pub use response::BidResponse;

// Re-export impression and media types
pub use audio::Audio;
pub use banner::{Banner, Format};
pub use imp::Imp;
pub use native::Native;
pub use video::Video;

// Re-export context types
pub use app::App;
pub use content::Content;
pub use data::{Data, Segment};
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

// Re-export common types (includes AdCOM and SupplyChain)
pub use crate::openrtb::common::*;
