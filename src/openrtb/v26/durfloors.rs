use crate::Extension;
/// OpenRTB 2.6 DurFloors Object
///
/// This module implements the DurFloors object for duration-based floor pricing.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DurFloors Object (Section 3.2.35)
///
/// Floor pricing by duration for video or audio creatives. Enables sellers to specify
/// different minimum bid prices based on creative duration ranges.
///
/// This is particularly useful for:
/// - Ad pods with variable duration slots
/// - Premium pricing for longer creative durations
/// - CTV inventory with duration-based yield optimization
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example - Duration Tiered Pricing
/// ```
/// use iab_specs::openrtb::v26::DurFloors;
///
/// // $5 CPM floor for 15-30 second creatives
/// let floor = DurFloors::builder()
///     .minduration(Some(15))
///     .maxduration(Some(30))
///     .bidfloor(Some(5.0))
///     .bidfloorcur(Some("USD".to_string()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DurFloors<Ext: Extension = serde_json::Value> {
    /// Minimum creative duration in seconds (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,

    /// Maximum creative duration in seconds (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,

    /// Bid floor for this duration range (CPM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,

    /// Currency of bid floor (ISO-4217 code)
    /// Default: "USD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloorcur: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DurFloors {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DurFloorsBuilder {
        DurFloorsBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_durfloors_serialization() {
        let floor = DurFloors::builder()
            .minduration(Some(10))
            .maxduration(Some(20))
            .bidfloor(Some(3.5))
            .bidfloorcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&floor).unwrap();
        assert!(json.contains("\"minduration\":10"));
        assert!(json.contains("\"bidfloor\":3.5"));

        let deserialized: DurFloors = serde_json::from_str(&json).unwrap();
        assert_eq!(floor, deserialized);
    }

    #[test]
    fn test_durfloors_tiered_pricing() {
        // Test multiple duration tiers
        let short_floor = DurFloors::builder()
            .minduration(Some(5))
            .maxduration(Some(15))
            .bidfloor(Some(2.0))
            .bidfloorcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let medium_floor = DurFloors::builder()
            .minduration(Some(16))
            .maxduration(Some(30))
            .bidfloor(Some(5.0))
            .bidfloorcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let long_floor = DurFloors::builder()
            .minduration(Some(31))
            .maxduration(Some(60))
            .bidfloor(Some(10.0))
            .bidfloorcur(Some("USD".to_string()))
            .build()
            .unwrap();

        assert!(short_floor.bidfloor.unwrap() < medium_floor.bidfloor.unwrap());
        assert!(medium_floor.bidfloor.unwrap() < long_floor.bidfloor.unwrap());
    }
}
