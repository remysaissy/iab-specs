use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pod deduplication.
///
/// Deduplication method for ad pods.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PodDeduplication {
    /// Unknown/not specified
    Unknown = 0,

    /// No deduplication
    None = 1,

    /// Deduplicate by creative ID
    ByCreativeId = 2,

    /// Deduplicate by advertiser domain
    ByAdvertiserDomain = 3,
}
