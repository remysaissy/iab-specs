//! Agentic RTB Framework (ARTB) Protocol Implementation
//!
//! This module implements the Agentic RTB Framework specification as defined by
//! IAB Tech Lab. ARTB defines a standard for deploying autonomous agent services
//! as containers within ad-tech host platforms to participate in real-time bidding
//! (RTB) bidstream processing.
//!
//! # Supported Versions
//!
//! - [`v10`] - ARTB 1.0 (November 2025)
//!
//! # Overview
//!
//! ARTB introduces the "OpenRTB Patch Protocol" where containerized agents propose
//! atomic, intent-declared mutations to OpenRTB bid requests and responses. The
//! orchestrator (host platform) can accept or reject each mutation independently.
//!
//! # Feature Flags
//!
//! - `artb_10` - Enable ARTB 1.0 support
//!
//! # Example
//!
//! ```rust
//! use iab_specs::artb::v10::{
//!     RTBRequest, RTBResponse, Mutation, Metadata,
//!     Lifecycle, Intent, Operation, IDsPayload,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let request = RTBRequest::builder()
//!     .lifecycle(Lifecycle::PublisherBidRequest)
//!     .id("req-001")
//!     .applicable_intents(vec![Intent::ActivateSegments])
//!     .build()?;
//!
//! let response = RTBResponse::builder()
//!     .id("req-001")
//!     .mutations(vec![
//!         Mutation::builder()
//!             .intent(Intent::ActivateSegments)
//!             .op(Operation::Add)
//!             .path("/user/data/segment".to_string())
//!             .ids(Some(IDsPayload::builder()
//!                 .id(vec!["seg-001".to_string()])
//!                 .build()?))
//!             .build()?,
//!     ])
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! # References
//!
//! - ARTB 1.0: <https://github.com/IABTechLab/agentic-rtb-framework>

#[cfg(feature = "artb_10")]
pub mod v10;
