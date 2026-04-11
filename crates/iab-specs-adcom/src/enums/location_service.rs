use serde_repr::{Deserialize_repr, Serialize_repr};

/// Location service provider.
///
/// Source of the location service being used.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LocationService {
    /// IP2Location
    Ip2Location = 1,

    /// Neustar (Quova)
    Neustar = 2,

    /// MaxMind
    MaxMind = 3,

    /// NetAcuity (Digital Element)
    NetAcuity = 4,

    /// 51Degrees (High Confidence)
    FiftyOneDegreesHigh = 511,

    /// 51Degrees (Medium Confidence)
    FiftyOneDegreesMed = 512,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid LocationService values: 1, 2, 3, 4, 511, 512
        let valid_values = [1, 2, 3, 4, 511, 512];
        for value in valid_values {
            let json = format!("{}", value);
            let result: Result<LocationService, _> = serde_json::from_str(&json);
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
        let result: Result<LocationService, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid LocationService and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<LocationService, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<LocationService, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            LocationService::Ip2Location,
            LocationService::Neustar,
            LocationService::MaxMind,
            LocationService::NetAcuity,
            LocationService::FiftyOneDegreesHigh,
            LocationService::FiftyOneDegreesMed,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: LocationService = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
