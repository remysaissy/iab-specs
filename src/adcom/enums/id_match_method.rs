use serde_repr::{Deserialize_repr, Serialize_repr};

/// ID matching methods for user identification.
///
/// Indicates the method used to match a user ID across different contexts.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IDMatchMethod {
    /// No matching - ID came directly from 3rd-party cookie or device IFA
    NoMatching = 0,

    /// First-party observation without user authentication
    FirstParty = 1,

    /// Probabilistic matching based on non-authenticated features
    Probabilistic = 2,

    /// Deterministic matching with user authentication
    Deterministic = 3,
}
