use crate::v10::enums::PricingTierType;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents a tiered pricing structure with multiple pricing tiers.
///
/// TieredPricing allows publishers to offer different pricing levels to different
/// buyer segments based on volume, buyer type, or negotiation status.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::TieredPricing;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let pricing = TieredPricing::builder()
///     .tiers(vec![])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct TieredPricing<Ext: Extension = crate::DefaultExt> {
    /// Collection of pricing tiers (REQUIRED).
    /// Defines the available pricing levels.
    #[builder(default)]
    pub tiers: Vec<PricingTier<Ext>>,

    /// Extension object for pricing-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl TieredPricing {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TieredPricingBuilder {
        TieredPricingBuilder::create_empty()
    }
}

/// Represents a single pricing tier with discounts and negotiation settings.
///
/// PricingTier defines pricing and terms for a specific buyer segment or volume level.
/// Each tier can have different discount rates and negotiation capabilities.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::PricingTier;
/// use iab_specs_seller_agent::v10::enums::PricingTierType;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tier = PricingTier::builder()
///     .tier_type(PricingTierType::Public)
///     .discount_percent(0.0)
///     .negotiation_enabled(false)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct PricingTier<Ext: Extension = crate::DefaultExt> {
    /// Type of pricing tier (REQUIRED).
    /// Categorizes the tier by buyer segment.
    #[builder(default)]
    pub tier_type: PricingTierType,

    /// Discount percentage applied to this tier (REQUIRED).
    /// Value between 0.0 and 100.0 representing the discount percentage.
    #[builder(default)]
    pub discount_percent: f64,

    /// Whether price negotiation is enabled for this tier (REQUIRED).
    /// Controls if buyers can negotiate beyond published rates.
    #[builder(default)]
    pub negotiation_enabled: bool,

    /// Minimum spend threshold for this tier.
    /// Optional floor on spend to qualify for this tier's rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_spend: Option<f64>,

    /// Extension object for tier-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl PricingTier {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PricingTierBuilder {
        PricingTierBuilder::create_empty()
    }
}

/// Represents a rate card defining pricing for a specific product.
///
/// RateCard establishes base, floor, and ceiling prices for media inventory,
/// typically used for CPM (cost per mille/thousand) pricing models.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::RateCard;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let card = RateCard::builder()
///     .product_id("prod-123")
///     .base_cpm(2.50)
///     .floor_cpm(1.50)
///     .currency("USD")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct RateCard<Ext: Extension = crate::DefaultExt> {
    /// Product identifier (REQUIRED).
    /// Unique identifier for the media product or inventory.
    #[builder(default, setter(into))]
    pub product_id: String,

    /// Base CPM price in currency units (REQUIRED).
    /// The standard price point for this product.
    #[builder(default)]
    pub base_cpm: f64,

    /// Floor CPM price in currency units (REQUIRED).
    /// The minimum acceptable price for this product.
    #[builder(default)]
    pub floor_cpm: f64,

    /// Ceiling CPM price in currency units.
    /// Optional maximum price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ceiling_cpm: Option<f64>,

    /// Currency code (REQUIRED).
    /// ISO 4217 three-letter currency code (e.g., "USD", "EUR").
    #[builder(default, setter(into))]
    pub currency: String,

    /// Extension object for rate card-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl RateCard {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RateCardBuilder {
        RateCardBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== TieredPricing Tests ==========

    #[test]
    fn test_tiered_pricing_with_four_tiers() {
        let pricing = TieredPricing::builder()
            .tiers(vec![
                PricingTier::builder()
                    .tier_type(PricingTierType::Public)
                    .discount_percent(0.0)
                    .negotiation_enabled(false)
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Seat)
                    .discount_percent(5.0)
                    .negotiation_enabled(true)
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Agency)
                    .discount_percent(10.0)
                    .negotiation_enabled(true)
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Advertiser)
                    .discount_percent(15.0)
                    .negotiation_enabled(true)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(pricing.tiers.len(), 4);
        assert_eq!(pricing.tiers[0].tier_type, PricingTierType::Public);
        assert_eq!(pricing.tiers[0].discount_percent, 0.0);
        assert_eq!(pricing.tiers[1].tier_type, PricingTierType::Seat);
        assert_eq!(pricing.tiers[1].discount_percent, 5.0);
        assert_eq!(pricing.tiers[2].tier_type, PricingTierType::Agency);
        assert_eq!(pricing.tiers[2].discount_percent, 10.0);
        assert_eq!(pricing.tiers[3].tier_type, PricingTierType::Advertiser);
        assert_eq!(pricing.tiers[3].discount_percent, 15.0);

        // Test roundtrip
        let json = serde_json::to_string(&pricing).unwrap();
        let parsed: TieredPricing = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, pricing);
    }

    // ========== PricingTier Tests ==========

    #[test]
    fn test_pricing_tier_creation() {
        let tier = PricingTier::builder()
            .tier_type(PricingTierType::Agency)
            .discount_percent(10.0)
            .negotiation_enabled(true)
            .min_spend(Some(5000.0))
            .build()
            .unwrap();

        assert_eq!(tier.tier_type, PricingTierType::Agency);
        assert_eq!(tier.discount_percent, 10.0);
        assert_eq!(tier.negotiation_enabled, true);
        assert_eq!(tier.min_spend, Some(5000.0));
        assert!(tier.ext.is_none());
    }

    #[test]
    fn test_pricing_tier_default() {
        let tier = PricingTier::builder().build().unwrap();

        assert_eq!(tier.tier_type, PricingTierType::Public);
        assert_eq!(tier.discount_percent, 0.0);
        assert_eq!(tier.negotiation_enabled, false);
        assert!(tier.min_spend.is_none());
        assert!(tier.ext.is_none());
    }

    // ========== RateCard Tests ==========

    #[test]
    fn test_rate_card_creation() {
        let card = RateCard::builder()
            .product_id("prod-abc-123")
            .base_cpm(2.50)
            .floor_cpm(1.50)
            .ceiling_cpm(Some(5.00))
            .currency("USD")
            .build()
            .unwrap();

        assert_eq!(card.product_id, "prod-abc-123");
        assert_eq!(card.base_cpm, 2.50);
        assert_eq!(card.floor_cpm, 1.50);
        assert_eq!(card.ceiling_cpm, Some(5.00));
        assert_eq!(card.currency, "USD");
        assert!(card.ext.is_none());
    }

    #[test]
    fn test_rate_card_roundtrip() {
        let original = RateCard::builder()
            .product_id("prod-123")
            .base_cpm(3.75)
            .floor_cpm(2.00)
            .ceiling_cpm(Some(7.50))
            .currency("EUR")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RateCard = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.product_id, "prod-123");
        assert_eq!(parsed.base_cpm, 3.75);
        assert_eq!(parsed.floor_cpm, 2.00);
        assert_eq!(parsed.ceiling_cpm, Some(7.50));
        assert_eq!(parsed.currency, "EUR");
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_rate_card_without_ceiling() {
        let card = RateCard::builder()
            .product_id("prod-no-ceiling")
            .base_cpm(1.50)
            .floor_cpm(0.75)
            .currency("GBP")
            .build()
            .unwrap();

        assert_eq!(card.product_id, "prod-no-ceiling");
        assert_eq!(card.base_cpm, 1.50);
        assert_eq!(card.floor_cpm, 0.75);
        assert!(card.ceiling_cpm.is_none());
        assert_eq!(card.currency, "GBP");
    }

    #[test]
    fn test_rate_card_high_precision_prices() {
        let card = RateCard::builder()
            .product_id("prod-precision")
            .base_cpm(2.505555)
            .floor_cpm(1.009999)
            .ceiling_cpm(Some(5.999999))
            .currency("USD")
            .build()
            .unwrap();

        assert_eq!(card.base_cpm, 2.505555);
        assert_eq!(card.floor_cpm, 1.009999);
        assert_eq!(card.ceiling_cpm, Some(5.999999));

        let json = serde_json::to_string(&card).unwrap();
        let parsed: RateCard = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.base_cpm, 2.505555);
        assert_eq!(parsed.floor_cpm, 1.009999);
        assert_eq!(parsed.ceiling_cpm, Some(5.999999));
    }

    /// Seller Agent 1.0 § TieredPricing — default builder yields empty tiers
    #[test]
    fn test_tiered_pricing_default() {
        let pricing = TieredPricing::builder().build().unwrap();
        assert!(pricing.tiers.is_empty());
        assert!(pricing.ext.is_none());
    }

    /// Seller Agent 1.0 § TieredPricing — optional fields omitted from JSON when None
    #[test]
    fn test_tiered_pricing_optional_fields_skipped() {
        let pricing = TieredPricing::builder().build().unwrap();
        let json = serde_json::to_string(&pricing).unwrap();
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § TieredPricing — clone produces identical value
    #[test]
    fn test_tiered_pricing_clone() {
        let pricing = TieredPricing::builder()
            .tiers(vec![PricingTier::builder()
                .tier_type(PricingTierType::Agency)
                .discount_percent(10.0)
                .negotiation_enabled(true)
                .build()
                .unwrap()])
            .build()
            .unwrap();

        let cloned = pricing.clone();
        assert_eq!(pricing, cloned);
    }

    /// Seller Agent 1.0 § TieredPricing — deserialization from minimal JSON
    #[test]
    fn test_tiered_pricing_deserialization_minimal() {
        let json = r#"{"tiers":[]}"#;
        let pricing: TieredPricing = serde_json::from_str(json).unwrap();
        assert!(pricing.tiers.is_empty());
        assert!(pricing.ext.is_none());
    }

    /// Seller Agent 1.0 § PricingTier — optional fields omitted from JSON when None
    #[test]
    fn test_pricing_tier_optional_fields_skipped() {
        let tier = PricingTier::builder()
            .tier_type(PricingTierType::Public)
            .discount_percent(0.0)
            .negotiation_enabled(false)
            .build()
            .unwrap();

        let json = serde_json::to_string(&tier).unwrap();
        assert!(!json.contains("\"min_spend\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § PricingTier — clone produces identical value
    #[test]
    fn test_pricing_tier_clone() {
        let tier = PricingTier::builder()
            .tier_type(PricingTierType::Seat)
            .discount_percent(5.0)
            .negotiation_enabled(true)
            .min_spend(Some(1000.0))
            .build()
            .unwrap();

        let cloned = tier.clone();
        assert_eq!(tier, cloned);
    }

    /// Seller Agent 1.0 § PricingTier — deserialization from minimal JSON
    #[test]
    fn test_pricing_tier_deserialization_minimal() {
        let json = r#"{"tier_type":"public","discount_percent":0.0,"negotiation_enabled":false}"#;
        let tier: PricingTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.tier_type, PricingTierType::Public);
        assert_eq!(tier.discount_percent, 0.0);
        assert!(!tier.negotiation_enabled);
        assert!(tier.min_spend.is_none());
    }

    /// Seller Agent 1.0 § RateCard — default builder yields empty rate card
    #[test]
    fn test_rate_card_default() {
        let card = RateCard::builder().build().unwrap();
        assert_eq!(card.product_id, "");
        assert_eq!(card.base_cpm, 0.0);
        assert_eq!(card.floor_cpm, 0.0);
        assert!(card.ceiling_cpm.is_none());
        assert_eq!(card.currency, "");
        assert!(card.ext.is_none());
    }

    /// Seller Agent 1.0 § RateCard — optional fields omitted from JSON when None
    #[test]
    fn test_rate_card_optional_fields_skipped() {
        let card = RateCard::builder()
            .product_id("p")
            .base_cpm(1.0)
            .floor_cpm(0.5)
            .currency("USD")
            .build()
            .unwrap();

        let json = serde_json::to_string(&card).unwrap();
        assert!(!json.contains("\"ceiling_cpm\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § RateCard — clone produces identical value
    #[test]
    fn test_rate_card_clone() {
        let card = RateCard::builder()
            .product_id("prod-c")
            .base_cpm(3.0)
            .floor_cpm(1.5)
            .ceiling_cpm(Some(6.0))
            .currency("EUR")
            .build()
            .unwrap();

        let cloned = card.clone();
        assert_eq!(card, cloned);
    }

    /// Seller Agent 1.0 § RateCard — deserialization from minimal JSON
    #[test]
    fn test_rate_card_deserialization_minimal() {
        let json = r#"{"product_id":"p","base_cpm":2.5,"floor_cpm":1.0,"currency":"USD"}"#;
        let card: RateCard = serde_json::from_str(json).unwrap();
        assert_eq!(card.product_id, "p");
        assert_eq!(card.base_cpm, 2.5);
        assert_eq!(card.floor_cpm, 1.0);
        assert_eq!(card.currency, "USD");
        assert!(card.ceiling_cpm.is_none());
    }
}
