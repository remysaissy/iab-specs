use super::{Item, Source};
use crate::Extension;
/// OpenRTB 3.0 Request Object
///
/// This module implements the Request object for bid requests.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `ContextExt` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, ContextExt: Extension",
    deserialize = "Ext: Extension, ContextExt: Extension"
))]
pub struct Request<Ext: Extension = crate::DefaultExt, ContextExt: Extension = crate::DefaultExt> {
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
    pub context: Option<Box<ContextExt>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Request {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RequestBuilder {
        RequestBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec: Object: Request — id, test, tmax, at fields and item vec populated correctly
    #[test]
    fn test_request_creation() {
        let item = Item::builder().id("item1".to_string()).build().unwrap();

        let request = Request::builder()
            .id("req-123".to_string())
            .test(Some(0))
            .tmax(Some(100))
            .at(Some(2))
            .item(vec![item])
            .build()
            .unwrap();

        assert_eq!(request.id, "req-123");
        assert_eq!(request.test, Some(0));
        assert_eq!(request.tmax, Some(100));
        assert_eq!(request.at, Some(2));
        assert_eq!(request.item.len(), 1);
    }

    // Spec: Object: Request — cur field accepts multiple ISO-4217 currency codes
    #[test]
    fn test_request_with_currencies() {
        let request = Request::builder()
            .id("req-123".to_string())
            .cur(Some(vec!["USD".to_string(), "EUR".to_string()]))
            .build()
            .unwrap();

        assert!(request.cur.is_some());
        let currencies = request.cur.unwrap();
        assert_eq!(currencies.len(), 2);
        assert!(currencies.contains(&"USD".to_string()));
        assert!(currencies.contains(&"EUR".to_string()));
    }

    // Spec: Object: Request — serialization produces correct JSON keys for id, test, tmax
    #[test]
    fn test_request_serialization() {
        let request = Request::builder()
            .id("req-123".to_string())
            .test(Some(0))
            .tmax(Some(100))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"id\":\"req-123\""));
        assert!(json.contains("\"test\":0"));
        assert!(json.contains("\"tmax\":100"));
    }

    // Spec: Object: Request — deserialization from JSON restores id, test, tmax, at fields
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

    // Spec: Object: Request — wseat and bseat allowlist/blocklist seat restrictions
    #[test]
    fn test_request_with_seat_restrictions() {
        let request = Request::builder()
            .id("req-123".to_string())
            .wseat(Some(vec!["seat1".to_string(), "seat2".to_string()]))
            .bseat(Some(vec!["seat3".to_string()]))
            .build()
            .unwrap();

        assert_eq!(request.wseat.as_ref().unwrap().len(), 2);
        assert_eq!(request.bseat.as_ref().unwrap().len(), 1);
    }

    // Spec: Object: Request — default() produces empty id, empty item vec, all Options None
    #[test]
    fn test_request_default() {
        let request: Request = Request::default();
        assert_eq!(request.id, "");
        assert!(request.item.is_empty());
        assert!(request.test.is_none());
        assert!(request.tmax.is_none());
        assert!(request.at.is_none());
        assert!(request.cur.is_none());
        assert!(request.wseat.is_none());
        assert!(request.bseat.is_none());
        assert!(request.wlang.is_none());
        assert!(request.source.is_none());
        assert!(request.context.is_none());
        assert!(request.ext.is_none());
    }

    // Spec: Object: Request — roundtrip serialize/deserialize preserves all fields
    #[test]
    fn test_request_roundtrip() {
        let original = Request::builder()
            .id("req-rt".to_string())
            .test(Some(1))
            .tmax(Some(200))
            .at(Some(1))
            .cur(Some(vec!["USD".to_string()]))
            .item(vec![
                Item::builder().id("item1".to_string()).build().unwrap(),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    // Spec: Object: Request — skip_serializing_if omits None optional fields from JSON
    #[test]
    fn test_request_optional_fields_not_in_json() {
        let request = Request::builder()
            .id("req-minimal".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"id\""));
        assert!(!json.contains("\"test\""));
        assert!(!json.contains("\"tmax\""));
        assert!(!json.contains("\"at\""));
        assert!(!json.contains("\"cur\""));
        assert!(!json.contains("\"wseat\""));
        assert!(!json.contains("\"bseat\""));
        assert!(!json.contains("\"wlang\""));
        assert!(!json.contains("\"source\""));
        assert!(!json.contains("\"context\""));
        assert!(!json.contains("\"ext\""));
    }

    // Spec: Object: Request — source field accepts a Source object
    #[test]
    fn test_request_with_source() {
        let source = Source::builder()
            .tid(Some("txn-123".to_string()))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-source".to_string())
            .source(Some(source))
            .build()
            .unwrap();

        assert!(request.source.is_some());
        assert_eq!(
            request.source.as_ref().unwrap().tid,
            Some("txn-123".to_string())
        );
    }

    // Spec: Object: Request — wlang field serializes and round-trips correctly
    #[test]
    fn test_request_with_wlang() {
        let request = Request::builder()
            .id("req-wlang".to_string())
            .wlang(Some(vec!["en".to_string()]))
            .build()
            .unwrap();

        assert_eq!(request.wlang, Some(vec!["en".to_string()]));

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.wlang, Some(vec!["en".to_string()]));
    }

    // Spec: Object: Request — all optional fields populated and accessible after build
    #[test]
    fn test_request_with_all_optional_fields() {
        let source = Source::builder()
            .tid(Some("txn-all".to_string()))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-all".to_string())
            .test(Some(1))
            .tmax(Some(300))
            .at(Some(2))
            .cur(Some(vec!["USD".to_string(), "EUR".to_string()]))
            .wseat(Some(vec!["seat1".to_string()]))
            .bseat(Some(vec!["seat2".to_string()]))
            .wlang(Some(vec!["en".to_string(), "fr".to_string()]))
            .item(vec![
                Item::builder().id("item1".to_string()).build().unwrap(),
            ])
            .source(Some(source))
            .build()
            .unwrap();

        assert_eq!(request.id, "req-all");
        assert_eq!(request.test, Some(1));
        assert_eq!(request.tmax, Some(300));
        assert_eq!(request.at, Some(2));
        assert_eq!(request.cur.as_ref().unwrap().len(), 2);
        assert_eq!(request.wseat.as_ref().unwrap().len(), 1);
        assert_eq!(request.bseat.as_ref().unwrap().len(), 1);
        assert_eq!(request.wlang.as_ref().unwrap().len(), 2);
        assert_eq!(request.item.len(), 1);
        assert!(request.source.is_some());
    }
}
