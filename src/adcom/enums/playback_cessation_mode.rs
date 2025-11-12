use serde_repr::{Deserialize_repr, Serialize_repr};

/// Options for the video content and ad play mode.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlaybackCessationMode {
    /// On video completion or when user exits
    OnCompletion = 1,

    /// On page exit
    OnExit = 2,

    /// On float
    OnFloat = 3,
}
