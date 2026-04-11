use serde_repr::{Deserialize_repr, Serialize_repr};

/// The type of margin calculation applied to a deal adjustment.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum CalculationType {
    /// Absolute margin adjustment (CPM-based).
    #[default]
    Cpm = 0,

    /// Relative margin adjustment (percentage-based).
    Percent = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        for value in 0..=1 {
            let json = format!("{}", value);
            let result: Result<CalculationType, _> = serde_json::from_str(&json);
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
        let result: Result<CalculationType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<CalculationType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [CalculationType::Cpm, CalculationType::Percent];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CalculationType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = CalculationType::default();
        assert_eq!(default, CalculationType::Cpm, "Default should be Cpm");
    }

    #[test]
    fn test_specific_values() {
        let json = "0";
        let result: CalculationType = serde_json::from_str(json).unwrap();
        assert_eq!(result, CalculationType::Cpm);

        let json = "1";
        let result: CalculationType = serde_json::from_str(json).unwrap();
        assert_eq!(result, CalculationType::Percent);
    }
}
