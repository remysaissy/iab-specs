use serde::{Deserialize, Serialize};

/// Availability status of advertising inventory.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProductAvailability {
    /// Product inventory is available.
    #[default]
    Available,
    /// Product inventory is limited.
    Limited,
    /// Product inventory is unavailable.
    Unavailable,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("available", ProductAvailability::Available),
            ("limited", ProductAvailability::Limited),
            ("unavailable", ProductAvailability::Unavailable),
        ];

        for (json_str, expected) in values {
            let result: ProductAvailability =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<ProductAvailability, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            ProductAvailability::Available,
            ProductAvailability::Limited,
            ProductAvailability::Unavailable,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ProductAvailability = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = ProductAvailability::default();
        assert_eq!(
            default,
            ProductAvailability::Available,
            "Default should be Available"
        );
    }
}
