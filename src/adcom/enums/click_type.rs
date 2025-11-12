use serde_repr::{Deserialize_repr, Serialize_repr};

/// Click type.
///
/// Types of ad click behavior.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClickType {
    /// Non-clickable
    NonClickable = 0,

    /// Clickable
    Clickable = 1,

    /// Clickable with embedded browser
    EmbeddedBrowser = 2,
}
