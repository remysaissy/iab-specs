use super::margin::Margin;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Payload for adjusting a deal's bid floor or margin.
///
/// Used with `Intent::AdjustDealFloor` and `Intent::AdjustDealMargin`.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::{AdjustDealPayload, Margin, CalculationType};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let payload = AdjustDealPayload::builder()
///     .bidfloor(5.00)
///     .margin(Some(Margin::builder()
///         .value(0.10)
///         .calculation_type(CalculationType::Percent)
///         .build()?))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AdjustDealPayload<Ext: Extension = crate::DefaultExt> {
    /// The adjusted bid floor value.
    pub bidfloor: f64,

    /// Optional margin adjustment for the deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub margin: Option<Margin<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AdjustDealPayload {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AdjustDealPayloadBuilder {
        AdjustDealPayloadBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artb::v10::enums::CalculationType;

    #[test]
    fn test_adjust_deal_payload_creation() {
        let payload = AdjustDealPayload::builder().bidfloor(5.00).build().unwrap();

        assert_eq!(payload.bidfloor, 5.00);
        assert!(payload.margin.is_none());
    }

    #[test]
    fn test_adjust_deal_payload_with_margin() {
        let payload = AdjustDealPayload::builder()
            .bidfloor(3.50)
            .margin(Some(
                Margin::builder()
                    .value(0.15)
                    .calculation_type(CalculationType::Percent)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(payload.bidfloor, 3.50);
        let margin = payload.margin.as_ref().unwrap();
        assert_eq!(margin.value, 0.15);
        assert_eq!(margin.calculation_type, CalculationType::Percent);
    }

    #[test]
    fn test_adjust_deal_payload_serialization() {
        let payload = AdjustDealPayload::builder().bidfloor(4.00).build().unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"bidfloor\":4.0"));
    }

    #[test]
    fn test_adjust_deal_payload_deserialization() {
        let json = r#"{"bidfloor":2.50,"margin":{"value":0.10,"calculation_type":0}}"#;
        let payload: AdjustDealPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.bidfloor, 2.50);
        assert!(payload.margin.is_some());
    }

    #[test]
    fn test_adjust_deal_payload_roundtrip() {
        let payload = AdjustDealPayload::builder()
            .bidfloor(6.75)
            .margin(Some(
                Margin::builder()
                    .value(1.00)
                    .calculation_type(CalculationType::Cpm)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: AdjustDealPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(payload, parsed);
    }
}
