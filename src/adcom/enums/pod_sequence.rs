use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pod sequence.
///
/// Position of ad within a pod.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum PodSequence {
    /// Unknown
    Unknown = 0,

    /// First ad in pod
    First = 1,

    /// Last ad in pod
    Last = 2,

    /// Middle ad in pod
    Middle = 3,

    /// Only ad in pod
    Only = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid PodSequence values (0-4)
        for value in 0..=4 {
            let json = format!("{}", value);
            let result: Result<PodSequence, _> = serde_json::from_str(&json);
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
        let result: Result<PodSequence, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<PodSequence, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            PodSequence::Unknown,
            PodSequence::First,
            PodSequence::Last,
            PodSequence::Middle,
            PodSequence::Only,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: PodSequence = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
