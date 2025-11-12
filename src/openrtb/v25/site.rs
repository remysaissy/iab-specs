use super::content::Content;
use super::publisher::Publisher;
use crate::Extension;
/// OpenRTB 2.5 Site Object
///
/// This module implements the Site object for website context.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Default category taxonomy (1 = IAB Content Category Taxonomy 1.0)
fn default_cattax() -> i32 {
    1
}

/// Site object describing publisher website (OpenRTB 2.5 Section 3.2.13)
///
/// A `Site` object should be included if the ad-supported content is a website (as opposed to
/// a non-browser application). A bid request must not contain both a Site and an App object.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Site<Ext: Extension = serde_json::Value> {
    /// Exchange-specific site ID.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Site name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// Domain of the site (e.g., "mysite.foo.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<String>,

    /// The taxonomy in use for cat attribute.
    /// Default is 1 (IAB Content Category Taxonomy 1.0).
    /// Refer to AdCOM `CategoryTaxonomy` enumeration.
    #[serde(default = "default_cattax")]
    #[builder(default = "default_cattax()")]
    pub cattax: i32,

    /// Array of IAB content categories of the site.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current section of the site.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current page or view of the site.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pagecat: Option<Vec<String>>,

    /// URL of the page where the impression will be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ref_: Option<String>,

    /// Search string that caused navigation to the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub search: Option<String>,

    /// Indicates if the site has been programmed to optimize layout when viewed on mobile devices:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mobile: Option<i32>,

    /// Indicates if the site has a privacy policy:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub privacypolicy: Option<i32>,

    /// Details about the Publisher of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub publisher: Option<Publisher>,

    /// Details about the Content within the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub content: Option<Content>,

    /// Comma-separated list of keywords about the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub keywords: Option<String>,

    /// Array of keywords about the site.
    /// Mutually exclusive with `keywords` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub kwarray: Option<Vec<String>>,

    /// Used for inventory authorization in chain of custody scenarios.
    /// Domain of the inventory partner authorized to sell this ad space.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub inventorypartnerdomain: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Site {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SiteBuilder {
        SiteBuilder::create_empty()
    }
}

impl<Ext: Extension> Default for Site<Ext> {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            domain: None,
            cattax: default_cattax(),
            cat: None,
            sectioncat: None,
            pagecat: None,
            page: None,
            ref_: None,
            search: None,
            mobile: None,
            privacypolicy: None,
            publisher: None,
            content: None,
            keywords: None,
            kwarray: None,
            inventorypartnerdomain: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_creation() {
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .name(Some("Example Site".to_string()))
            .domain(Some("example.com".to_string()))
            .page(Some("https://example.com/page".to_string()))
            .build()
            .unwrap();

        assert_eq!(site.id, Some("site123".to_string()));
        assert_eq!(site.name, Some("Example Site".to_string()));
        assert_eq!(site.domain, Some("example.com".to_string()));
        assert_eq!(site.cattax, 1); // Default value
    }

    #[test]
    fn test_site_with_publisher() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Publisher Inc".to_string()))
            .build()
            .unwrap();

        let site = Site::builder()
            .id(Some("site456".to_string()))
            .publisher(Some(publisher))
            .build()
            .unwrap();

        assert!(site.publisher.is_some());
        assert_eq!(
            site.publisher.as_ref().unwrap().id,
            Some("pub123".to_string())
        );
    }

    #[test]
    fn test_site_serialization() {
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .domain(Some("example.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&site).unwrap();
        assert!(json.contains("\"id\":\"site123\""));
        assert!(json.contains("\"domain\":\"example.com\""));
    }

    #[test]
    fn test_site_deserialization() {
        let json = r#"{"id":"site123","domain":"example.com"}"#;
        let site: Site = serde_json::from_str(json).unwrap();

        assert_eq!(site.id, Some("site123".to_string()));
        assert_eq!(site.domain, Some("example.com".to_string()));
    }
}
