//! ## Module Organization
//!
//! - [`adcom`] - AdCOM 1.0 enumerations (device types, auction types, protocols, etc.)
//! - [`openrtb`] - OpenRTB 2.5, 2.6, 3.0, and Native Ads 1.2 specifications
//!   - [`openrtb::v25`] - OpenRTB 2.5 specification
//!   - [`openrtb::v26`] - OpenRTB 2.6 with CTV and DOOH support
//!   - [`openrtb::v30`] - OpenRTB 3.0 with layered architecture
//!   - [`openrtb::native`] - OpenRTB Native Ads 1.2 specification
//!   - [`openrtb::common`] - Common objects shared between versions
//! - [`artb`] - Agentic RTB Framework 1.0 specification
//!   - [`artb::v10`] - ARTB 1.0 with OpenRTB Patch Protocol
//! - [`ads_txt`] - Ads.txt 1.1 parser and generator
//! - [`app_ads_txt`] - App-ads.txt 1.0 parser and generator
//! - [`sellers_json`] - Sellers.json 1.0 parser and generator
//!
//! ## Extension Trait
//!
//! The [`Extension`] trait provides a flexible mechanism for adding custom fields to IAB
//! specification objects throughout the crate. This is essential for:
//! - **Vendor-specific data**: Add custom fields for your platform
//! - **Internal tracking**: Include business-specific identifiers
//! - **Experimental features**: Test new capabilities without spec changes
//! - **Custom workflows**: Extend objects with application-specific data
//!
//! ### Types Supporting Extensions
//!
//! Many types across the crate support generic extensions via the `Ext` type parameter:
//!
//! - **AdCOM types**: [`adcom::media::Ad`], [`adcom::placement::Placement`], [`adcom::context::Site`],
//!   [`adcom::context::App`], [`adcom::context::User`], [`adcom::context::Device`], and many more
//! - **OpenRTB 2.5/2.6**: [`openrtb::v25::BidRequest`], [`openrtb::v25::BidResponse`],
//!   [`openrtb::v25::Imp`], [`openrtb::v25::Banner`], [`openrtb::v25::Video`], and many more
//! - **OpenRTB 3.0**: [`openrtb::v30::Request`], [`openrtb::v30::Response`], [`openrtb::v30::Item`],
//!   [`openrtb::v30::Bid`], and many more
//!
//! ### Quick Examples
//!
//! **Using default `Vec<u8>` extensions (opaque bytes):**
//!
//! ```
//! #[cfg(feature = "adcom")]
//! {
//! use iab_specs::adcom::media::Ad;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! // DefaultExt is Vec<u8>
//! let ad = Ad::builder()
//!     .id(Some("ad123".to_string()))
//!     .ext(Some(Box::new(vec![0x08, 0x96, 0x01])))
//!     .build()?;
//! # Ok(())
//! # }
//! }
//! ```
//!
//! **Using explicit JSON extensions:**
//!
//! ```
//! #[cfg(feature = "adcom")]
//! {
//! use iab_specs::adcom::media::AdBuilder;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! let ad = AdBuilder::<serde_json::Value>::default()
//!     .id(Some("ad123".to_string()))
//!     .ext(Some(Box::new(serde_json::json!({
//!         "vendor_id": "acme-123",
//!         "campaign_type": "seasonal"
//!     }))))
//!     .build()?;
//! # Ok(())
//! # }
//! }
//! ```
//!
//! **Using custom typed extensions:**
//!
//! ```
//! #[cfg(feature = "adcom")]
//! {
//! use iab_specs::adcom::media::{Ad, AdBuilder};
//! use serde::{Deserialize, Serialize};
//! use derive_builder::Builder;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! #[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
//! struct MyAdExt {
//!     vendor_id: String,
//!     priority: u8,
//! }
//!
//! impl MyAdExt {
//!     pub fn builder() -> MyAdExtBuilder {
//!         MyAdExtBuilder::create_empty()
//!     }
//! }
//!
//! let ext = MyAdExt::builder()
//!     .vendor_id("acme-123".to_string())
//!     .priority(5)
//!     .build()?;
//!
//! // Use AdBuilder with type parameter for custom extensions
//! let ad = AdBuilder::default()
//!     .id(Some("ad123".to_string()))
//!     .ext(Some(Box::new(ext)))
//!     .build()?;
//! # Ok(())
//! # }
//! }
//! ```
//!
//! For complete documentation and more examples, see the [`Extension`] trait documentation.

#[cfg(feature = "adcom")]
pub mod adcom;
#[cfg(feature = "ads_txt")]
pub mod ads_txt;
#[cfg(feature = "agentic_audience_10")]
pub mod agentic_audience;
#[cfg(feature = "agentic_direct_21")]
pub mod agentic_direct;
#[cfg(feature = "app_ads_txt")]
pub mod app_ads_txt;
#[cfg(feature = "artb_10")]
pub mod artb;
#[cfg(feature = "buyer_agent_10")]
pub mod buyer_agent;
mod errors;
#[cfg(any(feature = "openrtb_25", feature = "openrtb_26", feature = "openrtb_30"))]
pub mod openrtb;
#[cfg(feature = "registry_agent_10")]
pub mod registry_agent;
#[cfg(feature = "seller_agent_10")]
pub mod seller_agent;
#[cfg(feature = "sellers_json")]
pub mod sellers_json;
pub(crate) mod utils;

pub use errors::*;
pub use iab_specs_core::slice_up_to;
pub use utils::*;
