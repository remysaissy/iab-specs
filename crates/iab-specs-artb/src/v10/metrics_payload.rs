use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Payload containing OpenRTB Metric objects.
///
/// Used with `Intent::AddMetrics` to add measurement metrics to an impression.
/// The `metric` field contains OpenRTB `Metric` objects represented as JSON values,
/// allowing compatibility with any OpenRTB version.
///
/// # Generic Parameters
///
/// * `P` - Payload type for metric objects (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// #[cfg(feature = "artb_10")]
/// {
/// use iab_specs_artb::v10::MetricsPayloadBuilder;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let payload = MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
///     .metric(vec![
///         serde_json::json!({
///             "type": "viewability",
///             "value": 0.85,
///             "vendor": "vendor.com"
///         }),
///     ])
///     .build()?;
/// # Ok(())
/// # }
/// }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "P: Extension, Ext: Extension",
    deserialize = "P: Extension, Ext: Extension"
))]
pub struct MetricsPayload<P: Extension = crate::DefaultExt, Ext: Extension = crate::DefaultExt> {
    /// Array of OpenRTB Metric objects.
    /// Each metric describes a measurement the exchange supports for this item.
    #[builder(default, setter(into))]
    pub metric: Vec<P>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl MetricsPayload {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> MetricsPayloadBuilder<crate::DefaultExt, crate::DefaultExt> {
        MetricsPayloadBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_payload_creation() {
        let payload = MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
            .metric(vec![serde_json::json!({
                "type": "viewability",
                "value": 0.80,
                "vendor": "iab.com"
            })])
            .build()
            .unwrap();

        assert_eq!(payload.metric.len(), 1);
    }

    #[test]
    fn test_metrics_payload_empty() {
        let payload = MetricsPayload::builder().build().unwrap();
        assert!(payload.metric.is_empty());
    }

    #[test]
    fn test_metrics_payload_serialization() {
        let payload = MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
            .metric(vec![
                serde_json::json!({"type": "viewability", "value": 0.70}),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"viewability\""));
        assert!(json.contains("\"value\":0.7"));
    }

    #[test]
    fn test_metrics_payload_deserialization() {
        let json = r#"{"metric":[{"type":"attention","value":0.65,"vendor":"vendor.com"}]}"#;
        let payload: MetricsPayload<serde_json::Value> = serde_json::from_str(json).unwrap();

        assert_eq!(payload.metric.len(), 1);
        assert_eq!(payload.metric[0]["type"], "attention");
    }

    #[test]
    fn test_metrics_payload_roundtrip() {
        let payload = MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
            .metric(vec![
                serde_json::json!({"type": "viewability", "value": 0.80}),
                serde_json::json!({"type": "completion", "value": 0.90}),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: MetricsPayload<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(payload, parsed);
    }

    #[test]
    fn test_metrics_payload_multiple_metrics() {
        let payload = MetricsPayloadBuilder::<serde_json::Value, Vec<u8>>::default()
            .metric(vec![
                serde_json::json!({"type": "viewability", "value": 0.70}),
                serde_json::json!({"type": "attention", "value": 0.60}),
                serde_json::json!({"type": "completion", "value": 0.95}),
            ])
            .build()
            .unwrap();

        assert_eq!(payload.metric.len(), 3);
    }
}
