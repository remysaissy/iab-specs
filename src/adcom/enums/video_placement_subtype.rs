use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video placement subtype.
///
/// More specific video placement types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoPlacementSubtype {
    /// In-stream placement (pre/mid/post-roll)
    InStream = 1,

    /// In-banner video
    InBanner = 2,

    /// In-article video
    InArticle = 3,

    /// In-feed video
    InFeed = 4,

    /// Interstitial/floating video
    Interstitial = 5,
}
