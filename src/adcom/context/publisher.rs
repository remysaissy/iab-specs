use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Publisher Object
///
/// The publisher of the media in which ads will be displayed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Publisher<Ext: Extension = crate::DefaultExt> {
    /// Vendor-specific unique publisher identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Highest level domain of the publisher (e.g., "publisher.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories describing the publisher using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Publisher {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PublisherBuilder {
        PublisherBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_builder() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Publisher Inc".to_string()))
            .domain(Some("publisher.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(publisher.id, Some("pub123".to_string()));
        assert_eq!(publisher.name, Some("Publisher Inc".to_string()));
        assert_eq!(publisher.domain, Some("publisher.com".to_string()));
    }

    #[test]
    fn test_publisher_default() {
        let publisher = Publisher::builder().build().unwrap();

        assert!(publisher.id.is_none());
        assert!(publisher.name.is_none());
        assert!(publisher.domain.is_none());
    }

    #[test]
    fn test_publisher_serialization() {
        let publisher = Publisher::builder()
            .id(Some("pub456".to_string()))
            .name(Some("Test Publisher".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&publisher).unwrap();
        assert!(json.contains("\"id\":\"pub456\""));
        assert!(json.contains("\"name\":\"Test Publisher\""));
    }

    #[test]
    fn test_publisher_deserialization() {
        let json = r#"{"id":"pub789","name":"Example Publisher","domain":"example.com"}"#;
        let publisher: Publisher = serde_json::from_str(json).unwrap();

        assert_eq!(publisher.id, Some("pub789".to_string()));
        assert_eq!(publisher.name, Some("Example Publisher".to_string()));
        assert_eq!(publisher.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_publisher_with_categories() {
        let publisher = Publisher::builder()
            .id(Some("pub999".to_string()))
            .name(Some("News Publisher".to_string()))
            .cat(Some(vec!["IAB12".to_string(), "IAB12-1".to_string()]))
            .cattax(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            publisher.cat,
            Some(vec!["IAB12".to_string(), "IAB12-1".to_string()])
        );
        assert_eq!(publisher.cattax, Some(1));
    }

    /// AdCOM 1.0 Section 7 - Publisher serialization roundtrip
    #[test]
    fn test_publisher_serialization_roundtrip() {
        let original = Publisher::builder()
            .id(Some("pub_rt".to_string()))
            .name(Some("Roundtrip Publisher".to_string()))
            .domain(Some("roundtrip.com".to_string()))
            .cattax(Some(1))
            .build()
            .unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Publisher = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    /// AdCOM 1.0 Section 7 - Publisher extension field handling
    #[test]
    fn test_publisher_ext() {
        let obj = PublisherBuilder::<serde_json::Value>::default()
            .id(Some("pub_ext".to_string()))
            .ext(Some(Box::new(
                serde_json::json!({"custom_field": "custom_value"}),
            )))
            .build()
            .unwrap();
        let json = serde_json::to_string(&obj).unwrap();
        assert!(json.contains("custom_field"));
        let deserialized: Publisher<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert!(deserialized.ext.is_some());
    }
}
