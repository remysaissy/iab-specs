use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Geo Object (Section 7.6)
///
/// Geographic location information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Geo<Ext: Extension = serde_json::Value> {
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
    fn test_geo_builder() {
        let geo = Geo::builder()
            .lat(Some(37.7749))
            .lon(Some(-122.4194))
            .country(Some("USA".to_string()))
            .city(Some("San Francisco".to_string()))
            .build()
            .unwrap();

        assert_eq!(geo.lat, Some(37.7749));
        assert_eq!(geo.lon, Some(-122.4194));
        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.city, Some("San Francisco".to_string()));
    }

    #[test]
    fn test_geo_default() {
        let geo = Geo::builder().build().unwrap();

        assert!(geo.lat.is_none());
        assert!(geo.lon.is_none());
        assert!(geo.country.is_none());
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
        let json = r#"{"lat":37.7749,"lon":-122.4194,"country":"USA"}"#;
        let geo: Geo = serde_json::from_str(json).unwrap();

        assert_eq!(geo.lat, Some(37.7749));
        assert_eq!(geo.lon, Some(-122.4194));
        assert_eq!(geo.country, Some("USA".to_string()));
    }

    #[test]
    fn test_geo_with_all_fields() {
        let geo = Geo::builder()
            .type_(Some(1))
            .lat(Some(40.7128))
            .lon(Some(-74.0060))
            .accur(Some(100))
            .lastfix(Some(1609459200))
            .ipserv(Some(3))
            .country(Some("USA".to_string()))
            .region(Some("NY".to_string()))
            .metro(Some("501".to_string()))
            .city(Some("New York".to_string()))
            .zip(Some("10001".to_string()))
            .utcoffset(Some(-300))
            .build()
            .unwrap();

        assert_eq!(geo.type_, Some(1));
        assert_eq!(geo.lat, Some(40.7128));
        assert_eq!(geo.country, Some("USA".to_string()));
        assert_eq!(geo.zip, Some("10001".to_string()));
    }
}
