/// OpenRTB 3.0 Item Object
///
/// This module implements the Item object for inventory/impression items.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{Deal, Metric};

/// Item object (OpenRTB 3.0 Section 3.2.1)
///
/// The `Item` object represents a unit of goods being offered for sale.
/// In OpenRTB 3.0, this replaces the `Imp` object from 2.x and represents
/// an opportunity for ad placement.
///
/// # Key Differences from OpenRTB 2.x `Imp`
///
/// - Renamed from `Imp` to `Item` for clarity
/// - `spec` field references AdCOM placement specifications
/// - Simplified structure with clearer field names
/// - Better support for multiple items in a single request
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::Item;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let item = Item {
///     id: "item1".to_string(),
///     qty: Some(1),
///     seq: Some(1),
///     flr: Some(1.50),
///     flrcur: Some("USD".to_string()),
///     exp: Some(3600),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Item {
    /// Unique identifier for this item within the context of the request.
    /// REQUIRED by the specification.
    pub id: String,

    /// Quantity of billable events (e.g., ad impressions).
    /// Default is 1 if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub qty: Option<i32>,

    /// Sequence number for multiple items.
    /// Used for multi-item requests to specify the order.
    /// Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seq: Option<i32>,

    /// Minimum bid floor for this item in the currency specified by `flrcur`.
    /// Also referred to as the reserve price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flr: Option<f64>,

    /// Currency for the bid floor using ISO-4217 codes.
    /// If omitted, USD is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flrcur: Option<String>,

    /// Advisory as to the number of seconds that may elapse between
    /// auction and fulfillment (e.g., for roadside billboards).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub exp: Option<i32>,

    /// Timestamp when the item is expected to be fulfilled.
    /// Expressed as Unix epoch time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dt: Option<i64>,

    /// Delivery method for the creative:
    /// - 0 = Streaming or downloaded
    /// - 1 = Cached
    /// - 2 = Real-time
    /// - 3 = Progressive
    ///
    /// Refer to delivery method enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dlvy: Option<i32>,

    /// Array of metrics that are supported for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metric: Option<Vec<Metric>>,

    /// Array of deal objects that provide terms for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub deal: Option<Vec<Deal>>,

    /// Indicator of private auction eligibility:
    /// - 0 = all bids are accepted
    /// - 1 = bids are restricted to deals only
    ///
    /// Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub private: Option<i32>,

    /// Layer-4 domain specification for the placement details.
    /// References AdCOM Placement object (Display, Video, Audio).
    /// This is a JSON object that varies by placement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spec: Option<serde_json::Value>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item {
            id: "item1".to_string(),
            qty: Some(1),
            seq: Some(1),
            flr: Some(2.50),
            flrcur: Some("USD".to_string()),
            ..Default::default()
        };

        assert_eq!(item.id, "item1");
        assert_eq!(item.qty, Some(1));
        assert_eq!(item.seq, Some(1));
        assert_eq!(item.flr, Some(2.50));
        assert_eq!(item.flrcur, Some("USD".to_string()));
    }

    #[test]
    fn test_item_minimal() {
        let item = Item {
            id: "item1".to_string(),
            ..Default::default()
        };

        assert_eq!(item.id, "item1");
        assert_eq!(item.qty, None);
        assert_eq!(item.flr, None);
    }

    #[test]
    fn test_item_with_floor() {
        let item = Item {
            id: "item1".to_string(),
            flr: Some(5.00),
            flrcur: Some("EUR".to_string()),
            ..Default::default()
        };

        assert_eq!(item.flr, Some(5.00));
        assert_eq!(item.flrcur, Some("EUR".to_string()));
    }

    #[test]
    fn test_item_with_expiration() {
        let item = Item {
            id: "item1".to_string(),
            exp: Some(3600),
            dt: Some(1609459200),
            ..Default::default()
        };

        assert_eq!(item.exp, Some(3600));
        assert_eq!(item.dt, Some(1609459200));
    }

    #[test]
    fn test_item_private_auction() {
        let item = Item {
            id: "item1".to_string(),
            private: Some(1),
            deal: Some(vec![]),
            ..Default::default()
        };

        assert_eq!(item.private, Some(1));
        assert!(item.deal.is_some());
    }

    #[test]
    fn test_item_serialization() {
        let item = Item {
            id: "item1".to_string(),
            qty: Some(1),
            flr: Some(2.50),
            flrcur: Some("USD".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"id\":\"item1\""));
        assert!(json.contains("\"qty\":1"));
        assert!(json.contains("\"flr\":2.5"));
        assert!(json.contains("\"flrcur\":\"USD\""));
    }

    #[test]
    fn test_item_deserialization() {
        let json = r#"{
            "id": "item1",
            "qty": 1,
            "seq": 1,
            "flr": 2.50,
            "flrcur": "USD"
        }"#;

        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item1");
        assert_eq!(item.qty, Some(1));
        assert_eq!(item.seq, Some(1));
        assert_eq!(item.flr, Some(2.50));
        assert_eq!(item.flrcur, Some("USD".to_string()));
    }

    #[test]
    fn test_item_builder() {
        let item = ItemBuilder::default()
            .id("item1".to_string())
            .qty(Some(1))
            .flr(Some(3.00))
            .flrcur(Some("USD".to_string()))
            .build()
            .unwrap();

        assert_eq!(item.id, "item1");
        assert_eq!(item.qty, Some(1));
        assert_eq!(item.flr, Some(3.00));
    }

    #[test]
    fn test_item_with_delivery_method() {
        let item = Item {
            id: "item1".to_string(),
            dlvy: Some(2), // Real-time
            ..Default::default()
        };

        assert_eq!(item.dlvy, Some(2));
    }

    #[test]
    fn test_item_with_spec() {
        let spec = serde_json::json!({
            "placement": {
                "w": 300,
                "h": 250
            }
        });

        let item = Item {
            id: "item1".to_string(),
            spec: Some(spec),
            ..Default::default()
        };

        assert!(item.spec.is_some());
    }
}
