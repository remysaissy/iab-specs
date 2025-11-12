use serde_repr::{Deserialize_repr, Serialize_repr};

/// Device interface orientation.
///
/// The orientation of the device when the ad is shown.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum DeviceOrientation {
    /// Portrait orientation
    #[default]
    Portrait = 0,

    /// Landscape orientation
    Landscape = 1,
}
