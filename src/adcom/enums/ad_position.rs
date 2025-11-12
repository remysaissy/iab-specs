use serde_repr::{Deserialize_repr, Serialize_repr};

/// The position of the ad as a relative measure of visibility or prominence.
///
/// This OpenRTB list has values derived from the Inventory Quality Guidelines (IQG).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum AdPosition {
    /// Unknown position
    #[default]
    Unknown = 0,

    /// Above the fold
    AboveTheFold = 1,

    /// May or may not be initially visible (deprecated by OpenRTB)
    #[deprecated(note = "Use Unknown or other appropriate value")]
    MayNotBeVisible = 2,

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
