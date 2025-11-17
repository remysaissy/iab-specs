use crate::Extension;
use crate::adcom::context::{Content, Publisher};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Site Object (Section 7.1)
///
/// Distribution channel for website-based advertising.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Site<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique site identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Site name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Domain of the site (e.g., "example.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories using taxonomy IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Array of section categories using taxonomy IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of page categories using taxonomy IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// URL of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// Search string that caused navigation to the current page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Indicates if site is mobile optimized (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Privacy policy flag (1=has policy, 0=no policy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Site {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SiteBuilder {
        SiteBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_builder() {
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .name(Some("Example Site".to_string()))
            .domain(Some("example.com".to_string()))
            .page(Some("https://example.com/article".to_string()))
            .build()
            .unwrap();

        assert_eq!(site.id, Some("site123".to_string()));
        assert_eq!(site.name, Some("Example Site".to_string()));
        assert_eq!(site.domain, Some("example.com".to_string()));
        assert_eq!(site.page, Some("https://example.com/article".to_string()));
    }

    #[test]
    fn test_site_default() {
        let site = Site::builder().build().unwrap();

        assert!(site.id.is_none());
        assert!(site.name.is_none());
        assert!(site.domain.is_none());
        assert!(site.pub_.is_none());
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
            .pub_(Some(Box::new(publisher)))
            .build()
            .unwrap();

        assert!(site.pub_.is_some());
        assert_eq!(site.pub_.as_ref().unwrap().id, Some("pub123".to_string()));
    }

    #[test]
    fn test_site_serialization() {
        let site = Site::builder()
            .id(Some("site789".to_string()))
            .domain(Some("news.com".to_string()))
            .mobile(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&site).unwrap();
        assert!(json.contains("\"id\":\"site789\""));
        assert!(json.contains("\"domain\":\"news.com\""));
        assert!(json.contains("\"mobile\":1"));
    }

    #[test]
    fn test_site_deserialization() {
        let json =
            r#"{"id":"site999","name":"Tech Blog","domain":"techblog.com","privacypolicy":1}"#;
        let site: Site = serde_json::from_str(json).unwrap();

        assert_eq!(site.id, Some("site999".to_string()));
        assert_eq!(site.name, Some("Tech Blog".to_string()));
        assert_eq!(site.domain, Some("techblog.com".to_string()));
        assert_eq!(site.privacypolicy, Some(1));
    }

    #[test]
    fn test_site_with_categories() {
        let site = Site::builder()
            .id(Some("site111".to_string()))
            .cat(Some(vec!["IAB12".to_string(), "IAB12-1".to_string()]))
            .cattax(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            site.cat,
            Some(vec!["IAB12".to_string(), "IAB12-1".to_string()])
        );
        assert_eq!(site.cattax, Some(1));
    }
}
