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
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::Metric;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let metric = Metric {
///     typ: "viewability".to_string(),
///     val: 0.70,
///     vendor: Some("vendor.com".to_string()),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Metric {
    /// Type of metric being specified.
    /// Common values include:
    /// - "viewability" - Ad viewability
    /// - "attention" - Attention metrics
    /// - "completion" - Video completion rate
    ///
    /// REQUIRED by the specification.
    #[serde(rename = "type")]
    pub typ: String,

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
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_creation() {
        let metric = Metric {
            typ: "viewability".to_string(),
            val: 0.70,
            vendor: Some("iab.com".to_string()),
            ..Default::default()
        };

        assert_eq!(metric.typ, "viewability");
        assert_eq!(metric.val, 0.70);
        assert_eq!(metric.vendor, Some("iab.com".to_string()));
    }

    #[test]
    fn test_metric_viewability() {
        let metric = Metric {
            typ: "viewability".to_string(),
            val: 0.80,
            ..Default::default()
        };

        assert_eq!(metric.typ, "viewability");
        assert_eq!(metric.val, 0.80);
    }

    #[test]
    fn test_metric_attention() {
        let metric = Metric {
            typ: "attention".to_string(),
            val: 0.65,
            vendor: Some("attention_vendor.com".to_string()),
            ..Default::default()
        };

        assert_eq!(metric.typ, "attention");
        assert_eq!(metric.val, 0.65);
    }

    #[test]
    fn test_metric_completion() {
        let metric = Metric {
            typ: "completion".to_string(),
            val: 0.90,
            vendor: Some("video_vendor.com".to_string()),
            ..Default::default()
        };

        assert_eq!(metric.typ, "completion");
        assert_eq!(metric.val, 0.90);
    }

    #[test]
    fn test_metric_serialization() {
        let metric = Metric {
            typ: "viewability".to_string(),
            val: 0.75,
            vendor: Some("vendor.com".to_string()),
            ..Default::default()
        };

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
        assert_eq!(metric.typ, "viewability");
        assert_eq!(metric.val, 0.75);
        assert_eq!(metric.vendor, Some("vendor.com".to_string()));
    }

    #[test]
    fn test_metric_builder() {
        let metric = MetricBuilder::default()
            .typ("viewability".to_string())
            .val(0.80)
            .vendor(Some("iab.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(metric.typ, "viewability");
        assert_eq!(metric.val, 0.80);
    }

    #[test]
    fn test_metric_without_vendor() {
        let metric = Metric {
            typ: "custom_metric".to_string(),
            val: 0.50,
            ..Default::default()
        };

        assert_eq!(metric.vendor, None);
    }

    #[test]
    fn test_metric_high_value() {
        let metric = Metric {
            typ: "viewability".to_string(),
            val: 0.95,
            ..Default::default()
        };

        assert_eq!(metric.val, 0.95);
        assert!(metric.val > 0.90);
    }

    #[test]
    fn test_metric_multiple_types() {
        let viewability = Metric {
            typ: "viewability".to_string(),
            val: 0.70,
            ..Default::default()
        };

        let attention = Metric {
            typ: "attention".to_string(),
            val: 0.60,
            ..Default::default()
        };

        assert_ne!(viewability.typ, attention.typ);
        assert_ne!(viewability.val, attention.val);
    }
}
