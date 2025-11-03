//! # iab-specs
//!
//! An unofficial Rust implementation of various IAB (Interactive Advertising Bureau) specifications.
//!
//! This library provides typed Rust data structures for working with IAB advertising specifications,
//! using `serde` for JSON serialization/deserialization and idiomatic Rust patterns throughout.
//!
//! ## Features
//!
//! ⚠️ **Important**: By default, **no features are enabled**. You must explicitly enable the
//! specifications you need:
//!
//! - `adcom` - AdCOM 1.0 support (Advertising Common Object Model enumerations)
//! - `openrtb_25` - OpenRTB 2.5 support (automatically includes `adcom`)
//! - `openrtb_26` - OpenRTB 2.6 support (automatically includes `openrtb_25` and `adcom`)
//! - `ads_txt` - Ads.txt 1.1 support
//! - `app_ads_txt` - App-ads.txt 1.0 support (automatically includes `ads_txt`)
//! - `sellers_json` - Sellers.json 1.0 support
//!
//! ## Quick Start
//!
//! Add the library to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! iab-specs = { version = "0.1", features = ["openrtb_25"] }
//! ```
//!
//! ### Working with OpenRTB Bid Requests
//!
//! ```rust
//! use iab_specs::openrtb::v25::{BidRequest, Imp, Banner, Device};
//!
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! // Create a bid request with a banner impression
//! let request = BidRequest {
//!     id: "req-12345".to_string(),
//!     imp: vec![
//!         Imp {
//!             id: "imp1".to_string(),
//!             banner: Some(Banner {
//!                 w: Some(300),
//!                 h: Some(250),
//!                 ..Default::default()
//!             }),
//!             bidfloor: 0.50,
//!             bidfloorcur: "USD".to_string(),
//!             ..Default::default()
//!         }
//!     ],
//!     device: Some(Device {
//!         ua: Some("Mozilla/5.0...".to_string()),
//!         ip: Some("192.168.1.1".to_string()),
//!         ..Default::default()
//!     }),
//!     tmax: Some(100), // 100ms timeout
//!     ..Default::default()
//! };
//!
//! // Serialize to JSON
//! let json = serde_json::to_string(&request)?;
//!
//! // Deserialize from JSON
//! let parsed: BidRequest = serde_json::from_str(&json)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Using AdCOM Enumerations
//!
//! ```rust
//! use iab_specs::adcom::{AuctionType, DeviceType, ApiFramework};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let auction = AuctionType::FirstPrice;
//! let device = DeviceType::Phone;
//! let api = ApiFramework::Mraid3;
//!
//! // Enums serialize to their numeric values
//! assert_eq!(serde_json::to_string(&auction)?, "1");
//! assert_eq!(serde_json::to_string(&device)?, "4");
//! assert_eq!(serde_json::to_string(&api)?, "6");
//! # Ok(())
//! # }
//! ```
//!
//! ### Parsing Ads.txt Files
//!
//! ```rust
//! use iab_specs::ads_txt::AdsTxt;
//! use std::str::FromStr;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let content = "google.com, pub-1234567890123456, DIRECT, f08c47fec0942fa0";
//! let ads_txt = AdsTxt::from_str(content)?;
//!
//! assert_eq!(ads_txt.systems.len(), 1);
//! assert_eq!(ads_txt.systems[0].domain, "google.com");
//! # Ok(())
//! # }
//! ```
//!
//! ## Module Organization
//!
//! - [`adcom`] - AdCOM 1.0 enumerations (device types, auction types, protocols, etc.)
//! - [`openrtb`] - OpenRTB 2.5 and 2.6 bid request/response objects
//!   - [`openrtb::v25`] - OpenRTB 2.5 specification
//!   - [`openrtb::v26`] - OpenRTB 2.6 with CTV and DOOH support
//!   - [`openrtb::common`] - Common objects shared between versions
//! - [`ads_txt`] - Ads.txt 1.1 parser and generator
//! - [`app_ads_txt`] - App-ads.txt 1.0 parser and generator
//! - [`sellers_json`] - Sellers.json 1.0 parser and generator
//! - [`prelude`] - Re-exports all enabled specifications for convenient imports
//!
//! ## Using the Prelude
//!
//! The [`prelude`] module re-exports all types from enabled features for convenient imports:
//!
//! ```rust
//! use iab_specs::prelude::*;
//!
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! // All enabled types are available
//! let request = BidRequest {
//!     id: "req1".to_string(),
//!     imp: vec![],
//!     ..Default::default()
//! };
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The library provides a unified [`Error`] type that covers all error cases:
//!
//! ```rust
//! use iab_specs::{Error, ads_txt::AdsTxt};
//! use std::str::FromStr;
//!
//! fn parse_ads_txt(content: &str) -> Result<AdsTxt, Error> {
//!     AdsTxt::from_str(content)
//! }
//! ```
//!
//! ## Builder Pattern Support
//!
//! All major types support the builder pattern for ergonomic construction:
//!
//! ```rust
//! use iab_specs::openrtb::v25::{request::BidRequestBuilder, imp::ImpBuilder};
//!
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! let request = BidRequestBuilder::default()
//!     .id("req-123")
//!     .imp(vec![
//!         ImpBuilder::default()
//!             .id("imp1")
//!             .bidfloor(1.50)
//!             .build()?,
//!     ])
//!     .tmax(Some(100))
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Specification Compliance
//!
//! This library aims for full compliance with the official IAB specifications:
//!
//! - [AdCOM 1.0](https://github.com/InteractiveAdvertisingBureau/AdCOM)
//! - [OpenRTB 2.5](https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf)
//! - [OpenRTB 2.6](https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md)
//! - [Ads.txt 1.1](https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf)
//! - [App-ads.txt 1.0](https://iabtechlab.com/wp-content/uploads/2019/03/app-ads.txt-v1.0-final-.pdf)
//! - [Sellers.json 1.0](https://iabtechlab.com/wp-content/uploads/2019/07/Sellers.json_Final.pdf)

#[cfg(feature = "adcom")]
pub mod adcom;
#[cfg(feature = "ads_txt")]
pub mod ads_txt;
#[cfg(feature = "app_ads_txt")]
pub mod app_ads_txt;
mod errors;
#[cfg(any(feature = "openrtb_25", feature = "openrtb_26"))]
pub mod openrtb;
pub mod prelude;
#[cfg(feature = "sellers_json")]
pub mod sellers_json;
pub(crate) mod utils;

pub use errors::*;
