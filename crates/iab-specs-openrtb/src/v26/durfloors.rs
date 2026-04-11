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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example - Duration Tiered Pricing
/// ```
/// use iab_specs_openrtb::v26::DurFloors;
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
pub struct DurFloors<Ext: Extension = crate::DefaultExt> {
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
        // Spec: Section 3.2.35
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
        // Spec: Section 3.2.35
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

    #[test]
    fn test_durfloors_default_builder() {
        // Spec: Section 3.2.35
        let floor = DurFloors::builder().build().unwrap();
        assert!(floor.minduration.is_none());
        assert!(floor.maxduration.is_none());
        assert!(floor.bidfloor.is_none());
        assert!(floor.bidfloorcur.is_none());
        assert!(floor.ext.is_none());
        let json = serde_json::to_string(&floor).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_durfloors_ext_field() {
        // Spec: Section 3.2.35
        let ext = serde_json::json!({"custom_field": "value", "priority": 5});
        let floor = DurFloorsBuilder::<serde_json::Value>::default()
            .minduration(Some(15))
            .maxduration(Some(30))
            .ext(Some(Box::new(ext.clone())))
            .build()
            .unwrap();

        assert_eq!(*floor.ext.as_ref().unwrap().as_ref(), ext);

        let json = serde_json::to_string(&floor).unwrap();
        let deserialized: DurFloors<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(floor, deserialized);
    }

    #[test]
    fn test_durfloors_deserialization_from_json() {
        // Spec: Section 3.2.35
        let json = r#"{"minduration":15,"maxduration":30,"bidfloor":5.0,"bidfloorcur":"USD"}"#;
        let floor: DurFloors = serde_json::from_str(json).unwrap();
        assert_eq!(floor.minduration, Some(15));
        assert_eq!(floor.maxduration, Some(30));
        assert_eq!(floor.bidfloor, Some(5.0));
        assert_eq!(floor.bidfloorcur, Some("USD".to_string()));
    }

    #[test]
    fn test_durfloors_roundtrip_all_fields() {
        // Spec: Section 3.2.35
        let floor = DurFloors::builder()
            .minduration(Some(10))
            .maxduration(Some(60))
            .bidfloor(Some(7.5))
            .bidfloorcur(Some("EUR".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&floor).unwrap();
        let deserialized: DurFloors = serde_json::from_str(&json).unwrap();

        assert_eq!(floor.minduration, deserialized.minduration);
        assert_eq!(floor.maxduration, deserialized.maxduration);
        assert_eq!(floor.bidfloor, deserialized.bidfloor);
        assert_eq!(floor.bidfloorcur, deserialized.bidfloorcur);
        assert_eq!(floor, deserialized);
    }

    #[test]
    fn test_durfloors_partial_fields() {
        // Spec: Section 3.2.35
        let floor = DurFloors::builder().bidfloor(Some(3.0)).build().unwrap();

        assert!(floor.minduration.is_none());
        assert!(floor.maxduration.is_none());
        assert_eq!(floor.bidfloor, Some(3.0));
        assert!(floor.bidfloorcur.is_none());

        let json = serde_json::to_string(&floor).unwrap();
        assert!(json.contains("\"bidfloor\":3.0"));
        assert!(!json.contains("minduration"));
        assert!(!json.contains("maxduration"));
        assert!(!json.contains("bidfloorcur"));
    }
}
