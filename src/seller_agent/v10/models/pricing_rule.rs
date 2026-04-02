use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents a dynamic pricing rule that adjusts prices based on conditions.
///
/// PricingRule applies adjustments to base prices when specific conditions are met,
/// enabling dynamic pricing strategies based on buyer characteristics, volume,
/// timing, or other configurable factors.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::PricingRule;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let rule = PricingRule::builder()
///     .condition(serde_json::json!({"buyer_type": "agency"}))
///     .adjustment(0.10)
///     .adjustment_type("multiplier".to_string())
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct PricingRule<Ext: Extension = crate::DefaultExt> {
    /// Condition for applying this rule (REQUIRED).
    /// A JSON value representing the condition to be evaluated.
    /// Examples: {"buyer_type": "agency"}, {"impression_volume": {"min": 1000}}, {"region": "US"}
    #[builder(default)]
    pub condition: serde_json::Value,

    /// Adjustment value to apply (REQUIRED).
    /// The magnitude of the price adjustment (e.g., 0.10 for 10% increase, -0.05 for 5% decrease).
    #[builder(default)]
    pub adjustment: f64,

    /// Type of adjustment to apply (REQUIRED).
    /// Specifies how the adjustment is applied: "multiplier", "flat", or "percentage".
    #[builder(default, setter(into))]
    pub adjustment_type: String,

    /// Extension object for rule-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl PricingRule {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PricingRuleBuilder {
        PricingRuleBuilder::create_empty()
    }
}

/// Represents a volume discount tier offering reduced pricing at higher volumes.
///
/// VolumeDiscount defines pricing breaks based on impression volume thresholds,
/// enabling progressive discounting for high-volume buyers or campaigns.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::VolumeDiscount;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let discount = VolumeDiscount::builder()
///     .threshold(100000)
///     .discount_percent(5.0)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct VolumeDiscount<Ext: Extension = crate::DefaultExt> {
    /// Impression volume threshold (REQUIRED).
    /// Minimum number of impressions required to qualify for this discount tier.
    #[builder(default)]
    pub threshold: i64,

    /// Discount percentage for this tier (REQUIRED).
    /// Value between 0.0 and 100.0 representing the discount percentage applied.
    #[builder(default)]
    pub discount_percent: f64,

    /// Extension object for discount-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl VolumeDiscount {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VolumeDiscountBuilder {
        VolumeDiscountBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== PricingRule Tests ==========

    #[test]
    fn test_pricing_rule_creation() {
        let rule = PricingRule::builder()
            .condition(serde_json::json!({"buyer_type": "agency"}))
            .adjustment(0.10)
            .adjustment_type("multiplier".to_string())
            .build()
            .unwrap();

        assert_eq!(rule.condition, serde_json::json!({"buyer_type": "agency"}));
        assert_eq!(rule.adjustment, 0.10);
        assert_eq!(rule.adjustment_type, "multiplier");
        assert!(rule.ext.is_none());
    }

    #[test]
    fn test_pricing_rule_json_condition() {
        let rule = PricingRule::builder()
            .condition(serde_json::json!({
                "impression_volume": {"min": 1000, "max": 10000},
                "region": "US"
            }))
            .adjustment(-0.05)
            .adjustment_type("percentage".to_string())
            .build()
            .unwrap();

        assert_eq!(rule.adjustment, -0.05);
        assert_eq!(rule.adjustment_type, "percentage");
        assert!(rule.condition.is_object());
    }

    #[test]
    fn test_pricing_rule_roundtrip() {
        let original = PricingRule::builder()
            .condition(serde_json::json!({"dayofweek": "weekend"}))
            .adjustment(0.25)
            .adjustment_type("flat".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: PricingRule = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.condition, original.condition);
        assert_eq!(parsed.adjustment, original.adjustment);
        assert_eq!(parsed.adjustment_type, original.adjustment_type);
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_pricing_rule_complex_json() {
        let condition = serde_json::json!({
            "rules": [
                {"type": "audience", "id": "aud-123"},
                {"type": "geography", "countries": ["US", "CA", "MX"]}
            ]
        });

        let rule = PricingRule::builder()
            .condition(condition.clone())
            .adjustment(1.5)
            .adjustment_type("multiplier".to_string())
            .build()
            .unwrap();

        assert_eq!(rule.condition, condition);

        let json = serde_json::to_string(&rule).unwrap();
        let parsed: PricingRule = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.condition, condition);
    }

    #[test]
    fn test_pricing_rule_negative_adjustment() {
        let rule = PricingRule::builder()
            .condition(serde_json::json!({"is_clearance": true}))
            .adjustment(-0.50)
            .adjustment_type("percentage".to_string())
            .build()
            .unwrap();

        assert_eq!(rule.adjustment, -0.50);
        assert!(rule.adjustment < 0.0);
    }

    // ========== VolumeDiscount Tests ==========

    #[test]
    fn test_volume_discount_creation() {
        let discount = VolumeDiscount::builder()
            .threshold(100000)
            .discount_percent(5.0)
            .build()
            .unwrap();

        assert_eq!(discount.threshold, 100000);
        assert_eq!(discount.discount_percent, 5.0);
        assert!(discount.ext.is_none());
    }

    #[test]
    fn test_volume_discount_multiple_tiers() {
        let tier1 = VolumeDiscount::builder()
            .threshold(50000)
            .discount_percent(2.5)
            .build()
            .unwrap();

        let tier2 = VolumeDiscount::builder()
            .threshold(100000)
            .discount_percent(5.0)
            .build()
            .unwrap();

        let tier3 = VolumeDiscount::builder()
            .threshold(500000)
            .discount_percent(10.0)
            .build()
            .unwrap();

        assert_eq!(tier1.threshold, 50000);
        assert_eq!(tier1.discount_percent, 2.5);
        assert_eq!(tier2.threshold, 100000);
        assert_eq!(tier2.discount_percent, 5.0);
        assert_eq!(tier3.threshold, 500000);
        assert_eq!(tier3.discount_percent, 10.0);
    }

    #[test]
    fn test_volume_discount_roundtrip() {
        let original = VolumeDiscount::builder()
            .threshold(250000)
            .discount_percent(7.5)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: VolumeDiscount = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.threshold, 250000);
        assert_eq!(parsed.discount_percent, 7.5);
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_volume_discount_zero_threshold() {
        let discount = VolumeDiscount::builder()
            .threshold(0)
            .discount_percent(0.0)
            .build()
            .unwrap();

        assert_eq!(discount.threshold, 0);
        assert_eq!(discount.discount_percent, 0.0);
    }

    #[test]
    fn test_volume_discount_large_threshold() {
        let discount = VolumeDiscount::builder()
            .threshold(10000000)
            .discount_percent(25.0)
            .build()
            .unwrap();

        assert_eq!(discount.threshold, 10000000);
        assert_eq!(discount.discount_percent, 25.0);
    }

    #[test]
    fn test_volume_discount_high_precision() {
        let discount = VolumeDiscount::builder()
            .threshold(123456)
            .discount_percent(7.555555)
            .build()
            .unwrap();

        assert_eq!(discount.threshold, 123456);
        assert_eq!(discount.discount_percent, 7.555555);

        let json = serde_json::to_string(&discount).unwrap();
        let parsed: VolumeDiscount = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.discount_percent, 7.555555);
    }
}
