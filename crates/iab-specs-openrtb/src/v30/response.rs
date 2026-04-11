use super::SeatBid;
use crate::Extension;
/// OpenRTB 3.0 Response Object
///
/// This module implements the Response object for bid responses.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```rust
/// use iab_specs_openrtb::v30::Response;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let response = Response::builder()
///     .id("resp-12345".to_string())
///     .bidid(Some("bid-67890".to_string()))
///     .nbr(Some(0))
///     .cur(Some("USD".to_string()))
///     .seatbid(vec![])
///     .build()
///     .unwrap();
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Response<Ext: Extension = crate::DefaultExt> {
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
    pub seatbid: Vec<SeatBid<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Response {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec: Object: Response — id, bidid, nbr, cur fields populated correctly
    #[test]
    fn test_response_creation() {
        let response = Response::builder()
            .id("resp-123".to_string())
            .bidid(Some("bid-456".to_string()))
            .nbr(Some(0))
            .cur(Some("USD".to_string()))
            .seatbid(vec![])
            .build()
            .unwrap();

        assert_eq!(response.id, "resp-123");
        assert_eq!(response.bidid, Some("bid-456".to_string()));
        assert_eq!(response.nbr, Some(0));
        assert_eq!(response.cur, Some("USD".to_string()));
    }

    // Spec: Object: Response — nbr no-bid reason code with empty seatbid
    #[test]
    fn test_response_no_bid() {
        let response = Response::builder()
            .id("resp-123".to_string())
            .nbr(Some(2))
            .seatbid(vec![])
            .build()
            .unwrap();

        assert_eq!(response.nbr, Some(2));
        assert_eq!(response.seatbid.len(), 0);
    }

    // Spec: Object: Response — serialization produces correct JSON keys for id, bidid, cur
    #[test]
    fn test_response_serialization() {
        let response = Response::builder()
            .id("resp-123".to_string())
            .bidid(Some("bid-456".to_string()))
            .cur(Some("USD".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"id\":\"resp-123\""));
        assert!(json.contains("\"bidid\":\"bid-456\""));
        assert!(json.contains("\"cur\":\"USD\""));
    }

    // Spec: Object: Response — deserialization from JSON restores id, bidid, nbr, cur fields
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

    // Spec: Object: Response — cdata custom passthrough data field
    #[test]
    fn test_response_with_cdata() {
        let response = Response::builder()
            .id("resp-123".to_string())
            .cdata(Some("{\"tracking_id\":\"abc123\"}".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            response.cdata,
            Some("{\"tracking_id\":\"abc123\"}".to_string())
        );
    }

    // Spec: Object: Response — default() produces empty id, empty seatbid vec, all Options None
    #[test]
    fn test_response_default() {
        let response: Response = Response::default();
        assert_eq!(response.id, "");
        assert!(response.seatbid.is_empty());
        assert!(response.bidid.is_none());
        assert!(response.nbr.is_none());
        assert!(response.cur.is_none());
        assert!(response.cdata.is_none());
        assert!(response.ext.is_none());
    }

    // Spec: Object: Response — roundtrip serialize/deserialize preserves all fields
    #[test]
    fn test_response_roundtrip() {
        let original = Response::builder()
            .id("resp-rt".to_string())
            .bidid(Some("bid-rt".to_string()))
            .nbr(Some(0))
            .cur(Some("EUR".to_string()))
            .cdata(Some("custom".to_string()))
            .seatbid(vec![])
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Response = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    // Spec: Object: Response — skip_serializing_if omits None optional fields from JSON
    #[test]
    fn test_response_optional_fields_not_in_json() {
        let response = Response::builder()
            .id("resp-minimal".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"id\""));
        assert!(!json.contains("\"bidid\""));
        assert!(!json.contains("\"nbr\""));
        assert!(!json.contains("\"cur\""));
        assert!(!json.contains("\"cdata\""));
        assert!(!json.contains("\"ext\""));
    }

    // Spec: Object: Response — seatbid with actual Bid objects
    #[test]
    fn test_response_with_seatbid() {
        use crate::v30::Bid;

        let bid = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(3.50)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-1".to_string()))
            .bid(vec![bid])
            .build()
            .unwrap();

        let response = Response::builder()
            .id("resp-sb".to_string())
            .seatbid(vec![seatbid])
            .build()
            .unwrap();

        assert_eq!(response.seatbid.len(), 1);
        assert_eq!(response.seatbid[0].bid.len(), 1);
        assert_eq!(response.seatbid[0].bid[0].price, 3.50);
    }

    // Spec: Object: Response — various nbr no-bid reason codes serialize correctly
    #[test]
    fn test_response_all_nbr_codes() {
        let codes = [0, 1, 2, 5, 10];
        for code in codes {
            let response = Response::builder()
                .id(format!("resp-nbr-{}", code))
                .nbr(Some(code))
                .build()
                .unwrap();

            let json = serde_json::to_string(&response).unwrap();
            let parsed: Response = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.nbr, Some(code), "Failed for nbr code {}", code);
        }
    }
}
