use serde_repr::{Deserialize_repr, Serialize_repr};

/// API frameworks supported by the publisher.
///
/// Note that MRAID-1, MRAID-2, and MRAID-3 are numbered 3, 5, and 6 since it was
/// determined that their predecessors, values 1 and 2, were duplicates as the
/// VPAID 1.0 and VPAID 2.0 specifications are inherently HTML5 compliant.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApiFramework {
    /// VPAID 1.0
    Vpaid1 = 1,

    /// VPAID 2.0
    Vpaid2 = 2,

    /// MRAID-1
    Mraid1 = 3,

    /// ORMMA
    Ormma = 4,

    /// MRAID-2
    Mraid2 = 5,

    /// MRAID-3
    Mraid3 = 6,

    /// OMID-1
    Omid1 = 7,

    /// SIMID-1
    Simid1 = 8,

    /// SIMID-1.1
    Simid1_1 = 9,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid ApiFramework values (1-9)
        for value in 1..=9 {
            let json = format!("{}", value);
            let result: Result<ApiFramework, _> = serde_json::from_str(&json);
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
        let result: Result<ApiFramework, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid ApiFramework and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<ApiFramework, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<ApiFramework, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            ApiFramework::Vpaid1,
            ApiFramework::Vpaid2,
            ApiFramework::Mraid1,
            ApiFramework::Ormma,
            ApiFramework::Mraid2,
            ApiFramework::Mraid3,
            ApiFramework::Omid1,
            ApiFramework::Simid1,
            ApiFramework::Simid1_1,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ApiFramework = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
