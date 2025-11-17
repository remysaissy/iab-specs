use serde_repr::{Deserialize_repr, Serialize_repr};

/// DOOH multiplier measurement source types.
///
/// Identifies the entity providing quantity measurement for impression multipliers
/// in Digital Out-of-Home advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DOOHMultiplierMeasurementSource {
    /// Unknown source
    Unknown = 0,

    /// Measurement vendor provided
    MeasurementVendor = 1,

    /// Publisher provided
    Publisher = 2,

    /// Exchange provided
    Exchange = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid DOOHMultiplierMeasurementSource values (0-3)
        for value in 0..=3 {
            let json = format!("{}", value);
            let result: Result<DOOHMultiplierMeasurementSource, _> = serde_json::from_str(&json);
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
        let result: Result<DOOHMultiplierMeasurementSource, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<DOOHMultiplierMeasurementSource, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            DOOHMultiplierMeasurementSource::Unknown,
            DOOHMultiplierMeasurementSource::MeasurementVendor,
            DOOHMultiplierMeasurementSource::Publisher,
            DOOHMultiplierMeasurementSource::Exchange,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DOOHMultiplierMeasurementSource =
                serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
