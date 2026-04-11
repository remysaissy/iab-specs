//! AdCOM Context Objects
//!
//! Context objects represent the environment in which ads will be displayed,
//! including information about users, devices, locations, distribution channels,
//! publishers, content, and regulatory constraints.
//!
//! Reference: AdCOM v1.0 Section 5 - Context Objects

mod app;
mod brand_version;
mod channel;
mod content;
mod data;
mod device;
mod distribution_channel;
mod dooh;
mod geo;
mod network;
mod producer;
mod publisher;
mod regs;
mod segment;
mod site;
mod user;
mod user_agent;

pub use app::*;
pub use brand_version::*;
pub use channel::*;
pub use content::*;
pub use data::*;
pub use device::*;
pub use distribution_channel::*;
pub use dooh::*;
pub use geo::*;
pub use network::*;
pub use producer::*;
pub use publisher::*;
pub use regs::*;
pub use segment::*;
pub use site::*;
pub use user::*;
pub use user_agent::*;
