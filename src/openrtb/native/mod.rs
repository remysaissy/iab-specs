//! OpenRTB Native Ads Protocol Implementation
//!
//! This module implements the OpenRTB Native Ads specification, which defines
//! standardized formats for trading native advertising across platforms.
//!
//! # Supported Versions
//!
//! - [`v12`] - OpenRTB Native Ads 1.2 (March 2017)
//!
//! # Overview
//!
//! Native advertising allows ads to match the form and function of the platform
//! on which they appear. This specification enables automated trading of native
//! ad formats through standardized request and response protocols.
//!
//! # Feature Flags
//!
//! - `openrtb_native_12` - Enable OpenRTB Native Ads 1.2 support
//!
//! # Example
//!
//! ```rust
//! use iab_specs::openrtb::native::v12::{NativeRequest, Asset, Title};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a native ad request
//! let request = NativeRequest::builder()
//!     .ver("1.2")
//!     .context(Some(1)) // Content-centric
//!     .plcmttype(Some(1)) // In-feed
//!     .assets(vec![
//!         Asset::builder()
//!             .id(1)
//!             .required(Some(1))
//!             .title(Some(Title::builder()
//!                 .len(90)
//!                 .build()?))
//!             .build()?
//!     ])
//!     .build()?;
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "openrtb_native_12")]
pub mod v12;

#[cfg(feature = "openrtb_native_12")]
pub use v12::*;
