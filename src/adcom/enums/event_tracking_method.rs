use serde_repr::{Deserialize_repr, Serialize_repr};

/// Event tracking methods.
///
/// Methods for tracking ad events.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventTrackingMethod {
    /// Image-pixel tracking (1x1 pixel)
    ImagePixel = 1,

    /// JavaScript tracking
    JavaScript = 2,
}
