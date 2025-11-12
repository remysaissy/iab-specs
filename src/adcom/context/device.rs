use crate::Extension;
use crate::adcom::context::{Geo, UserAgent};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Device Object (Section 7.4)
///
/// Details about the user's device.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Device<Ext: Extension = serde_json::Value> {
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
    pub ext: Option<Box<Ext>>,
}

impl Device {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::create_empty()
    }
}
