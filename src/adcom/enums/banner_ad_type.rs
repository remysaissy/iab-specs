use serde_repr::{Deserialize_repr, Serialize_repr};

/// Banner ad types.
///
/// The type of banner creative to be served using an AdUnit.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BannerAdType {
    /// XHTML Text Ad (usually mobile)
    XhtmlTextAd = 1,

    /// XHTML Banner Ad (usually mobile)
    XhtmlBannerAd = 2,

    /// JavaScript Ad; must be valid XHTML (i.e., script tags included)
    JavaScriptAd = 3,

    /// iFrame
    IFrame = 4,
}
