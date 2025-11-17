use serde_repr::{Deserialize_repr, Serialize_repr};

/// Production quality.
///
/// The production quality of the content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ProductionQuality {
    /// Unknown
    Unknown = 0,

    /// Professionally Produced
    Professional = 1,

    /// Prosumer
    Prosumer = 2,

    /// User Generated (UGC)
    UserGenerated = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid ProductionQuality values (0-3)
        for value in 0..=3 {
            let json = format!("{}", value);
            let result: Result<ProductionQuality, _> = serde_json::from_str(&json);
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
        let result: Result<ProductionQuality, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<ProductionQuality, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            ProductionQuality::Unknown,
            ProductionQuality::Professional,
            ProductionQuality::Prosumer,
            ProductionQuality::UserGenerated,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ProductionQuality = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
