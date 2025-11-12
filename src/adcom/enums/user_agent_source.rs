use serde_repr::{Deserialize_repr, Serialize_repr};

/// User agent source.
///
/// Source of the user agent string.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UserAgentSource {
    /// Unknown source
    Unknown = 0,

    /// User-agent HTTP header
    HttpHeader = 1,

    /// Client hints
    ClientHints = 2,

    /// Server-side detection
    ServerSide = 3,
}
