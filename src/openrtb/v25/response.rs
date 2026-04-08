use super::SeatBid;
use crate::Extension;
/// OpenRTB 2.5 Response Objects
///
/// This module contains the BidResponse and BidRequest objects for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{BidResponse, SeatBid, Bid};
///
/// let bid = Bid::builder()
///     .id("bid1".to_string())
///     .impid("imp1".to_string())
///     .price(1.50)
///     .adm(Some("<ad markup>".to_string()))
///     .build()
///     .unwrap();
///
/// let seatbid = SeatBid::builder()
///     .bid(vec![bid])
///     .seat(Some("seat123".to_string()))
///     .build()
///     .unwrap();
///
/// let response = BidResponse::builder()
///     .id("request123".to_string())
///     .seatbid(Some(vec![seatbid]))
///     .cur("USD".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct BidResponse<Ext: Extension = crate::DefaultExt> {
    /// ID of the bid request to which this is a response.
    /// **Required field** - must match the request ID.
    #[builder(setter(into))]
    pub id: String,

    /// Array of seatbid objects; one for each buyer seat bidding on this request.
    /// Optional - omitting or empty array indicates a no-bid.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seatbid: Option<Vec<SeatBid<Ext>>>,

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
    pub ext: Option<Box<Ext>>,
}

impl BidResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BidResponseBuilder {
        BidResponseBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openrtb::v25::{Bid, SeatBid};

    #[test]
    fn test_bid_response_creation() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.0)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder().bid(vec![bid]).build().unwrap();

        let response = BidResponse::builder()
            .id("req123".to_string())
            .seatbid(Some(vec![seatbid]))
            .cur("USD".to_string())
            .build()
            .unwrap();

        assert_eq!(response.id, "req123");
        assert!(response.seatbid.is_some());
        assert_eq!(response.cur, "USD");
    }

    #[test]
    fn test_bid_response_no_bid() {
        // Test no-bid response (empty seatbid array)
        let response = BidResponse::builder()
            .id("req789".to_string())
            .seatbid(Some(vec![]))
            .nbr(Some(2)) // No-bid reason: timeout
            .build()
            .unwrap();

        assert_eq!(response.id, "req789");
        assert!(response.seatbid.is_some());
        assert_eq!(response.seatbid.as_ref().unwrap().len(), 0);
        assert_eq!(response.nbr, Some(2));
    }

    #[test]
    fn test_bid_response_no_bid_omitted() {
        // Test no-bid response (omitted seatbid)
        let response = BidResponse::builder()
            .id("req999".to_string())
            .nbr(Some(1)) // No-bid reason: technical error
            .build()
            .unwrap();

        assert_eq!(response.id, "req999");
        assert!(response.seatbid.is_none());
        assert_eq!(response.nbr, Some(1));
    }

    #[test]
    fn test_bid_response_serialization() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.5)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .bid(vec![bid])
            .seat(Some("seat123".to_string()))
            .build()
            .unwrap();

        let response = BidResponse::builder()
            .id("req123".to_string())
            .seatbid(Some(vec![seatbid]))
            .bidid(Some("bidder_response_456".to_string()))
            .cur("EUR".to_string())
            .build()
            .unwrap();

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
        let bid1 = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(1.0)
            .build()
            .unwrap();

        let bid2 = Bid::builder()
            .id("bid2".to_string())
            .impid("imp2".to_string())
            .price(2.0)
            .build()
            .unwrap();

        let seatbid1 = SeatBid::builder()
            .bid(vec![bid1])
            .seat(Some("seat1".to_string()))
            .build()
            .unwrap();

        let seatbid2 = SeatBid::builder()
            .bid(vec![bid2])
            .seat(Some("seat2".to_string()))
            .build()
            .unwrap();

        let response = BidResponse::builder()
            .id("req123".to_string())
            .seatbid(Some(vec![seatbid1, seatbid2]))
            .build()
            .unwrap();

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
        let response = BidResponse::builder()
            .id("req123".to_string())
            .customdata(Some("custom_bidder_data".to_string()))
            .build()
            .unwrap();

        assert_eq!(response.customdata, Some("custom_bidder_data".to_string()));
    }

    #[test]
    fn test_bid_response_with_ext() {
        let ext_value = Box::new(serde_json::json!({"custom_field": "custom_value"}));

        let response = BidResponseBuilder::<serde_json::Value>::default()
            .id("req123".to_string())
            .ext(Some(ext_value.clone()))
            .build()
            .unwrap();

        assert_eq!(response.ext, Some(ext_value));
    }

    // === Spec-Driven Hardening Tests ===

    #[test]
    fn test_bid_response_nbr_field() {
        // Spec: Section 4.2.1
        // NoBidReason codes: 0=Unknown, 1=Technical Error, 2=Invalid Request,
        // 3=Known Web Spider, 4=Suspected Non-Human, 5=Cloud/Data Center/Proxy IP,
        // 6=Unsupported Device, 7=Blocked Publisher/Site, 8=Unmatched User, 9=Daily Reader Cap Met,
        // 10=Daily Domain Cap Met
        let nbr_values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for &nbr_code in &nbr_values {
            let response = BidResponse::builder()
                .id(format!("req-nbr-{}", nbr_code))
                .nbr(Some(nbr_code))
                .build()
                .unwrap();

            assert_eq!(response.nbr, Some(nbr_code));

            // Verify serde round-trip
            let json = serde_json::to_string(&response).unwrap();
            let deserialized: BidResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.nbr, Some(nbr_code));
        }

        // Exchange-specific codes (>= 10000) are also valid
        let response = BidResponse::builder()
            .id("req-nbr-custom".to_string())
            .nbr(Some(10001))
            .build()
            .unwrap();
        assert_eq!(response.nbr, Some(10001));
    }

    #[test]
    fn test_bid_response_roundtrip_all_fields() {
        // Spec: Section 4.2.1
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(3.50)
            .adm(Some("<div>ad</div>".to_string()))
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .bid(vec![bid])
            .seat(Some("seat-xyz".to_string()))
            .group(1)
            .build()
            .unwrap();

        let response = BidResponse::builder()
            .id("req-full".to_string())
            .seatbid(Some(vec![seatbid]))
            .bidid(Some("bidder-resp-123".to_string()))
            .cur("EUR".to_string())
            .customdata(Some("base85data".to_string()))
            .nbr(Some(0))
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: BidResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "req-full");
        assert_eq!(deserialized.seatbid.as_ref().unwrap().len(), 1);
        assert_eq!(
            deserialized.seatbid.as_ref().unwrap()[0].seat,
            Some("seat-xyz".to_string())
        );
        assert_eq!(deserialized.bidid, Some("bidder-resp-123".to_string()));
        assert_eq!(deserialized.cur, "EUR");
        assert_eq!(deserialized.customdata, Some("base85data".to_string()));
        assert_eq!(deserialized.nbr, Some(0));
    }

    #[test]
    fn test_bid_response_bidid_field() {
        // Spec: Section 4.2.1
        // bidid: Bidder-generated response ID for logging/tracking
        let response = BidResponse::builder()
            .id("req-123".to_string())
            .bidid(Some("bidder-unique-456".to_string()))
            .build()
            .unwrap();

        assert_eq!(response.bidid, Some("bidder-unique-456".to_string()));

        // Verify it serializes when present
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"bidid\":\"bidder-unique-456\""));

        // Verify it's omitted when None
        let response_no_bidid = BidResponse::builder()
            .id("req-789".to_string())
            .build()
            .unwrap();
        let json_no_bidid = serde_json::to_string(&response_no_bidid).unwrap();
        assert!(!json_no_bidid.contains("bidid"));
    }

    #[test]
    fn test_bid_response_empty_seatbid_vs_none() {
        // Spec: Section 4.2.1
        // Both empty seatbid array and omitted seatbid indicate a no-bid,
        // but they are structurally different.

        // Case 1: seatbid = None (omitted entirely)
        let response_none = BidResponse::builder()
            .id("req-none".to_string())
            .build()
            .unwrap();
        assert!(response_none.seatbid.is_none());

        let json_none = serde_json::to_string(&response_none).unwrap();
        assert!(
            !json_none.contains("seatbid"),
            "None seatbid should be omitted from JSON"
        );

        // Case 2: seatbid = Some(vec![]) (empty array)
        let response_empty = BidResponse::builder()
            .id("req-empty".to_string())
            .seatbid(Some(vec![]))
            .build()
            .unwrap();
        assert!(response_empty.seatbid.is_some());
        assert_eq!(response_empty.seatbid.as_ref().unwrap().len(), 0);

        let json_empty = serde_json::to_string(&response_empty).unwrap();
        assert!(
            json_empty.contains("\"seatbid\":[]"),
            "Empty seatbid array should serialize as []"
        );

        // Verify deserialization distinguishes the two cases
        let deser_none: BidResponse = serde_json::from_str(&json_none).unwrap();
        assert!(deser_none.seatbid.is_none());

        let deser_empty: BidResponse = serde_json::from_str(&json_empty).unwrap();
        assert!(deser_empty.seatbid.is_some());
        assert_eq!(deser_empty.seatbid.unwrap().len(), 0);
    }
}
