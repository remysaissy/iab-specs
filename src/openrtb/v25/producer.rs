use crate::Extension;
/// OpenRTB 2.5 Producer Object
///
/// This module implements the Producer object for content origination.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Default category taxonomy (1 = IAB Content Category Taxonomy 1.0)
fn default_cattax() -> i32 {
    1
}

/// Producer object defining content producer (OpenRTB 2.5 Section 3.2.17)
///
/// A `Producer` object defines the producer or originator of the content in which
/// the ad will be displayed. This is particularly useful in cases of syndicated content.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Producer;
///
/// let producer = Producer::builder()
///     .id(Some("prod123".to_string()))
///     .name(Some("Warner Bros".to_string()))
///     .domain(Some("warnerbros.com".to_string()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Producer<Ext: Extension = serde_json::Value> {
    /// Content producer or originator ID useful in syndication.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Content producer or originator name (e.g., "Warner Bros").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// The taxonomy in use for cat attribute.
    /// Refer to AdCOM `CategoryTaxonomy` enumeration.
    /// Default is 1 (IAB Content Category Taxonomy 1.0).
    #[serde(default = "default_cattax")]
    #[builder(default = "default_cattax()")]
    pub cattax: i32,

    /// Array of IAB content categories that describe the content producer.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the content producer (e.g., "producer.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Producer {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ProducerBuilder {
        ProducerBuilder::create_empty()
    }
}

impl<Ext: Extension> Default for Producer<Ext> {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            cattax: default_cattax(),
            cat: None,
            domain: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_creation() {
        let producer = Producer::builder()
            .id(Some("prod123".to_string()))
            .name(Some("Warner Bros".to_string()))
            .domain(Some("warnerbros.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(producer.id, Some("prod123".to_string()));
        assert_eq!(producer.name, Some("Warner Bros".to_string()));
        assert_eq!(producer.domain, Some("warnerbros.com".to_string()));
        assert_eq!(producer.cattax, 1); // Default value
    }

    #[test]
    fn test_producer_with_categories() {
        let producer = Producer::builder()
            .id(Some("prod456".to_string()))
            .cat(Some(vec!["IAB1".to_string(), "IAB2".to_string()]))
            .build()
            .unwrap();

        assert_eq!(producer.cat.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_producer_serialization() {
        let producer = Producer::builder()
            .id(Some("prod123".to_string()))
            .name(Some("Warner Bros".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&producer).unwrap();
        assert!(json.contains("\"id\":\"prod123\""));
        assert!(json.contains("\"name\":\"Warner Bros\""));
    }

    #[test]
    fn test_producer_deserialization() {
        let json = r#"{"id":"prod123","name":"Warner Bros"}"#;
        let producer: Producer = serde_json::from_str(json).unwrap();

        assert_eq!(producer.id, Some("prod123".to_string()));
        assert_eq!(producer.name, Some("Warner Bros".to_string()));
    }
}
