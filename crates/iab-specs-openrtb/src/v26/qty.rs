use crate::Extension;
/// OpenRTB 2.6 Qty Object
///
/// This module implements the Qty object for DOOH (Digital Out-Of-Home) multipliers.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Qty Object (Section 3.2.31)
///
/// Represents the quantity or multiplier indicating the total number of impressions
/// that may be displayed to multiple persons. This is particularly relevant for
/// DOOH (Digital Out-Of-Home) inventory where a single ad display serves multiple viewers.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example - DOOH Multiplier
/// ```
/// use iab_specs_openrtb::v26::Qty;
///
/// let qty = Qty::builder()
///     .multiplier(Some(150.0))  // 150 people viewing the ad
///     .source(Some("MeasurementVendor".to_string()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Qty<Ext: Extension = crate::DefaultExt> {
    /// Quantity multiplier (e.g., number of people viewing)
    /// For DOOH: estimated number of impressions delivered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,

    /// Source of the multiplier measurement
    /// Refer to List: Measurement Source in AdCOM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Qty {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> QtyBuilder {
        QtyBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qty_serialization() {
        // Spec: Section 3.2.31
        let qty = Qty::builder()
            .multiplier(Some(100.0))
            .source(Some("Publisher".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&qty).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("Publisher"));

        let deserialized: Qty = serde_json::from_str(&json).unwrap();
        assert_eq!(qty, deserialized);
    }

    #[test]
    fn test_skip_serializing_none() {
        // Spec: Section 3.2.31
        let qty = Qty::builder().build().unwrap();
        let json = serde_json::to_string(&qty).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_qty_ext_field() {
        // Spec: Section 3.2.31
        let ext = serde_json::json!({"custom_field": "value", "priority": 5});
        let qty = QtyBuilder::<serde_json::Value>::default()
            .multiplier(Some(150.0))
            .ext(Some(Box::new(ext.clone())))
            .build()
            .unwrap();

        assert_eq!(*qty.ext.as_ref().unwrap().as_ref(), ext);

        let json = serde_json::to_string(&qty).unwrap();
        let deserialized: Qty<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(qty, deserialized);
    }

    #[test]
    fn test_qty_deserialization_from_json() {
        // Spec: Section 3.2.31
        let json = r#"{"multiplier":150.0,"source":"MeasurementVendor"}"#;
        let qty: Qty = serde_json::from_str(json).unwrap();
        assert_eq!(qty.multiplier, Some(150.0));
        assert_eq!(qty.source, Some("MeasurementVendor".to_string()));
    }

    #[test]
    fn test_qty_roundtrip_all_fields() {
        // Spec: Section 3.2.31
        let qty = Qty::builder()
            .multiplier(Some(250.0))
            .source(Some("venue_measurement".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&qty).unwrap();
        let deserialized: Qty = serde_json::from_str(&json).unwrap();

        assert_eq!(qty.multiplier, deserialized.multiplier);
        assert_eq!(qty.source, deserialized.source);
        assert_eq!(qty, deserialized);
    }

    #[test]
    fn test_qty_multiplier_edge_cases() {
        // Spec: Section 3.2.31
        let qty_zero = Qty::builder().multiplier(Some(0.0)).build().unwrap();
        assert_eq!(qty_zero.multiplier, Some(0.0));

        let qty_one = Qty::builder().multiplier(Some(1.0)).build().unwrap();
        assert_eq!(qty_one.multiplier, Some(1.0));

        let qty_large = Qty::builder().multiplier(Some(99999.0)).build().unwrap();
        assert_eq!(qty_large.multiplier, Some(99999.0));

        let json = serde_json::to_string(&qty_large).unwrap();
        let deserialized: Qty = serde_json::from_str(&json).unwrap();
        assert_eq!(qty_large, deserialized);
    }
}
