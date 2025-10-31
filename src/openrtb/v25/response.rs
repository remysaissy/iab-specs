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
}

