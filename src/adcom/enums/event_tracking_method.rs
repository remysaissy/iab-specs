use serde_repr::{Deserialize_repr, Serialize_repr};

/// Event tracking methods.
///
/// Methods for tracking ad events.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventTrackingMethod {
    /// Image-pixel tracking (1x1 pixel)
    ImagePixel = 1,

    /// JavaScript tracking
    JavaScript = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid EventTrackingMethod values (1-2)
        for value in 1..=2 {
            let json = format!("{}", value);
            let result: Result<EventTrackingMethod, _> = serde_json::from_str(&json);
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
        let result: Result<EventTrackingMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid EventTrackingMethod and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<EventTrackingMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<EventTrackingMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            EventTrackingMethod::ImagePixel,
            EventTrackingMethod::JavaScript,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EventTrackingMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
