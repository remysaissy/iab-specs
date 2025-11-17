use serde_repr::{Deserialize_repr, Serialize_repr};

/// Expandable direction.
///
/// Direction in which an expandable ad may expand.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExpandableDirection {
    /// Left
    Left = 1,

    /// Right
    Right = 2,

    /// Up
    Up = 3,

    /// Down
    Down = 4,

    /// Full Screen
    FullScreen = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid ExpandableDirection values (1-5)
        for value in 1..=5 {
            let json = format!("{}", value);
            let result: Result<ExpandableDirection, _> = serde_json::from_str(&json);
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
        let result: Result<ExpandableDirection, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid ExpandableDirection and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<ExpandableDirection, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<ExpandableDirection, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            ExpandableDirection::Left,
            ExpandableDirection::Right,
            ExpandableDirection::Up,
            ExpandableDirection::Down,
            ExpandableDirection::FullScreen,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ExpandableDirection = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
