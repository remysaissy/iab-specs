/// OpenRTB 2.5/2.6 Request Objects
///
/// This module contains the BidRequest object for OpenRTB 2.5 and 2.6.
/// OpenRTB 2.6 adds support for the dooh field.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::app::App;
use super::device::Device;
use super::imp::Imp;
use super::regs::Regs;
use super::site::Site;
use super::source::Source;
use super::user::User;

// Import Dooh from AdCOM when openrtb_26 feature is enabled
use crate::Extension;
#[cfg(feature = "openrtb_26")]
use crate::adcom::context::Dooh;

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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{BidRequest, Imp, Banner};
///
/// let imp = Imp::builder()
///     .id("imp1".to_string())
///     .banner(Some(Banner::builder().w(Some(300)).h(Some(250)).build().unwrap()))
///     .build()
///     .unwrap();
///
/// let request = BidRequest::builder()
///     .id("request123".to_string())
///     .imp(vec![imp])
///     .at(2)  // Second price auction
///     .tmax(Some(120))
///     .build()
///     .unwrap();
/// ```
///
/// All objects in BidRequest are now fully typed as of Phase 2, Commit 7.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct BidRequest<Ext: Extension = serde_json::Value> {
    /// Unique ID of the bid request, provided by the exchange.
    /// **Required field**.
    #[builder(setter(into))]
    pub id: String,

    /// Array of Imp objects representing the impressions offered.
    /// **Required field** - must contain at least one impression.
    #[builder(setter(into))]
    pub imp: Vec<Imp<Ext>>,

    /// Details via a Site object about the publisher's website.
    /// Only applicable and recommended for websites.
    /// Exactly one of Site or App should be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub site: Option<Site<Ext>>,

    /// Details via an App object about the publisher's app.
    /// Only applicable and recommended for apps.
    /// Exactly one of Site, App, or Dooh should be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub app: Option<App<Ext>>,

    /// Details via a Dooh object about the digital out-of-home ad placement (OpenRTB 2.6+).
    /// Only applicable for DOOH inventory (billboards, transit displays, etc.).
    /// Exactly one of Site, App, or Dooh should be included.
    #[cfg(feature = "openrtb_26")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dooh: Option<Dooh<Ext>>,

    /// Details via a Device object about the user's device.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub device: Option<Device<Ext>>,

    /// Details via a User object about the human user of the device.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user: Option<User<Ext>>,

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
    ///
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
    /// represent all impressions available in context:
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
    pub source: Option<Source<Ext>>,

    /// A Regs object that specifies any industry, legal, or governmental
    /// regulations in force for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub regs: Option<Regs<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl BidRequest {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BidRequestBuilder {
        BidRequestBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_request_creation() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .test(0)
            .at(2)
            .build()
            .unwrap();

        assert_eq!(request.id, "req123");
        assert_eq!(request.imp.len(), 1);
        assert_eq!(request.imp[0].id, "imp1");
        assert_eq!(request.at, 2);
        assert_eq!(request.test, 0);
        assert_eq!(request.allimps, 0);
    }

    #[test]
    fn test_bid_request_serialization() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .build()
            .unwrap();

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
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let site = Site::builder()
            .id(Some("site123".to_string()))
            .domain(Some("example.com".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .site(Some(site))
            .build()
            .unwrap();

        assert!(request.site.is_some());
        assert_eq!(
            request.site.as_ref().unwrap().domain,
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_bid_request_with_app() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let app = App::builder()
            .id(Some("app123".to_string()))
            .bundle(Some("com.example.app".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .app(Some(app))
            .build()
            .unwrap();

        assert!(request.app.is_some());
        assert_eq!(
            request.app.as_ref().unwrap().bundle,
            Some("com.example.app".to_string())
        );
    }

    #[test]
    fn test_bid_request_with_device() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let device = Device::builder()
            .ua(Some("Mozilla/5.0".to_string()))
            .ip(Some("192.168.1.1".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .device(Some(device))
            .build()
            .unwrap();

        assert!(request.device.is_some());
        assert_eq!(
            request.device.as_ref().unwrap().ip,
            Some("192.168.1.1".to_string())
        );
    }

    #[test]
    fn test_bid_request_with_user() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let user = User::builder()
            .id(Some("user123".to_string()))
            .yob(Some(1990))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .user(Some(user))
            .build()
            .unwrap();

        assert!(request.user.is_some());
        assert_eq!(
            request.user.as_ref().unwrap().id,
            Some("user123".to_string())
        );
    }

    #[test]
    fn test_bid_request_test_mode() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .test(1) // Test mode
            .build()
            .unwrap();

        assert_eq!(request.test, 1);
    }

    #[test]
    fn test_bid_request_with_blocklists() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .bcat(Some(vec!["IAB25".to_string(), "IAB26".to_string()]))
            .badv(Some(vec!["competitor.com".to_string()]))
            .bapp(Some(vec!["com.competitor.app".to_string()]))
            .build()
            .unwrap();

        assert_eq!(request.bcat.as_ref().unwrap().len(), 2);
        assert_eq!(request.badv.as_ref().unwrap().len(), 1);
        assert_eq!(request.bapp.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_bid_request_with_tmax() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .tmax(Some(120)) // 120ms timeout
            .build()
            .unwrap();

        assert_eq!(request.tmax, Some(120));
    }

    #[test]
    fn test_bid_request_with_source() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let source = Source::builder()
            .fd(Some(1))
            .tid(Some("transaction123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .source(Some(source))
            .build()
            .unwrap();

        assert!(request.source.is_some());
        assert_eq!(request.source.as_ref().unwrap().fd, Some(1));
    }

    #[test]
    fn test_bid_request_with_regs() {
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();

        let regs = Regs::builder().coppa(Some(1)).build().unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .regs(Some(regs))
            .build()
            .unwrap();

        assert!(request.regs.is_some());
        assert_eq!(request.regs.as_ref().unwrap().coppa, Some(1));
    }

    // === Phase 1.2: Required Field Validation Tests ===

    #[test]
    fn test_missing_required_id_field() {
        // Test deserialization without required 'id' field
        let json = r#"{"imp":[{"id":"imp1"}]}"#;
        let result: Result<BidRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "BidRequest without required 'id' field should fail deserialization"
        );
    }

    #[test]
    fn test_empty_required_id_field() {
        // Test that empty string is currently allowed (documents current behavior)
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let result = BidRequest::builder()
            .id("".to_string()) // Empty string
            .imp(vec![imp])
            .build();

        // Currently no validation prevents empty strings
        assert!(result.is_ok(), "Empty id string currently passes");
        // TODO: Consider adding validation to reject empty required strings
    }

    #[test]
    fn test_missing_required_imp_field() {
        // Test deserialization without required 'imp' array
        let json = r#"{"id":"req123"}"#;
        let result: Result<BidRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "BidRequest without required 'imp' array should fail deserialization"
        );
    }

    #[test]
    fn test_empty_required_imp_array() {
        // Test that empty imp array is currently allowed
        // Per OpenRTB spec: "at least one impression" required
        let result = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![]) // Empty array - violates spec
            .build();

        // Currently no validation prevents empty imp array
        assert!(result.is_ok(), "Empty imp array currently passes");
        // TODO: Consider adding validation to enforce "at least one impression" requirement
    }

    #[test]
    fn test_null_id_field() {
        // Test explicit null for required field
        let json = r#"{"id":null,"imp":[{"id":"imp1"}]}"#;
        let result: Result<BidRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "BidRequest with null 'id' should fail deserialization"
        );
    }

    #[test]
    fn test_null_imp_field() {
        // Test explicit null for required field
        let json = r#"{"id":"req123","imp":null}"#;
        let result: Result<BidRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "BidRequest with null 'imp' should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_id_type() {
        // Test wrong type for id field
        let json = r#"{"id":123,"imp":[{"id":"imp1"}]}"#;
        let result: Result<BidRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "BidRequest with numeric 'id' instead of string should fail"
        );
    }

    #[test]
    fn test_invalid_imp_type() {
        // Test wrong type for imp field
        let json = r#"{"id":"req123","imp":"not_an_array"}"#;
        let result: Result<BidRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "BidRequest with string 'imp' instead of array should fail"
        );
    }

    #[test]
    fn test_minimal_valid_request() {
        // Test absolute minimum valid request
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .build()
            .unwrap();

        assert_eq!(request.id, "req123");
        assert_eq!(request.imp.len(), 1);
        assert_eq!(request.at, 2); // Default auction type
    }

    // === Phase 2.2: Mutually Exclusive Field Tests (site/app/dooh) ===

    #[test]
    fn test_bidrequest_with_no_distribution_channel() {
        // Per OpenRTB spec: BidRequest should have exactly ONE of site, app, or dooh
        // Test that request with NONE of these fields currently passes
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .build();

        assert!(
            request.is_ok(),
            "BidRequest with no distribution channel (site/app/dooh) currently passes"
        );

        let request = request.unwrap();
        assert!(request.site.is_none());
        assert!(request.app.is_none());
        // TODO: Per OpenRTB spec, exactly ONE of site, app, or dooh should be present
        // Consider adding validation to enforce this requirement
    }

    #[test]
    fn test_bidrequest_with_site_only() {
        // Valid: BidRequest with exactly ONE distribution channel (site)
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .site(Some(site))
            .build()
            .unwrap();

        assert!(request.site.is_some());
        assert!(request.app.is_none());
        #[cfg(feature = "openrtb_26")]
        assert!(request.dooh.is_none());
    }

    #[test]
    fn test_bidrequest_with_app_only() {
        // Valid: BidRequest with exactly ONE distribution channel (app)
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let app = App::builder()
            .id(Some("app123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .app(Some(app))
            .build()
            .unwrap();

        assert!(request.site.is_none());
        assert!(request.app.is_some());
        #[cfg(feature = "openrtb_26")]
        assert!(request.dooh.is_none());
    }

    #[test]
    fn test_bidrequest_with_both_site_and_app() {
        // Per OpenRTB spec: BidRequest must NOT contain both Site and App
        // Test that having BOTH currently passes
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .build()
            .unwrap();
        let app = App::builder()
            .id(Some("app123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .site(Some(site))
            .app(Some(app))
            .build();

        assert!(
            request.is_ok(),
            "BidRequest with both site and app currently passes"
        );

        let request = request.unwrap();
        assert!(request.site.is_some());
        assert!(request.app.is_some());
        // TODO: Per OpenRTB spec Section 3.2.13/3.2.14, a bid request must NOT
        // contain both Site and App objects. Should be rejected.
    }

    #[test]
    fn test_bidrequest_deserialization_with_multiple_distribution_channels() {
        // Test deserialization behavior when JSON contains multiple distribution channels
        let json = r#"{
            "id": "req123",
            "imp": [{"id": "imp1"}],
            "site": {"id": "site123"},
            "app": {"id": "app123"}
        }"#;

        let result: Result<BidRequest, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Deserialization with both site and app currently passes"
        );

        let request = result.unwrap();
        assert_eq!(
            request.site.as_ref().unwrap().id,
            Some("site123".to_string())
        );
        assert_eq!(request.app.as_ref().unwrap().id, Some("app123".to_string()));
        // TODO: Should deserialization validate mutual exclusivity?
    }

    // OpenRTB 2.6 specific tests (with dooh field)
    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_bidrequest_with_dooh_only() {
        use crate::adcom::context::Dooh;

        // Valid: BidRequest with exactly ONE distribution channel (dooh)
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let dooh = Dooh::builder()
            .id(Some("dooh123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .dooh(Some(dooh))
            .build()
            .unwrap();

        assert!(request.site.is_none());
        assert!(request.app.is_none());
        assert!(request.dooh.is_some());
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_bidrequest_with_site_and_dooh() {
        use crate::adcom::context::Dooh;

        // Per OpenRTB 2.6 spec: Exactly ONE of site, app, or dooh should be included
        // Test that having BOTH site and dooh currently passes
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .build()
            .unwrap();
        let dooh = Dooh::builder()
            .id(Some("dooh123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .site(Some(site))
            .dooh(Some(dooh))
            .build();

        assert!(
            request.is_ok(),
            "BidRequest with both site and dooh currently passes"
        );

        let request = request.unwrap();
        assert!(request.site.is_some());
        assert!(request.dooh.is_some());
        // TODO: Should be rejected - can only have ONE distribution channel
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_bidrequest_with_all_three_distribution_channels() {
        use crate::adcom::context::Dooh;

        // Test that having ALL THREE distribution channels currently passes
        let imp = Imp::builder().id("imp1".to_string()).build().unwrap();
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .build()
            .unwrap();
        let app = App::builder()
            .id(Some("app123".to_string()))
            .build()
            .unwrap();
        let dooh = Dooh::builder()
            .id(Some("dooh123".to_string()))
            .build()
            .unwrap();

        let request = BidRequest::builder()
            .id("req123".to_string())
            .imp(vec![imp])
            .site(Some(site))
            .app(Some(app))
            .dooh(Some(dooh))
            .build();

        assert!(
            request.is_ok(),
            "BidRequest with all three distribution channels currently passes"
        );

        let request = request.unwrap();
        assert!(request.site.is_some());
        assert!(request.app.is_some());
        assert!(request.dooh.is_some());
        // TODO: Per OpenRTB 2.6 spec, exactly ONE of site, app, or dooh should be present
        // Should be rejected when multiple are present
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_bidrequest_deserialization_with_all_distribution_channels() {
        // Test deserialization with all three distribution channels in JSON
        let json = r#"{
            "id": "req123",
            "imp": [{"id": "imp1"}],
            "site": {"id": "site123"},
            "app": {"id": "app123"},
            "dooh": {"id": "dooh123"}
        }"#;

        let result: Result<BidRequest, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Deserialization with all three channels currently passes"
        );

        let request = result.unwrap();
        assert_eq!(
            request.site.as_ref().unwrap().id,
            Some("site123".to_string())
        );
        assert_eq!(request.app.as_ref().unwrap().id, Some("app123".to_string()));
        assert_eq!(
            request.dooh.as_ref().unwrap().id,
            Some("dooh123".to_string())
        );
        // TODO: Should deserialization validate mutual exclusivity for site/app/dooh?
    }
}
