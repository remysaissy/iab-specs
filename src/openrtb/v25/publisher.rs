use crate::Extension;
/// OpenRTB 2.5 Publisher Object
///
/// This module implements the Publisher object for inventory suppliers.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Default category taxonomy (1 = IAB Content Category Taxonomy 1.0)
fn default_cattax() -> i32 {
    1
}

/// Publisher object representing inventory supplier (OpenRTB 2.5 Section 3.2.15)
///
/// A `Publisher` object describes the publisher of the media in which the ad will be displayed.
/// The publisher is typically the seller in an RTB transaction.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Publisher;
///
/// let publisher = Publisher::builder()
///     .id(Some("pub123".to_string()))
///     .name(Some("Publisher Inc".to_string()))
///     .domain(Some("publisher.com".to_string()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Publisher<Ext: Extension = crate::DefaultExt> {
    /// Exchange-specific publisher ID.
    /// This ID maps to the `seller_id` in the ads.txt specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Publisher name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// The taxonomy in use for cat attribute.
    /// Refer to AdCOM `CategoryTaxonomy` enumeration.
    /// Default is 1 (IAB Content Category Taxonomy 1.0).
    #[serde(default = "default_cattax")]
    #[builder(default = "default_cattax()")]
    pub cattax: i32,

    /// Array of IAB content categories that describe the publisher.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the publisher (e.g., "publisher.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Publisher {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PublisherBuilder {
        PublisherBuilder::create_empty()
    }
}

impl<Ext: Extension> Default for Publisher<Ext> {
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
    fn test_publisher_creation() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Publisher Inc".to_string()))
            .domain(Some("publisher.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(publisher.id, Some("pub123".to_string()));
        assert_eq!(publisher.name, Some("Publisher Inc".to_string()));
        assert_eq!(publisher.domain, Some("publisher.com".to_string()));
        assert_eq!(publisher.cattax, 1); // Default value
    }

    #[test]
    fn test_publisher_with_categories() {
        let publisher = Publisher::builder()
            .id(Some("pub456".to_string()))
            .cat(Some(vec!["IAB1".to_string(), "IAB2".to_string()]))
            .build()
            .unwrap();

        assert_eq!(publisher.cat.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_publisher_serialization() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Publisher Inc".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&publisher).unwrap();
        assert!(json.contains("\"id\":\"pub123\""));
        assert!(json.contains("\"name\":\"Publisher Inc\""));
    }

    #[test]
    fn test_publisher_deserialization() {
        let json = r#"{"id":"pub123","name":"Publisher Inc"}"#;
        let publisher: Publisher = serde_json::from_str(json).unwrap();

        assert_eq!(publisher.id, Some("pub123".to_string()));
        assert_eq!(publisher.name, Some("Publisher Inc".to_string()));
    }

    #[test]
    fn test_publisher_domain_field() {
        // Spec: Section 3.2.15
        let publisher = Publisher::builder()
            .id(Some("pub1".to_string()))
            .domain(Some("news-site.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(publisher.domain, Some("news-site.com".to_string()));
    }

    #[test]
    fn test_publisher_name_field() {
        // Spec: Section 3.2.15
        let publisher = Publisher::builder()
            .name(Some("Acme Media Group".to_string()))
            .build()
            .unwrap();

        assert_eq!(publisher.name, Some("Acme Media Group".to_string()));
    }

    #[test]
    fn test_publisher_ext_with_serde_json_value() {
        // Spec: Section 3.2.15
        let ext = serde_json::json!({"tier": "premium", "rep_score": 95});

        let publisher = PublisherBuilder::<serde_json::Value>::default()
            .id(Some("pub1".to_string()))
            .ext(Some(Box::new(ext)))
            .build()
            .unwrap();

        assert!(publisher.ext.is_some());
        assert_eq!(publisher.ext.as_ref().unwrap()["tier"], "premium");
        assert_eq!(publisher.ext.as_ref().unwrap()["rep_score"], 95);
    }

    #[test]
    fn test_publisher_serde_roundtrip_all_fields() {
        // Spec: Section 3.2.15
        let publisher = PublisherBuilder::<serde_json::Value>::default()
            .id(Some("pub-full".to_string()))
            .name(Some("Full Publisher".to_string()))
            .cattax(1)
            .cat(Some(vec!["IAB1".to_string(), "IAB3".to_string()]))
            .domain(Some("fullpub.com".to_string()))
            .ext(Some(Box::new(serde_json::json!({"verified": true}))))
            .build()
            .unwrap();

        let json = serde_json::to_string(&publisher).unwrap();
        let deserialized: Publisher<serde_json::Value> = serde_json::from_str(&json).unwrap();

        assert_eq!(publisher.id, deserialized.id);
        assert_eq!(publisher.name, deserialized.name);
        assert_eq!(publisher.cattax, deserialized.cattax);
        assert_eq!(publisher.cat, deserialized.cat);
        assert_eq!(publisher.domain, deserialized.domain);
        assert_eq!(publisher.ext, deserialized.ext);
    }
}
