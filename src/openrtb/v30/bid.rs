use crate::Extension;
/// OpenRTB 3.0 Bid Object
///
/// This module implements the Bid object for individual bids in a response.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Bid object (OpenRTB 3.0 Section 3.3.2)
///
/// The `Bid` object represents an offer to buy a specific item for a given price.
/// Multiple bids can be submitted for the same item at different price points.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `MediaExt` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `MacroExt` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, MediaExt: Extension, MacroExt: Extension",
    deserialize = "Ext: Extension, MediaExt: Extension, MacroExt: Extension"
))]
pub struct Bid<
    Ext: Extension = crate::DefaultExt,
    MediaExt: Extension = crate::DefaultExt,
    MacroExt: Extension = crate::DefaultExt,
> {
    /// Bidder-generated bid identifier.
    /// Used for logging and tracking.
    /// REQUIRED by the specification.
    pub id: String,

    /// ID of the item object in the request to which this bid applies.
    /// REQUIRED by the specification.
    pub item: String,

    /// Bid price in CPM (cost per mille/thousand impressions).
    /// Note: This is the bid price, not the clearing price.
    /// REQUIRED by the specification.
    pub price: f64,

    /// Reference to a deal from the request if this bid pertains to a deal.
    /// Must match a deal.id from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub deal: Option<String>,

    /// Campaign ID to assist with ad quality checking.
    /// The collection of creatives under this campaign should be consistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cid: Option<String>,

    /// Tactic ID to enable buyers to label bids for reporting.
    /// Useful for granular performance analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tactic: Option<String>,

    /// Win notice URL to be called if the bid wins.
    /// The URL can contain macros that will be substituted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nurl: Option<String>,

    /// Billing notice URL to be called when the media is rendered.
    /// Useful for viewable impression tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub burl: Option<String>,

    /// Loss notice URL to be called if the bid loses.
    /// Can contain macros including loss reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lurl: Option<String>,

    /// Advisory as to the number of seconds that may elapse between
    /// auction and fulfillment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub exp: Option<i32>,

    /// Timestamp when the creative is expected to be fulfilled.
    /// Expressed as Unix epoch time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dt: Option<i64>,

    /// Array of advertiser domains for the creative (for block list checking).
    /// Required for transparency and brand safety.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub adomain: Option<Vec<String>>,

    /// Array of content categories of the creative using IAB taxonomy.
    /// Refer to IAB Content Category taxonomy.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Array of attribute IDs that describe the creative.
    /// Refer to AdCOM Creative Attributes list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub attr: Option<Vec<i32>>,

    /// Language of the creative using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lang: Option<String>,

    /// Layer-4 domain specification for the media (Display, Video, Audio).
    /// References AdCOM Media object.
    /// This is a JSON object that varies by media type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub media: Option<Box<MediaExt>>,

    /// Self-declared creative API frameworks supported.
    /// Refer to AdCOM API Frameworks list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub apis: Option<Vec<i32>>,

    /// Bundle or package name of the app for deep linking.
    /// Used for mobile app install campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bundle: Option<String>,

    /// Advisory whether the buyer's secure rendering endpoint supports
    /// HTTPS:
    /// - 0 = non-secure
    /// - 1 = secure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub secure: Option<i32>,

    /// Indicator that the buyer has creative approval:
    /// - 0 = pending approval
    /// - 1 = pre-approved
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub purl: Option<String>,

    /// Macro object containing macro values for substitution.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub macro_: Option<Box<MacroExt>>,

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

    // Spec: Object: Bid — required id, item, price with optional deal and cid
    #[test]
    fn test_bid_creation() {
        let bid = Bid::builder()
            .id("bid-123".to_string())
            .item("item1".to_string())
            .price(5.50)
            .deal(Some("deal-456".to_string()))
            .cid(Some("campaign-789".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.id, "bid-123");
        assert_eq!(bid.item, "item1");
        assert_eq!(bid.price, 5.50);
        assert_eq!(bid.deal, Some("deal-456".to_string()));
    }

    // Spec: Object: Bid — minimal bid with id, item, price only
    #[test]
    fn test_bid_minimal() {
        let bid = Bid::builder()
            .id("bid-456".to_string())
            .item("item2".to_string())
            .price(2.00)
            .build()
            .unwrap();

        assert_eq!(bid.id, "bid-456");
        assert_eq!(bid.item, "item2");
        assert_eq!(bid.price, 2.00);
        assert_eq!(bid.deal, None);
    }

    // Spec: Object: Bid — win/billing/loss notice URLs (nurl, burl, lurl)
    #[test]
    fn test_bid_with_tracking_urls() {
        let bid = Bid::builder()
            .id("bid-789".to_string())
            .item("item3".to_string())
            .price(10.00)
            .nurl(Some(
                "https://win.example.com/?price=${AUCTION_PRICE}".to_string(),
            ))
            .burl(Some("https://bill.example.com/".to_string()))
            .lurl(Some(
                "https://loss.example.com/?reason=${AUCTION_LOSS}".to_string(),
            ))
            .build()
            .unwrap();

        assert!(bid.nurl.is_some());
        assert!(bid.burl.is_some());
        assert!(bid.lurl.is_some());
    }

    // Spec: Object: Bid — advertiser domains (adomain) and content categories (cat)
    #[test]
    fn test_bid_with_advertiser_info() {
        let bid = Bid::builder()
            .id("bid-abc".to_string())
            .item("item4".to_string())
            .price(7.25)
            .adomain(Some(vec!["advertiser.com".to_string()]))
            .cat(Some(vec!["IAB1".to_string(), "IAB2".to_string()]))
            .build()
            .unwrap();

        assert_eq!(bid.adomain.as_ref().unwrap().len(), 1);
        assert_eq!(bid.cat.as_ref().unwrap().len(), 2);
    }

    // Spec: Object: Bid — creative attributes (attr) and language (lang)
    #[test]
    fn test_bid_with_creative_attributes() {
        let bid = Bid::builder()
            .id("bid-def".to_string())
            .item("item5".to_string())
            .price(3.50)
            .attr(Some(vec![1, 2, 3]))
            .lang(Some("en".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.attr.as_ref().unwrap().len(), 3);
        assert_eq!(bid.lang, Some("en".to_string()));
    }

    // Spec: Object: Bid — JSON serialization of id, item, price, cid
    #[test]
    fn test_bid_serialization() {
        let bid = Bid::builder()
            .id("bid-123".to_string())
            .item("item1".to_string())
            .price(5.50)
            .cid(Some("campaign-789".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"id\":\"bid-123\""));
        assert!(json.contains("\"item\":\"item1\""));
        assert!(json.contains("\"price\":5.5"));
        assert!(json.contains("\"cid\":\"campaign-789\""));
    }

    // Spec: Object: Bid — JSON deserialization of id, item, price, deal, cid
    #[test]
    fn test_bid_deserialization() {
        let json = r#"{
            "id": "bid-123",
            "item": "item1",
            "price": 5.50,
            "deal": "deal-456",
            "cid": "campaign-789"
        }"#;

        let bid: Bid = serde_json::from_str(json).unwrap();
        assert_eq!(bid.id, "bid-123");
        assert_eq!(bid.item, "item1");
        assert_eq!(bid.price, 5.50);
        assert_eq!(bid.deal, Some("deal-456".to_string()));
    }

    // Spec: Object: Bid — bundle field for app install deep linking
    #[test]
    fn test_bid_with_bundle() {
        let bid = Bid::builder()
            .id("bid-mobile".to_string())
            .item("item6".to_string())
            .price(4.00)
            .bundle(Some("com.example.app".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.bundle, Some("com.example.app".to_string()));
    }

    // Spec: Object: Bid — secure flag (1=HTTPS) for rendering endpoint
    #[test]
    fn test_bid_secure_flag() {
        let bid = Bid::builder()
            .id("bid-secure".to_string())
            .item("item7".to_string())
            .price(6.00)
            .secure(Some(1))
            .build()
            .unwrap();

        assert_eq!(bid.secure, Some(1));
    }

    // Spec: Object: Bid — expiration (exp) and delivery timestamp (dt)
    #[test]
    fn test_bid_with_expiration() {
        let bid = Bid::builder()
            .id("bid-exp".to_string())
            .item("item8".to_string())
            .price(8.00)
            .exp(Some(3600))
            .dt(Some(1609459200))
            .build()
            .unwrap();

        assert_eq!(bid.exp, Some(3600));
        assert_eq!(bid.dt, Some(1609459200));
    }

    // Spec: Object: Bid — default() produces empty id, empty item, price 0.0
    #[test]
    fn test_bid_default() {
        let bid: Bid = Bid::default();
        assert_eq!(bid.id, "");
        assert_eq!(bid.item, "");
        assert_eq!(bid.price, 0.0);
        assert_eq!(bid.deal, None);
        assert_eq!(bid.cid, None);
        assert_eq!(bid.tactic, None);
        assert_eq!(bid.nurl, None);
        assert_eq!(bid.burl, None);
        assert_eq!(bid.lurl, None);
        assert_eq!(bid.exp, None);
        assert_eq!(bid.dt, None);
        assert_eq!(bid.adomain, None);
        assert_eq!(bid.cat, None);
        assert_eq!(bid.attr, None);
        assert_eq!(bid.lang, None);
        assert_eq!(bid.media, None);
        assert_eq!(bid.apis, None);
        assert_eq!(bid.bundle, None);
        assert_eq!(bid.secure, None);
        assert_eq!(bid.purl, None);
        assert_eq!(bid.macro_, None);
        assert_eq!(bid.ext, None);
    }

    // Spec: Object: Bid — serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_bid_roundtrip() {
        let bid = Bid::builder()
            .id("bid-rt".to_string())
            .item("item-rt".to_string())
            .price(7.77)
            .deal(Some("deal-rt".to_string()))
            .cid(Some("camp-rt".to_string()))
            .tactic(Some("tactic-rt".to_string()))
            .nurl(Some("https://win.example.com".to_string()))
            .lang(Some("en".to_string()))
            .secure(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&bid).unwrap();
        let deserialized: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid, deserialized);
    }

    // Spec: Object: Bid — optional fields omitted from JSON when None
    #[test]
    fn test_bid_optional_fields_not_in_json() {
        let bid = Bid::builder()
            .id("bid-min".to_string())
            .item("item-min".to_string())
            .price(1.00)
            .build()
            .unwrap();

        let json = serde_json::to_string(&bid).unwrap();
        assert!(json.contains("\"id\":\"bid-min\""));
        assert!(json.contains("\"item\":\"item-min\""));
        assert!(json.contains("\"price\":1"));
        assert!(!json.contains("\"deal\""));
        assert!(!json.contains("\"cid\""));
        assert!(!json.contains("\"tactic\""));
        assert!(!json.contains("\"nurl\""));
        assert!(!json.contains("\"burl\""));
        assert!(!json.contains("\"lurl\""));
        assert!(!json.contains("\"exp\""));
        assert!(!json.contains("\"dt\""));
        assert!(!json.contains("\"adomain\""));
        assert!(!json.contains("\"cat\""));
        assert!(!json.contains("\"attr\""));
        assert!(!json.contains("\"lang\""));
        assert!(!json.contains("\"media\""));
        assert!(!json.contains("\"apis\""));
        assert!(!json.contains("\"bundle\""));
        assert!(!json.contains("\"secure\""));
        assert!(!json.contains("\"purl\""));
        assert!(!json.contains("\"ext\""));
    }

    // Spec: Object: Bid — tactic field for buyer reporting labels
    #[test]
    fn test_bid_with_tactic() {
        let bid = Bid::builder()
            .id("bid-tac".to_string())
            .item("item-tac".to_string())
            .price(4.00)
            .tactic(Some("retargeting-q2".to_string()))
            .build()
            .unwrap();

        assert_eq!(bid.tactic, Some("retargeting-q2".to_string()));

        let json = serde_json::to_string(&bid).unwrap();
        let deserialized: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tactic, Some("retargeting-q2".to_string()));
    }

    // Spec: Object: Bid — apis field for self-declared API framework IDs
    #[test]
    fn test_bid_with_apis() {
        let bid = Bid::builder()
            .id("bid-apis".to_string())
            .item("item-apis".to_string())
            .price(3.00)
            .apis(Some(vec![3, 5, 6]))
            .build()
            .unwrap();

        let apis = bid.apis.as_ref().unwrap();
        assert_eq!(apis.len(), 3);
        assert_eq!(apis[0], 3);
        assert_eq!(apis[1], 5);
        assert_eq!(apis[2], 6);
    }

    // Spec: Object: Bid — purl field for previously approved creative URL
    #[test]
    fn test_bid_purl_field() {
        let bid = Bid::builder()
            .id("bid-purl".to_string())
            .item("item-purl".to_string())
            .price(5.00)
            .purl(Some(
                "https://creative.example.com/approved/123".to_string(),
            ))
            .build()
            .unwrap();

        assert_eq!(
            bid.purl,
            Some("https://creative.example.com/approved/123".to_string())
        );

        let json = serde_json::to_string(&bid).unwrap();
        let deserialized: Bid = serde_json::from_str(&json).unwrap();
        assert_eq!(bid.purl, deserialized.purl);
    }
}
