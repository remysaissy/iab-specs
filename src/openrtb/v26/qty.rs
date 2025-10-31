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
/// # Example - DOOH Multiplier
/// ```
/// use iab_specs::openrtb::v26::Qty;
///
/// let qty = Qty {
///     multiplier: Some(150.0),  // 150 people viewing the ad
///     source: Some("MeasurementVendor".to_string()),
///     ext: None,
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Qty {
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
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qty_builder() {
        let qty = QtyBuilder::default()
            .multiplier(Some(150.5))
            .source(Some("MeasurementVendor".to_string()))
            .build()
            .unwrap();

        assert_eq!(qty.multiplier, Some(150.5));
        assert_eq!(qty.source, Some("MeasurementVendor".to_string()));
    }

    #[test]
    fn test_qty_serialization() {
        let qty = Qty {
            multiplier: Some(100.0),
            source: Some("Publisher".to_string()),
            ext: None,
        };

        let json = serde_json::to_string(&qty).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("Publisher"));

        let deserialized: Qty = serde_json::from_str(&json).unwrap();
        assert_eq!(qty, deserialized);
    }

    #[test]
    fn test_skip_serializing_none() {
        let qty = Qty::default();
        let json = serde_json::to_string(&qty).unwrap();
        assert_eq!(json, "{}");
    }
}
