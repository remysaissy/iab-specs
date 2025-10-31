/// OpenRTB 2.5 Response Objects
///
/// This module contains the BidResponse and BidRequest objects for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::bid::SeatBid;

/// Default currency for bid responses (USD per OpenRTB 2.5 spec)
fn default_currency() -> String {
    "USD".to_string()
}

/// Top-level bid response object (OpenRTB 2.5 Section 4.2.1)
///
/// A `BidResponse` is the top-level response object returned by a bidder to
/// an exchange. It contains one or more `SeatBid` objects, each representing
/// bids from a specific buyer seat.
///
/// At minimum, a response must echo the request ID. An empty `seatbid` array
/// or omitting it entirely indicates a no-bid.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{BidResponse, SeatBid, Bid};
///
/// let bid = Bid {
///     id: "bid1".to_string(),
///     impid: "imp1".to_string(),
///     price: 1.50,
///     adm: Some("<ad markup>".to_string()),
///     ..Default::default()
/// };
///
/// let seatbid = SeatBid {
///     bid: vec![bid],
///     seat: Some("seat123".to_string()),
///     ..Default::default()
/// };
///
/// let response = BidResponse {
///     id: "request123".to_string(),
///     seatbid: Some(vec![seatbid]),
///     cur: "USD".to_string(),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct BidResponse {
    /// ID of the bid request to which this is a response.
    /// **Required field** - must match the request ID.
    #[builder(setter(into))]
    pub id: String,

    /// Array of seatbid objects; one for each buyer seat bidding on this request.
    /// Optional - omitting or empty array indicates a no-bid.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seatbid: Option<Vec<SeatBid>>,

    /// Bidder generated response ID to assist with logging/tracking.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bidid: Option<String>,

    /// Bid currency using ISO-4217 alpha codes.
    /// Default is "USD" if not specified.
    #[serde(default = "default_currency")]
    #[builder(default = "default_currency()")]
    pub cur: String,

    /// Optional feature to allow a bidder to set data in the exchange's cookie.
    /// The string must be in base85 cookie safe characters and be in any format.
    /// Proper JSON encoding must be used to include "escaped" quotation marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub customdata: Option<String>,

    /// Reason for not bidding. Refer to enum `NoBidReason`.
    /// Should only be used when `seatbid` is empty or omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nbr: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openrtb::v25::{Bid, SeatBid};

    #[test]
    fn test_bid_response_creation() {
        let bid = Bid {
            id: "bid1".to_string(),
            impid: "imp1".to_string(),
            price: 2.0,
            ..Default::default()
        };

        let seatbid = SeatBid {
            bid: vec![bid],
            ..Default::default()
        };

        let response = BidResponse {
            id: "req123".to_string(),
            seatbid: Some(vec![seatbid]),
            cur: "USD".to_string(),
            ..Default::default()
        };

        assert_eq!(response.id, "req123");
        assert!(response.seatbid.is_some());
        assert_eq!(response.cur, "USD");
    }

    #[test]
    fn test_bid_response_builder() {
        let bid = Bid {
            id: "bid1".to_string(),
            impid: "imp1".to_string(),
            price: 1.5,
            ..Default::default()
        };

        let seatbid = SeatBid {
            bid: vec![bid],
            seat: Some("seat1".to_string()),
            ..Default::default()
        };

        let response = BidResponseBuilder::default()
            .id("req456")
            .seatbid(Some(vec![seatbid]))
            .bidid(Some("bidder_resp_123".to_string()))
            .build()
            .unwrap();

        assert_eq!(response.id, "req456");
        assert_eq!(response.bidid, Some("bidder_resp_123".to_string()));
        assert_eq!(response.cur, "USD"); // Default value
    }

    #[test]
    fn test_bid_response_no_bid() {
        // Test no-bid response (empty seatbid array)
        let response = BidResponse {
            id: "req789".to_string(),
            seatbid: Some(vec![]),
            nbr: Some(2), // No-bid reason: timeout
            ..Default::default()
        };

        assert_eq!(response.id, "req789");
        assert!(response.seatbid.is_some());
        assert_eq!(response.seatbid.as_ref().unwrap().len(), 0);
        assert_eq!(response.nbr, Some(2));
    }

    #[test]
    fn test_bid_response_no_bid_omitted() {
        // Test no-bid response (omitted seatbid)
        let response = BidResponse {
            id: "req999".to_string(),
            seatbid: None,
            nbr: Some(1), // No-bid reason: technical error
            ..Default::default()
        };

        assert_eq!(response.id, "req999");
        assert!(response.seatbid.is_none());
        assert_eq!(response.nbr, Some(1));
    }

    #[test]
    fn test_bid_response_serialization() {
        let bid = Bid {
            id: "bid1".to_string(),
            impid: "imp1".to_string(),
            price: 2.5,
            ..Default::default()
        };

        let seatbid = SeatBid {
            bid: vec![bid],
            seat: Some("seat123".to_string()),
            ..Default::default()
        };

        let response = BidResponse {
            id: "req123".to_string(),
            seatbid: Some(vec![seatbid]),
            bidid: Some("bidder_response_456".to_string()),
            cur: "EUR".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"id\":\"req123\""));
        assert!(json.contains("\"bidid\":\"bidder_response_456\""));
        assert!(json.contains("\"cur\":\"EUR\""));
        assert!(json.contains("\"seatbid\":["));
    }

    #[test]
    fn test_bid_response_deserialization() {
        let json = r#"{"id":"req123","seatbid":[{"bid":[{"id":"bid1","impid":"imp1","price":2.5}]}],"cur":"USD"}"#;
        let response: BidResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.id, "req123");
        assert!(response.seatbid.is_some());
        assert_eq!(response.cur, "USD");
        assert_eq!(response.seatbid.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_bid_response_default_currency() {
        // Test that deserialization uses default currency when not specified
        let json = r#"{"id":"req123"}"#;
        let response: BidResponse = serde_json::from_str(json).unwrap();

        // Should default to "USD" per OpenRTB 2.5 spec when deserialized
        assert_eq!(response.cur, "USD");
    }

    #[test]
    fn test_bid_response_multiple_seatbids() {
        let bid1 = Bid {
            id: "bid1".to_string(),
            impid: "imp1".to_string(),
            price: 1.0,
            ..Default::default()
        };

        let bid2 = Bid {
            id: "bid2".to_string(),
            impid: "imp2".to_string(),
            price: 2.0,
            ..Default::default()
        };

        let seatbid1 = SeatBid {
            bid: vec![bid1],
            seat: Some("seat1".to_string()),
            ..Default::default()
        };

        let seatbid2 = SeatBid {
            bid: vec![bid2],
            seat: Some("seat2".to_string()),
            ..Default::default()
        };

        let response = BidResponse {
            id: "req123".to_string(),
            seatbid: Some(vec![seatbid1, seatbid2]),
            ..Default::default()
        };

        assert_eq!(response.seatbid.as_ref().unwrap().len(), 2);
        assert_eq!(
            response.seatbid.as_ref().unwrap()[0].seat,
            Some("seat1".to_string())
        );
        assert_eq!(
            response.seatbid.as_ref().unwrap()[1].seat,
            Some("seat2".to_string())
        );
    }

    #[test]
    fn test_bid_response_with_customdata() {
        let response = BidResponse {
            id: "req123".to_string(),
            customdata: Some("custom_bidder_data".to_string()),
            ..Default::default()
        };

        assert_eq!(response.customdata, Some("custom_bidder_data".to_string()));
    }

    #[test]
    fn test_bid_response_with_ext() {
        let ext_value = serde_json::json!({"custom_field": "custom_value"});

        let response = BidResponse {
            id: "req123".to_string(),
            ext: Some(ext_value.clone()),
            ..Default::default()
        };

        assert_eq!(response.ext, Some(ext_value));
    }
}
