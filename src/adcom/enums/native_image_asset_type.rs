use serde_repr::{Deserialize_repr, Serialize_repr};

/// Native image asset types.
///
/// Types of image assets in native ads.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NativeImageAssetType {
    /// Icon image (typically small, square)
    Icon = 1,

    /// Logo image
    Logo = 2,

    /// Large image (main creative image)
    Main = 3,
}
