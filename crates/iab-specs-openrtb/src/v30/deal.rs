use crate::Extension;
/// OpenRTB 3.0 Deal Object
///
/// This module implements the Deal object for private marketplace transactions.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Deal object (OpenRTB 3.0 Section 3.2.2)
///
/// The `Deal` object describes terms of a private marketplace (PMP) deal between
/// a buyer and seller. Deals enable guaranteed inventory, preferred pricing, and
/// other special terms negotiated outside the open auction.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```rust
/// use iab_specs_openrtb::v30::Deal;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let deal = Deal::builder()
///     .id("deal-123".to_string())
///     .flr(Some(5.00))
///     .flrcur(Some("USD".to_string()))
///     .at(Some(3)) // Fixed price
///     .wseat(Some(vec!["seat1".to_string(), "seat2".to_string()]))
///     .wadomain(Some(vec!["advertiser.com".to_string()]))
///     .build()
///     .unwrap();
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Deal<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the deal.
    /// REQUIRED by the specification.
    pub id: String,

    /// Minimum bid floor for this deal in the currency specified by `flrcur`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flr: Option<f64>,

    /// Currency for the deal floor using ISO-4217 codes.
    /// If omitted, USD is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flrcur: Option<String>,

    /// Auction type for this deal:
    /// - 1 = First Price
    /// - 2 = Second Price Plus (default)
    /// - 3 = Fixed Price
    ///
    /// If not specified, uses the auction type from the parent request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub at: Option<i32>,

    /// Allowlist of buyer seats allowed to bid on this deal.
    /// Seat IDs must be communicated between parties beforehand.
    /// Omission implies the deal is available to all seats.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wseat: Option<Vec<String>>,

    /// Allowlist of advertiser domains allowed for this deal.
    /// Omission implies no restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wadomain: Option<Vec<String>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Deal {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DealBuilder {
        DealBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec: Object: Deal — required id with optional flr, flrcur, and at fields
    #[test]
    fn test_deal_creation() {
        let deal = Deal::builder()
            .id("deal-123".to_string())
            .flr(Some(5.00))
            .flrcur(Some("USD".to_string()))
            .at(Some(3))
            .build()
            .unwrap();

        assert_eq!(deal.id, "deal-123");
        assert_eq!(deal.flr, Some(5.00));
        assert_eq!(deal.flrcur, Some("USD".to_string()));
        assert_eq!(deal.at, Some(3));
    }

    // Spec: Object: Deal — minimal deal with id only, all optionals None
    #[test]
    fn test_deal_minimal() {
        let deal = Deal::builder().id("deal-456".to_string()).build().unwrap();

        assert_eq!(deal.id, "deal-456");
        assert_eq!(deal.flr, None);
        assert_eq!(deal.at, None);
    }

    // Spec: Object: Deal — wseat allowlist for buyer seat restrictions
    #[test]
    fn test_deal_with_seat_restrictions() {
        let deal = Deal::builder()
            .id("deal-789".to_string())
            .wseat(Some(vec![
                "seat1".to_string(),
                "seat2".to_string(),
                "seat3".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(deal.wseat.as_ref().unwrap().len(), 3);
        assert!(deal.wseat.as_ref().unwrap().contains(&"seat1".to_string()));
    }

    // Spec: Object: Deal — wadomain allowlist for advertiser domain restrictions
    #[test]
    fn test_deal_with_domain_restrictions() {
        let deal = Deal::builder()
            .id("deal-abc".to_string())
            .wadomain(Some(vec![
                "advertiser1.com".to_string(),
                "advertiser2.com".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(deal.wadomain.as_ref().unwrap().len(), 2);
        assert!(deal
            .wadomain
            .as_ref()
            .unwrap()
            .contains(&"advertiser1.com".to_string()));
    }

    // Spec: Object: Deal — fixed price auction type (at=3) with floor
    #[test]
    fn test_deal_fixed_price() {
        let deal = Deal::builder()
            .id("deal-fixed".to_string())
            .flr(Some(10.00))
            .flrcur(Some("USD".to_string()))
            .at(Some(3))
            .build()
            .unwrap();

        assert_eq!(deal.at, Some(3));
        assert_eq!(deal.flr, Some(10.00));
    }

    // Spec: Object: Deal — JSON serialization of id, flr, flrcur fields
    #[test]
    fn test_deal_serialization() {
        let deal = Deal::builder()
            .id("deal-123".to_string())
            .flr(Some(5.00))
            .flrcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&deal).unwrap();
        assert!(json.contains("\"id\":\"deal-123\""));
        assert!(json.contains("\"flr\":5"));
        assert!(json.contains("\"flrcur\":\"USD\""));
    }

    // Spec: Object: Deal — JSON deserialization with wseat array
    #[test]
    fn test_deal_deserialization() {
        let json = r#"{
            "id": "deal-123",
            "flr": 5.00,
            "flrcur": "USD",
            "at": 3,
            "wseat": ["seat1", "seat2"]
        }"#;

        let deal: Deal = serde_json::from_str(json).unwrap();
        assert_eq!(deal.id, "deal-123");
        assert_eq!(deal.flr, Some(5.00));
        assert_eq!(deal.at, Some(3));
        assert_eq!(deal.wseat.as_ref().unwrap().len(), 2);
    }

    // Spec: Object: Deal — combined wseat and wadomain restrictions
    #[test]
    fn test_deal_with_multiple_restrictions() {
        let deal = Deal::builder()
            .id("deal-restricted".to_string())
            .flr(Some(8.50))
            .wseat(Some(vec!["premium_seat".to_string()]))
            .wadomain(Some(vec!["premium_advertiser.com".to_string()]))
            .build()
            .unwrap();

        assert_eq!(deal.wseat.as_ref().unwrap().len(), 1);
        assert_eq!(deal.wadomain.as_ref().unwrap().len(), 1);
    }

    // Spec: Object: Deal — default() produces empty id and None for all optionals
    #[test]
    fn test_deal_default() {
        let deal: Deal = Deal::default();
        assert_eq!(deal.id, "");
        assert_eq!(deal.flr, None);
        assert_eq!(deal.flrcur, None);
        assert_eq!(deal.at, None);
        assert_eq!(deal.wseat, None);
        assert_eq!(deal.wadomain, None);
        assert_eq!(deal.ext, None);
    }

    // Spec: Object: Deal — serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_deal_roundtrip() {
        let deal = Deal::builder()
            .id("deal-rt".to_string())
            .flr(Some(3.50))
            .flrcur(Some("EUR".to_string()))
            .at(Some(2))
            .wseat(Some(vec!["s1".to_string()]))
            .wadomain(Some(vec!["adv.com".to_string()]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&deal).unwrap();
        let deserialized: Deal = serde_json::from_str(&json).unwrap();
        assert_eq!(deal, deserialized);
    }

    // Spec: Object: Deal — optional fields omitted from JSON when None
    #[test]
    fn test_deal_optional_fields_not_in_json() {
        let deal = Deal::builder().id("deal-min".to_string()).build().unwrap();

        let json = serde_json::to_string(&deal).unwrap();
        assert!(json.contains("\"id\":\"deal-min\""));
        assert!(!json.contains("\"flr\""));
        assert!(!json.contains("\"flrcur\""));
        assert!(!json.contains("\"at\""));
        assert!(!json.contains("\"wseat\""));
        assert!(!json.contains("\"wadomain\""));
        assert!(!json.contains("\"ext\""));
    }

    // Spec: Object: Deal — auction types: 1=first price, 2=second price, 3=fixed price
    #[test]
    fn test_deal_auction_types() {
        let first_price = Deal::builder()
            .id("deal-fp".to_string())
            .at(Some(1))
            .build()
            .unwrap();
        assert_eq!(first_price.at, Some(1));

        let second_price = Deal::builder()
            .id("deal-sp".to_string())
            .at(Some(2))
            .build()
            .unwrap();
        assert_eq!(second_price.at, Some(2));

        let fixed_price = Deal::builder()
            .id("deal-fix".to_string())
            .at(Some(3))
            .flr(Some(15.00))
            .build()
            .unwrap();
        assert_eq!(fixed_price.at, Some(3));
        assert_eq!(fixed_price.flr, Some(15.00));
    }
}
