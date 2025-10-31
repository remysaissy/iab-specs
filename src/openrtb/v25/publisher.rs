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
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Publisher;
///
/// let publisher = Publisher {
///     id: Some("pub123".to_string()),
///     name: Some("Publisher Inc".to_string()),
///     domain: Some("publisher.com".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Publisher {
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
    pub ext: Option<serde_json::Value>,
}

impl Default for Publisher {
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
        let publisher = Publisher {
            id: Some("pub123".to_string()),
            name: Some("Publisher Inc".to_string()),
            domain: Some("publisher.com".to_string()),
            ..Default::default()
        };

        assert_eq!(publisher.id, Some("pub123".to_string()));
        assert_eq!(publisher.name, Some("Publisher Inc".to_string()));
        assert_eq!(publisher.domain, Some("publisher.com".to_string()));
        assert_eq!(publisher.cattax, 1); // Default value
    }

    #[test]
    fn test_publisher_with_categories() {
        let publisher = Publisher {
            id: Some("pub456".to_string()),
            cat: Some(vec!["IAB1".to_string(), "IAB2".to_string()]),
            ..Default::default()
        };

        assert_eq!(publisher.cat.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_publisher_serialization() {
        let publisher = Publisher {
            id: Some("pub123".to_string()),
            name: Some("Publisher Inc".to_string()),
            ..Default::default()
        };

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
}
