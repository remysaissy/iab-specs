use serde_repr::{Deserialize_repr, Serialize_repr};

/// Playback methods available for video inventory.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlaybackMethod {
    /// Initiates on page load with sound on
    AutoPlaySoundOn = 1,

    /// Initiates on page load with sound off by default
    AutoPlaySoundOff = 2,

    /// Initiates on click with sound on
    ClickToPlay = 3,

    /// Initiates on mouse-over with sound on
    MouseOver = 4,

    /// Initiates on entering viewport with sound on
    EnterViewportSoundOn = 5,

    /// Initiates on entering viewport with sound off by default
    EnterViewportSoundOff = 6,
}
