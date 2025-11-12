use serde_repr::{Deserialize_repr, Serialize_repr};

/// Placement position.
///
/// Ad position on screen (may duplicate AdPosition for legacy reasons).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlacementPosition {
    /// Unknown
    Unknown = 0,

    /// Above the fold
    AboveTheFold = 1,

    /// Below the fold
    BelowTheFold = 3,

    /// Header
    Header = 4,

    /// Footer
    Footer = 5,

    /// Sidebar
    Sidebar = 6,

    /// Full screen
    FullScreen = 7,
}
