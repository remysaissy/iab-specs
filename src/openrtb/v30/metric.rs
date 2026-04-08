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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
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
pub struct Metric<Ext: Extension = crate::DefaultExt> {
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

    // Spec: Object: Metric — required type_ and val fields with optional vendor
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

    // Spec: Object: Metric — viewability metric type with val threshold
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

    // Spec: Object: Metric — attention metric type with vendor
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

    // Spec: Object: Metric — completion metric type with vendor
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

    // Spec: Object: Metric — JSON serialization with type→"type" serde rename
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

    // Spec: Object: Metric — JSON deserialization with "type"→type_ serde rename
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

    // Spec: Object: Metric — vendor is optional, None when omitted
    #[test]
    fn test_metric_without_vendor() {
        let metric = Metric::builder()
            .type_("custom_metric".to_string())
            .val(0.50)
            .build()
            .unwrap();

        assert_eq!(metric.vendor, None);
    }

    // Spec: Object: Metric — high val threshold (0.95) for viewability
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

    // Spec: Object: Metric — different metric types are distinct objects
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

    // Spec: Object: Metric — default() produces empty type_ and val 0.0
    #[test]
    fn test_metric_default() {
        let metric: Metric = Metric::default();
        assert_eq!(metric.type_, "");
        assert_eq!(metric.val, 0.0);
        assert_eq!(metric.vendor, None);
        assert_eq!(metric.ext, None);
    }

    // Spec: Object: Metric — serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_metric_roundtrip() {
        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.85)
            .vendor(Some("vendor.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&metric).unwrap();
        let deserialized: Metric = serde_json::from_str(&json).unwrap();
        assert_eq!(metric, deserialized);
    }

    // Spec: Object: Metric — serde rename type_→"type" in JSON output
    #[test]
    fn test_metric_type_rename_in_json() {
        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.70)
            .build()
            .unwrap();

        let json = serde_json::to_string(&metric).unwrap();
        assert!(json.contains("\"type\":"));
        assert!(!json.contains("\"type_\":"));
    }

    // Spec: Object: Metric — optional vendor field omitted from JSON when None
    #[test]
    fn test_metric_optional_fields_not_in_json() {
        let metric = Metric::builder()
            .type_("attention".to_string())
            .val(0.60)
            .build()
            .unwrap();

        let json = serde_json::to_string(&metric).unwrap();
        assert!(!json.contains("\"vendor\""));
        assert!(!json.contains("\"ext\""));
    }
}
