use crate::Extension;
/// OpenRTB 3.0 Metric Object
///
/// This module implements the Metric object for measurement specifications.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Metric object (OpenRTB 3.0 Section 3.2.3)
///
/// The `Metric` object describes a measurement metric that the exchange supports
/// for this item. Common metrics include viewability, attention, and completion rates.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v30::Metric;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let metric = Metric::builder()
///     .type_("viewability".to_string())
///     .val(0.70)
///     .vendor(Some("vendor.com".to_string()))
///     .build()
///     .unwrap();
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Metric<Ext: Extension = serde_json::Value> {
    /// Type of metric being specified.
    /// Common values include:
    /// - "viewability" - Ad viewability
    /// - "attention" - Attention metrics
    /// - "completion" - Video completion rate
    ///
    /// REQUIRED by the specification.
    #[serde(rename = "type")]
    pub type_: String,

    /// Target measurement value for the metric.
    /// The meaning depends on the metric type.
    /// For viewability, this is typically a decimal between 0-1 (e.g., 0.70 for 70%).
    /// REQUIRED by the specification.
    pub val: f64,

    /// Measurement vendor responsible for calculating this metric.
    /// Domain name of the vendor (e.g., "vendor.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub vendor: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Metric {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> MetricBuilder {
        MetricBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_creation() {
        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.70)
            .vendor(Some("iab.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(metric.type_, "viewability");
        assert_eq!(metric.val, 0.70);
        assert_eq!(metric.vendor, Some("iab.com".to_string()));
    }

    #[test]
    fn test_metric_viewability() {
        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.80)
            .build()
            .unwrap();

        assert_eq!(metric.type_, "viewability");
        assert_eq!(metric.val, 0.80);
    }

    #[test]
    fn test_metric_attention() {
        let metric = Metric::builder()
            .type_("attention".to_string())
            .val(0.65)
            .vendor(Some("attention_vendor.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(metric.type_, "attention");
        assert_eq!(metric.val, 0.65);
    }

    #[test]
    fn test_metric_completion() {
        let metric = Metric::builder()
            .type_("completion".to_string())
            .val(0.90)
            .vendor(Some("video_vendor.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(metric.type_, "completion");
        assert_eq!(metric.val, 0.90);
    }

    #[test]
    fn test_metric_serialization() {
        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.75)
            .vendor(Some("vendor.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&metric).unwrap();
        assert!(json.contains("\"type\":\"viewability\""));
        assert!(json.contains("\"val\":0.75"));
        assert!(json.contains("\"vendor\":\"vendor.com\""));
    }

    #[test]
    fn test_metric_deserialization() {
        let json = r#"{
            "type": "viewability",
            "val": 0.75,
            "vendor": "vendor.com"
        }"#;

        let metric: Metric = serde_json::from_str(json).unwrap();
        assert_eq!(metric.type_, "viewability");
        assert_eq!(metric.val, 0.75);
        assert_eq!(metric.vendor, Some("vendor.com".to_string()));
    }

    #[test]
    fn test_metric_without_vendor() {
        let metric = Metric::builder()
            .type_("custom_metric".to_string())
            .val(0.50)
            .build()
            .unwrap();

        assert_eq!(metric.vendor, None);
    }

    #[test]
    fn test_metric_high_value() {
        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.95)
            .build()
            .unwrap();

        assert_eq!(metric.val, 0.95);
        assert!(metric.val > 0.90);
    }

    #[test]
    fn test_metric_multiple_types() {
        let viewability = Metric::builder()
            .type_("viewability".to_string())
            .val(0.70)
            .build()
            .unwrap();

        let attention = Metric::builder()
            .type_("attention".to_string())
            .val(0.60)
            .build()
            .unwrap();

        assert_ne!(viewability.type_, attention.type_);
        assert_ne!(viewability.val, attention.val);
    }
}
