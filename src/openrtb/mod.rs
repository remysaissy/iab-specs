/// OpenRTB (Real-Time Bidding) Protocol Implementation
///
/// This module implements the OpenRTB protocol versions 2.5 and 2.6 as specified by the
/// Interactive Advertising Bureau (IAB).
///
/// OpenRTB is a protocol for real-time bidding in digital advertising that facilitates
/// communication between supply-side platforms (SSPs) and demand-side platforms (DSPs).
///
/// ## Features
///
/// - `openrtb_25`: Enables OpenRTB 2.5 support (automatically includes `adcom`)
/// - `openrtb_26`: Enables OpenRTB 2.6 support (includes 2.5 and `adcom`)
///
/// ## Module Organization
///
/// - `common`: OpenRTB-specific common types and AdCOM re-exports
/// - `v25`: OpenRTB 2.5 specific types (enabled with `openrtb_25` feature)
/// - `v26`: OpenRTB 2.6 specific types (enabled with `openrtb_26` feature)
///
/// ## AdCOM Integration
///
/// OpenRTB 2.5+ uses AdCOM (Advertising Common Object Model) for domain objects like
/// enumerations, media types, and context objects. All AdCOM types are accessible via
/// the `common` module or directly from the `adcom` crate module.
///
/// ```
/// // Import from OpenRTB common:
/// use iab_specs::openrtb::common::{AuctionType, DeviceType};
/// # let _ = AuctionType::FirstPrice;
/// ```
///
/// Or import directly from AdCOM:
/// ```
/// // Import from AdCOM:
/// use iab_specs::adcom::{AuctionType, DeviceType};
/// # let _ = AuctionType::FirstPrice;
/// ```
///
/// ## References
///
/// - OpenRTB 2.5: <https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf>
/// - OpenRTB 2.6: <https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md>
/// - AdCOM 1.0: <https://github.com/InteractiveAdvertisingBureau/AdCOM>
pub mod common;

#[cfg(feature = "openrtb_25")]
pub mod v25;

#[cfg(feature = "openrtb_26")]
pub mod v26;

// Re-export common types for convenience
pub use common::*;
