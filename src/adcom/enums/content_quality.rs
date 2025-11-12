use serde_repr::{Deserialize_repr, Serialize_repr};

/// Quality of content.
///
/// This enum is deprecated in favor of prodq below. See content object.
#[deprecated(note = "This enum is deprecated in favor of prodq below. See content object.")]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentQuality {
    /// Unknown
    Unknown = 0,

    /// Professionally Produced
    Professional = 1,

    /// Prosumer
    Prosumer = 2,

    /// User Generated (UGC)
    UserGenerated = 3,
}
