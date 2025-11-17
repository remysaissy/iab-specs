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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid NoBidReason values (0-10)
        for value in 0..=10 {
            let json = format!("{}", value);
            let result: Result<NoBidReason, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<NoBidReason, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<NoBidReason, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            NoBidReason::UnknownError,
            NoBidReason::TechnicalError,
            NoBidReason::InvalidRequest,
            NoBidReason::KnownSpider,
            NoBidReason::SuspectedNonHuman,
            NoBidReason::CloudDatacenterProxy,
            NoBidReason::UnsupportedDevice,
            NoBidReason::BlockedPublisher,
            NoBidReason::UnmatchedUser,
            NoBidReason::DailyCapMet,
            NoBidReason::DailyDomainCapMet,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: NoBidReason = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
