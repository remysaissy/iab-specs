use serde_repr::{Deserialize_repr, Serialize_repr};

/// Slot position within an ad pod.
///
/// Indicates the position of the individual ad slot within an ad pod for video/audio.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum SlotPosition {
    /// Last ad in the pod
    Last = -1,

    /// Any other position (middle of pod)
    Any = 0,

    /// First ad in the pod
    First = 1,

    /// First or last position in the pod
    FirstOrLast = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid SlotPosition values (-1, 0, 1, 2)
        // Note: -1 is valid (Last position in pod)
        let valid_values = [-1, 0, 1, 2];
        for value in valid_values {
            let json = format!("{}", value);
            let result: Result<SlotPosition, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    #[test]
    fn test_invalid_value_out_of_range_positive() {
        let json = "99";
        let result: Result<SlotPosition, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_out_of_range_negative() {
        // -2 is invalid (only -1 is valid negative value)
        let json = "-2";
        let result: Result<SlotPosition, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value -2 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            SlotPosition::Last,        // -1
            SlotPosition::Any,         // 0
            SlotPosition::First,       // 1
            SlotPosition::FirstOrLast, // 2
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: SlotPosition = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
