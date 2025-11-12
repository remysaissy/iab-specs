//! AdCOM Media Objects
//!
//! Media objects define advertising media instances with rendering instructions
//! and metadata. These include root Ad objects and their specialized subtypes
//! for display, video, audio, native, and banner formats.
//!
//! Reference: AdCOM v1.0 Section 3 - Media Objects

mod ad;
mod asset;
mod audio;
mod audit;
mod banner;
mod data_asset;
mod display;
mod event;
mod image_asset;
mod link_asset;
mod native;
mod title_asset;
mod video;
mod video_asset;

pub use ad::*;
pub use asset::*;
pub use audio::*;
pub use audit::*;
pub use banner::*;
pub use data_asset::*;
pub use display::*;
pub use event::*;
pub use image_asset::*;
pub use link_asset::*;
pub use native::*;
pub use title_asset::*;
pub use video::*;
pub use video_asset::*;
