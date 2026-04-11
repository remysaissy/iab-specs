use serde_repr::{Deserialize_repr, Serialize_repr};

/// The type of business entity that originates the RTB request or response.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum OriginatorType {
    /// Unspecified originator type.
    #[default]
    Unspecified = 0,

    /// Publisher (content owner/operator).
    Publisher = 1,

    /// Supply-Side Platform.
    Ssp = 2,

    /// Ad Exchange.
    Exchange = 3,

    /// Demand-Side Platform.
    Dsp = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        for value in 0..=4 {
            let json = format!("{}", value);
            let result: Result<OriginatorType, _> = serde_json::from_str(&json);
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
        let result: Result<OriginatorType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<OriginatorType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            OriginatorType::Unspecified,
            OriginatorType::Publisher,
            OriginatorType::Ssp,
            OriginatorType::Exchange,
            OriginatorType::Dsp,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: OriginatorType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = OriginatorType::default();
        assert_eq!(
            default,
            OriginatorType::Unspecified,
            "Default should be Unspecified"
        );
    }

    #[test]
    fn test_specific_values() {
        let json = "1";
        let result: OriginatorType = serde_json::from_str(json).unwrap();
        assert_eq!(result, OriginatorType::Publisher);

        let json = "2";
        let result: OriginatorType = serde_json::from_str(json).unwrap();
        assert_eq!(result, OriginatorType::Ssp);

        let json = "3";
        let result: OriginatorType = serde_json::from_str(json).unwrap();
        assert_eq!(result, OriginatorType::Exchange);

        let json = "4";
        let result: OriginatorType = serde_json::from_str(json).unwrap();
        assert_eq!(result, OriginatorType::Dsp);
    }
}
