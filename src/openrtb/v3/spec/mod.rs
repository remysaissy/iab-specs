mod audio;
/// OpenRTB 3.0 Specification Objects
///
/// This module implements placement and media specifications for OpenRTB 3.0.
/// These objects are referenced via the `spec` field in Item objects and
/// the `media` field in Bid objects.
///
/// OpenRTB 3.0 uses AdCOM 1.0 for domain layer specifications, providing
/// standardized objects for different media types.
mod display;
mod video;

pub use audio::AudioPlacement;
pub use display::{DisplayFormat, DisplayPlacement};
pub use video::VideoPlacement;
