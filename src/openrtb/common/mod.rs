/// Common types shared between OpenRTB versions.
///
/// This module provides:
/// - Re-exports of AdCOM types (enumerations and future objects)
/// - OpenRTB-specific common objects (SupplyChain)
///
/// ## AdCOM Integration
///
/// OpenRTB 2.5+ uses AdCOM (Advertising Common Object Model) for common domain objects.
/// All AdCOM types are re-exported here for convenience, allowing you to use them
/// directly from the OpenRTB namespace:
///
/// ```
/// // Import from OpenRTB common (recommended):
/// use iab_specs::openrtb::common::{AuctionType, DeviceType};
/// # let _ = AuctionType::FirstPrice;
/// # let _ = DeviceType::Phone;
/// ```
///
/// Or equivalently, import directly from AdCOM:
/// ```
/// // Import from AdCOM directly:
/// use iab_specs::adcom::{AuctionType, DeviceType};
/// # let _ = AuctionType::FirstPrice;
/// # let _ = DeviceType::Phone;
/// ```
///
/// ## OpenRTB-Specific Types
///
/// Some objects are specific to the OpenRTB transaction protocol and are not part of AdCOM:
/// - `SupplyChain` / `SupplyChainNode`: Supply chain transparency objects
// Re-export AdCOM types (required dependency for OpenRTB)
pub use crate::adcom::*;

// OpenRTB-specific common types
mod supply_chain;
pub use supply_chain::*;
