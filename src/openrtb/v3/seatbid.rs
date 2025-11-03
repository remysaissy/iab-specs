/// OpenRTB 3.0 Seatbid Object
///
/// This module implements the Seatbid object for seat-level bid responses.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::Bid;

/// Seatbid object (OpenRTB 3.0 Section 3.3.1)
///
/// The `Seatbid` object represents a set of bids from a buyer seat.
/// A bidder may submit multiple seatbid objects, each representing different
/// buyer seats or different strategies.
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::{Seatbid, Bid};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let seatbid = Seatbid {
///     seat: Some("seat-123".to_string()),
///     package: Some(1),
///     bid: vec![
///         Bid {
///             id: "bid-1".to_string(),
///             item: "item1".to_string(),
///             price: 5.50,
///             ..Default::default()
///         },
///         Bid {
///             id: "bid-2".to_string(),
///             item: "item2".to_string(),
///             price: 3.25,
///             ..Default::default()
///         },
///     ],
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Seatbid {
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
    pub bid: Vec<Bid>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seatbid_creation() {
        let seatbid = Seatbid {
            seat: Some("seat-123".to_string()),
            package: Some(0),
            bid: vec![
                Bid {
                    id: "bid-1".to_string(),
                    item: "item1".to_string(),
                    price: 5.50,
                    ..Default::default()
                },
                Bid {
                    id: "bid-2".to_string(),
                    item: "item2".to_string(),
                    price: 3.25,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(seatbid.seat, Some("seat-123".to_string()));
        assert_eq!(seatbid.package, Some(0));
        assert_eq!(seatbid.bid.len(), 2);
    }

    #[test]
    fn test_seatbid_single_bid() {
        let seatbid = Seatbid {
            seat: Some("seat-456".to_string()),
            bid: vec![Bid {
                id: "bid-1".to_string(),
                item: "item1".to_string(),
                price: 10.00,
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(seatbid.bid.len(), 1);
        assert_eq!(seatbid.bid[0].price, 10.00);
    }

    #[test]
    fn test_seatbid_package_bid() {
        let seatbid = Seatbid {
            seat: Some("seat-789".to_string()),
            package: Some(1), // All or nothing
            bid: vec![
                Bid {
                    id: "bid-1".to_string(),
                    item: "item1".to_string(),
                    price: 5.00,
                    ..Default::default()
                },
                Bid {
                    id: "bid-2".to_string(),
                    item: "item2".to_string(),
                    price: 5.00,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(seatbid.package, Some(1));
        assert_eq!(seatbid.bid.len(), 2);
    }

    #[test]
    fn test_seatbid_without_seat_id() {
        let seatbid = Seatbid {
            seat: None,
            bid: vec![Bid {
                id: "bid-1".to_string(),
                item: "item1".to_string(),
                price: 7.50,
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(seatbid.seat, None);
        assert_eq!(seatbid.bid.len(), 1);
    }

    #[test]
    fn test_seatbid_serialization() {
        let seatbid = Seatbid {
            seat: Some("seat-123".to_string()),
            package: Some(0),
            bid: vec![Bid {
                id: "bid-1".to_string(),
                item: "item1".to_string(),
                price: 5.50,
                ..Default::default()
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&seatbid).unwrap();
        assert!(json.contains("\"seat\":\"seat-123\""));
        assert!(json.contains("\"package\":0"));
        assert!(json.contains("\"bid\""));
    }

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

        let seatbid: Seatbid = serde_json::from_str(json).unwrap();
        assert_eq!(seatbid.seat, Some("seat-123".to_string()));
        assert_eq!(seatbid.package, Some(0));
        assert_eq!(seatbid.bid.len(), 1);
    }

    #[test]
    fn test_seatbid_builder() {
        let seatbid = SeatbidBuilder::default()
            .seat(Some("seat-123".to_string()))
            .package(Some(0))
            .bid(vec![Bid {
                id: "bid-1".to_string(),
                item: "item1".to_string(),
                price: 5.50,
                ..Default::default()
            }])
            .build()
            .unwrap();

        assert_eq!(seatbid.seat, Some("seat-123".to_string()));
        assert_eq!(seatbid.bid.len(), 1);
    }

    #[test]
    fn test_seatbid_multiple_bids_different_prices() {
        let seatbid = Seatbid {
            seat: Some("seat-multi".to_string()),
            bid: vec![
                Bid {
                    id: "bid-1".to_string(),
                    item: "item1".to_string(),
                    price: 10.00,
                    ..Default::default()
                },
                Bid {
                    id: "bid-2".to_string(),
                    item: "item1".to_string(),
                    price: 8.00,
                    ..Default::default()
                },
                Bid {
                    id: "bid-3".to_string(),
                    item: "item2".to_string(),
                    price: 6.00,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(seatbid.bid.len(), 3);
        assert_eq!(seatbid.bid[0].price, 10.00);
        assert_eq!(seatbid.bid[1].price, 8.00);
        assert_eq!(seatbid.bid[2].price, 6.00);
    }

    #[test]
    fn test_seatbid_empty_bid_array() {
        let seatbid = Seatbid {
            seat: Some("seat-empty".to_string()),
            bid: vec![],
            ..Default::default()
        };

        assert_eq!(seatbid.bid.len(), 0);
    }
}
