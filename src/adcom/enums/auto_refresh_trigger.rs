use serde_repr::{Deserialize_repr, Serialize_repr};

/// Auto-refresh trigger.
///
/// Trigger that causes a placement to auto-refresh.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AutoRefreshTrigger {
    /// User-initiated refresh
    UserInitiated = 1,

    /// Time-based expiration
    TimeExpiration = 2,

    /// Scroll-based refresh
    Scroll = 3,
}
