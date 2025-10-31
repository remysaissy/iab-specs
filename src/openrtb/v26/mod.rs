// Allow ambiguous glob re-exports since v26 intentionally extends v25
#![allow(ambiguous_glob_reexports)]

/// OpenRTB 2.6 Protocol Implementation
///
/// This module implements the OpenRTB 2.6 specification as defined by the IAB.
///
/// OpenRTB 2.6 extends version 2.5 with support for:
/// - Ad Pods for CTV transactions (podid, podseq, slotinpod)
/// - Structured User-Agent object (available via AdCOM)
/// - Enhanced privacy controls (GDPR, US Privacy, GPP)
/// - Network and Channel objects for TV content (available via AdCOM)
/// - DOOH (Digital Out-Of-Home) inventory support (available via AdCOM)
/// - Qty object for DOOH multipliers
/// - Refresh settings for rotating ad slots
/// - Duration-based floor pricing (DurFloors)
///
/// ## AdCOM Integration
///
/// Like OpenRTB 2.5, version 2.6 uses AdCOM (Advertising Common Object Model) for
/// common domain objects. All AdCOM and OpenRTB 2.5 types are available:
///
/// ```
/// use iab_specs::openrtb::v26::{AuctionType, DeviceType, Qty, DurFloors};
/// ```
///
/// ## New Objects in 2.6
///
/// - **Qty**: DOOH multiplier for multi-viewer impressions
/// - **Refresh**: Ad slot refresh configuration
/// - **RefSettings**: Refresh interval and type settings
/// - **DurFloors**: Duration-based floor pricing for video/audio
///
/// ## Reference
///
/// OpenRTB 2.6 Specification:
/// <https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md>
///
/// ## Implementation Status
///
/// Phase 3/4 complete: All 2.6-specific objects implemented with full test coverage.
// OpenRTB 2.6 specific objects
pub mod durfloors;
pub mod qty;
pub mod refresh;

// Re-export 2.6-specific objects
pub use durfloors::DurFloors;
pub use qty::Qty;
pub use refresh::{RefSettings, Refresh};

// Re-export common types (includes AdCOM)
pub use crate::openrtb::common::*;

// Re-export OpenRTB 2.5 types (2.6 extends 2.5)
pub use crate::openrtb::v25::*;
