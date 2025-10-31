//! AdCOM Context Objects
//!
//! Context objects represent the environment in which ads will be displayed,
//! including information about users, devices, locations, distribution channels,
//! publishers, content, and regulatory constraints.
//!
//! Reference: AdCOM v1.0 Section 5 - Context Objects

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Distribution Channel abstract base
///
/// Abstract base for distribution channel types (Site, App, DOOH).
/// This is the parent object for describing the properties of the medium
/// through which advertising is being offered.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DistributionChannel {
    /// Vendor-specific unique identifier of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Publisher Object
///
/// The publisher of the media in which ads will be displayed.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Publisher {
    /// Vendor-specific unique publisher identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Highest level domain of the publisher (e.g., "publisher.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories describing the publisher using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Content Object
///
/// Details about the content within which an ad will be displayed.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Content {
    /// Unique content identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Episode number for episodic content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,

    /// Content title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Content series
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// Content season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    /// Artist credited with the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// Genre(s) of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// Album to which the content belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// International Standard Recording Code (ISRC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    /// URL of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Content categories using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Production quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<i32>,

    /// Content context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,

    /// Content rating (e.g., MPAA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<String>,

    /// User rating of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userrating: Option<String>,

    /// Media rating per IQG guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// 1 = content is live, 0 = not live
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestream: Option<i32>,

    /// 1 = src relationship is direct, 0 = indirect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srcrel: Option<i32>,

    /// Length of content in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Content language using ISO-639-1-alpha-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// 1 = content is embedded, 0 = not embedded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<i32>,

    /// Producer details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Box<Producer>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Producer Object
///
/// The producer of the content in which ads will be displayed.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Producer {
    /// Vendor-specific unique producer identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Highest level domain of the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories describing the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Site Object (Section 7.1)
///
/// Distribution channel for website-based advertising.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Site {
    /// Vendor-specific unique site identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Site name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Domain of the site (e.g., "example.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories using taxonomy IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Array of section categories using taxonomy IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of page categories using taxonomy IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// URL of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// Search string that caused navigation to the current page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Indicates if site is mobile optimized (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Privacy policy flag (1=has policy, 0=no policy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// App Object (Section 7.2)
///
/// Distribution channel for mobile/tablet application advertising.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct App {
    /// Vendor-specific unique app identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// App name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// App domain (e.g., "example.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Category taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Section categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Page categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// App version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// App store bundle/package name (e.g., "com.example.app")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// Privacy policy flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// Paid app flag (0=free, 1=paid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// App store URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Dooh Object (Section 7.3)
///
/// Distribution channel for Digital Out-of-Home (DOOH) advertising.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Dooh {
    /// Vendor-specific unique DOOH identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// DOOH venue name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the DOOH venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Publisher domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Venue type categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Venue type taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Array of venue type IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetype: Option<Vec<String>>,

    /// Venue type taxonomy used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetypetax: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// BrandVersion helper struct for UserAgent
///
/// Brand and version information for user agent client hints.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct BrandVersion {
    /// Brand name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Version numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,
}

/// UserAgent Object (Section 7.5)
///
/// Structured user agent information per User-Agent Client Hints.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct UserAgent {
    /// Browser marketing name array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<Vec<BrandVersion>>,

    /// Platform/OS name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<BrandVersion>>,

    /// Mobile device flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Platform architecture (e.g., "x86", "arm")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Platform bitness (e.g., "64")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,

    /// Device model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Source of user agent data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Geo Object (Section 7.6)
///
/// Geographic location information.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Geo {
    /// Location type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Latitude (-90 to 90, negative is south)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    /// Longitude (-180 to 180, negative is west)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    /// Accuracy in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accur: Option<i32>,

    /// Timestamp of location fix (Unix time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastfix: Option<i64>,

    /// Service used to determine location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipserv: Option<i32>,

    /// Country using ISO-3166-1-alpha-3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Region using ISO-3166-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Metropolitan region code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metro: Option<String>,

    /// City name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// ZIP/postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// UTC offset in minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utcoffset: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Device Object (Section 7.4)
///
/// Details about the user's device.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Device {
    /// Device type (mobile, tablet, desktop, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// User agent string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<String>,

    /// Structured user agent information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sua: Option<Box<UserAgent>>,

    /// IPv4 address in dotted-quad notation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// IPv6 address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// Geographic location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Box<Geo>>,

    /// Do Not Track flag (0=tracking unrestricted, 1=tracking restricted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnt: Option<i32>,

    /// Limit Ad Tracking flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i32>,

    /// Device make (e.g., "Apple")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,

    /// Device model (e.g., "iPhone")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Device operating system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<i32>,

    /// OS version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,

    /// Hardware version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,

    /// Physical width of screen in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Physical height of screen in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Pixels per linear inch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,

    /// Physical pixel ratio (e.g., 2.0 for Retina)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,

    /// JavaScript support (0=no, 1=yes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<i32>,

    /// Browser language using ISO-639-1-alpha-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Carrier or ISP using Mobile Country Code and Mobile Network Code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Mobile carrier as derived from IP address (MCC-MNC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,

    /// Network connection type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contype: Option<i32>,

    /// Device geofencing capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geofetch: Option<i32>,

    /// ID for advertisers (iOS IDFA, Android AdID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifa: Option<String>,

    /// Type of device ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ift: Option<String>,

    /// MAC address (hashed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macsha1: Option<String>,

    /// MAC address (MD5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Segment Object (Section 7.9)
///
/// Specific data segment about a user.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Segment {
    /// Segment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Segment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Segment value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Data Object (Section 7.8)
///
/// First-party data segment with user information.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Data {
    /// Vendor-specific data provider identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Data provider name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Array of data segments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<Segment>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// User Object (Section 7.7)
///
/// Information about the human user.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct User {
    /// Vendor-specific user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Buyer-specific user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<String>,

    /// Year of birth (4-digit integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<i32>,

    /// Gender (M=male, F=female, O=other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Consent string per GDPR/USP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<String>,

    /// Geographic location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Box<Geo>>,

    /// Additional user data segments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Regs Object (Section 7.10)
///
/// Regulatory conditions in effect.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Regs {
    /// COPPA compliance flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coppa: Option<i32>,

    /// GDPR applicability (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Network Object (Section 7.11)
///
/// Details about the distribution network.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Network {
    /// Network identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Network domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

/// Channel Object (Section 7.12)
///
/// Details about the distribution channel.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Channel {
    /// Channel identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Channel domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_builder() {
        let publisher = PublisherBuilder::default()
            .id(Some("pub123".to_string()))
            .name(Some("Test Publisher".to_string()))
            .domain(Some("testpub.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(publisher.id, Some("pub123".to_string()));
        assert_eq!(publisher.name, Some("Test Publisher".to_string()));
        assert_eq!(publisher.domain, Some("testpub.com".to_string()));
    }

    #[test]
    fn test_publisher_serialization() {
        let publisher = PublisherBuilder::default()
            .id(Some("pub123".to_string()))
            .name(Some("Test Publisher".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&publisher).unwrap();
        assert!(json.contains("pub123"));
        assert!(json.contains("Test Publisher"));

        let deserialized: Publisher = serde_json::from_str(&json).unwrap();
        assert_eq!(publisher, deserialized);
    }

    #[test]
    fn test_content_builder() {
        let content = ContentBuilder::default()
            .id(Some("content123".to_string()))
            .title(Some("Test Content".to_string()))
            .series(Some("Test Series".to_string()))
            .livestream(Some(1))
            .len(Some(3600))
            .build()
            .unwrap();

        assert_eq!(content.id, Some("content123".to_string()));
        assert_eq!(content.title, Some("Test Content".to_string()));
        assert_eq!(content.livestream, Some(1));
        assert_eq!(content.len, Some(3600));
    }

    #[test]
    fn test_producer_builder() {
        let producer = ProducerBuilder::default()
            .id(Some("prod123".to_string()))
            .name(Some("Test Producer".to_string()))
            .domain(Some("producer.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(producer.id, Some("prod123".to_string()));
        assert_eq!(producer.name, Some("Test Producer".to_string()));
    }

    #[test]
    fn test_site_builder() {
        let publisher = PublisherBuilder::default()
            .id(Some("pub123".to_string()))
            .build()
            .unwrap();

        let site = SiteBuilder::default()
            .id(Some("site123".to_string()))
            .name(Some("Test Site".to_string()))
            .domain(Some("example.com".to_string()))
            .pub_(Some(Box::new(publisher)))
            .mobile(Some(1))
            .privacypolicy(Some(1))
            .build()
            .unwrap();

        assert_eq!(site.id, Some("site123".to_string()));
        assert_eq!(site.domain, Some("example.com".to_string()));
        assert_eq!(site.mobile, Some(1));
    }

    #[test]
    fn test_site_serialization() {
        let site = SiteBuilder::default()
            .id(Some("site123".to_string()))
            .domain(Some("example.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&site).unwrap();
        let deserialized: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(site, deserialized);
    }

    #[test]
    fn test_app_builder() {
        let app = AppBuilder::default()
            .id(Some("app123".to_string()))
            .name(Some("Test App".to_string()))
            .bundle(Some("com.example.app".to_string()))
            .storeurl(Some("https://apps.apple.com/app/123".to_string()))
            .paid(Some(0))
            .privacypolicy(Some(1))
            .build()
            .unwrap();

        assert_eq!(app.id, Some("app123".to_string()));
        assert_eq!(app.bundle, Some("com.example.app".to_string()));
        assert_eq!(app.paid, Some(0));
    }

    #[test]
    fn test_app_serialization() {
        let app = AppBuilder::default()
            .id(Some("app123".to_string()))
            .bundle(Some("com.example.app".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&app).unwrap();
        assert!(json.contains("app123"));
        assert!(json.contains("com.example.app"));

        let deserialized: App = serde_json::from_str(&json).unwrap();
        assert_eq!(app, deserialized);
    }

    #[test]
    fn test_dooh_builder() {
        let dooh = DoohBuilder::default()
            .id(Some("dooh123".to_string()))
            .name(Some("Billboard Station".to_string()))
            .domain(Some("dooh.com".to_string()))
            .venuetype(Some(vec!["transit".to_string(), "outdoor".to_string()]))
            .build()
            .unwrap();

        assert_eq!(dooh.id, Some("dooh123".to_string()));
        assert_eq!(dooh.name, Some("Billboard Station".to_string()));
        assert!(dooh.venuetype.is_some());
    }

    #[test]
    fn test_brand_version_builder() {
        let brand = BrandVersionBuilder::default()
            .brand(Some("Chrome".to_string()))
            .version(Some(vec!["120".to_string(), "0".to_string()]))
            .build()
            .unwrap();

        assert_eq!(brand.brand, Some("Chrome".to_string()));
        assert!(brand.version.is_some());
    }

    #[test]
    fn test_user_agent_builder() {
        let brand1 = BrandVersionBuilder::default()
            .brand(Some("Chrome".to_string()))
            .version(Some(vec!["120".to_string()]))
            .build()
            .unwrap();

        let platform = BrandVersionBuilder::default()
            .brand(Some("macOS".to_string()))
            .version(Some(vec!["14".to_string()]))
            .build()
            .unwrap();

        let ua = UserAgentBuilder::default()
            .browsers(Some(vec![brand1]))
            .platform(Some(Box::new(platform)))
            .mobile(Some(0))
            .architecture(Some("x86".to_string()))
            .bitness(Some("64".to_string()))
            .build()
            .unwrap();

        assert_eq!(ua.mobile, Some(0));
        assert_eq!(ua.architecture, Some("x86".to_string()));
        assert!(ua.browsers.is_some());
    }

    #[test]
    fn test_geo_builder() {
        let geo = GeoBuilder::default()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .country(Some("USA".to_string()))
            .region(Some("CA".to_string()))
            .city(Some("San Francisco".to_string()))
            .zip(Some("94102".to_string()))
            .utcoffset(Some(-480))
            .build()
            .unwrap();

        assert_eq!(geo.lat, Some(37.7749));
        assert_eq!(geo.lon, Some(-122.4194));
        assert_eq!(geo.city, Some("San Francisco".to_string()));
    }

    #[test]
    fn test_geo_serialization() {
        let geo = GeoBuilder::default()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .country(Some("USA".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&geo).unwrap();
        let deserialized: Geo = serde_json::from_str(&json).unwrap();
        assert_eq!(geo, deserialized);
    }

    #[test]
    fn test_device_builder() {
        let geo = GeoBuilder::default()
            .country(Some("USA".to_string()))
            .build()
            .unwrap();

        let device = DeviceBuilder::default()
            .type_(Some(1))
            .ua(Some("Mozilla/5.0...".to_string()))
            .ip(Some("192.168.1.1".to_string()))
            .geo(Some(Box::new(geo)))
            .make(Some("Apple".to_string()))
            .model(Some("iPhone".to_string()))
            .os(Some(13))
            .w(Some(390))
            .h(Some(844))
            .js(Some(1))
            .build()
            .unwrap();

        assert_eq!(device.make, Some("Apple".to_string()));
        assert_eq!(device.model, Some("iPhone".to_string()));
        assert_eq!(device.w, Some(390));
        assert_eq!(device.h, Some(844));
    }

    #[test]
    fn test_device_serialization() {
        let device = DeviceBuilder::default()
            .type_(Some(1))
            .make(Some("Apple".to_string()))
            .model(Some("iPhone".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&device).unwrap();
        assert!(json.contains("Apple"));
        assert!(json.contains("iPhone"));

        let deserialized: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(device, deserialized);
    }

    #[test]
    fn test_segment_builder() {
        let segment = SegmentBuilder::default()
            .id(Some("seg123".to_string()))
            .name(Some("Tech Enthusiasts".to_string()))
            .value(Some("high".to_string()))
            .build()
            .unwrap();

        assert_eq!(segment.id, Some("seg123".to_string()));
        assert_eq!(segment.name, Some("Tech Enthusiasts".to_string()));
    }

    #[test]
    fn test_data_builder() {
        let segment = SegmentBuilder::default()
            .id(Some("seg1".to_string()))
            .build()
            .unwrap();

        let data = DataBuilder::default()
            .id(Some("data123".to_string()))
            .name(Some("First Party Data".to_string()))
            .segment(Some(vec![segment]))
            .build()
            .unwrap();

        assert_eq!(data.id, Some("data123".to_string()));
        assert!(data.segment.is_some());
    }

    #[test]
    fn test_user_builder() {
        let geo = GeoBuilder::default()
            .country(Some("USA".to_string()))
            .build()
            .unwrap();

        let user = UserBuilder::default()
            .id(Some("user123".to_string()))
            .buyeruid(Some("buyer456".to_string()))
            .yob(Some(1990))
            .gender(Some("M".to_string()))
            .geo(Some(Box::new(geo)))
            .build()
            .unwrap();

        assert_eq!(user.id, Some("user123".to_string()));
        assert_eq!(user.yob, Some(1990));
        assert_eq!(user.gender, Some("M".to_string()));
    }

    #[test]
    fn test_user_serialization() {
        let user = UserBuilder::default()
            .id(Some("user123".to_string()))
            .yob(Some(1990))
            .build()
            .unwrap();

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }

    #[test]
    fn test_regs_builder() {
        let regs = RegsBuilder::default()
            .coppa(Some(1))
            .gdpr(Some(1))
            .build()
            .unwrap();

        assert_eq!(regs.coppa, Some(1));
        assert_eq!(regs.gdpr, Some(1));
    }

    #[test]
    fn test_network_builder() {
        let network = NetworkBuilder::default()
            .id(Some("net123".to_string()))
            .name(Some("Test Network".to_string()))
            .domain(Some("network.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(network.id, Some("net123".to_string()));
        assert_eq!(network.name, Some("Test Network".to_string()));
    }

    #[test]
    fn test_channel_builder() {
        let channel = ChannelBuilder::default()
            .id(Some("ch123".to_string()))
            .name(Some("Test Channel".to_string()))
            .domain(Some("channel.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(channel.id, Some("ch123".to_string()));
        assert_eq!(channel.name, Some("Test Channel".to_string()));
    }

    #[test]
    fn test_nested_objects() {
        let producer = ProducerBuilder::default()
            .id(Some("prod123".to_string()))
            .build()
            .unwrap();

        let content = ContentBuilder::default()
            .id(Some("content123".to_string()))
            .producer(Some(Box::new(producer)))
            .build()
            .unwrap();

        let publisher = PublisherBuilder::default()
            .id(Some("pub123".to_string()))
            .build()
            .unwrap();

        let site = SiteBuilder::default()
            .id(Some("site123".to_string()))
            .pub_(Some(Box::new(publisher)))
            .content(Some(Box::new(content)))
            .build()
            .unwrap();

        assert!(site.pub_.is_some());
        assert!(site.content.is_some());
        assert!(site.content.as_ref().unwrap().producer.is_some());
    }

    #[test]
    fn test_skip_serializing_none() {
        let publisher = PublisherBuilder::default().build().unwrap();
        let json = serde_json::to_string(&publisher).unwrap();

        // Should be empty object since all fields are None
        assert_eq!(json, "{}");
    }
}
