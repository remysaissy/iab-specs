use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A NegotiationConfig defines the negotiation parameters and strategy
/// for autonomous price negotiations with buyers during deal management.
///
/// The config specifies maximum rounds allowed, concession caps per round and total,
/// and the negotiation strategy type.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::NegotiationConfig;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = NegotiationConfig::builder()
///     .max_rounds(5)
///     .per_round_concession_cap(0.50)
///     .total_concession_cap(2.00)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NegotiationConfig<Ext: Extension = crate::DefaultExt> {
    /// Maximum number of negotiation rounds allowed (REQUIRED).
    /// Limits the number of back-and-forth offers before negotiation ends.
    #[builder(default)]
    pub max_rounds: i32,

    /// Maximum concession per round in currency units (REQUIRED).
    /// Limits the price reduction allowed in each individual round.
    #[builder(default)]
    pub per_round_concession_cap: f64,

    /// Total maximum concession in currency units (REQUIRED).
    /// Limits the cumulative price reduction across all rounds.
    #[builder(default)]
    pub total_concession_cap: f64,

    /// Negotiation strategy type (REQUIRED).
    /// Defines the approach taken during negotiations with buyers.
    #[builder(default)]
    pub strategy: crate::seller_agent::v10::enums::NegotiationStrategyType,

    /// Extension object for config-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl NegotiationConfig {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NegotiationConfigBuilder {
        NegotiationConfigBuilder::create_empty()
    }
}

/// A NegotiationRound represents a single round in a multi-round price negotiation.
///
/// Rounds track the proposed prices from buyer and seller, the concession amount,
/// and the acceptance status of the round.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::NegotiationRound;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let round = NegotiationRound::builder()
///     .round_number(2)
///     .buyer_price(3.50)
///     .seller_price(4.00)
///     .concession(0.25)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NegotiationRound<Ext: Extension = crate::DefaultExt> {
    /// The round number in the negotiation sequence (REQUIRED).
    /// Indicates which round this is (1-based).
    #[builder(default)]
    pub round_number: i32,

    /// The buyer's proposed price in currency units (REQUIRED).
    /// Represents the price the buyer is offering.
    #[builder(default)]
    pub buyer_price: f64,

    /// The seller's proposed price in currency units (REQUIRED).
    /// Represents the price the seller is requesting.
    #[builder(default)]
    pub seller_price: f64,

    /// The concession amount in currency units (REQUIRED).
    /// The price difference that represents the concession made in this round.
    #[builder(default)]
    pub concession: f64,

    /// Whether the round's offer has been accepted (REQUIRED).
    /// Indicates if both parties have agreed on the price.
    #[builder(default)]
    pub accepted: bool,

    /// Extension object for round-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl NegotiationRound {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NegotiationRoundBuilder {
        NegotiationRoundBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== NegotiationConfig Tests ==========

    #[test]
    fn test_negotiation_config_creation() {
        let config = NegotiationConfig::builder()
            .max_rounds(5)
            .per_round_concession_cap(0.50)
            .total_concession_cap(2.00)
            .build()
            .unwrap();

        assert_eq!(config.max_rounds, 5);
        assert_eq!(config.per_round_concession_cap, 0.50);
        assert_eq!(config.total_concession_cap, 2.00);
        assert_eq!(
            config.strategy,
            crate::seller_agent::v10::enums::NegotiationStrategyType::Standard
        );
        assert!(config.ext.is_none());
    }

    #[test]
    fn test_negotiation_config_roundtrip() {
        let original = NegotiationConfig::builder()
            .max_rounds(10)
            .per_round_concession_cap(0.75)
            .total_concession_cap(3.50)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NegotiationConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.max_rounds, 10);
        assert_eq!(parsed.per_round_concession_cap, 0.75);
        assert_eq!(parsed.total_concession_cap, 3.50);
        assert_eq!(parsed.strategy, original.strategy);
    }

    #[test]
    fn test_negotiation_config_with_strategy() {
        let config = NegotiationConfig::builder()
            .max_rounds(8)
            .per_round_concession_cap(0.25)
            .total_concession_cap(1.50)
            .strategy(crate::seller_agent::v10::enums::NegotiationStrategyType::Aggressive)
            .build()
            .unwrap();

        assert_eq!(
            config.strategy,
            crate::seller_agent::v10::enums::NegotiationStrategyType::Aggressive
        );
    }

    #[test]
    fn test_negotiation_config_high_precision() {
        let config = NegotiationConfig::builder()
            .max_rounds(12)
            .per_round_concession_cap(0.555555)
            .total_concession_cap(4.999999)
            .build()
            .unwrap();

        assert_eq!(config.per_round_concession_cap, 0.555555);
        assert_eq!(config.total_concession_cap, 4.999999);

        let json = serde_json::to_string(&config).unwrap();
        let parsed: NegotiationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.per_round_concession_cap, 0.555555);
        assert_eq!(parsed.total_concession_cap, 4.999999);
    }

    // ========== NegotiationRound Tests ==========

    #[test]
    fn test_negotiation_round_creation() {
        let round = NegotiationRound::builder()
            .round_number(3)
            .buyer_price(4.50)
            .seller_price(5.00)
            .concession(0.50)
            .build()
            .unwrap();

        assert_eq!(round.round_number, 3);
        assert_eq!(round.buyer_price, 4.50);
        assert_eq!(round.seller_price, 5.00);
        assert_eq!(round.concession, 0.50);
        assert_eq!(round.accepted, false);
        assert!(round.ext.is_none());
    }

    #[test]
    fn test_negotiation_round_roundtrip() {
        let original = NegotiationRound::builder()
            .round_number(2)
            .buyer_price(3.75)
            .seller_price(4.25)
            .concession(0.25)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: NegotiationRound = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.round_number, 2);
        assert_eq!(parsed.buyer_price, 3.75);
        assert_eq!(parsed.seller_price, 4.25);
        assert_eq!(parsed.concession, 0.25);
        assert_eq!(parsed.accepted, false);
    }

    #[test]
    fn test_negotiation_round_accepted() {
        let round = NegotiationRound::builder()
            .round_number(1)
            .buyer_price(3.00)
            .seller_price(3.50)
            .concession(0.25)
            .accepted(true)
            .build()
            .unwrap();

        assert_eq!(round.accepted, true);
    }

    #[test]
    fn test_negotiation_round_high_precision() {
        let round = NegotiationRound::builder()
            .round_number(4)
            .buyer_price(3.755555)
            .seller_price(4.009999)
            .concession(0.254444)
            .build()
            .unwrap();

        assert_eq!(round.buyer_price, 3.755555);
        assert_eq!(round.seller_price, 4.009999);
        assert_eq!(round.concession, 0.254444);

        let json = serde_json::to_string(&round).unwrap();
        let parsed: NegotiationRound = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.buyer_price, 3.755555);
        assert_eq!(parsed.seller_price, 4.009999);
        assert_eq!(parsed.concession, 0.254444);
    }

    #[test]
    fn test_negotiation_round_serialization() {
        let round = NegotiationRound::builder()
            .round_number(2)
            .buyer_price(2.50)
            .seller_price(3.00)
            .concession(0.25)
            .accepted(false)
            .build()
            .unwrap();

        let json = serde_json::to_string(&round).unwrap();
        assert!(json.contains("\"round_number\":2"));
        assert!(json.contains("\"buyer_price\":2.5"));
        assert!(json.contains("\"seller_price\":3"));
        assert!(json.contains("\"concession\":0.25"));
        assert!(json.contains("\"accepted\":false"));
    }

    // ========== Hardened: NegotiationConfig ==========

    /// Seller Agent 1.0 § NegotiationConfig — default builder yields zero-valued config
    #[test]
    fn test_negotiation_config_default() {
        let config = NegotiationConfig::builder().build().unwrap();
        assert_eq!(config.max_rounds, 0);
        assert_eq!(config.per_round_concession_cap, 0.0);
        assert_eq!(config.total_concession_cap, 0.0);
        assert_eq!(
            config.strategy,
            crate::seller_agent::v10::enums::NegotiationStrategyType::Standard
        );
        assert!(config.ext.is_none());
    }

    /// Seller Agent 1.0 § NegotiationConfig — optional fields omitted from JSON when None
    #[test]
    fn test_negotiation_config_optional_fields_skipped() {
        let config = NegotiationConfig::builder().max_rounds(3).build().unwrap();

        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § NegotiationConfig — clone produces identical value
    #[test]
    fn test_negotiation_config_clone() {
        let config = NegotiationConfig::builder()
            .max_rounds(5)
            .per_round_concession_cap(0.50)
            .total_concession_cap(2.00)
            .strategy(crate::seller_agent::v10::enums::NegotiationStrategyType::Aggressive)
            .build()
            .unwrap();

        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    /// Seller Agent 1.0 § NegotiationConfig — deserialization from minimal JSON
    #[test]
    fn test_negotiation_config_deserialization_minimal() {
        let json = r#"{"max_rounds":3,"per_round_concession_cap":0.5,"total_concession_cap":1.0,"strategy":"standard"}"#;
        let config: NegotiationConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.max_rounds, 3);
        assert_eq!(config.per_round_concession_cap, 0.5);
        assert_eq!(config.total_concession_cap, 1.0);
        assert_eq!(
            config.strategy,
            crate::seller_agent::v10::enums::NegotiationStrategyType::Standard
        );
        assert!(config.ext.is_none());
    }

    // ========== Hardened: NegotiationRound ==========

    /// Seller Agent 1.0 § NegotiationRound — default builder yields zero-valued round
    #[test]
    fn test_negotiation_round_default() {
        let round = NegotiationRound::builder().build().unwrap();
        assert_eq!(round.round_number, 0);
        assert_eq!(round.buyer_price, 0.0);
        assert_eq!(round.seller_price, 0.0);
        assert_eq!(round.concession, 0.0);
        assert!(!round.accepted);
        assert!(round.ext.is_none());
    }

    /// Seller Agent 1.0 § NegotiationRound — optional fields omitted from JSON when None
    #[test]
    fn test_negotiation_round_optional_fields_skipped() {
        let round = NegotiationRound::builder().round_number(1).build().unwrap();

        let json = serde_json::to_string(&round).unwrap();
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § NegotiationRound — clone produces identical value
    #[test]
    fn test_negotiation_round_clone() {
        let round = NegotiationRound::builder()
            .round_number(2)
            .buyer_price(3.50)
            .seller_price(4.00)
            .concession(0.25)
            .accepted(true)
            .build()
            .unwrap();

        let cloned = round.clone();
        assert_eq!(round, cloned);
    }

    /// Seller Agent 1.0 § NegotiationRound — deserialization from minimal JSON
    #[test]
    fn test_negotiation_round_deserialization_minimal() {
        let json = r#"{"round_number":1,"buyer_price":3.0,"seller_price":4.0,"concession":0.5,"accepted":false}"#;
        let round: NegotiationRound = serde_json::from_str(json).unwrap();
        assert_eq!(round.round_number, 1);
        assert_eq!(round.buyer_price, 3.0);
        assert_eq!(round.seller_price, 4.0);
        assert_eq!(round.concession, 0.5);
        assert!(!round.accepted);
        assert!(round.ext.is_none());
    }
}
