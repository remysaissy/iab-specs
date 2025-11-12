use serde_repr::{Deserialize_repr, Serialize_repr};

/// Production quality.
///
/// The production quality of the content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ProductionQuality {
    /// Unknown
    Unknown = 0,

    /// Professionally Produced
    Professional = 1,

    /// Prosumer
    Prosumer = 2,

    /// User Generated (UGC)
    UserGenerated = 3,
}
