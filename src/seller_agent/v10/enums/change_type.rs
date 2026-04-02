use serde::{Deserialize, Serialize};

/// The type of change being requested in a change request.
///
/// This enum defines the various categories of modifications that can be made
/// to a seller order or campaign. All serialization uses snake_case format
/// (e.g., `"date_shift"` for `DateShift`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    /// Shift the dates of the campaign or order.
    #[default]
    DateShift,

    /// Adjust the number of impressions.
    ImpressionAdjustment,

    /// Modify the pricing or rate.
    PriceChange,

    /// Cancel the campaign or order.
    Cancellation,

    /// Swap out the creative content.
    CreativeSwap,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            ChangeType::DateShift,
            ChangeType::ImpressionAdjustment,
            ChangeType::PriceChange,
            ChangeType::Cancellation,
            ChangeType::CreativeSwap,
        ];

        for variant in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            assert!(
                serialized.starts_with('"') && serialized.ends_with('"'),
                "Serialized value {} should be a JSON string",
                serialized
            );
            let unquoted = &serialized[1..serialized.len() - 1];
            assert!(
                unquoted.chars().all(|c| c.is_lowercase() || c == '_'),
                "Serialized value {} should be snake_case",
                unquoted
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_type\"";
        let result: Result<ChangeType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid type should fail deserialization");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            ChangeType::DateShift,
            ChangeType::ImpressionAdjustment,
            ChangeType::PriceChange,
            ChangeType::Cancellation,
            ChangeType::CreativeSwap,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: ChangeType =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(
                original, &deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = ChangeType::default();
        assert_eq!(
            default,
            ChangeType::DateShift,
            "Default should be DateShift"
        );
    }
}
