use serde_repr::{Deserialize_repr, Serialize_repr};

/// Creative subtype for display ads.
///
/// More granular categorization of display creative types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeSubtypeDisplay {
    /// HTML banner
    HtmlBanner = 1,

    /// VAST tag for video
    Vast = 2,

    /// VPAID for interactive video
    Vpaid = 3,

    /// JavaScript tag
    JavaScript = 4,

    /// iFrame
    IFrame = 5,
}
