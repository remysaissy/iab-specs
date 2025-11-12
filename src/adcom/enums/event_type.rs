use serde_repr::{Deserialize_repr, Serialize_repr};

/// Event types.
///
/// Types of ad-related events that can be tracked.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
    /// Impression (ad rendered)
    Impression = 1,

    /// Viewable impression (meets viewability standard)
    ViewableImpression = 2,

    /// Click
    Click = 3,

    /// Ad expanded
    Expand = 4,

    /// Ad collapsed
    Collapse = 5,

    /// Creative loaded
    CreativeLoaded = 6,
}
