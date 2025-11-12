use serde_repr::{Deserialize_repr, Serialize_repr};

/// Display placement type.
///
/// General type or context of the display placement.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayPlacementType {
    /// In-feed placement (e.g., newsfeed, content stream)
    InFeed = 1,

    /// Sidebar placement
    Sidebar = 2,

    /// Interstitial/Overlay placement
    Interstitial = 3,

    /// Floating placement
    Floating = 4,
}
