use serde::{Deserialize, Serialize};

/// The type of pricing tier offered in a package.
///
/// Pricing tiers categorize different pricing models for media offerings.
/// All serialization uses snake_case format (e.g., `"seat"` for `Seat`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum PricingTierType {
    /// Public pricing tier available to all buyers.
    #[default]
    Public,

    /// Seat-based pricing tier for per-user subscriptions.
    Seat,

    /// Agency pricing tier for agency partners.
    Agency,

    /// Advertiser-specific pricing tier.
    Advertiser,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            PricingTierType::Public,
            PricingTierType::Seat,
            PricingTierType::Agency,
            PricingTierType::Advertiser,
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
        let json = "\"nonexistent_tier\"";
        let result: Result<PricingTierType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid tier should fail deserialization");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            PricingTierType::Public,
            PricingTierType::Seat,
            PricingTierType::Agency,
            PricingTierType::Advertiser,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: PricingTierType =
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
        let default = PricingTierType::default();
        assert_eq!(default, PricingTierType::Public, "Default should be Public");
    }
}
