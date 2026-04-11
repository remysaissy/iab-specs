use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A NegotiationStrategy defines the pricing strategy and negotiation parameters
/// for autonomous price negotiation during ad buying.
///
/// The strategy specifies target and maximum CPM prices, concession increments,
/// and the maximum number of negotiation rounds allowed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::NegotiationStrategy;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let strategy = NegotiationStrategy::builder()
///     .target_cpm(2.50)
///     .max_cpm(5.00)
///     .concession_step(0.25)
///     .max_rounds(5)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NegotiationStrategy<Ext: Extension = crate::DefaultExt> {
    /// Target CPM price in currency units (REQUIRED).
    /// Represents the desired price point for negotiations.
    #[builder(default)]
    pub target_cpm: f64,

    /// Maximum CPM price in currency units (REQUIRED).
    /// Represents the highest price willing to pay.
    #[builder(default)]
    pub max_cpm: f64,

    /// Concession step size in currency units (REQUIRED).
    /// Amount to increment in each negotiation round.
    #[builder(default)]
    pub concession_step: f64,

    /// Maximum number of negotiation rounds allowed (REQUIRED).
    #[builder(default)]
    pub max_rounds: i32,

    /// Extension object for strategy-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl NegotiationStrategy {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NegotiationStrategyBuilder {
        NegotiationStrategyBuilder::create_empty()
    }
}

/// A NegotiationOffer represents a single price offer or counter-offer
/// in the negotiation process.
///
/// Offers track the proposed price, direction (buyer or seller initiated),
/// acceptance status, and counter-offers during a negotiation session.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::NegotiationOffer;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let offer = NegotiationOffer::builder()
///     .price(3.75)
///     .round(2)
///     .from_buyer(true)
///     .accepted(Some(false))
///     .counter_price(Some(4.00))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NegotiationOffer<Ext: Extension = crate::DefaultExt> {
    /// Price in currency units (REQUIRED).
    /// The proposed price for this offer.
    #[builder(default)]
    pub price: f64,

    /// Negotiation round number (REQUIRED).
    /// Tracks which round this offer belongs to.
    #[builder(default)]
    pub round: i32,

    /// Whether the offer is from the buyer (REQUIRED).
    /// true = buyer initiated; false = seller initiated.
    #[builder(default)]
    pub from_buyer: bool,

    /// Whether the offer has been accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub accepted: Option<bool>,

    /// Counter-offer price if this offer was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub counter_price: Option<f64>,

    /// Extension object for offer-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl NegotiationOffer {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NegotiationOfferBuilder {
        NegotiationOfferBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== NegotiationStrategy Tests ==========

    #[test]
    fn test_negotiation_strategy_minimal() {
        let strategy = NegotiationStrategy::builder()
            .target_cpm(2.50)
            .max_cpm(5.00)
            .concession_step(0.25)
            .max_rounds(5)
            .build()
            .unwrap();

        assert_eq!(strategy.target_cpm, 2.50);
        assert_eq!(strategy.max_cpm, 5.00);
        assert_eq!(strategy.concession_step, 0.25);
        assert_eq!(strategy.max_rounds, 5);
        assert!(strategy.ext.is_none());
    }

    #[test]
    fn test_negotiation_strategy_precision_roundtrip() {
        let original = NegotiationStrategy::builder()
            .target_cpm(2.50)
            .max_cpm(5.00)
            .concession_step(0.25)
            .max_rounds(5)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NegotiationStrategy = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.target_cpm, 2.50);
        assert_eq!(parsed.max_cpm, 5.00);
        assert_eq!(parsed.concession_step, 0.25);
        assert_eq!(parsed.max_rounds, 5);
    }

    #[test]
    fn test_negotiation_strategy_high_precision_values() {
        let strategy = NegotiationStrategy::builder()
            .target_cpm(2.505555)
            .max_cpm(5.009999)
            .concession_step(0.255555)
            .max_rounds(10)
            .build()
            .unwrap();

        assert_eq!(strategy.target_cpm, 2.505555);
        assert_eq!(strategy.max_cpm, 5.009999);
        assert_eq!(strategy.concession_step, 0.255555);
        assert_eq!(strategy.max_rounds, 10);

        let json = serde_json::to_string(&strategy).unwrap();
        let parsed: NegotiationStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.target_cpm, 2.505555);
        assert_eq!(parsed.max_cpm, 5.009999);
        assert_eq!(parsed.concession_step, 0.255555);
    }

    #[test]
    fn test_negotiation_strategy_zero_values() {
        let strategy = NegotiationStrategy::builder()
            .target_cpm(0.0)
            .max_cpm(0.0)
            .concession_step(0.0)
            .max_rounds(0)
            .build()
            .unwrap();

        assert_eq!(strategy.target_cpm, 0.0);
        assert_eq!(strategy.max_cpm, 0.0);
        assert_eq!(strategy.concession_step, 0.0);
        assert_eq!(strategy.max_rounds, 0);
    }

    #[test]
    fn test_negotiation_strategy_large_values() {
        let strategy = NegotiationStrategy::builder()
            .target_cpm(1000.50)
            .max_cpm(10000.99)
            .concession_step(50.25)
            .max_rounds(100)
            .build()
            .unwrap();

        assert_eq!(strategy.target_cpm, 1000.50);
        assert_eq!(strategy.max_cpm, 10000.99);
        assert_eq!(strategy.concession_step, 50.25);
        assert_eq!(strategy.max_rounds, 100);
    }

    #[test]
    fn test_negotiation_strategy_serialization() {
        let strategy = NegotiationStrategy::builder()
            .target_cpm(3.00)
            .max_cpm(6.00)
            .concession_step(0.50)
            .max_rounds(8)
            .build()
            .unwrap();

        let json = serde_json::to_string(&strategy).unwrap();
        assert!(json.contains("\"target_cpm\":3"));
        assert!(json.contains("\"max_cpm\":6"));
        assert!(json.contains("\"concession_step\":0.5"));
        assert!(json.contains("\"max_rounds\":8"));
    }

    // ========== NegotiationOffer Tests ==========

    #[test]
    fn test_negotiation_offer_minimal() {
        let offer = NegotiationOffer::builder()
            .price(3.75)
            .round(2)
            .from_buyer(true)
            .build()
            .unwrap();

        assert_eq!(offer.price, 3.75);
        assert_eq!(offer.round, 2);
        assert_eq!(offer.from_buyer, true);
        assert!(offer.accepted.is_none());
        assert!(offer.counter_price.is_none());
        assert!(offer.ext.is_none());
    }

    #[test]
    fn test_negotiation_offer_full() {
        let offer = NegotiationOffer::builder()
            .price(3.75)
            .round(2)
            .from_buyer(true)
            .accepted(Some(false))
            .counter_price(Some(4.00))
            .build()
            .unwrap();

        assert_eq!(offer.price, 3.75);
        assert_eq!(offer.round, 2);
        assert_eq!(offer.from_buyer, true);
        assert_eq!(offer.accepted, Some(false));
        assert_eq!(offer.counter_price, Some(4.00));
        assert!(offer.ext.is_none());
    }

    #[test]
    fn test_negotiation_offer_precision_roundtrip() {
        let original = NegotiationOffer::builder()
            .price(3.75)
            .round(2)
            .from_buyer(true)
            .accepted(Some(false))
            .counter_price(Some(4.00))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NegotiationOffer = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.price, 3.75);
        assert_eq!(parsed.round, 2);
        assert_eq!(parsed.from_buyer, true);
        assert_eq!(parsed.accepted, Some(false));
        assert_eq!(parsed.counter_price, Some(4.00));
    }

    #[test]
    fn test_negotiation_offer_from_buyer_false() {
        let offer = NegotiationOffer::builder()
            .price(4.50)
            .round(1)
            .from_buyer(false)
            .build()
            .unwrap();

        assert_eq!(offer.price, 4.50);
        assert_eq!(offer.round, 1);
        assert_eq!(offer.from_buyer, false);
    }

    #[test]
    fn test_negotiation_offer_accepted_true() {
        let offer = NegotiationOffer::builder()
            .price(3.50)
            .round(3)
            .from_buyer(true)
            .accepted(Some(true))
            .build()
            .unwrap();

        assert_eq!(offer.accepted, Some(true));
        assert!(offer.counter_price.is_none());
    }

    #[test]
    fn test_negotiation_offer_high_precision_prices() {
        let offer = NegotiationOffer::builder()
            .price(3.755555)
            .round(2)
            .from_buyer(true)
            .counter_price(Some(4.009999))
            .build()
            .unwrap();

        assert_eq!(offer.price, 3.755555);
        assert_eq!(offer.counter_price, Some(4.009999));

        let json = serde_json::to_string(&offer).unwrap();
        let parsed: NegotiationOffer = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.price, 3.755555);
        assert_eq!(parsed.counter_price, Some(4.009999));
    }

    #[test]
    fn test_negotiation_offer_serialization() {
        let offer = NegotiationOffer::builder()
            .price(2.25)
            .round(1)
            .from_buyer(true)
            .accepted(Some(false))
            .counter_price(Some(2.75))
            .build()
            .unwrap();

        let json = serde_json::to_string(&offer).unwrap();
        assert!(json.contains("\"price\":2.25"));
        assert!(json.contains("\"round\":1"));
        assert!(json.contains("\"from_buyer\":true"));
        assert!(json.contains("\"accepted\":false"));
        assert!(json.contains("\"counter_price\":2.75"));
    }

    #[test]
    fn test_negotiation_offer_deserialization() {
        let json = r#"{
            "price": 3.50,
            "round": 2,
            "from_buyer": false,
            "accepted": true
        }"#;

        let offer: NegotiationOffer = serde_json::from_str(json).unwrap();
        assert_eq!(offer.price, 3.50);
        assert_eq!(offer.round, 2);
        assert_eq!(offer.from_buyer, false);
        assert_eq!(offer.accepted, Some(true));
    }

    #[test]
    fn test_negotiation_offer_roundtrip_partial_fields() {
        let original = NegotiationOffer::builder()
            .price(2.99)
            .round(1)
            .from_buyer(true)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NegotiationOffer = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.price, original.price);
        assert_eq!(parsed.round, original.round);
        assert_eq!(parsed.from_buyer, original.from_buyer);
        assert_eq!(parsed.accepted, original.accepted);
        assert_eq!(parsed.counter_price, original.counter_price);
    }

    #[test]
    fn test_negotiation_strategy_default_trait() {
        let s: NegotiationStrategy = NegotiationStrategy::default();
        assert_eq!(s.target_cpm, 0.0);
        assert_eq!(s.max_cpm, 0.0);
        assert_eq!(s.concession_step, 0.0);
        assert_eq!(s.max_rounds, 0);
        assert!(s.ext.is_none());
    }

    #[test]
    fn test_negotiation_strategy_negative_cpm_accepted() {
        let s = NegotiationStrategy::builder()
            .target_cpm(-1.0)
            .max_cpm(-0.5)
            .build()
            .unwrap();
        assert_eq!(s.target_cpm, -1.0);
        assert_eq!(s.max_cpm, -0.5);
    }

    #[test]
    fn test_negotiation_offer_default_trait() {
        let o: NegotiationOffer = NegotiationOffer::default();
        assert_eq!(o.price, 0.0);
        assert_eq!(o.round, 0);
        assert!(!o.from_buyer);
        assert!(o.accepted.is_none());
        assert!(o.counter_price.is_none());
        assert!(o.ext.is_none());
    }

    #[test]
    fn test_negotiation_offer_round_zero() {
        let o = NegotiationOffer::builder()
            .price(1.0)
            .round(0)
            .from_buyer(true)
            .build()
            .unwrap();
        assert_eq!(o.round, 0);
    }

    #[test]
    fn test_negotiation_strategy_with_json_extension() {
        let s = NegotiationStrategyBuilder::<serde_json::Value>::default()
            .target_cpm(2.0)
            .max_cpm(5.0)
            .concession_step(0.5)
            .max_rounds(3)
            .ext(Some(Box::new(serde_json::json!({"algo": "linear"}))))
            .build()
            .unwrap();

        assert!(s.ext.is_some());
        assert_eq!(s.ext.as_ref().unwrap()["algo"], "linear");

        let json = serde_json::to_string(&s).unwrap();
        let parsed: NegotiationStrategy<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ext.as_ref().unwrap()["algo"], "linear");
    }
}
