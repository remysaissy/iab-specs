use serde_repr::{Deserialize_repr, Serialize_repr};

/// The type of operation a mutation proposes to perform on the
/// OpenRTB bid request or response.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Operation {
    /// Unspecified operation (invalid for actual mutations).
    #[default]
    Unspecified = 0,

    /// Add new data to the target path.
    Add = 1,

    /// Remove existing data at the target path.
    Remove = 2,

    /// Replace existing data at the target path.
    Replace = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        for value in 0..=3 {
            let json = format!("{}", value);
            let result: Result<Operation, _> = serde_json::from_str(&json);
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
        let result: Result<Operation, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<Operation, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            Operation::Unspecified,
            Operation::Add,
            Operation::Remove,
            Operation::Replace,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Operation = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = Operation::default();
        assert_eq!(
            default,
            Operation::Unspecified,
            "Default should be Unspecified"
        );
    }

    #[test]
    fn test_specific_values() {
        let json = "1";
        let result: Operation = serde_json::from_str(json).unwrap();
        assert_eq!(result, Operation::Add);

        let json = "2";
        let result: Operation = serde_json::from_str(json).unwrap();
        assert_eq!(result, Operation::Remove);

        let json = "3";
        let result: Operation = serde_json::from_str(json).unwrap();
        assert_eq!(result, Operation::Replace);
    }
}
