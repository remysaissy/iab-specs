use serde_repr::{Deserialize_repr, Serialize_repr};

/// Creative subtype for audio/video ads.
///
/// Categorization of audio and video creative formats.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeSubtypeAudioVideo {
    /// VAST (Video Ad Serving Template)
    Vast = 1,

    /// DAAST (Digital Audio Ad Serving Template)
    Daast = 2,

    /// VPAID (Video Player-Ad Interface Definition)
    Vpaid = 3,

    /// Proprietary format
    Proprietary = 4,
}
