use super::enums::{Intent, Lifecycle};
use super::originator::Originator;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ARTB RTB Request sent by the orchestrator to an agent.
///
/// Wraps an OpenRTB bid request (and optionally a bid response) along with
/// ARTB-specific metadata. The agent processes this request and returns
/// an `RTBResponse` containing proposed mutations.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::{RTBRequest, Lifecycle, Intent, Originator, OriginatorType};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let request = RTBRequest::builder()
///     .lifecycle(Lifecycle::PublisherBidRequest)
///     .id("req-12345".to_string())
///     .tmax(Some(100))
///     .bid_request(Some(serde_json::json!({
///         "id": "auction-1",
///         "imp": [{"id": "imp-1"}]
///     })))
///     .originator(Some(Originator::builder()
///         .type_(OriginatorType::Ssp)
///         .name("Example SSP")
///         .build()?))
///     .applicable_intents(vec![
///         Intent::ActivateSegments,
///         Intent::ActivateDeals,
///     ])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct RTBRequest<Ext: Extension = serde_json::Value> {
    /// The auction lifecycle stage when this request is being made.
    pub lifecycle: Lifecycle,

    /// Unique request ID assigned by the exchange/orchestrator.
    /// **Required field**
    #[builder(setter(into))]
    pub id: String,

    /// Maximum response time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tmax: Option<i32>,

    /// Full OpenRTB bid request payload.
    /// Present during `Lifecycle::PublisherBidRequest` and `Lifecycle::DspBidResponse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bid_request: Option<serde_json::Value>,

    /// Full OpenRTB bid response payload.
    /// Present only during `Lifecycle::DspBidResponse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bid_response: Option<serde_json::Value>,

    /// The business entity that owns the bid request/response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub originator: Option<Originator<Ext>>,

    /// Intents the agent is eligible to return mutations for.
    /// The orchestrator uses this to restrict what types of mutations
    /// the agent may propose.
    #[builder(default, setter(into))]
    pub applicable_intents: Vec<Intent>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl RTBRequest {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RTBRequestBuilder {
        RTBRequestBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artb::v10::enums::OriginatorType;

    #[test]
    fn test_rtb_request_creation() {
        let request = RTBRequest::builder()
            .lifecycle(Lifecycle::PublisherBidRequest)
            .id("req-001".to_string())
            .tmax(Some(100))
            .bid_request(Some(serde_json::json!({"id": "auction-1"})))
            .applicable_intents(vec![Intent::ActivateSegments, Intent::ActivateDeals])
            .build()
            .unwrap();

        assert_eq!(request.lifecycle, Lifecycle::PublisherBidRequest);
        assert_eq!(request.id, "req-001");
        assert_eq!(request.tmax, Some(100));
        assert!(request.bid_request.is_some());
        assert!(request.bid_response.is_none());
        assert_eq!(request.applicable_intents.len(), 2);
    }

    #[test]
    fn test_rtb_request_dsp_bid_response() {
        let request = RTBRequest::builder()
            .lifecycle(Lifecycle::DspBidResponse)
            .id("req-002".to_string())
            .bid_request(Some(serde_json::json!({"id": "auction-1"})))
            .bid_response(Some(serde_json::json!({"id": "auction-1", "seatbid": []})))
            .applicable_intents(vec![Intent::BidShade])
            .build()
            .unwrap();

        assert_eq!(request.lifecycle, Lifecycle::DspBidResponse);
        assert!(request.bid_request.is_some());
        assert!(request.bid_response.is_some());
    }

    #[test]
    fn test_rtb_request_with_originator() {
        let request = RTBRequest::builder()
            .lifecycle(Lifecycle::PublisherBidRequest)
            .id("req-003".to_string())
            .originator(Some(
                Originator::builder()
                    .type_(OriginatorType::Ssp)
                    .name("Test SSP")
                    .domain("ssp.test.com")
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let originator = request.originator.as_ref().unwrap();
        assert_eq!(originator.type_, OriginatorType::Ssp);
        assert_eq!(originator.name, Some("Test SSP".to_string()));
    }

    #[test]
    fn test_rtb_request_serialization() {
        let request = RTBRequest::builder()
            .lifecycle(Lifecycle::PublisherBidRequest)
            .id("req-004".to_string())
            .tmax(Some(50))
            .applicable_intents(vec![Intent::ActivateSegments])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"lifecycle\":1"));
        assert!(json.contains("\"id\":\"req-004\""));
        assert!(json.contains("\"tmax\":50"));
        assert!(json.contains("\"applicable_intents\":[1]"));
    }

    #[test]
    fn test_rtb_request_deserialization() {
        let json = r#"{
            "lifecycle": 1,
            "id": "req-005",
            "tmax": 75,
            "bid_request": {"id": "auction-5", "imp": [{"id": "imp-1"}]},
            "applicable_intents": [1, 2, 6]
        }"#;

        let request: RTBRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.lifecycle, Lifecycle::PublisherBidRequest);
        assert_eq!(request.id, "req-005");
        assert_eq!(request.tmax, Some(75));
        assert!(request.bid_request.is_some());
        assert_eq!(request.applicable_intents.len(), 3);
    }

    #[test]
    fn test_rtb_request_roundtrip() {
        let request = RTBRequest::builder()
            .lifecycle(Lifecycle::PublisherBidRequest)
            .id("req-006".to_string())
            .tmax(Some(100))
            .bid_request(Some(serde_json::json!({"id": "a1"})))
            .applicable_intents(vec![Intent::ActivateSegments])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: RTBRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, parsed);
    }

    #[test]
    fn test_rtb_request_default() {
        let request = RTBRequest::builder().build().unwrap();
        assert_eq!(request.lifecycle, Lifecycle::Unspecified);
        assert!(request.id.is_empty());
        assert!(request.tmax.is_none());
        assert!(request.bid_request.is_none());
        assert!(request.bid_response.is_none());
        assert!(request.originator.is_none());
        assert!(request.applicable_intents.is_empty());
    }

    #[test]
    fn test_rtb_request_empty_intents() {
        let request = RTBRequest::builder()
            .id("req-007".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"applicable_intents\":[]"));
    }
}
