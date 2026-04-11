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

    /// Seller Agent 1.0 § ChangeType — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = ChangeType::DateShift;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, ChangeType::DateShift);
    }

    /// Seller Agent 1.0 § ChangeType — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ChangeType::DateShift);
        set.insert(ChangeType::ImpressionAdjustment);
        set.insert(ChangeType::PriceChange);
        set.insert(ChangeType::Cancellation);
        set.insert(ChangeType::CreativeSwap);

        assert_eq!(set.len(), 5);
        assert!(set.contains(&ChangeType::DateShift));
        assert!(set.contains(&ChangeType::CreativeSwap));
    }

    /// Seller Agent 1.0 § ChangeType — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(ChangeType::DateShift, ChangeType::ImpressionAdjustment);
        assert_ne!(ChangeType::ImpressionAdjustment, ChangeType::PriceChange);
        assert_ne!(ChangeType::PriceChange, ChangeType::Cancellation);
        assert_ne!(ChangeType::Cancellation, ChangeType::CreativeSwap);
    }

    /// Seller Agent 1.0 § ChangeType — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"DateShift\"", "\"ImpressionAdjustment\""];

        for example in &pascal_case_examples {
            let result: Result<ChangeType, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § ChangeType — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (ChangeType::DateShift, "\"date_shift\""),
            (
                ChangeType::ImpressionAdjustment,
                "\"impression_adjustment\"",
            ),
            (ChangeType::PriceChange, "\"price_change\""),
            (ChangeType::Cancellation, "\"cancellation\""),
            (ChangeType::CreativeSwap, "\"creative_swap\""),
        ];

        for (variant, expected_json) in &expected {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(
                &json, expected_json,
                "Mismatch for {:?}: got {}, expected {}",
                variant, json, expected_json
            );
        }
    }
}
