use super::Bid;
use crate::Extension;
/// OpenRTB 3.0 Seatbid Object
///
/// This module implements the Seatbid object for seat-level bid responses.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Seatbid object (OpenRTB 3.0 Section 3.3.1)
///
/// The `Seatbid` object represents a set of bids from a buyer seat.
/// A bidder may submit multiple seatbid objects, each representing different
/// buyer seats or different strategies.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```rust
/// use iab_specs_openrtb::v30::{SeatBid, Bid};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let seatbid = SeatBid::builder()
///     .seat(Some("seat-123".to_string()))
///     .package(Some(1))
///     .bid(vec![
///         Bid::builder()
///             .id("bid-1".to_string())
///             .item("item1".to_string())
///             .price(5.50)
///             .build()
///             .unwrap(),
///         Bid::builder()
///             .id("bid-2".to_string())
///             .item("item2".to_string())
///             .price(3.25)
///             .build()
///             .unwrap()
///     ])
///     .build()
///     .unwrap();
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SeatBid<Ext: Extension = crate::DefaultExt> {
    /// ID of the buyer seat on whose behalf this bid is made.
    /// This allows buyers to submit bids for multiple seats.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seat: Option<String>,

    /// Indicator that the bids should be treated as a package:
    /// - 0 = individual bids can be accepted separately (default)
    /// - 1 = package bid (all or nothing)
    ///
    /// If package=1, all bids must win together or all lose together.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub package: Option<i32>,

    /// Array of bid objects.
    /// At least one bid is required.
    /// Each bid represents an offer for a specific item.
    pub bid: Vec<Bid<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
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

    // Spec: Object: SeatBid — seat, package flag, and bid array with multiple bids
    #[test]
    fn test_seatbid_creation() {
        let bid1 = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(5.50)
            .build()
            .unwrap();

        let bid2 = Bid::builder()
            .id("bid-2".to_string())
            .item("item2".to_string())
            .price(3.25)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-123".to_string()))
            .package(Some(0))
            .bid(vec![bid1, bid2])
            .build()
            .unwrap();

        assert_eq!(seatbid.seat, Some("seat-123".to_string()));
        assert_eq!(seatbid.package, Some(0));
        assert_eq!(seatbid.bid.len(), 2);
    }

    // Spec: Object: SeatBid — single bid in bid array
    #[test]
    fn test_seatbid_single_bid() {
        let bid = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(10.00)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-456".to_string()))
            .bid(vec![bid])
            .build()
            .unwrap();

        assert_eq!(seatbid.bid.len(), 1);
        assert_eq!(seatbid.bid[0].price, 10.00);
    }

    // Spec: Object: SeatBid — package=1 for all-or-nothing bidding
    #[test]
    fn test_seatbid_package_bid() {
        let bid1 = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(5.00)
            .build()
            .unwrap();

        let bid2 = Bid::builder()
            .id("bid-2".to_string())
            .item("item2".to_string())
            .price(5.00)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-789".to_string()))
            .package(Some(1))
            .bid(vec![bid1, bid2])
            .build()
            .unwrap();

        assert_eq!(seatbid.package, Some(1));
        assert_eq!(seatbid.bid.len(), 2);
    }

    // Spec: Object: SeatBid — seat is optional, None when omitted
    #[test]
    fn test_seatbid_without_seat_id() {
        let bid = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(7.50)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(None)
            .bid(vec![bid])
            .build()
            .unwrap();

        assert_eq!(seatbid.seat, None);
        assert_eq!(seatbid.bid.len(), 1);
    }

    // Spec: Object: SeatBid — JSON serialization of seat, package, bid array
    #[test]
    fn test_seatbid_serialization() {
        let bid = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(5.50)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-123".to_string()))
            .package(Some(0))
            .bid(vec![bid])
            .build()
            .unwrap();

        let json = serde_json::to_string(&seatbid).unwrap();
        assert!(json.contains("\"seat\":\"seat-123\""));
        assert!(json.contains("\"package\":0"));
        assert!(json.contains("\"bid\""));
    }

    // Spec: Object: SeatBid — JSON deserialization of seat, package, bid array
    #[test]
    fn test_seatbid_deserialization() {
        let json = r#"{
            "seat": "seat-123",
            "package": 0,
            "bid": [
                {
                    "id": "bid-1",
                    "item": "item1",
                    "price": 5.50
                }
            ]
        }"#;

        let seatbid: SeatBid = serde_json::from_str(json).unwrap();
        assert_eq!(seatbid.seat, Some("seat-123".to_string()));
        assert_eq!(seatbid.package, Some(0));
        assert_eq!(seatbid.bid.len(), 1);
    }

    // Spec: Object: SeatBid — multiple bids at different price points for same/different items
    #[test]
    fn test_seatbid_multiple_bids_different_prices() {
        let bid1 = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(10.00)
            .build()
            .unwrap();

        let bid2 = Bid::builder()
            .id("bid-2".to_string())
            .item("item1".to_string())
            .price(8.00)
            .build()
            .unwrap();

        let bid3 = Bid::builder()
            .id("bid-3".to_string())
            .item("item2".to_string())
            .price(6.00)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-multi".to_string()))
            .bid(vec![bid1, bid2, bid3])
            .build()
            .unwrap();

        assert_eq!(seatbid.bid.len(), 3);
        assert_eq!(seatbid.bid[0].price, 10.00);
        assert_eq!(seatbid.bid[1].price, 8.00);
        assert_eq!(seatbid.bid[2].price, 6.00);
    }

    // Spec: Object: SeatBid — empty bid array is allowed by builder
    #[test]
    fn test_seatbid_empty_bid_array() {
        let seatbid = SeatBid::builder()
            .seat(Some("seat-empty".to_string()))
            .bid(vec![])
            .build()
            .unwrap();

        assert_eq!(seatbid.bid.len(), 0);
    }

    // Spec: Object: SeatBid — default() has None seat, None package, empty bid vec
    #[test]
    fn test_seatbid_default() {
        let seatbid = SeatBid::<crate::DefaultExt>::default();
        assert_eq!(seatbid.seat, None);
        assert_eq!(seatbid.package, None);
        assert!(seatbid.bid.is_empty());
        assert_eq!(seatbid.ext, None);
    }

    // Spec: Object: SeatBid — serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_seatbid_roundtrip() {
        let bid = Bid::builder()
            .id("bid-rt".to_string())
            .item("item-rt".to_string())
            .price(4.50)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-rt".to_string()))
            .package(Some(1))
            .bid(vec![bid])
            .build()
            .unwrap();

        let json = serde_json::to_string(&seatbid).unwrap();
        let deserialized: SeatBid = serde_json::from_str(&json).unwrap();
        assert_eq!(seatbid, deserialized);
    }

    // Spec: Object: SeatBid — optional seat and package omitted from JSON when None
    #[test]
    fn test_seatbid_optional_fields_not_in_json() {
        let bid = Bid::builder()
            .id("bid-min".to_string())
            .item("item-min".to_string())
            .price(2.00)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder().bid(vec![bid]).build().unwrap();

        let json = serde_json::to_string(&seatbid).unwrap();
        assert!(json.contains("\"bid\""));
        assert!(!json.contains("\"seat\""));
        assert!(!json.contains("\"package\""));
        assert!(!json.contains("\"ext\""));
    }
}
