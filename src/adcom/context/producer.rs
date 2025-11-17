use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Producer Object
///
/// The producer of the content in which ads will be displayed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Producer<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique producer identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Highest level domain of the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories describing the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Producer {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ProducerBuilder {
        ProducerBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_builder() {
        let producer = Producer::builder()
            .id(Some("prod123".to_string()))
            .name(Some("Content Studio".to_string()))
            .domain(Some("studio.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(producer.id, Some("prod123".to_string()));
        assert_eq!(producer.name, Some("Content Studio".to_string()));
        assert_eq!(producer.domain, Some("studio.com".to_string()));
    }

    #[test]
    fn test_producer_default() {
        let producer = Producer::builder().build().unwrap();

        assert!(producer.id.is_none());
        assert!(producer.name.is_none());
        assert!(producer.cat.is_none());
    }

    #[test]
    fn test_producer_serialization() {
        let producer = Producer::builder()
            .id(Some("prod456".to_string()))
            .name(Some("Film Productions".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&producer).unwrap();
        assert!(json.contains("\"id\":\"prod456\""));
        assert!(json.contains("\"name\":\"Film Productions\""));
    }

    #[test]
    fn test_producer_deserialization() {
        let json = r#"{"id":"prod789","name":"Media Company","domain":"media.com"}"#;
        let producer: Producer = serde_json::from_str(json).unwrap();

        assert_eq!(producer.id, Some("prod789".to_string()));
        assert_eq!(producer.name, Some("Media Company".to_string()));
        assert_eq!(producer.domain, Some("media.com".to_string()));
    }

    #[test]
    fn test_producer_with_categories() {
        let producer = Producer::builder()
            .id(Some("prod999".to_string()))
            .name(Some("Entertainment Studio".to_string()))
            .cat(Some(vec!["IAB1".to_string(), "IAB1-1".to_string()]))
            .cattax(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            producer.cat,
            Some(vec!["IAB1".to_string(), "IAB1-1".to_string()])
        );
        assert_eq!(producer.cattax, Some(1));
    }
}
