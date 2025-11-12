use serde_repr::{Deserialize_repr, Serialize_repr};

/// Feed types for audio content.
///
/// Type of audio feed.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FeedType {
    /// Music Service
    MusicService = 1,

    /// FM/AM Broadcast
    Broadcast = 2,

    /// Podcast
    Podcast = 3,
}
