use serde_repr::{Deserialize_repr, Serialize_repr};

/// No-Bid Reason Codes.
///
/// The following table lists the options for a bidder to signal the exchange as to why
/// it did not bid on the impression.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoBidReason {
    /// Unknown Error
    UnknownError = 0,

    /// Technical Error
    TechnicalError = 1,

    /// Invalid Request
    InvalidRequest = 2,

    /// Known Web Spider
    KnownSpider = 3,

    /// Suspected Non-Human Traffic
    SuspectedNonHuman = 4,

    /// Cloud, Data Center, or Proxy IP
    CloudDatacenterProxy = 5,

    /// Unsupported Device
    UnsupportedDevice = 6,

    /// Blocked Publisher or Site
    BlockedPublisher = 7,

    /// Unmatched User
    UnmatchedUser = 8,

    /// Daily Reader Cap Met
    DailyCapMet = 9,

    /// Daily Domain Cap Met
    DailyDomainCapMet = 10,
}
