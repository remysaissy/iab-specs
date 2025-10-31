/// OpenRTB 2.6 Protocol Implementation
///
/// This module implements the OpenRTB 2.6 specification as defined by the IAB.
///
/// OpenRTB 2.6 extends version 2.5 with support for:
/// - Ad Pods for CTV transactions
/// - Structured User-Agent object
/// - Enhanced privacy controls (GDPR, US Privacy, GPP)
/// - Network and Channel objects for TV content
/// - DOOH (Digital Out-Of-Home) inventory support
///
/// ## AdCOM Integration
///
/// Like OpenRTB 2.5, version 2.6 uses AdCOM (Advertising Common Object Model) for
/// common domain objects. All AdCOM and OpenRTB 2.5 types are available:
///
/// ```
/// use iab_specs::openrtb::v26::{AuctionType, DeviceType};
/// ```
///
/// ## Reference
///
/// OpenRTB 2.6 Specification:
/// <https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md>
///
/// ## Implementation Status
///
/// This module is under development. Extensions to OpenRTB 2.5 will be added in Phase 4.

// Re-export common types (includes AdCOM)
pub use crate::openrtb::common::*;

// Re-export OpenRTB 2.5 types (2.6 extends 2.5)
pub use crate::openrtb::v25::*;
