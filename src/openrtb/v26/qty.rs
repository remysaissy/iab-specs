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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example - DOOH Multiplier
/// ```
/// use iab_specs::openrtb::v26::Qty;
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
pub struct Qty<Ext: Extension = serde_json::Value> {
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
        let qty = Qty::builder().build().unwrap();
        let json = serde_json::to_string(&qty).unwrap();
        assert_eq!(json, "{}");
    }
}
