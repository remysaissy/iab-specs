use super::enums::OriginatorType;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Business entity that owns/originates the bid request or response.
///
/// Identifies the party that created the OpenRTB payload being processed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::{Originator, OriginatorType};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let originator = Originator::builder()
///     .type_(OriginatorType::Ssp)
///     .name("Example SSP")
///     .domain("ssp.example.com")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Originator<Ext: Extension = crate::DefaultExt> {
    /// The type of business entity.
    #[serde(rename = "type")]
    pub type_: OriginatorType,

    /// Human-readable name of the originator.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    /// Domain of the originator.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub domain: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Originator {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> OriginatorBuilder {
        OriginatorBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_originator_creation() {
        let originator = Originator::builder()
            .type_(OriginatorType::Ssp)
            .name("Test SSP")
            .domain("ssp.test.com")
            .build()
            .unwrap();

        assert_eq!(originator.type_, OriginatorType::Ssp);
        assert_eq!(originator.name, Some("Test SSP".to_string()));
        assert_eq!(originator.domain, Some("ssp.test.com".to_string()));
    }

    #[test]
    fn test_originator_publisher() {
        let originator = Originator::builder()
            .type_(OriginatorType::Publisher)
            .name("News Publisher")
            .build()
            .unwrap();

        assert_eq!(originator.type_, OriginatorType::Publisher);
        assert_eq!(originator.name, Some("News Publisher".to_string()));
        assert!(originator.domain.is_none());
    }

    #[test]
    fn test_originator_serialization() {
        let originator = Originator::builder()
            .type_(OriginatorType::Exchange)
            .name("Ad Exchange")
            .domain("exchange.com")
            .build()
            .unwrap();

        let json = serde_json::to_string(&originator).unwrap();
        assert!(json.contains("\"type\":3"));
        assert!(json.contains("\"name\":\"Ad Exchange\""));
        assert!(json.contains("\"domain\":\"exchange.com\""));
    }

    #[test]
    fn test_originator_deserialization() {
        let json = r#"{"type":4,"name":"Test DSP","domain":"dsp.test.com"}"#;
        let originator: Originator = serde_json::from_str(json).unwrap();

        assert_eq!(originator.type_, OriginatorType::Dsp);
        assert_eq!(originator.name, Some("Test DSP".to_string()));
        assert_eq!(originator.domain, Some("dsp.test.com".to_string()));
    }

    #[test]
    fn test_originator_roundtrip() {
        let originator = Originator::builder()
            .type_(OriginatorType::Ssp)
            .name("SSP")
            .domain("ssp.com")
            .build()
            .unwrap();

        let json = serde_json::to_string(&originator).unwrap();
        let parsed: Originator = serde_json::from_str(&json).unwrap();
        assert_eq!(originator, parsed);
    }

    #[test]
    fn test_originator_default() {
        let originator = Originator::builder().build().unwrap();
        assert_eq!(originator.type_, OriginatorType::Unspecified);
        assert!(originator.name.is_none());
        assert!(originator.domain.is_none());
    }
}
