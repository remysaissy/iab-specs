use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Payload for adjusting a bid price (bid shading).
///
/// Used with `Intent::BidShade` to propose a modified bid price.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::AdjustBidPayload;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let payload = AdjustBidPayload::builder()
///     .price(2.50)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AdjustBidPayload<Ext: Extension = crate::DefaultExt> {
    /// The adjusted bid price.
    pub price: f64,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AdjustBidPayload {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AdjustBidPayloadBuilder {
        AdjustBidPayloadBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_bid_payload_creation() {
        let payload = AdjustBidPayload::builder().price(2.50).build().unwrap();

        assert_eq!(payload.price, 2.50);
        assert!(payload.ext.is_none());
    }

    #[test]
    fn test_adjust_bid_payload_serialization() {
        let payload = AdjustBidPayload::builder().price(3.75).build().unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"price\":3.75"));
    }

    #[test]
    fn test_adjust_bid_payload_deserialization() {
        let json = r#"{"price":1.25}"#;
        let payload: AdjustBidPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.price, 1.25);
    }

    #[test]
    fn test_adjust_bid_payload_roundtrip() {
        let payload = AdjustBidPayload::builder().price(4.99).build().unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: AdjustBidPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(payload, parsed);
    }

    #[test]
    fn test_adjust_bid_payload_default() {
        let payload = AdjustBidPayload::builder().build().unwrap();
        assert_eq!(payload.price, 0.0);
    }
}
