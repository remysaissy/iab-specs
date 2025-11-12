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
