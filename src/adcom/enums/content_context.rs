use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of content being displayed.
///
/// The nature of the content on the site, app, or other property.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentContext {
    /// Video (i.e., video file or stream such as Internet TV broadcasts)
    Video = 1,

    /// Game (i.e., an interactive software game)
    Game = 2,

    /// Music (i.e., audio file or stream such as Internet radio broadcasts)
    Music = 3,

    /// Application (i.e., an interactive software application)
    Application = 4,

    /// Text (i.e., primarily textual document such as a web page, eBook, or news article)
    Text = 5,

    /// Other (i.e., none of the other categories applies)
    Other = 6,

    /// Unknown
    Unknown = 7,
}
