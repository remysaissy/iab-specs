use serde_repr::{Deserialize_repr, Serialize_repr};

/// Size unit.
///
/// Units of measurement for sizes.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SizeUnit {
    /// Device Independent Pixels (DIPS)
    Dips = 1,

    /// Physical pixels
    Pixels = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid SizeUnit values (1-2)
        for value in 1..=2 {
            let json = format!("{}", value);
            let result: Result<SizeUnit, _> = serde_json::from_str(&json);
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
        let result: Result<SizeUnit, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid SizeUnit and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<SizeUnit, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<SizeUnit, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [SizeUnit::Dips, SizeUnit::Pixels];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: SizeUnit = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
