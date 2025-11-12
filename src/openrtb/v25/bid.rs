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

/// Individual bid for a specific impression (OpenRTB 2.5 Section 4.2.3)
///
/// A `Bid` object represents a bidder's offer to serve an ad for a specific
/// impression. It includes the bid price, creative information, and optional
/// metadata for tracking and verification.
///
/// At least one of `adm` or `nurl` is typically required for a winning bid.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Bid;
///
/// let bid = Bid::builder()
///     .id("bid123".to_string())
///     .impid("imp1".to_string())
///     .price(2.50)
///     .adm(Some("<ad markup>".to_string()))
///     .build()
///     .unwrap();
///
/// let json = serde_json::to_string(&bid).unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Bid<Ext: Extension = serde_json::Value> {
    /// Bidder generated bid ID to assist with logging/tracking.
    /// Recommended by the OpenRTB specification.
    #[builder(setter(into))]
    pub id: String,

    /// ID of the Imp object in the related bid request.
    /// **Required field** - must reference a valid impression from the request.
    #[serde(rename = "impid")]
    #[builder(setter(into))]
    pub impid: String,

    /// Bid price expressed as CPM although the actual transaction is for a unit
    /// impression only. Note that while the type is float, integer pricing is highly
    /// recommended.
    /// **Required field** - must be greater than or equal to 0.
    pub price: f64,

    /// ID of a preloaded ad to be served if the bid wins.
    /// Optional field for tracking/auditing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub adid: Option<String>,

    /// Win notice URL called by the exchange if the bid wins; optional means of
    /// serving ad markup. Substitution macros may be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nurl: Option<String>,

    /// Billing notice URL called by the exchange when a winning bid becomes
    /// billable based on exchange-specific business policy.
    /// Added in OpenRTB 2.3; substitution macros may be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub burl: Option<String>,

    /// Loss notice URL called by the exchange when a bid is known to have been lost.
    /// Substitution macros may be included. Exchange-specific policy may preclude
    /// support for loss notices or the disclosure of winning clearing prices
    /// resulting in ${AUCTION_PRICE} macros being removed (i.e., replaced with a
    /// zero-length string).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lurl: Option<String>,

    /// Optional means of conveying ad markup in case the bid wins; supersedes the
    /// win notice if markup is included in both. For native ad bids, this will
    /// be a JSON-encoded Native response. For banner/video, this will be XHTML
    /// or VAST XML respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub adm: Option<String>,

    /// Advertiser domain for block list checking (e.g., "ford.com"). This can be
    /// an array of domains for multiple advertisers. Exchanges may mandate which
    /// URL to use depending on their requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub adomain: Option<Vec<String>>,

    /// Sample image URL (without cache busting) for content checking.
    /// Primarily used for banner ads and may be required by some exchanges.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub iurl: Option<String>,

    /// Campaign ID to assist with ad quality checking; the collection of creatives
    /// for which `iurl` should be representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cid: Option<String>,

    /// Creative ID to assist with ad quality checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub crid: Option<String>,

    /// Tactic ID to enable buyers to label bids for reporting to the exchange the
    /// tactic through which their bid was submitted. The specific usage and meaning
    /// of the tactic ID should be communicated between buyer and exchanges a priori.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tactic: Option<String>,

    /// IAB content categories of the creative using the taxonomy specified in the
    /// `cattax` field of the bid request. Refer to enum `CategoryTaxonomy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Set of attributes describing the creative. Refer to AdCOM `CreativeAttribute`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub attr: Option<Vec<i32>>,

    /// Language of the creative using ISO-639-1-alpha-2. In OpenRTB 2.5, only one
    /// language is supported per bid. OpenRTB 2.6 introduced `langb` for BCP 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub language: Option<String>,

    /// Reference to the deal ID from the bid request if this bid pertains to a
    /// private marketplace direct deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dealid: Option<String>,

    /// Width of the creative in device independent pixels (DIPS).
    /// Recommended for banner and native ads when not using sizes from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Height of the creative in device independent pixels (DIPS).
    /// Recommended for banner and native ads when not using sizes from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Bid {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BidBuilder {
        BidBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_creation_and_serialization() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(1.50)
            .adm(Some("<ad markup>".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.id, "bid1");
        assert_eq!(bid.price, 1.50);

        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"id\":\"bid1\""));
        assert!(json.contains("\"price\":1.5"));
    }

    #[test]
    fn test_bid_deserialization() {
        let json = r#"{"id":"bid1","impid":"imp1","price":2.5}"#;
        let bid: Bid = serde_json::from_str(json).unwrap();

        assert_eq!(bid.id, "bid1");
        assert_eq!(bid.impid, "imp1");
        assert_eq!(bid.price, 2.5);
    }

    #[test]
    fn test_bid_with_win_notice() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.5)
            .nurl(Some(
                "https://win.example.com?price=${AUCTION_PRICE}".to_string(),
            ))
            .build()
            .unwrap();

        assert_eq!(
            bid.nurl,
            Some("https://win.example.com?price=${AUCTION_PRICE}".to_string())
        );
    }

    #[test]
    fn test_bid_with_billing_loss_notice() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.5)
            .burl(Some("https://billing.example.com".to_string()))
            .lurl(Some(
                "https://loss.example.com?reason=${AUCTION_LOSS}".to_string(),
            ))
            .build()
            .unwrap();

        assert_eq!(bid.burl, Some("https://billing.example.com".to_string()));
        assert_eq!(
            bid.lurl,
            Some("https://loss.example.com?reason=${AUCTION_LOSS}".to_string())
        );
    }

    #[test]
    fn test_bid_with_adm() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(3.0)
            .adm(Some("<html><body>Ad Content</body></html>".to_string()))
            .build()
            .unwrap();

        assert!(bid.adm.is_some());
        assert!(bid.adm.unwrap().contains("Ad Content"));
    }

    #[test]
    fn test_bid_with_adomain() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(1.5)
            .adomain(Some(vec![
                "advertiser.com".to_string(),
                "brand.com".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(bid.adomain.as_ref().unwrap().len(), 2);
        assert_eq!(bid.adomain.as_ref().unwrap()[0], "advertiser.com");
    }

    #[test]
    fn test_bid_with_creative_info() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.0)
            .cid(Some("campaign123".to_string()))
            .crid(Some("creative456".to_string()))
            .iurl(Some("https://sample.example.com/creative.jpg".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.cid, Some("campaign123".to_string()));
        assert_eq!(bid.crid, Some("creative456".to_string()));
        assert!(bid.iurl.is_some());
    }

    #[test]
    fn test_bid_with_dimensions() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(1.75)
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(bid.w, Some(300));
        assert_eq!(bid.h, Some(250));
    }

    #[test]
    fn test_bid_with_dealid() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(5.0)
            .dealid(Some("deal789".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.dealid, Some("deal789".to_string()));
    }

    #[test]
    fn test_bid_with_categories_and_attributes() {
        let bid = Bid::builder()
            .id("bid1".to_string())
            .impid("imp1".to_string())
            .price(2.0)
            .cat(Some(vec!["IAB1".to_string(), "IAB2".to_string()]))
            .attr(Some(vec![1, 2, 3]))
            .build()
            .unwrap();

        assert_eq!(bid.cat.as_ref().unwrap().len(), 2);
        assert_eq!(bid.attr.as_ref().unwrap().len(), 3);
    }
}
