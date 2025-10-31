/// OpenRTB 2.5 Request Objects
///
/// This module contains the BidRequest object for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::app::App;
use super::device::Device;
use super::imp::Imp;
use super::regs::Regs;
use super::site::Site;
use super::source::Source;
use super::user::User;

/// Default auction type for bid requests (Second Price Plus per OpenRTB 2.5 spec)
fn default_auction_type() -> i32 {
    2
}

/// Top-level bid request object (OpenRTB 2.5 Section 3.2.1)
///
/// A `BidRequest` is the top-level object sent from an exchange to a bidder.
/// It must contain at least one impression object and a unique request ID.
///
/// The bid request provides context about the impression(s), the user, the device,
/// and any regulatory or publisher requirements. Bidders use this information
/// to decide whether and how much to bid.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{BidRequest, Imp, Banner};
///
/// let imp = Imp {
///     id: "imp1".to_string(),
///     banner: Some(Banner { w: Some(300), h: Some(250), ..Default::default() }),
///     ..Default::default()
/// };
///
/// let request = BidRequest {
///     id: "request123".to_string(),
///     imp: vec![imp],
///     at: 2,  // Second price auction
///     tmax: Some(120),
///     ..Default::default()
/// };
/// ```
///
/// All objects in BidRequest are now fully typed as of Phase 2, Commit 7.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct BidRequest {
    /// Unique ID of the bid request, provided by the exchange.
    /// **Required field**.
    #[builder(setter(into))]
    pub id: String,

    /// Array of Imp objects representing the impressions offered.
    /// **Required field** - must contain at least one impression.
    #[builder(setter(into))]
    pub imp: Vec<Imp>,

    /// Details via a Site object about the publisher's website.
    /// Only applicable and recommended for websites.
    /// Exactly one of Site or App should be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub site: Option<Site>,

    /// Details via an App object about the publisher's app.
    /// Only applicable and recommended for apps.
    /// Exactly one of Site or App should be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub app: Option<App>,

    /// Details via a Device object about the user's device.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub device: Option<Device>,

    /// Details via a User object about the human user of the device.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user: Option<User>,

    /// Indicator of test mode in which auctions are not billable:
    /// - 0 = live mode (default)
    /// - 1 = test mode
    #[serde(default)]
    #[builder(default)]
    pub test: i32,

    /// Auction type, where:
    /// - 1 = First Price
    /// - 2 = Second Price Plus (default)
    /// - 3 = Fixed Price (for deals)
    /// Exchange-specific auction types can be defined using values > 500.
    #[serde(default = "default_auction_type")]
    #[builder(default = "default_auction_type()")]
    pub at: i32,

    /// Maximum time in milliseconds the exchange allows for bids to be received,
    /// including network routing time. Exceeding this may result in bid loss.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tmax: Option<i32>,

    /// Whitelist of buyer seats (e.g., advertisers, agencies) allowed to bid on
    /// this impression. Knowledge of buyer's customers to which this restriction
    /// applies must be coordinated between the buyer and exchange a priori.
    /// Omission implies no seat restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wseat: Option<Vec<String>>,

    /// Blocklist of buyer seats restricted from bidding on this impression.
    /// Knowledge of buyer's customers to which this restriction applies must
    /// be coordinated between the buyer and exchange a priori.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bseat: Option<Vec<String>>,

    /// Flag to indicate if Exchange can verify that the impressions offered
    /// represent all of the impressions available in context:
    /// - 0 = no or unknown (default)
    /// - 1 = yes, all impressions represented
    #[serde(default)]
    #[builder(default)]
    pub allimps: i32,

    /// Array of allowed currencies for bids on this bid request using ISO-4217
    /// alpha codes. Recommended if the exchange accepts multiple currencies.
    /// If omitted, assume USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cur: Option<Vec<String>>,

    /// Whitelist of languages for creatives using ISO-639-1-alpha-2.
    /// Omission implies no specific restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wlang: Option<Vec<String>>,

    /// Blocked advertiser categories using the IAB Content Category taxonomy.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bcat: Option<Vec<String>>,

    /// Block list of advertiser domains (e.g., "ford.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub badv: Option<Vec<String>>,

    /// Block list of applications by their platform-specific exchange-independent
    /// application identifiers. On Android, these should be bundle or package
    /// names (e.g., com.foo.mygame). On iOS, these are numeric IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bapp: Option<Vec<String>>,

    /// A Source object that provides data about the inventory source and
    /// which entity makes the final decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,

    /// A Regs object that specifies any industry, legal, or governmental
    /// regulations in force for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub regs: Option<Regs>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_request_creation() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            test: 0,
            at: 2,
            ..Default::default()
        };

        assert_eq!(request.id, "req123");
        assert_eq!(request.imp.len(), 1);
        assert_eq!(request.imp[0].id, "imp1");
        assert_eq!(request.at, 2);
        assert_eq!(request.test, 0);
        assert_eq!(request.allimps, 0);
    }

    #[test]
    fn test_bid_request_serialization() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"id\":\"req123\""));
        assert!(json.contains("\"imp\":["));
        assert!(json.contains("\"id\":\"imp1\""));
    }

    #[test]
    fn test_bid_request_deserialization() {
        let json = r#"{"id":"req123","imp":[{"id":"imp1"}],"at":2}"#;
        let request: BidRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.id, "req123");
        assert_eq!(request.imp.len(), 1);
        assert_eq!(request.imp[0].id, "imp1");
        assert_eq!(request.at, 2);
    }

    #[test]
    fn test_bid_request_default_auction_type() {
        // Test that deserialization uses default auction type when not specified
        let json = r#"{"id":"req123","imp":[{"id":"imp1"}]}"#;
        let request: BidRequest = serde_json::from_str(json).unwrap();

        // Should default to 2 (Second Price Plus) per OpenRTB 2.5 spec when deserialized
        assert_eq!(request.at, 2);
    }

    #[test]
    fn test_bid_request_with_site() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let site = crate::openrtb::v25::Site {
            id: Some("site123".to_string()),
            domain: Some("example.com".to_string()),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            site: Some(site),
            ..Default::default()
        };

        assert!(request.site.is_some());
        assert_eq!(
            request.site.as_ref().unwrap().domain,
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_bid_request_with_app() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let app = crate::openrtb::v25::App {
            id: Some("app123".to_string()),
            bundle: Some("com.example.app".to_string()),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            app: Some(app),
            ..Default::default()
        };

        assert!(request.app.is_some());
        assert_eq!(
            request.app.as_ref().unwrap().bundle,
            Some("com.example.app".to_string())
        );
    }

    #[test]
    fn test_bid_request_with_device() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let device = crate::openrtb::v25::Device {
            ua: Some("Mozilla/5.0".to_string()),
            ip: Some("192.168.1.1".to_string()),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            device: Some(device),
            ..Default::default()
        };

        assert!(request.device.is_some());
        assert_eq!(
            request.device.as_ref().unwrap().ip,
            Some("192.168.1.1".to_string())
        );
    }

    #[test]
    fn test_bid_request_with_user() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let user = crate::openrtb::v25::User {
            id: Some("user123".to_string()),
            yob: Some(1990),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            user: Some(user),
            ..Default::default()
        };

        assert!(request.user.is_some());
        assert_eq!(
            request.user.as_ref().unwrap().id,
            Some("user123".to_string())
        );
    }

    #[test]
    fn test_bid_request_test_mode() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            test: 1, // Test mode
            ..Default::default()
        };

        assert_eq!(request.test, 1);
    }

    #[test]
    fn test_bid_request_with_blocklists() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            bcat: Some(vec!["IAB25".to_string(), "IAB26".to_string()]),
            badv: Some(vec!["competitor.com".to_string()]),
            bapp: Some(vec!["com.competitor.app".to_string()]),
            ..Default::default()
        };

        assert_eq!(request.bcat.as_ref().unwrap().len(), 2);
        assert_eq!(request.badv.as_ref().unwrap().len(), 1);
        assert_eq!(request.bapp.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_bid_request_with_tmax() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            tmax: Some(120), // 120ms timeout
            ..Default::default()
        };

        assert_eq!(request.tmax, Some(120));
    }

    #[test]
    fn test_bid_request_with_source() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let source = crate::openrtb::v25::Source {
            fd: Some(1),
            tid: Some("transaction123".to_string()),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            source: Some(source),
            ..Default::default()
        };

        assert!(request.source.is_some());
        assert_eq!(request.source.as_ref().unwrap().fd, Some(1));
    }

    #[test]
    fn test_bid_request_with_regs() {
        let imp = Imp {
            id: "imp1".to_string(),
            ..Default::default()
        };

        let regs = crate::openrtb::v25::Regs {
            coppa: Some(1),
            ..Default::default()
        };

        let request = BidRequest {
            id: "req123".to_string(),
            imp: vec![imp],
            regs: Some(regs),
            ..Default::default()
        };

        assert!(request.regs.is_some());
        assert_eq!(request.regs.as_ref().unwrap().coppa, Some(1));
    }
}
