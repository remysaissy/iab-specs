use serde::{Deserialize, Serialize};

/// Type of delivery guarantee for advertising inventory.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryType {
    /// Guaranteed delivery inventory.
    #[default]
    Guaranteed,
    /// Non-guaranteed delivery inventory.
    NonGuaranteed,
    /// Programmatic delivery inventory.
    Programmatic,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("guaranteed", DeliveryType::Guaranteed),
            ("non_guaranteed", DeliveryType::NonGuaranteed),
            ("programmatic", DeliveryType::Programmatic),
        ];

        for (json_str, expected) in values {
            let result: DeliveryType = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<DeliveryType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            DeliveryType::Guaranteed,
            DeliveryType::NonGuaranteed,
            DeliveryType::Programmatic,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DeliveryType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = DeliveryType::default();
        assert_eq!(
            default,
            DeliveryType::Guaranteed,
            "Default should be Guaranteed"
        );
    }
}
