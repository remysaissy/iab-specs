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

// Re-export common types (includes AdCOM)
pub use crate::openrtb::common::*;
