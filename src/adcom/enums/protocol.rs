use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video or audio protocols supported.
///
/// OpenRTB version 2.5 list. VAST versions are numbered in a sub-range to distinguish
/// from other protocol values. DAAST is included for audio ads. OpenRTB 2.6 adds support
/// for VAST 4.2 and 4.3.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Protocol {
    /// VAST 1.0
    Vast1 = 1,

    /// VAST 2.0
    Vast2 = 2,

    /// VAST 3.0
    Vast3 = 3,

    /// VAST 1.0 Wrapper
    Vast1Wrapper = 4,

    /// VAST 2.0 Wrapper
    Vast2Wrapper = 5,

    /// VAST 3.0 Wrapper
    Vast3Wrapper = 6,

    /// VAST 4.0
    Vast4 = 7,

    /// VAST 4.0 Wrapper
    Vast4Wrapper = 8,

    /// DAAST 1.0
    Daast1 = 9,

    /// DAAST 1.0 Wrapper
    Daast1Wrapper = 10,

    /// VAST 4.1
    Vast4_1 = 11,

    /// VAST 4.1 Wrapper
    Vast4_1Wrapper = 12,

    /// VAST 4.2
    Vast4_2 = 13,

    /// VAST 4.2 Wrapper
    Vast4_2Wrapper = 14,

    /// VAST 4.3
    Vast4_3 = 15,

    /// VAST 4.3 Wrapper
    Vast4_3Wrapper = 16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid Protocol values (1-16)
        for value in 1..=16 {
            let json = format!("{}", value);
            let result: Result<Protocol, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }
    #[test]
    fn test_invalid_value_zero() {
        let json = "0";
        let result: Result<Protocol, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid Protocol and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<Protocol, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<Protocol, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            Protocol::Vast1,
            Protocol::Vast2,
            Protocol::Vast3,
            Protocol::Vast1Wrapper,
            Protocol::Vast2Wrapper,
            Protocol::Vast3Wrapper,
            Protocol::Vast4,
            Protocol::Vast4Wrapper,
            Protocol::Daast1,
            Protocol::Daast1Wrapper,
            Protocol::Vast4_1,
            Protocol::Vast4_1Wrapper,
            Protocol::Vast4_2,
            Protocol::Vast4_2Wrapper,
            Protocol::Vast4_3,
            Protocol::Vast4_3Wrapper,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Protocol = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
