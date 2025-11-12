use serde_repr::{Deserialize_repr, Serialize_repr};

/// Companion type.
///
/// Types of companion ads that can accompany video/audio.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompanionType {
    /// Static resource
    Static = 0,

    /// HTML resource
    Html = 1,

    /// iFrame resource
    IFrame = 2,
}
