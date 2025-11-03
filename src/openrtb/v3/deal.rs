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
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::Deal;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let deal = Deal {
///     id: "deal-123".to_string(),
///     flr: Some(5.00),
///     flrcur: Some("USD".to_string()),
///     at: Some(3), // Fixed price
///     wseat: Some(vec!["seat1".to_string(), "seat2".to_string()]),
///     wadomain: Some(vec!["advertiser.com".to_string()]),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Deal {
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
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_creation() {
        let deal = Deal {
            id: "deal-123".to_string(),
            flr: Some(5.00),
            flrcur: Some("USD".to_string()),
            at: Some(3),
            ..Default::default()
        };

        assert_eq!(deal.id, "deal-123");
        assert_eq!(deal.flr, Some(5.00));
        assert_eq!(deal.flrcur, Some("USD".to_string()));
        assert_eq!(deal.at, Some(3));
    }

    #[test]
    fn test_deal_minimal() {
        let deal = Deal {
            id: "deal-456".to_string(),
            ..Default::default()
        };

        assert_eq!(deal.id, "deal-456");
        assert_eq!(deal.flr, None);
        assert_eq!(deal.at, None);
    }

    #[test]
    fn test_deal_with_seat_restrictions() {
        let deal = Deal {
            id: "deal-789".to_string(),
            wseat: Some(vec![
                "seat1".to_string(),
                "seat2".to_string(),
                "seat3".to_string(),
            ]),
            ..Default::default()
        };

        assert_eq!(deal.wseat.as_ref().unwrap().len(), 3);
        assert!(deal.wseat.as_ref().unwrap().contains(&"seat1".to_string()));
    }

    #[test]
    fn test_deal_with_domain_restrictions() {
        let deal = Deal {
            id: "deal-abc".to_string(),
            wadomain: Some(vec![
                "advertiser1.com".to_string(),
                "advertiser2.com".to_string(),
            ]),
            ..Default::default()
        };

        assert_eq!(deal.wadomain.as_ref().unwrap().len(), 2);
        assert!(
            deal.wadomain
                .as_ref()
                .unwrap()
                .contains(&"advertiser1.com".to_string())
        );
    }

    #[test]
    fn test_deal_fixed_price() {
        let deal = Deal {
            id: "deal-fixed".to_string(),
            flr: Some(10.00),
            flrcur: Some("USD".to_string()),
            at: Some(3), // Fixed price
            ..Default::default()
        };

        assert_eq!(deal.at, Some(3));
        assert_eq!(deal.flr, Some(10.00));
    }

    #[test]
    fn test_deal_serialization() {
        let deal = Deal {
            id: "deal-123".to_string(),
            flr: Some(5.00),
            flrcur: Some("USD".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&deal).unwrap();
        assert!(json.contains("\"id\":\"deal-123\""));
        assert!(json.contains("\"flr\":5"));
        assert!(json.contains("\"flrcur\":\"USD\""));
    }

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

    #[test]
    fn test_deal_builder() {
        let deal = DealBuilder::default()
            .id("deal-123".to_string())
            .flr(Some(5.00))
            .flrcur(Some("USD".to_string()))
            .at(Some(3))
            .build()
            .unwrap();

        assert_eq!(deal.id, "deal-123");
        assert_eq!(deal.flr, Some(5.00));
        assert_eq!(deal.at, Some(3));
    }

    #[test]
    fn test_deal_with_multiple_restrictions() {
        let deal = Deal {
            id: "deal-restricted".to_string(),
            flr: Some(8.50),
            wseat: Some(vec!["premium_seat".to_string()]),
            wadomain: Some(vec!["premium_advertiser.com".to_string()]),
            ..Default::default()
        };

        assert_eq!(deal.wseat.as_ref().unwrap().len(), 1);
        assert_eq!(deal.wadomain.as_ref().unwrap().len(), 1);
    }
}
