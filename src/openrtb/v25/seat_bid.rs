use super::Bid;
use crate::Extension;

/// OpenRTB 2.5 Bid Objects
///
/// This module implements the core bidding objects for OpenRTB 2.5:
/// - Bid: Individual bid for a specific impression
/// - SeatBid: Container for bids from a single buyer seat
/// - BidResponse: Top-level bid response object
/// - BidRequest: Top-level bid request object
///
/// # OpenRTB 2.5 Specification
///
/// These objects represent the fundamental transaction protocol for real-time
/// bidding. The request flows from publisher to exchange to bidder, and the
/// response flows back with bid prices and creative information.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Collection of bids from a single buyer seat (OpenRTB 2.5 Section 4.2.2)
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SeatBid<Ext: Extension = serde_json::Value> {
    /// Array of 1+ Bid objects
    #[builder(setter(into))]
    pub bid: Vec<Bid<Ext>>,

    /// ID of the buyer seat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,

    /// Group flag: 0=individual, 1=group (default 0)
    #[serde(default)]
    #[builder(default)]
    pub group: i32,

    /// Exchange-specific extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl SeatBid {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SeatBidBuilder {
        SeatBidBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_bid_creation() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(1.0)
            .build()
            .unwrap();

        let seat_bid = SeatBid::builder()
            .bid(vec![bid])
            .seat(Some("seat1".to_string()))
            .group(0)
            .build()
            .unwrap();

        assert_eq!(seat_bid.bid.len(), 1);
        assert_eq!(seat_bid.seat, Some("seat1".to_string()));
        assert_eq!(seat_bid.group, 0);
    }

    #[test]
    fn test_seat_bid_with_multiple_bids() {
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

        let seat_bid = SeatBid::builder()
            .bid(vec![bid1, bid2])
            .seat(Some("seat1".to_string()))
            .group(0)
            .build()
            .unwrap();

        assert_eq!(seat_bid.bid.len(), 2);
        assert_eq!(seat_bid.bid[0].id, "bid1");
        assert_eq!(seat_bid.bid[1].id, "bid2");
    }

    #[test]
    fn test_seat_bid_serialization() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.5)
            .build()
            .unwrap();

        let seat_bid = SeatBid::builder()
            .bid(vec![bid])
            .seat(Some("seat123".to_string()))
            .group(0)
            .build()
            .unwrap();

        let json = serde_json::to_string(&seat_bid).unwrap();
        assert!(json.contains("\"bid\":["));
        assert!(json.contains("\"seat\":\"seat123\""));
        assert!(json.contains("\"group\":0"));
    }

    #[test]
    fn test_seat_bid_with_ext() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(1.0)
            .build()
            .unwrap();

        let ext_value = Box::new(serde_json::json!({"custom": "value"}));

        let seat_bid = SeatBid::builder()
            .bid(vec![bid])
            .seat(Some("seat1".to_string()))
            .group(0)
            .ext(Some(ext_value.clone()))
            .build()
            .unwrap();

        assert_eq!(seat_bid.ext, Some(ext_value));
    }
}
