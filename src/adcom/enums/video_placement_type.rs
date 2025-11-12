use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video placement types.
///
/// These values are derived from the IAB's Digital Video Guidelines for programmatic
/// video advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoPlacementType {
    /// In-Stream: Played before, during or after the streaming video content that the
    /// consumer has requested (Pre-roll, Mid-roll, Post-roll).
    InStream = 1,

    /// In-Banner: Exists within a web banner that leverages the banner space to deliver
    /// a video experience as opposed to another static or rich media format.
    InBanner = 2,

    /// In-Article: Loads and plays dynamically between paragraphs of editorial content;
    /// existing as a standalone branded message.
    InArticle = 3,

    /// In-Feed: Found in content, social, or product feeds.
    InFeed = 4,

    /// Interstitial/Slider/Floating: Covers the entire or a portion of screen area, but
    /// is always on screen while displayed (i.e. cannot be scrolled out of view).
    Interstitial = 5,
}
