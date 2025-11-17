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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_builder() {
        let device = Device::builder()
            .type_(Some(4))
            .ua(Some(
                "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X)".to_string(),
            ))
            .ip(Some("192.168.1.1".to_string()))
            .make(Some("Apple".to_string()))
            .model(Some("iPhone 12".to_string()))
            .os(Some(3))
            .osv(Some("14.0".to_string()))
            .build()
            .unwrap();

        assert_eq!(device.type_, Some(4));
        assert_eq!(device.make, Some("Apple".to_string()));
        assert_eq!(device.model, Some("iPhone 12".to_string()));
        assert_eq!(device.os, Some(3));
        assert_eq!(device.osv, Some("14.0".to_string()));
    }

    #[test]
    fn test_device_default() {
        let device = Device::builder().build().unwrap();

        assert!(device.type_.is_none());
        assert!(device.ua.is_none());
        assert!(device.ip.is_none());
        assert!(device.geo.is_none());
        assert!(device.make.is_none());
    }

    #[test]
    fn test_device_with_geo() {
        let geo = Geo::builder()
            .country(Some("USA".to_string()))
            .city(Some("San Francisco".to_string()))
            .build()
            .unwrap();

        let device = Device::builder()
            .type_(Some(4))
            .geo(Some(Box::new(geo)))
            .build()
            .unwrap();

        assert!(device.geo.is_some());
        assert_eq!(
            device.geo.as_ref().unwrap().country,
            Some("USA".to_string())
        );
    }

    #[test]
    fn test_device_serialization() {
        let device = Device::builder()
            .type_(Some(1))
            .make(Some("Samsung".to_string()))
            .model(Some("Galaxy S21".to_string()))
            .w(Some(1080))
            .h(Some(2400))
            .build()
            .unwrap();

        let json = serde_json::to_string(&device).unwrap();
        assert!(json.contains("\"type_\":1"));
        assert!(json.contains("\"make\":\"Samsung\""));
        assert!(json.contains("\"model\":\"Galaxy S21\""));
        assert!(json.contains("\"w\":1080"));
        assert!(json.contains("\"h\":2400"));
    }

    #[test]
    fn test_device_deserialization() {
        let json = r#"{"type_":4,"make":"Apple","model":"iPad","os":3,"osv":"15.0"}"#;
        let device: Device = serde_json::from_str(json).unwrap();

        assert_eq!(device.type_, Some(4));
        assert_eq!(device.make, Some("Apple".to_string()));
        assert_eq!(device.model, Some("iPad".to_string()));
        assert_eq!(device.os, Some(3));
        assert_eq!(device.osv, Some("15.0".to_string()));
    }

    #[test]
    fn test_device_with_identifiers() {
        let device = Device::builder()
            .type_(Some(4))
            .ifa(Some("AEBE52E7-03EE-455A-B3C4-E57283966239".to_string()))
            .ift(Some("idfa".to_string()))
            .macsha1(Some("e14a6b6e3e62e4dbeb39e62e4d6e3e62".to_string()))
            .dnt(Some(0))
            .lmt(Some(0))
            .build()
            .unwrap();

        assert_eq!(
            device.ifa,
            Some("AEBE52E7-03EE-455A-B3C4-E57283966239".to_string())
        );
        assert_eq!(device.ift, Some("idfa".to_string()));
        assert!(device.macsha1.is_some());
        assert_eq!(device.dnt, Some(0));
        assert_eq!(device.lmt, Some(0));
    }

    #[test]
    fn test_device_with_screen_specs() {
        let device = Device::builder()
            .type_(Some(4))
            .w(Some(1170))
            .h(Some(2532))
            .ppi(Some(460))
            .pxratio(Some(3.0))
            .build()
            .unwrap();

        assert_eq!(device.w, Some(1170));
        assert_eq!(device.h, Some(2532));
        assert_eq!(device.ppi, Some(460));
        assert_eq!(device.pxratio, Some(3.0));
    }
}
