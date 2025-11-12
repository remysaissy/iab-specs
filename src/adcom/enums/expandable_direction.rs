use serde_repr::{Deserialize_repr, Serialize_repr};

/// Expandable direction.
///
/// Direction in which an expandable ad may expand.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExpandableDirection {
    /// Left
    Left = 1,

    /// Right
    Right = 2,

    /// Up
    Up = 3,

    /// Down
    Down = 4,

    /// Full Screen
    FullScreen = 5,
}
