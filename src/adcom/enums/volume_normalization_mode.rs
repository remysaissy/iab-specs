use serde_repr::{Deserialize_repr, Serialize_repr};

/// Volume normalization modes.
///
/// Volume normalization modes for audio content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VolumeNormalizationMode {
    /// None
    None = 0,

    /// Ad Volume Average Normalized to Content
    AverageVolume = 1,

    /// Ad Volume Peak Normalized to Content
    PeakVolume = 2,

    /// Ad Loudness Normalized to Content
    Loudness = 3,

    /// Custom Volume Normalization
    Custom = 4,
}
