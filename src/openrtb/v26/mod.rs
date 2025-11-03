// Allow ambiguous glob re-exports since v26 intentionally extends v25
#![allow(ambiguous_glob_reexports)]

/// OpenRTB 2.6 Protocol Implementation
///
/// This module implements the complete OpenRTB 2.6 specification as defined by the IAB.
///
/// OpenRTB 2.6 extends version 2.5 with support for:
/// - **CTV Ad Pods**: Sequential ad placement with podid, podseq, slotinpod fields
/// - **DOOH Support**: Digital out-of-home advertising with multi-viewer multipliers
/// - **Duration-Based Pricing**: Floor prices based on creative duration ranges
/// - **Structured User-Agent**: Parsed browser/OS details from User-Agent Client Hints
/// - **Enhanced Privacy**: GDPR, US Privacy, and GPP support
///
/// ## Example: CTV Ad Pod Configuration
///
/// ```rust
/// use iab_specs::openrtb::v26::{Video, DurFloors};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let video = Video {
///     mimes: vec!["video/mp4".to_string()],
///     minduration: 15,
///     maxduration: Some(30),
///     protocols: Some(vec![7]), // VAST 4.0
///     // CTV ad pod configuration
///     podid: Some("pod-abc123".to_string()),
///     podseq: 0, // First ad in pod
///     slotinpod: 1, // Guaranteed first position
///     // Duration-based floor pricing
///     durfloors: Some(vec![
///         DurFloors {
///             minduration: Some(15),
///             maxduration: Some(30),
///             bidfloor: Some(5.00),
///             bidfloorcur: Some("USD".to_string()),
///             ..Default::default()
///         },
///     ]),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
///
/// ## Example: DOOH with Viewer Multiplier
///
/// ```rust
/// use iab_specs::openrtb::v26::{Imp, Qty};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let imp = Imp {
///     id: "dooh-imp-1".to_string(),
///     // DOOH multiplier for multi-viewer impressions
///     qty: Some(Qty {
///         multiplier: Some(150.0), // 150 people viewing
///         source: Some("venue_measurement".to_string()),
///         ..Default::default()
///     }),
///     bidfloor: 2.00,
///     bidfloorcur: "USD".to_string(),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
///
/// ## New Objects in OpenRTB 2.6
///
/// - [`Qty`] - DOOH multiplier for multi-viewer impressions
/// - [`Refresh`] - Ad slot refresh configuration
/// - [`RefSettings`] - Refresh interval and type settings
/// - [`DurFloors`] - Duration-based floor pricing for video/audio
///
/// ## Using OpenRTB 2.5 Types
///
/// All OpenRTB 2.5 types are available through this module:
///
/// ```rust
/// use iab_specs::openrtb::v26::{BidRequest, Imp, Banner, AuctionType};
/// # let _ = AuctionType::FirstPrice;
/// ```
///
/// ## Reference
///
/// OpenRTB 2.6 Specification:
/// <https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md>
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
