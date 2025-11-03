/// OpenRTB 2.5/2.6 Device Object
///
/// This module implements the Device object for device information.
/// OpenRTB 2.6 adds the sua (structured user-agent) field.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::geo::Geo;

// Import UserAgent from AdCOM when openrtb_26 feature is enabled
#[cfg(feature = "openrtb_26")]
use crate::adcom::UserAgent;

/// Device object representing user's device (OpenRTB 2.5 Section 3.2.18)
///
/// A `Device` object provides information pertaining to the device through which the
/// user is interacting. Device information includes its hardware, platform, location,
/// and carrier data. The device can refer to a mobile handset, a desktop computer,
/// set-top box, or other digital device.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{Device, Geo};
///
/// let device = Device {
///     ua: Some("Mozilla/5.0...".to_string()),
///     ip: Some("192.168.1.1".to_string()),
///     devicetype: Some(4), // Phone
///     make: Some("Apple".to_string()),
///     model: Some("iPhone".to_string()),
///     os: Some("iOS".to_string()),
///     osv: Some("14.0".to_string()),
///     geo: Some(Geo {
///         country: Some("USA".to_string()),
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Device {
    /// Browser user agent string.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ua: Option<String>,

    /// Structured user agent information (OpenRTB 2.6+).
    /// Provides parsed browser, OS, and device details from User-Agent Client Hints.
    /// Complements or replaces the ua string field.
    #[cfg(feature = "openrtb_26")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sua: Option<UserAgent>,

    /// Location of the device assumed to be the user's current location.
    /// Recommended if IP address is not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub geo: Option<Geo>,

    /// Standard "Do Not Track" flag as set in the header by the browser:
    /// - 0 = tracking is unrestricted
    /// - 1 = do not track
    ///
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dnt: Option<i32>,

    /// "Limit Ad Tracking" signal commercially endorsed (e.g., iOS, Android):
    /// - 0 = tracking is unrestricted
    /// - 1 = tracking must be limited per commercial guidelines
    ///
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lmt: Option<i32>,

    /// IPv4 address closest to device.
    /// Recommended if geo is not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ip: Option<String>,

    /// IP address closest to device as IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ipv6: Option<String>,

    /// The general type of device.
    /// Refer to AdCOM `DeviceType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub devicetype: Option<i32>,

    /// Device make (e.g., "Apple").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub make: Option<String>,

    /// Device model (e.g., "iPhone").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub model: Option<String>,

    /// Device operating system (e.g., "iOS").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub os: Option<String>,

    /// Device operating system version (e.g., "3.1.2").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub osv: Option<String>,

    /// Hardware version of the device (e.g., "5S" for iPhone 5S).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hwv: Option<String>,

    /// Physical height of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Physical width of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Screen size as pixels per linear inch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ppi: Option<i32>,

    /// The ratio of physical pixels to device independent pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pxratio: Option<f64>,

    /// Support for JavaScript:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub js: Option<i32>,

    /// Indicates if the geolocation API will be available to JavaScript code:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub geofetch: Option<i32>,

    /// Version of Flash supported by the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flashver: Option<String>,

    /// Browser language using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub language: Option<String>,

    /// Browser language using IETF BCP 47.
    /// OpenRTB 2.6+ field for more detailed language specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub langb: Option<String>,

    /// Carrier or ISP (e.g., "VERIZON") using Mobile Country Code (MCC) and
    /// Mobile Network Code (MNC), using the format: `<MCC>-<MNC>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub carrier: Option<String>,

    /// Mobile carrier as the concatenated MCC-MNC code (e.g., "310-005").
    /// Identifies wireless carrier and device using the format: `<MCC>-<MNC>-<MNO>`.
    /// Prefer over carrier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mccmnc: Option<String>,

    /// Network connection type.
    /// Refer to AdCOM `ConnectionType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub connectiontype: Option<i32>,

    /// ID sanctioned for advertiser use in the clear (i.e., not hashed).
    /// - iOS: IDFA (Identifier for Advertising)
    /// - Android: Google Advertising ID
    /// - Windows: Microsoft Advertising ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ifa: Option<String>,

    /// Hardware device ID (e.g., IMEI); hashed via SHA1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub didsha1: Option<String>,

    /// Hardware device ID (e.g., IMEI); hashed via MD5.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub didmd5: Option<String>,

    /// Platform device ID (e.g., Android ID); hashed via SHA1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dpidsha1: Option<String>,

    /// Platform device ID (e.g., Android ID); hashed via MD5.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dpidmd5: Option<String>,

    /// MAC address of the device; hashed via SHA1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub macsha1: Option<String>,

    /// MAC address of the device; hashed via MD5.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub macmd5: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        let device = Device {
            ua: Some("Mozilla/5.0".to_string()),
            ip: Some("192.168.1.1".to_string()),
            devicetype: Some(4),
            make: Some("Apple".to_string()),
            model: Some("iPhone".to_string()),
            os: Some("iOS".to_string()),
            osv: Some("14.0".to_string()),
            ..Default::default()
        };

        assert_eq!(device.ua, Some("Mozilla/5.0".to_string()));
        assert_eq!(device.ip, Some("192.168.1.1".to_string()));
        assert_eq!(device.devicetype, Some(4));
        assert_eq!(device.make, Some("Apple".to_string()));
        assert_eq!(device.os, Some("iOS".to_string()));
    }

    #[test]
    fn test_device_with_geo() {
        let geo = Geo {
            country: Some("USA".to_string()),
            region: Some("CA".to_string()),
            ..Default::default()
        };

        let device = Device {
            ip: Some("192.168.1.1".to_string()),
            geo: Some(geo),
            ..Default::default()
        };

        assert!(device.geo.is_some());
        assert_eq!(
            device.geo.as_ref().unwrap().country,
            Some("USA".to_string())
        );
    }

    #[test]
    fn test_device_tracking_flags() {
        let device = Device {
            dnt: Some(1),
            lmt: Some(1),
            ..Default::default()
        };

        assert_eq!(device.dnt, Some(1));
        assert_eq!(device.lmt, Some(1));
    }

    #[test]
    fn test_device_serialization() {
        let device = Device {
            ua: Some("Mozilla/5.0".to_string()),
            ip: Some("192.168.1.1".to_string()),
            devicetype: Some(4),
            ..Default::default()
        };

        let json = serde_json::to_string(&device).unwrap();
        assert!(json.contains("\"ua\":\"Mozilla/5.0\""));
        assert!(json.contains("\"ip\":\"192.168.1.1\""));
        assert!(json.contains("\"devicetype\":4"));
    }

    #[test]
    fn test_device_deserialization() {
        let json = r#"{"ua":"Mozilla/5.0","ip":"192.168.1.1","devicetype":4}"#;
        let device: Device = serde_json::from_str(json).unwrap();

        assert_eq!(device.ua, Some("Mozilla/5.0".to_string()));
        assert_eq!(device.ip, Some("192.168.1.1".to_string()));
        assert_eq!(device.devicetype, Some(4));
    }

    #[test]
    fn test_device_with_ifa() {
        let device = Device {
            ifa: Some("AEBE52E7-03EE-455A-B3C4-E57283966239".to_string()),
            lmt: Some(0),
            ..Default::default()
        };

        assert_eq!(
            device.ifa,
            Some("AEBE52E7-03EE-455A-B3C4-E57283966239".to_string())
        );
        assert_eq!(device.lmt, Some(0));
    }
}
