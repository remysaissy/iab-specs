/// OpenRTB 3.0 Response Object
///
/// This module implements the Response object for bid responses.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::Seatbid;

/// Response object (OpenRTB 3.0 Section 3.3)
///
/// The `Response` object is the top-level object in a bid response payload.
/// In OpenRTB 3.0, the response is wrapped within an [`Openrtb`](super::Openrtb)
/// container and contains seat bids with individual bid objects.
///
/// # Key Differences from OpenRTB 2.x
///
/// - Wrapped in [`Openrtb`](super::Openrtb) container
/// - Simplified structure focusing on seat bids
/// - Currency handling more explicit
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::Response;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let response = Response {
///     id: "resp-12345".to_string(),
///     bidid: Some("bid-67890".to_string()),
///     nbr: Some(0),
///     cur: Some("USD".to_string()),
///     seatbid: vec![],
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Response {
    /// ID of the bid request to which this is a response.
    /// REQUIRED by the specification.
    pub id: String,

    /// Bidder generated response ID to assist with logging/tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bidid: Option<String>,

    /// Reason for not bidding (No-Bid Reason Code):
    /// - 0 = Unknown Error
    /// - 1 = Technical Error
    /// - 2 = Invalid Request
    /// - 3 = Known Web Spider
    /// - 4 = Suspected Non-Human Traffic
    /// - 5 = Cloud, Data Center, or Proxy IP
    /// - 6 = Unsupported Device
    /// - 7 = Blocked Publisher or Site
    /// - 8 = Unmatched User
    /// - 9 = Daily Reader Cap Met
    /// - 10 = Daily Domain Cap Met
    ///
    /// If provided, seatbid must be empty or omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nbr: Option<i32>,

    /// Bid currency using ISO-4217 codes.
    /// If omitted, USD is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cur: Option<String>,

    /// Custom data that the bidder wishes to pass through to the creative markup.
    /// May be used for late binding of parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cdata: Option<String>,

    /// Array of seat bid objects.
    /// At least one seat bid is required if responding with a bid.
    #[serde(default)]
    #[builder(default)]
    pub seatbid: Vec<Seatbid>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_creation() {
        let response = Response {
            id: "resp-123".to_string(),
            bidid: Some("bid-456".to_string()),
            nbr: Some(0),
            cur: Some("USD".to_string()),
            seatbid: vec![],
            ..Default::default()
        };

        assert_eq!(response.id, "resp-123");
        assert_eq!(response.bidid, Some("bid-456".to_string()));
        assert_eq!(response.nbr, Some(0));
        assert_eq!(response.cur, Some("USD".to_string()));
    }

    #[test]
    fn test_response_no_bid() {
        let response = Response {
            id: "resp-123".to_string(),
            nbr: Some(2), // Invalid Request
            seatbid: vec![],
            ..Default::default()
        };

        assert_eq!(response.nbr, Some(2));
        assert_eq!(response.seatbid.len(), 0);
    }

    #[test]
    fn test_response_serialization() {
        let response = Response {
            id: "resp-123".to_string(),
            bidid: Some("bid-456".to_string()),
            cur: Some("USD".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"id\":\"resp-123\""));
        assert!(json.contains("\"bidid\":\"bid-456\""));
        assert!(json.contains("\"cur\":\"USD\""));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "id": "resp-123",
            "bidid": "bid-456",
            "nbr": 0,
            "cur": "USD",
            "seatbid": []
        }"#;

        let response: Response = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "resp-123");
        assert_eq!(response.bidid, Some("bid-456".to_string()));
        assert_eq!(response.nbr, Some(0));
        assert_eq!(response.cur, Some("USD".to_string()));
    }

    #[test]
    fn test_response_builder() {
        let response = ResponseBuilder::default()
            .id("resp-123".to_string())
            .bidid(Some("bid-456".to_string()))
            .cur(Some("USD".to_string()))
            .build()
            .unwrap();

        assert_eq!(response.id, "resp-123");
        assert_eq!(response.bidid, Some("bid-456".to_string()));
        assert_eq!(response.cur, Some("USD".to_string()));
    }

    #[test]
    fn test_response_with_cdata() {
        let response = Response {
            id: "resp-123".to_string(),
            cdata: Some("{\"tracking_id\":\"abc123\"}".to_string()),
            ..Default::default()
        };

        assert_eq!(
            response.cdata,
            Some("{\"tracking_id\":\"abc123\"}".to_string())
        );
    }
}
