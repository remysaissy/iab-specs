//! ## Module Organization
//!
//! - [`adcom`] - AdCOM 1.0 enumerations (device types, auction types, protocols, etc.)
//! - [`openrtb`] - OpenRTB 2.5, 2.6, and 3.0 bid request/response objects
//!   - [`openrtb::v25`] - OpenRTB 2.5 specification
//!   - [`openrtb::v26`] - OpenRTB 2.6 with CTV and DOOH support
//!   - [`openrtb::v30`] - OpenRTB 3.0 with layered architecture
//!   - [`openrtb::common`] - Common objects shared between versions
//! - [`ads_txt`] - Ads.txt 1.1 parser and generator
//! - [`app_ads_txt`] - App-ads.txt 1.0 parser and generator
//! - [`sellers_json`] - Sellers.json 1.0 parser and generator

#[cfg(feature = "adcom")]
pub mod adcom;
#[cfg(feature = "ads_txt")]
pub mod ads_txt;
#[cfg(feature = "app_ads_txt")]
pub mod app_ads_txt;
mod errors;
#[cfg(any(feature = "openrtb_25", feature = "openrtb_26", feature = "openrtb_30"))]
pub mod openrtb;
#[cfg(feature = "sellers_json")]
pub mod sellers_json;
pub(crate) mod utils;

pub use errors::*;
pub use utils::*;
