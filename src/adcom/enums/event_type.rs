use serde_repr::{Deserialize_repr, Serialize_repr};

/// Event types.
///
/// Types of ad-related events that can be tracked.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
    /// Impression (ad rendered)
    Impression = 1,

    /// Viewable impression (meets viewability standard)
    ViewableImpression = 2,

    /// Click
    Click = 3,

    /// Ad expanded
    Expand = 4,

    /// Ad collapsed
    Collapse = 5,

    /// Creative loaded
    CreativeLoaded = 6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid EventType values (1-6)
        for value in 1..=6 {
            let json = format!("{}", value);
            let result: Result<EventType, _> = serde_json::from_str(&json);
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
        let result: Result<EventType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid EventType and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<EventType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<EventType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            EventType::Impression,
            EventType::ViewableImpression,
            EventType::Click,
            EventType::Expand,
            EventType::Collapse,
            EventType::CreativeLoaded,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EventType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
