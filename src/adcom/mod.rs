pub mod context;
/// AdCOM (Advertising Common Object Model) v1.0 Implementation
///
/// AdCOM is a standardized specification developed by the IAB Technology Lab that defines
/// common domain objects used across multiple advertising protocols. It serves as a reusable
/// foundational layer beneath transaction protocols like OpenRTB v3.0.
///
/// ## Purpose
///
/// Rather than each specification independently defining concepts like ads, placements, and
/// users, AdCOM centralizes these definitions, enabling consistency and reducing redundancy
/// across the advertising technology ecosystem.
///
/// ## Architecture
///
/// AdCOM operates within the IAB's layered OpenMedia model at Layer 4 (Domain Concepts),
/// providing three categories of objects:
///
/// 1. **Media Objects**: Actual advertisements with rendering instructions and metadata
///    - Ad, Display, Banner, Video, Audio, Native, Asset, Event, etc.
///
/// 2. **Placement Objects**: Ad slot specifications and acceptance criteria
///    - Placement, DisplayPlacement, VideoPlacement, AudioPlacement, etc.
///
/// 3. **Context Objects**: Environmental and user context where ads appear
///    - Site, App, Device, User, Geo, Content, Publisher, Regs, etc.
///
/// ## Usage
///
/// AdCOM can be used independently or as a foundation for OpenRTB:
///
/// ```toml
/// # Use AdCOM alone
/// iab-specs = { version = "0.1", features = ["adcom"] }
///
/// # Use with OpenRTB (automatically includes AdCOM)
/// iab-specs = { version = "0.1", features = ["openrtb_25"] }
/// ```
///
/// ## Example
///
/// ```
/// use iab_specs::adcom::{AuctionType, ApiFramework, DeviceType};
///
/// let auction = AuctionType::FirstPrice;
/// let device = DeviceType::Phone;
/// let api = ApiFramework::Mraid3;
/// ```
///
/// ## Future Extension
///
/// This module currently implements AdCOM enumerations. Media, Placement, and Context
/// objects will be added in future phases as needed by OpenRTB implementations.
///
/// The clean module separation allows for easy extraction to a separate crate
/// (e.g., `iab-adcom`) when beneficial.
///
/// ## References
///
/// - AdCOM 1.0 Specification: <https://github.com/InteractiveAdvertisingBureau/AdCOM>
/// - IAB Tech Lab: <https://iabtechlab.com>
/// - OpenMedia Layered Model: Part of IAB's standardization framework
mod enums;

pub use context::*;
pub use enums::*;

// Future modules (to be implemented as needed):
// pub mod media;      // Ad, Display, Video, Audio, Banner, Native, etc.
// pub mod placement;  // Placement, DisplayPlacement, VideoPlacement, etc.
