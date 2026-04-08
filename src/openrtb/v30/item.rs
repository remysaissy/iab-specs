use super::{Deal, Metric};
use crate::Extension;
/// OpenRTB 3.0 Item Object
///
/// This module implements the Item object for inventory/impression items.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `SpecExt` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, SpecExt: Extension",
    deserialize = "Ext: Extension, SpecExt: Extension"
))]
pub struct Item<Ext: Extension = crate::DefaultExt, SpecExt: Extension = crate::DefaultExt> {
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
    pub spec: Option<Box<SpecExt>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Item {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ItemBuilder {
        ItemBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec: Object: Item — required id field and optional inventory fields (qty, seq, flr, flrcur)
    #[test]
    fn test_item_creation() {
        let item = Item::builder()
            .id("item1".to_string())
            .qty(Some(1))
            .seq(Some(1))
            .flr(Some(2.50))
            .flrcur(Some("USD".to_string()))
            .build()
            .unwrap();

        assert_eq!(item.id, "item1");
        assert_eq!(item.qty, Some(1));
        assert_eq!(item.seq, Some(1));
        assert_eq!(item.flr, Some(2.50));
        assert_eq!(item.flrcur, Some("USD".to_string()));
    }

    // Spec: Object: Item — minimal item with id only, optionals are None
    #[test]
    fn test_item_minimal() {
        let item = Item::builder().id("item1".to_string()).build().unwrap();

        assert_eq!(item.id, "item1");
        assert_eq!(item.qty, None);
        assert_eq!(item.flr, None);
    }

    // Spec: Object: Item — floor price (flr) with EUR currency (flrcur)
    #[test]
    fn test_item_with_floor() {
        let item = Item::builder()
            .id("item1".to_string())
            .flr(Some(5.00))
            .flrcur(Some("EUR".to_string()))
            .build()
            .unwrap();

        assert_eq!(item.flr, Some(5.00));
        assert_eq!(item.flrcur, Some("EUR".to_string()));
    }

    // Spec: Object: Item — expiration (exp) and delivery timestamp (dt)
    #[test]
    fn test_item_with_expiration() {
        let item = Item::builder()
            .id("item1".to_string())
            .exp(Some(3600))
            .dt(Some(1609459200))
            .build()
            .unwrap();

        assert_eq!(item.exp, Some(3600));
        assert_eq!(item.dt, Some(1609459200));
    }

    // Spec: Object: Item — private auction flag (private=1) restricts to deals only
    #[test]
    fn test_item_private_auction() {
        let item = Item::builder()
            .id("item1".to_string())
            .private(Some(1))
            .deal(Some(vec![]))
            .build()
            .unwrap();

        assert_eq!(item.private, Some(1));
        assert!(item.deal.is_some());
    }

    // Spec: Object: Item — JSON serialization of id, qty, flr, flrcur fields
    #[test]
    fn test_item_serialization() {
        let item = Item::builder()
            .id("item1".to_string())
            .qty(Some(1))
            .flr(Some(2.50))
            .flrcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"id\":\"item1\""));
        assert!(json.contains("\"qty\":1"));
        assert!(json.contains("\"flr\":2.5"));
        assert!(json.contains("\"flrcur\":\"USD\""));
    }

    // Spec: Object: Item — JSON deserialization of id, qty, seq, flr, flrcur
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

    // Spec: Object: Item — delivery method (dlvy) enumeration value
    #[test]
    fn test_item_with_delivery_method() {
        let item = Item::builder()
            .id("item1".to_string())
            .dlvy(Some(2))
            .build()
            .unwrap();

        assert_eq!(item.dlvy, Some(2));
    }

    // Spec: Object: Item — spec field with serde_json::Value for placement details
    #[test]
    fn test_item_with_spec() {
        let spec = Box::new(serde_json::json!({
            "placement": {
                "w": 300,
                "h": 250
            }
        }));

        let item = ItemBuilder::<Vec<u8>, serde_json::Value>::default()
            .id("item1".to_string())
            .spec(Some(spec))
            .build()
            .unwrap();

        assert!(item.spec.is_some());
    }

    // Spec: Object: Item — default() produces empty id and None for all optionals
    #[test]
    fn test_item_default() {
        let item: Item = Item::default();
        assert_eq!(item.id, "");
        assert_eq!(item.qty, None);
        assert_eq!(item.seq, None);
        assert_eq!(item.flr, None);
        assert_eq!(item.flrcur, None);
        assert_eq!(item.exp, None);
        assert_eq!(item.dt, None);
        assert_eq!(item.dlvy, None);
        assert_eq!(item.metric, None);
        assert_eq!(item.deal, None);
        assert_eq!(item.private, None);
        assert_eq!(item.spec, None);
        assert_eq!(item.ext, None);
    }

    // Spec: Object: Item — serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_item_roundtrip() {
        let item = Item::builder()
            .id("item-rt".to_string())
            .qty(Some(2))
            .seq(Some(1))
            .flr(Some(1.50))
            .flrcur(Some("USD".to_string()))
            .exp(Some(300))
            .dt(Some(1700000000))
            .dlvy(Some(1))
            .private(Some(0))
            .build()
            .unwrap();

        let json = serde_json::to_string(&item).unwrap();
        let deserialized: Item = serde_json::from_str(&json).unwrap();
        assert_eq!(item, deserialized);
    }

    // Spec: Object: Item — optional fields omitted from JSON when None
    #[test]
    fn test_item_optional_fields_not_in_json() {
        let item = Item::builder().id("item-min".to_string()).build().unwrap();

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"id\":\"item-min\""));
        assert!(!json.contains("\"qty\""));
        assert!(!json.contains("\"seq\""));
        assert!(!json.contains("\"flr\""));
        assert!(!json.contains("\"flrcur\""));
        assert!(!json.contains("\"exp\""));
        assert!(!json.contains("\"dt\""));
        assert!(!json.contains("\"dlvy\""));
        assert!(!json.contains("\"metric\""));
        assert!(!json.contains("\"deal\""));
        assert!(!json.contains("\"private\""));
        assert!(!json.contains("\"spec\""));
        assert!(!json.contains("\"ext\""));
    }

    // Spec: Object: Item — metric array populated with actual Metric objects
    #[test]
    fn test_item_with_metrics() {
        let m1 = Metric::builder()
            .type_("viewability".to_string())
            .val(0.80)
            .vendor(Some("vendor1.com".to_string()))
            .build()
            .unwrap();

        let m2 = Metric::builder()
            .type_("completion".to_string())
            .val(0.90)
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item-met".to_string())
            .metric(Some(vec![m1, m2]))
            .build()
            .unwrap();

        let metrics = item.metric.as_ref().unwrap();
        assert_eq!(metrics.len(), 2);
        assert_eq!(metrics[0].type_, "viewability");
        assert_eq!(metrics[0].val, 0.80);
        assert_eq!(metrics[1].type_, "completion");
    }

    // Spec: Object: Item — deal array populated with actual Deal objects
    #[test]
    fn test_item_with_deals() {
        let d1 = Deal::builder()
            .id("deal-1".to_string())
            .flr(Some(5.00))
            .flrcur(Some("USD".to_string()))
            .at(Some(3))
            .build()
            .unwrap();

        let d2 = Deal::builder()
            .id("deal-2".to_string())
            .flr(Some(2.00))
            .at(Some(1))
            .wseat(Some(vec!["seat-a".to_string()]))
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item-deals".to_string())
            .private(Some(1))
            .deal(Some(vec![d1, d2]))
            .build()
            .unwrap();

        let deals = item.deal.as_ref().unwrap();
        assert_eq!(deals.len(), 2);
        assert_eq!(deals[0].id, "deal-1");
        assert_eq!(deals[0].at, Some(3));
        assert_eq!(deals[1].id, "deal-2");
        assert_eq!(deals[1].wseat.as_ref().unwrap().len(), 1);
    }
}
