//! AdCOM Placement Objects
//!
//! Placement objects define ad slot specifications and acceptance criteria,
//! describing where ads can be displayed and what formats are acceptable.
//! These include display, video, and audio placement types.
//!
//! Reference: AdCOM v1.0 Section 4 - Placement Objects

mod asset_format;
mod audio_placement;
mod companion;
mod data_asset_format;
mod display_format;
mod display_placement;
mod event_spec;
mod image_asset_format;
mod native_format;
#[allow(clippy::module_inception)]
mod placement;
mod title_asset_format;
mod video_placement;

pub use asset_format::*;
pub use audio_placement::*;
pub use companion::*;
pub use data_asset_format::*;
pub use display_format::*;
pub use display_placement::*;
pub use event_spec::*;
pub use image_asset_format::*;
pub use native_format::*;
pub use placement::*;
pub use title_asset_format::*;
pub use video_placement::*;
