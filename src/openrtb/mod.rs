/// OpenRTB (Real-Time Bidding) Protocol Implementation
///
/// This module implements the OpenRTB protocol versions 2.5, 2.6, and 3.0 as specified by the
/// Interactive Advertising Bureau (IAB).
///
/// OpenRTB is a protocol for real-time bidding in digital advertising that facilitates
/// communication between supply-side platforms (SSPs) and demand-side platforms (DSPs).
///
/// ## Features
///
/// - `openrtb_25`: Enables OpenRTB 2.5 support (automatically includes `adcom`)
/// - `openrtb_26`: Enables OpenRTB 2.6 support (includes 2.5 and `adcom`)
/// - `openrtb_30`: Enables OpenRTB 3.0 support (automatically includes `adcom`)
///
/// ## Module Organization
///
/// - `common`: OpenRTB-specific common types
/// - `v25`: OpenRTB 2.5 specific types (enabled with `openrtb_25` feature)
/// - `v26`: OpenRTB 2.6 specific types (enabled with `openrtb_26` feature)
/// - `v30`: OpenRTB 3.0 specific types (enabled with `openrtb_30` feature)
///
/// ## AdCOM Integration
///
/// OpenRTB 2.5+ uses AdCOM (Advertising Common Object Model) for domain objects like
/// enumerations, media types, and context objects. All AdCOM types are accessible via the `adcom` crate module.
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
/// ## References
///
/// - OpenRTB 2.5: <https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf>
/// - OpenRTB 2.6: <https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md>
/// - OpenRTB 3.0: <https://github.com/InteractiveAdvertisingBureau/openrtb/blob/main/OpenRTB%20v3.0%20FINAL.md>
/// - AdCOM 1.0: <https://github.com/InteractiveAdvertisingBureau/AdCOM>
pub mod common;

#[cfg(feature = "openrtb_25")]
pub mod v25;

#[cfg(feature = "openrtb_26")]
pub mod v26;

#[cfg(feature = "openrtb_30")]
pub mod v30;

// Re-export common types for convenience
pub use common::*;
