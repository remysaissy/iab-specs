//! Agentic RTB Framework 1.0 Specification
//!
//! This module implements the complete ARTB (Agentic RTB Framework) 1.0 specification
//! for deploying autonomous agent services within ad-tech platforms to participate
//! in OpenRTB bidstream processing.
//!
//! # Overview
//!
//! ARTB defines a standard for containerized agents that propose atomic mutations
//! to OpenRTB bid requests and responses. The protocol enables:
//!
//! - **Segment activation**: Adding user segments to bid requests
//! - **Deal management**: Activating, suppressing, and adjusting deals
//! - **Bid shading**: Adjusting bid prices on responses
//! - **Metrics**: Adding measurement metrics to impressions
//! - **Content data**: Adding extended content IDs
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Protocol enumerations (Lifecycle, Intent, Operation, etc.)
//! - Protocol messages: [`RTBRequest`], [`RTBResponse`], [`Mutation`], [`Originator`], [`Metadata`]
//! - Payload types: [`IDsPayload`], [`AdjustDealPayload`], [`AdjustBidPayload`],
//!   [`MetricsPayload`], [`DataPayload`], [`Margin`]
//!
//! # Quick Start
//!
//! ## Processing a Bid Request
//!
//! ```rust
//! #[cfg(feature = "artb_10")]
//! {
//! use iab_specs_artb::v10::{
//!     RTBRequest, RTBRequestBuilder, RTBResponse, RTBResponseBuilder,
//!     Mutation, MutationBuilder, Metadata,
//!     Lifecycle, Intent, Operation, IDsPayload, OriginatorType, Originator,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Orchestrator creates a request for the agent (using serde_json::Value for payloads)
//! let request = RTBRequestBuilder::<serde_json::Value, Vec<u8>>::default()
//!     .lifecycle(Lifecycle::PublisherBidRequest)
//!     .id("req-12345")
//!     .tmax(Some(100))
//!     .bid_request(Some(serde_json::json!({
//!         "id": "auction-1",
//!         "imp": [{"id": "imp-1"}]
//!     })))
//!     .originator(Some(Originator::builder()
//!         .type_(OriginatorType::Ssp)
//!         .name("Example SSP")
//!         .build()?))
//!     .applicable_intents(vec![
//!         Intent::ActivateSegments,
//!         Intent::ActivateDeals,
//!     ])
//!     .build()?;
//!
//! // Agent processes and returns mutations
//! let response = RTBResponseBuilder::<serde_json::Value, Vec<u8>>::default()
//!     .id("req-12345")
//!     .mutations(vec![
//!         MutationBuilder::<serde_json::Value, Vec<u8>>::default()
//!             .intent(Intent::ActivateSegments)
//!             .op(Operation::Add)
//!             .path("/user/data/segment".to_string())
//!             .ids(Some(IDsPayload::builder()
//!                 .id(vec!["seg-001".to_string(), "seg-002".to_string()])
//!                 .build()?))
//!             .build()?,
//!     ])
//!     .metadata(Some(Metadata::builder()
//!         .api_version("1.0")
//!         .model_version("v0.10.0")
//!         .build()?))
//!     .build()?;
//!
//! assert_eq!(request.id, response.id);
//! # Ok(())
//! # }
//! }
//! ```
//!
//! # Extension Support
//!
//! All objects support custom extensions via the generic `Ext` parameter.
//! By default, extensions use [`DefaultExt`](crate::DefaultExt) which is `Vec<u8>` (opaque bytes).
//! Callers can use `serde_json::Value` or custom types as explicit type parameters.
//!
//! # Specification Reference
//!
//! This implementation follows the [Agentic RTB Framework Version 1.0](https://github.com/IABTechLab/agentic-rtb-framework)
//! specification published by IAB Tech Lab.

pub mod enums;

mod adjust_bid_payload;
mod adjust_deal_payload;
mod data_payload;
mod ids_payload;
mod margin;
mod metadata;
mod metrics_payload;
mod mutation;
mod originator;
mod rtb_request;
mod rtb_response;

// Re-export enums
pub use enums::{CalculationType, Intent, Lifecycle, Operation, OriginatorType};

// Re-export payload types
pub use adjust_bid_payload::{AdjustBidPayload, AdjustBidPayloadBuilder};
pub use adjust_deal_payload::{AdjustDealPayload, AdjustDealPayloadBuilder};
pub use data_payload::{DataPayload, DataPayloadBuilder};
pub use ids_payload::{IDsPayload, IDsPayloadBuilder};
pub use margin::{Margin, MarginBuilder};
pub use metadata::{Metadata, MetadataBuilder};
pub use metrics_payload::{MetricsPayload, MetricsPayloadBuilder};

// Re-export protocol messages
pub use mutation::{Mutation, MutationBuilder};
pub use originator::{Originator, OriginatorBuilder};
pub use rtb_request::{RTBRequest, RTBRequestBuilder};
pub use rtb_response::{RTBResponse, RTBResponseBuilder};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_request_response_cycle() {
        // Create a bid request processing scenario
        let request = RTBRequestBuilder::<serde_json::Value, Vec<u8>>::default()
            .lifecycle(Lifecycle::PublisherBidRequest)
            .id("req-integration-001".to_string())
            .tmax(Some(100))
            .bid_request(Some(serde_json::json!({
                "id": "auction-1",
                "imp": [{"id": "imp-1", "bidfloor": 1.50}],
                "site": {"domain": "example.com"},
                "user": {"id": "user-123"}
            })))
            .originator(Some(
                Originator::builder()
                    .type_(OriginatorType::Ssp)
                    .name("Integration Test SSP")
                    .domain("ssp.test.com")
                    .build()
                    .unwrap(),
            ))
            .applicable_intents(vec![
                Intent::ActivateSegments,
                Intent::ActivateDeals,
                Intent::AdjustDealFloor,
            ])
            .build()
            .unwrap();

        // Agent returns multiple mutations
        let response = RTBResponseBuilder::<serde_json::Value, Vec<u8>>::default()
            .id(request.id.clone())
            .mutations(vec![
                // Activate user segments
                MutationBuilder::<serde_json::Value, Vec<u8>>::default()
                    .intent(Intent::ActivateSegments)
                    .op(Operation::Add)
                    .path("/user/data/segment".to_string())
                    .ids(Some(
                        IDsPayload::builder()
                            .id(vec![
                                "auto-enthusiast".to_string(),
                                "luxury-buyer".to_string(),
                            ])
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                // Activate a deal
                MutationBuilder::<serde_json::Value, Vec<u8>>::default()
                    .intent(Intent::ActivateDeals)
                    .op(Operation::Add)
                    .path("/imp/imp-1".to_string())
                    .ids(Some(
                        IDsPayload::builder()
                            .id(vec!["premium-deal-500".to_string()])
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                // Adjust deal floor
                MutationBuilder::<serde_json::Value, Vec<u8>>::default()
                    .intent(Intent::AdjustDealFloor)
                    .op(Operation::Replace)
                    .path("/imp/imp-1/pmp/deals/premium-deal-500".to_string())
                    .adjust_deal(Some(
                        AdjustDealPayload::builder()
                            .bidfloor(3.50)
                            .margin(Some(
                                Margin::builder()
                                    .value(0.10)
                                    .calculation_type(CalculationType::Percent)
                                    .build()
                                    .unwrap(),
                            ))
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .metadata(Some(
                Metadata::builder()
                    .api_version("1.0")
                    .model_version("v0.10.0")
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        // Verify response matches request
        assert_eq!(request.id, response.id);
        assert_eq!(response.mutations.len(), 3);

        // Verify each mutation has the correct intent
        assert_eq!(response.mutations[0].intent, Intent::ActivateSegments);
        assert_eq!(response.mutations[1].intent, Intent::ActivateDeals);
        assert_eq!(response.mutations[2].intent, Intent::AdjustDealFloor);

        // Verify serialization roundtrip
        let request_json = serde_json::to_string(&request).unwrap();
        let parsed_request: RTBRequest<serde_json::Value> =
            serde_json::from_str(&request_json).unwrap();
        assert_eq!(request, parsed_request);

        let response_json = serde_json::to_string(&response).unwrap();
        let parsed_response: RTBResponse<serde_json::Value> =
            serde_json::from_str(&response_json).unwrap();
        assert_eq!(response, parsed_response);
    }

    #[test]
    fn test_bid_shading_flow() {
        // DSP bid response processing scenario
        let request = RTBRequestBuilder::<serde_json::Value, Vec<u8>>::default()
            .lifecycle(Lifecycle::DspBidResponse)
            .id("req-shade-001".to_string())
            .tmax(Some(50))
            .bid_request(Some(serde_json::json!({
                "id": "auction-2",
                "imp": [{"id": "imp-1", "bidfloor": 2.00}]
            })))
            .bid_response(Some(serde_json::json!({
                "id": "auction-2",
                "seatbid": [{
                    "seat": "dsp-1",
                    "bid": [{"id": "bid-1", "impid": "imp-1", "price": 5.00}]
                }]
            })))
            .applicable_intents(vec![Intent::BidShade])
            .build()
            .unwrap();

        let response = RTBResponseBuilder::<serde_json::Value, Vec<u8>>::default()
            .id(request.id.clone())
            .mutations(vec![
                MutationBuilder::<serde_json::Value, Vec<u8>>::default()
                    .intent(Intent::BidShade)
                    .op(Operation::Replace)
                    .path("/seatbid/dsp-1/bid/bid-1".to_string())
                    .adjust_bid(Some(
                        AdjustBidPayload::builder().price(3.75).build().unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .metadata(Some(
                Metadata::builder().api_version("1.0").build().unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(request.id, response.id);
        assert_eq!(response.mutations.len(), 1);
        assert_eq!(response.mutations[0].intent, Intent::BidShade);
        assert_eq!(
            response.mutations[0].adjust_bid.as_ref().unwrap().price,
            3.75
        );
    }

    #[test]
    fn test_metrics_flow() {
        let request = RTBRequest::builder()
            .lifecycle(Lifecycle::PublisherBidRequest)
            .id("req-metrics-001".to_string())
            .applicable_intents(vec![Intent::AddMetrics])
            .build()
            .unwrap();

        let response = RTBResponseBuilder::<serde_json::Value, Vec<u8>>::default()
            .id(request.id.clone())
            .mutations(vec![MutationBuilder::<serde_json::Value, Vec<u8>>::default()
                .intent(Intent::AddMetrics)
                .op(Operation::Add)
                .path("/imp/imp-1/metric".to_string())
                .metrics(Some(
                    MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
                        .metric(vec![
                            serde_json::json!({"type": "viewability", "value": 0.85, "vendor": "iab.com"}),
                            serde_json::json!({"type": "attention", "value": 0.70}),
                        ])
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap()])
            .build()
            .unwrap();

        assert_eq!(response.mutations[0].intent, Intent::AddMetrics);
        let metrics = response.mutations[0].metrics.as_ref().unwrap();
        assert_eq!(metrics.metric.len(), 2);
    }

    #[test]
    fn test_all_intents_have_correct_payload() {
        // ActivateSegments -> IDsPayload
        let m = Mutation::builder()
            .intent(Intent::ActivateSegments)
            .op(Operation::Add)
            .path("/user/data/segment".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["s1".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.ids.is_some());

        // ActivateDeals -> IDsPayload
        let m = Mutation::builder()
            .intent(Intent::ActivateDeals)
            .op(Operation::Add)
            .path("/imp/1".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["d1".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.ids.is_some());

        // SuppressDeals -> IDsPayload
        let m = Mutation::builder()
            .intent(Intent::SuppressDeals)
            .op(Operation::Remove)
            .path("/imp/1".to_string())
            .ids(Some(
                IDsPayload::builder()
                    .id(vec!["d2".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.ids.is_some());

        // AdjustDealFloor -> AdjustDealPayload
        let m = Mutation::builder()
            .intent(Intent::AdjustDealFloor)
            .op(Operation::Replace)
            .path("/imp/1/pmp/deals/d1".to_string())
            .adjust_deal(Some(
                AdjustDealPayload::builder().bidfloor(5.0).build().unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.adjust_deal.is_some());

        // AdjustDealMargin -> AdjustDealPayload
        let m = Mutation::builder()
            .intent(Intent::AdjustDealMargin)
            .op(Operation::Replace)
            .path("/imp/1/pmp/deals/d1".to_string())
            .adjust_deal(Some(
                AdjustDealPayload::builder()
                    .bidfloor(0.0)
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
        assert!(m.adjust_deal.is_some());

        // BidShade -> AdjustBidPayload
        let m = Mutation::builder()
            .intent(Intent::BidShade)
            .op(Operation::Replace)
            .path("/seatbid/0/bid/1".to_string())
            .adjust_bid(Some(
                AdjustBidPayload::builder().price(2.50).build().unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.adjust_bid.is_some());

        // AddMetrics -> MetricsPayload
        let m = MutationBuilder::<serde_json::Value, Vec<u8>>::default()
            .intent(Intent::AddMetrics)
            .op(Operation::Add)
            .path("/imp/1/metric".to_string())
            .metrics(Some(
                MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
                    .metric(vec![
                        serde_json::json!({"type": "viewability", "value": 0.8}),
                    ])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.metrics.is_some());

        // AddCids -> DataPayload
        let m = MutationBuilder::<serde_json::Value, Vec<u8>>::default()
            .intent(Intent::AddCids)
            .op(Operation::Add)
            .path("/user/data".to_string())
            .content_data(Some(
                DataPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
                    .data(vec![serde_json::json!({"id": "dp-1"})])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();
        assert!(m.content_data.is_some());
    }

    #[test]
    fn test_roundtrip_with_custom_extensions() {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
        struct CustomExt {
            agent_name: String,
            confidence: f64,
        }

        let response: RTBResponse<serde_json::Value, CustomExt> = RTBResponseBuilder::default()
            .id("req-ext-001".to_string())
            .mutations(vec![])
            .ext(Some(Box::new(CustomExt {
                agent_name: "fraud-detector".to_string(),
                confidence: 0.95,
            })))
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: RTBResponse<serde_json::Value, CustomExt> =
            serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ext.as_ref().unwrap().agent_name, "fraud-detector");
        assert_eq!(parsed.ext.as_ref().unwrap().confidence, 0.95);
    }
}
