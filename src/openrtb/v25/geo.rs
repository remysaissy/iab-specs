use crate::Extension;
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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Geo;
///
/// let geo = Geo::builder()
///     .lat(Some(37.7749))
///     .lon(Some(-122.4194))
///     .country(Some("USA".to_string()))
///     .region(Some("CA".to_string()))
///     .city(Some("San Francisco".to_string()))
///     .zip(Some("94102".to_string()))
///     .type_(Some(2)) // IP address
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Geo<Ext: Extension = crate::DefaultExt> {
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
    pub ext: Option<Box<Ext>>,
}

impl Geo {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> GeoBuilder {
        GeoBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_creation() {
        let geo = Geo::builder()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .country(Some("USA".to_string()))
            .region(Some("CA".to_string()))
            .city(Some("San Francisco".to_string()))
            .zip(Some("94102".to_string()))
            .type_(Some(2))
            .build()
            .unwrap();

        assert_eq!(geo.lat, Some(37.7749));
        assert_eq!(geo.lon, Some(-122.4194));
        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.region, Some("CA".to_string()));
        assert_eq!(geo.type_, Some(2));
    }

    #[test]
    fn test_geo_minimal() {
        let geo = Geo::builder()
            .country(Some("USA".to_string()))
            .build()
            .unwrap();

        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.lat, None);
        assert_eq!(geo.lon, None);
    }

    #[test]
    fn test_geo_serialization() {
        let geo = Geo::builder()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .country(Some("USA".to_string()))
            .build()
            .unwrap();

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
        let geo = Geo::builder()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .type_(Some(1)) // GPS
            .accuracy(Some(10))
            .lastfix(Some(60))
            .build()
            .unwrap();

        assert_eq!(geo.accuracy, Some(10));
        assert_eq!(geo.lastfix, Some(60));
    }

    #[test]
    fn test_geo_type_field() {
        // Spec: Section 3.2.19
        let gps = Geo::builder().type_(Some(1)).build().unwrap();
        assert_eq!(gps.type_, Some(1));

        let ip = Geo::builder().type_(Some(2)).build().unwrap();
        assert_eq!(ip.type_, Some(2));

        let user_provided = Geo::builder().type_(Some(3)).build().unwrap();
        assert_eq!(user_provided.type_, Some(3));
    }

    #[test]
    fn test_geo_region_fields() {
        // Spec: Section 3.2.19
        let geo = Geo::builder()
            .region(Some("CA".to_string()))
            .regionfips104(Some("US06".to_string()))
            .metro(Some("807".to_string()))
            .zip(Some("94102".to_string()))
            .build()
            .unwrap();

        assert_eq!(geo.region, Some("CA".to_string()));
        assert_eq!(geo.regionfips104, Some("US06".to_string()));
        assert_eq!(geo.metro, Some("807".to_string()));
        assert_eq!(geo.zip, Some("94102".to_string()));
    }

    #[test]
    fn test_geo_utcoffset_field() {
        // Spec: Section 3.2.19
        let positive = Geo::builder().utcoffset(Some(60)).build().unwrap();
        assert_eq!(positive.utcoffset, Some(60));

        let negative = Geo::builder().utcoffset(Some(-480)).build().unwrap();
        assert_eq!(negative.utcoffset, Some(-480));

        let zero = Geo::builder().utcoffset(Some(0)).build().unwrap();
        assert_eq!(zero.utcoffset, Some(0));
    }

    #[test]
    fn test_geo_ext_field() {
        // Spec: Section 3.2.19
        let geo = GeoBuilder::<serde_json::Value>::default()
            .country(Some("USA".to_string()))
            .ext(Some(Box::new(serde_json::json!({
                "dma": "San Francisco"
            }))))
            .build()
            .unwrap();

        assert!(geo.ext.is_some());
        assert_eq!(geo.ext.as_ref().unwrap()["dma"], "San Francisco");
    }

    #[test]
    fn test_geo_roundtrip_all_fields() {
        // Spec: Section 3.2.19
        let geo = Geo::builder()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .type_(Some(1))
            .accuracy(Some(50))
            .lastfix(Some(30))
            .ipservice(Some(3))
            .country(Some("USA".to_string()))
            .region(Some("CA".to_string()))
            .regionfips104(Some("US06".to_string()))
            .metro(Some("807".to_string()))
            .city(Some("San Francisco".to_string()))
            .zip(Some("94102".to_string()))
            .utcoffset(Some(-480))
            .build()
            .unwrap();

        let json = serde_json::to_string(&geo).unwrap();
        let deserialized: Geo = serde_json::from_str(&json).unwrap();

        assert_eq!(geo.lat, deserialized.lat);
        assert_eq!(geo.lon, deserialized.lon);
        assert_eq!(geo.type_, deserialized.type_);
        assert_eq!(geo.accuracy, deserialized.accuracy);
        assert_eq!(geo.lastfix, deserialized.lastfix);
        assert_eq!(geo.ipservice, deserialized.ipservice);
        assert_eq!(geo.country, deserialized.country);
        assert_eq!(geo.region, deserialized.region);
        assert_eq!(geo.regionfips104, deserialized.regionfips104);
        assert_eq!(geo.metro, deserialized.metro);
        assert_eq!(geo.city, deserialized.city);
        assert_eq!(geo.zip, deserialized.zip);
        assert_eq!(geo.utcoffset, deserialized.utcoffset);
    }
}
