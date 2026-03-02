use super::enums::CalculationType;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Margin adjustment for a deal.
///
/// Specifies the margin value and how it should be calculated
/// (absolute CPM or percentage).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::{Margin, CalculationType};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let margin = Margin::builder()
///     .value(0.15)
///     .calculation_type(CalculationType::Percent)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Margin<Ext: Extension = serde_json::Value> {
    /// The margin value.
    /// Interpretation depends on `calculation_type`:
    /// - CPM: absolute margin in currency units
    /// - Percent: relative margin as a decimal (e.g., 0.15 for 15%)
    pub value: f64,

    /// How the margin value should be calculated.
    pub calculation_type: CalculationType,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Margin {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> MarginBuilder {
        MarginBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_creation_cpm() {
        let margin = Margin::builder()
            .value(2.50)
            .calculation_type(CalculationType::Cpm)
            .build()
            .unwrap();

        assert_eq!(margin.value, 2.50);
        assert_eq!(margin.calculation_type, CalculationType::Cpm);
    }

    #[test]
    fn test_margin_creation_percent() {
        let margin = Margin::builder()
            .value(0.15)
            .calculation_type(CalculationType::Percent)
            .build()
            .unwrap();

        assert_eq!(margin.value, 0.15);
        assert_eq!(margin.calculation_type, CalculationType::Percent);
    }

    #[test]
    fn test_margin_serialization() {
        let margin = Margin::builder()
            .value(0.20)
            .calculation_type(CalculationType::Percent)
            .build()
            .unwrap();

        let json = serde_json::to_string(&margin).unwrap();
        assert!(json.contains("\"value\":0.2"));
        assert!(json.contains("\"calculation_type\":1"));
    }

    #[test]
    fn test_margin_deserialization() {
        let json = r#"{"value":3.50,"calculation_type":0}"#;
        let margin: Margin = serde_json::from_str(json).unwrap();

        assert_eq!(margin.value, 3.50);
        assert_eq!(margin.calculation_type, CalculationType::Cpm);
    }

    #[test]
    fn test_margin_roundtrip() {
        let margin = Margin::builder()
            .value(1.75)
            .calculation_type(CalculationType::Cpm)
            .build()
            .unwrap();

        let json = serde_json::to_string(&margin).unwrap();
        let parsed: Margin = serde_json::from_str(&json).unwrap();
        assert_eq!(margin, parsed);
    }

    #[test]
    fn test_margin_default() {
        let margin = Margin::builder().build().unwrap();
        assert_eq!(margin.value, 0.0);
        assert_eq!(margin.calculation_type, CalculationType::Cpm);
    }
}
