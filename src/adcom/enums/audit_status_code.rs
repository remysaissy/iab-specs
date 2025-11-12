use serde_repr::{Deserialize_repr, Serialize_repr};

/// Audit status codes.
///
/// Status codes for creative audits and approval.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AuditStatusCode {
    /// Approved
    Approved = 1,

    /// Approved with changes
    ApprovedWithChanges = 2,

    /// Rejected
    Rejected = 3,

    /// Rejected for impressions (creative not served)
    RejectedForImpressions = 4,
}
