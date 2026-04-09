use super::adjust_bid_payload::AdjustBidPayload;
use super::adjust_deal_payload::AdjustDealPayload;
use super::data_payload::DataPayload;
use super::enums::{Intent, Operation};
use super::ids_payload::IDsPayload;
use super::metrics_payload::MetricsPayload;
#[cfg(test)]
use super::metrics_payload::MetricsPayloadBuilder;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// An atomic mutation proposed by an agent to modify an OpenRTB bid request or response.
///
/// Each mutation declares its intent, operation type, a semantic path targeting
/// a specific business concept in the OpenRTB payload, and exactly one payload
/// matching the intent. Mutations are independent and can be accepted or rejected
/// individually by the orchestrator.
///
/// # Payload Selection
///
/// Exactly one of the payload fields (`ids`, `adjust_deal`, `adjust_bid`,
/// `metrics`, `content_data`) should be set, matching the `intent`:
///
/// | Intent | Expected Payload |
/// |--------|-----------------|
/// | `ActivateSegments` | `ids` |
/// | `ActivateDeals` | `ids` |
/// | `SuppressDeals` | `ids` |
/// | `AdjustDealFloor` | `adjust_deal` |
/// | `AdjustDealMargin` | `adjust_deal` |
/// | `BidShade` | `adjust_bid` |
/// | `AddMetrics` | `metrics` |
/// | `AddCids` | `content_data` |
///
/// # Semantic Paths
///
/// Paths reference OpenRTB business entities rather than literal JSON paths:
/// - `/user/data/segment` — User segment data
/// - `/imp/{impId}` — Specific impression
/// - `/imp/{impId}/pmp/deals/{dealId}` — Specific deal within an impression
/// - `/seatbid/{seat}/bid/{bidId}` — Specific bid
/// - `/imp/{impId}/metric` — Impression metrics
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::{Mutation, Intent, Operation, IDsPayload};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mutation = Mutation::builder()
///     .intent(Intent::ActivateSegments)
///     .op(Operation::Add)
///     .path("/user/data/segment".to_string())
///     .ids(Some(IDsPayload::builder()
///         .id(vec!["seg-001".to_string(), "seg-002".to_string()])
///         .build()?))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "P: Extension, Ext: Extension",
    deserialize = "P: Extension, Ext: Extension"
))]
pub struct Mutation<P: Extension = crate::DefaultExt, Ext: Extension = crate::DefaultExt> {
    /// The declared purpose of this mutation.
    pub intent: Intent,

    /// The type of operation to perform.
    pub op: Operation,

    /// Semantic business-domain path targeting the OpenRTB concept to mutate.
    #[builder(setter(into))]
    pub path: String,

    /// Payload for ID-based mutations (segments, deals).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ids: Option<IDsPayload<Ext>>,

    /// Payload for deal floor/margin adjustments.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub adjust_deal: Option<AdjustDealPayload<Ext>>,

    /// Payload for bid price adjustments (bid shading).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub adjust_bid: Option<AdjustBidPayload<Ext>>,

    /// Payload for adding OpenRTB Metric objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metrics: Option<MetricsPayload<P, Ext>>,

    /// Payload for adding OpenRTB Data objects (content data).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub content_data: Option<DataPayload<P, Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Mutation {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> MutationBuilder<crate::DefaultExt, crate::DefaultExt> {
        MutationBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artb::v10::adjust_deal_payload::AdjustDealPayload;
    use crate::artb::v10::data_payload::DataPayloadBuilder;
    use crate::artb::v10::enums::CalculationType;
    use crate::artb::v10::margin::Margin;

    #[test]
    fn test_mutation_activate_segments() {
        let mutation = Mutation::builder()
            .intent(Intent::ActivateSegments)
            .op(Operation::Add)
            .path("/user/data/segment".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["seg-1".to_string(), "seg-2".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::ActivateSegments);
        assert_eq!(mutation.op, Operation::Add);
        assert_eq!(mutation.path, "/user/data/segment");
        assert!(mutation.ids.is_some());
        assert!(mutation.adjust_deal.is_none());
        assert!(mutation.adjust_bid.is_none());
        assert!(mutation.metrics.is_none());
        assert!(mutation.content_data.is_none());
    }

    #[test]
    fn test_mutation_bid_shade() {
        let mutation = Mutation::builder()
            .intent(Intent::BidShade)
            .op(Operation::Replace)
            .path("/seatbid/0/bid/bid-1".to_string())
            .adjust_bid(Some(
                AdjustBidPayload::builder().price(2.50).build().unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::BidShade);
        assert_eq!(mutation.op, Operation::Replace);
        assert!(mutation.adjust_bid.is_some());
        assert_eq!(mutation.adjust_bid.as_ref().unwrap().price, 2.50);
    }

    #[test]
    fn test_mutation_adjust_deal_floor() {
        let mutation = Mutation::builder()
            .intent(Intent::AdjustDealFloor)
            .op(Operation::Replace)
            .path("/imp/imp-1/pmp/deals/deal-500".to_string())
            .adjust_deal(Some(
                AdjustDealPayload::builder().bidfloor(5.00).build().unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::AdjustDealFloor);
        assert!(mutation.adjust_deal.is_some());
        assert_eq!(mutation.adjust_deal.as_ref().unwrap().bidfloor, 5.00);
    }

    #[test]
    fn test_mutation_add_metrics() {
        let mutation = MutationBuilder::<serde_json::Value, Vec<u8>>::default()
            .intent(Intent::AddMetrics)
            .op(Operation::Add)
            .path("/imp/imp-1/metric".to_string())
            .metrics(Some(
                MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
                    .metric(vec![
                        serde_json::json!({"type": "viewability", "value": 0.80}),
                    ])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::AddMetrics);
        assert!(mutation.metrics.is_some());
    }

    #[test]
    fn test_mutation_serialization() {
        let mutation = Mutation::builder()
            .intent(Intent::ActivateDeals)
            .op(Operation::Add)
            .path("/imp/imp-1".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["deal-100".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&mutation).unwrap();
        assert!(json.contains("\"intent\":2"));
        assert!(json.contains("\"op\":1"));
        assert!(json.contains("\"path\":\"/imp/imp-1\""));
        assert!(json.contains("\"ids\""));
    }

    #[test]
    fn test_mutation_deserialization() {
        let json = r#"{
            "intent": 6,
            "op": 3,
            "path": "/seatbid/0/bid/1",
            "adjust_bid": {"price": 3.75}
        }"#;

        let mutation: Mutation = serde_json::from_str(json).unwrap();
        assert_eq!(mutation.intent, Intent::BidShade);
        assert_eq!(mutation.op, Operation::Replace);
        assert_eq!(mutation.adjust_bid.as_ref().unwrap().price, 3.75);
    }

    #[test]
    fn test_mutation_roundtrip() {
        let mutation = Mutation::builder()
            .intent(Intent::SuppressDeals)
            .op(Operation::Remove)
            .path("/imp/imp-1".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["deal-200".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&mutation).unwrap();
        let parsed: Mutation = serde_json::from_str(&json).unwrap();
        assert_eq!(mutation, parsed);
    }

    #[test]
    fn test_mutation_default() {
        let mutation = Mutation::builder().build().unwrap();
        assert_eq!(mutation.intent, Intent::Unspecified);
        assert_eq!(mutation.op, Operation::Unspecified);
        assert!(mutation.path.is_empty());
    }

    #[test]
    fn test_mutation_adjust_deal_margin() {
        // Spec: Intent AdjustDealMargin(5) uses AdjustDealPayload with margin
        let mutation = Mutation::builder()
            .intent(Intent::AdjustDealMargin)
            .op(Operation::Replace)
            .path("/imp/imp-1/pmp/deals/deal-1".to_string())
            .adjust_deal(Some(
                AdjustDealPayload::builder()
                    .margin(Some(
                        Margin::builder()
                            .value(0.15)
                            .calculation_type(CalculationType::Percent)
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::AdjustDealMargin);
        assert!(mutation.adjust_deal.is_some());
        let deal = mutation.adjust_deal.as_ref().unwrap();
        assert!(deal.margin.is_some());
        assert_eq!(deal.margin.as_ref().unwrap().value, 0.15);
    }

    #[test]
    fn test_mutation_suppress_deals() {
        // Spec: Intent SuppressDeals(3) uses IDsPayload with Operation Remove
        let mutation = Mutation::builder()
            .intent(Intent::SuppressDeals)
            .op(Operation::Remove)
            .path("/imp/imp-1".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["deal-99".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::SuppressDeals);
        assert_eq!(mutation.op, Operation::Remove);
        assert!(mutation.ids.is_some());
        assert_eq!(mutation.ids.as_ref().unwrap().id[0], "deal-99");
    }

    #[test]
    fn test_mutation_add_cids() {
        // Spec: Intent AddCids(8) uses DataPayload for content data
        let mutation = MutationBuilder::<serde_json::Value, Vec<u8>>::default()
            .intent(Intent::AddCids)
            .op(Operation::Add)
            .path("/user/data".to_string())
            .content_data(Some(
                DataPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
                    .data(vec![serde_json::json!({
                        "id": "dp-1",
                        "name": "Content Provider"
                    })])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(mutation.intent, Intent::AddCids);
        assert!(mutation.content_data.is_some());
        assert_eq!(mutation.content_data.as_ref().unwrap().data.len(), 1);
    }

    #[test]
    fn test_mutation_deserialization_extra_fields() {
        // Spec: ARTB JSON payloads must tolerate unknown fields
        let json = r#"{
            "intent": 1,
            "op": 1,
            "path": "/user/data/segment",
            "ids": {"id": ["s1"]},
            "unknown": true
        }"#;
        let mutation: Mutation = serde_json::from_str(json).unwrap();
        assert_eq!(mutation.intent, Intent::ActivateSegments);
        assert_eq!(mutation.op, Operation::Add);
    }

    #[test]
    fn test_mutation_with_extension() {
        // Spec: Mutation ext field for exchange-specific data
        let mutation = MutationBuilder::<serde_json::Value, serde_json::Value>::default()
            .intent(Intent::ActivateSegments)
            .op(Operation::Add)
            .path("/user/data/segment".to_string())
            .ext(Some(Box::new(serde_json::json!({"priority": "high"}))))
            .build()
            .unwrap();

        let json = serde_json::to_string(&mutation).unwrap();
        let parsed: Mutation<serde_json::Value, serde_json::Value> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(mutation, parsed);
    }
}
