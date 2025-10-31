/// OpenRTB 2.5 Geo Object
///
/// This module implements the Geo object for geographic location data.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Geo object representing geographic location (OpenRTB 2.5 Section 3.2.19)
///
/// A `Geo` object encapsulates various methods for specifying a geographic location.
/// When subordinate to a Device object, it indicates the location of the device.
/// When subordinate to a User object, it indicates the user's home base (not necessarily
/// their current location).
///
/// **Note**: The lat/lon fields should only be passed if they conform to the accuracy
/// depicted in the type attribute. For example, the centroid of a geographic region such
/// as postal code should not be passed.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Geo;
///
/// let geo = Geo {
///     lat: Some(37.7749),
///     lon: Some(-122.4194),
///     country: Some("USA".to_string()),
///     region: Some("CA".to_string()),
///     city: Some("San Francisco".to_string()),
///     zip: Some("94102".to_string()),
///     type_: Some(2), // IP address
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Geo {
    /// Latitude from -90.0 to +90.0, where negative is south.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lat: Option<f64>,

    /// Longitude from -180.0 to +180.0, where negative is west.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lon: Option<f64>,

    /// Source of location data.
    /// Recommended when passing lat/lon.
    /// Refer to AdCOM `LocationType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: Option<i32>,

    /// Estimated location accuracy in meters.
    /// Recommended when passing lat/lon.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub accuracy: Option<i32>,

    /// Number of seconds since this geolocation fix was established.
    /// Note that devices may cache location data across multiple fetches.
    /// Ideally, this value should be from the time the actual fix was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lastfix: Option<i32>,

    /// Service or provider used to determine geolocation from IP address if applicable.
    /// Refer to AdCOM `IPLocationService` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ipservice: Option<i32>,

    /// Country code using ISO-3166-1-alpha-3.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub country: Option<String>,

    /// Region code using ISO-3166-2; 2-letter state code if USA.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub region: Option<String>,

    /// Region of a country using FIPS 10-4 notation.
    /// While OpenRTB 2.5 supports this, note that NIST withdrew this standard in 2008.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub regionfips104: Option<String>,

    /// Google metro code; similar to but not exactly Nielsen DMAs.
    /// See Appendix A for a link to the codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metro: Option<String>,

    /// City using United Nations Code for Trade & Transport Locations.
    /// See Appendix A for a link to the codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub city: Option<String>,

    /// Zip or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zip: Option<String>,

    /// Local time as the number +/- of minutes from UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub utcoffset: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_creation() {
        let geo = Geo {
            lat: Some(37.7749),
            lon: Some(-122.4194),
            country: Some("USA".to_string()),
            region: Some("CA".to_string()),
            city: Some("San Francisco".to_string()),
            zip: Some("94102".to_string()),
            type_: Some(2),
            ..Default::default()
        };

        assert_eq!(geo.lat, Some(37.7749));
        assert_eq!(geo.lon, Some(-122.4194));
        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.region, Some("CA".to_string()));
        assert_eq!(geo.type_, Some(2));
    }

    #[test]
    fn test_geo_minimal() {
        let geo = Geo {
            country: Some("USA".to_string()),
            ..Default::default()
        };

        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.lat, None);
        assert_eq!(geo.lon, None);
    }

    #[test]
    fn test_geo_serialization() {
        let geo = Geo {
            lat: Some(37.7749),
            lon: Some(-122.4194),
            country: Some("USA".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&geo).unwrap();
        assert!(json.contains("\"lat\":37.7749"));
        assert!(json.contains("\"lon\":-122.4194"));
        assert!(json.contains("\"country\":\"USA\""));
    }

    #[test]
    fn test_geo_deserialization() {
        let json = r#"{"lat":37.7749,"lon":-122.4194,"country":"USA","type":2}"#;
        let geo: Geo = serde_json::from_str(json).unwrap();

        assert_eq!(geo.lat, Some(37.7749));
        assert_eq!(geo.lon, Some(-122.4194));
        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.type_, Some(2));
    }

    #[test]
    fn test_geo_with_accuracy() {
        let geo = Geo {
            lat: Some(37.7749),
            lon: Some(-122.4194),
            type_: Some(1), // GPS
            accuracy: Some(10),
            lastfix: Some(60),
            ..Default::default()
        };

        assert_eq!(geo.accuracy, Some(10));
        assert_eq!(geo.lastfix, Some(60));
    }
}
