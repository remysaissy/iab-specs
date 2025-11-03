/// OpenRTB 3.0 Request Object
///
/// This module implements the Request object for bid requests.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{Item, Source};

/// Request object (OpenRTB 3.0 Section 3.2)
///
/// The `Request` object contains the top-level bid request information.
/// In OpenRTB 3.0, the request object is wrapped within an [`Openrtb`](super::Openrtb)
/// container and references AdCOM objects for domain-specific data.
///
/// # Key Differences from OpenRTB 2.x
///
/// - Wrapped in [`Openrtb`](super::Openrtb) container
/// - Uses [`Item`] instead of `Imp` for inventory
/// - Context moved to AdCOM (site, app, device, user)
/// - Supply chain promoted to core field
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::{Request, Item};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let request = Request {
///     id: "req-12345".to_string(),
///     test: Some(0),
///     tmax: Some(100),
///     at: Some(2),
///     cur: Some(vec!["USD".to_string()]),
///     item: vec![
///         Item {
///             id: "item1".to_string(),
///             qty: Some(1),
///             ..Default::default()
///         }
///     ],
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Request {
    /// Unique ID of the bid request.
    /// REQUIRED by the specification.
    pub id: String,

    /// Indicator of test mode in which auctions are not billable:
    /// - 0 = live mode (default)
    /// - 1 = test mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<i32>,

    /// Maximum time in milliseconds the exchange allows for bids to be received.
    /// This value is a recommendation but may be enforced by the exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tmax: Option<i32>,

    /// Auction type:
    /// - 1 = First Price
    /// - 2 = Second Price Plus (default)
    /// - 3 = Fixed Price
    ///
    /// Additional auction types can be defined by the exchange using values > 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub at: Option<i32>,

    /// Array of accepted currencies for bids using ISO-4217 codes.
    /// If omitted, the default currency is USD.
    /// Recommended by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cur: Option<Vec<String>>,

    /// Allowlist of buyer seats allowed to bid on this request.
    /// Seat IDs must be communicated between parties beforehand.
    /// Omission implies no restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wseat: Option<Vec<String>>,

    /// Blocklist of buyer seats not allowed to bid on this request.
    /// Seat IDs must be communicated between parties beforehand.
    /// Omission implies no restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bseat: Option<Vec<String>>,

    /// Flag indicating if the Exchange can verify that all impressions offered
    /// represent distinct opportunities:
    /// - 0 = verification not available
    /// - 1 = impressions are verified distinct
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wlang: Option<Vec<String>>,

    /// Array of item objects representing the offered impressions.
    /// At least one item is required.
    /// REQUIRED by the specification (non-empty array).
    #[serde(default)]
    #[builder(default)]
    pub item: Vec<Item>,

    /// Source of the request and any supply chain details.
    /// Includes the supply chain object for transparency.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,

    /// Context object describing the environment (site, app, dooh, device, user).
    /// References AdCOM Context object.
    /// Recommended by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub context: Option<serde_json::Value>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let request = Request {
            id: "req-123".to_string(),
            test: Some(0),
            tmax: Some(100),
            at: Some(2),
            item: vec![Item {
                id: "item1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(request.id, "req-123");
        assert_eq!(request.test, Some(0));
        assert_eq!(request.tmax, Some(100));
        assert_eq!(request.at, Some(2));
        assert_eq!(request.item.len(), 1);
    }

    #[test]
    fn test_request_with_currencies() {
        let request = Request {
            id: "req-123".to_string(),
            cur: Some(vec!["USD".to_string(), "EUR".to_string()]),
            ..Default::default()
        };

        assert!(request.cur.is_some());
        let currencies = request.cur.unwrap();
        assert_eq!(currencies.len(), 2);
        assert!(currencies.contains(&"USD".to_string()));
        assert!(currencies.contains(&"EUR".to_string()));
    }

    #[test]
    fn test_request_serialization() {
        let request = Request {
            id: "req-123".to_string(),
            test: Some(0),
            tmax: Some(100),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"id\":\"req-123\""));
        assert!(json.contains("\"test\":0"));
        assert!(json.contains("\"tmax\":100"));
    }

    #[test]
    fn test_request_deserialization() {
        let json = r#"{
            "id": "req-123",
            "test": 0,
            "tmax": 100,
            "at": 2,
            "item": []
        }"#;

        let request: Request = serde_json::from_str(json).unwrap();
        assert_eq!(request.id, "req-123");
        assert_eq!(request.test, Some(0));
        assert_eq!(request.tmax, Some(100));
        assert_eq!(request.at, Some(2));
    }

    #[test]
    fn test_request_builder() {
        let request = RequestBuilder::default()
            .id("req-123".to_string())
            .test(Some(0))
            .tmax(Some(100))
            .build()
            .unwrap();

        assert_eq!(request.id, "req-123");
        assert_eq!(request.test, Some(0));
        assert_eq!(request.tmax, Some(100));
    }

    #[test]
    fn test_request_with_seat_restrictions() {
        let request = Request {
            id: "req-123".to_string(),
            wseat: Some(vec!["seat1".to_string(), "seat2".to_string()]),
            bseat: Some(vec!["seat3".to_string()]),
            ..Default::default()
        };

        assert_eq!(request.wseat.as_ref().unwrap().len(), 2);
        assert_eq!(request.bseat.as_ref().unwrap().len(), 1);
    }
}
