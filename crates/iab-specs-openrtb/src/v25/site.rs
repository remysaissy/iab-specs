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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Site<Ext: Extension = crate::DefaultExt> {
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

    // === Phase 2.2: Mutually Exclusive Field Tests (keywords vs kwarray) ===

    #[test]
    fn test_site_with_keywords_only() {
        // Valid: Site with legacy keywords field (comma-separated string)
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .keywords(Some("technology,advertising,mobile".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            site.keywords,
            Some("technology,advertising,mobile".to_string())
        );
        assert!(site.kwarray.is_none());
    }

    #[test]
    fn test_site_with_kwarray_only() {
        // Valid: Site with kwarray field (array of strings)
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .kwarray(Some(vec![
                "technology".to_string(),
                "advertising".to_string(),
                "mobile".to_string(),
            ]))
            .build()
            .unwrap();

        assert!(site.keywords.is_none());
        assert_eq!(
            site.kwarray,
            Some(vec![
                "technology".to_string(),
                "advertising".to_string(),
                "mobile".to_string()
            ])
        );
    }

    #[test]
    fn test_site_with_both_keywords_and_kwarray() {
        // Per spec: kwarray is mutually exclusive with keywords
        // Test that having BOTH currently passes
        let site = Site::builder()
            .id(Some("site123".to_string()))
            .keywords(Some("tech,ads".to_string()))
            .kwarray(Some(vec!["mobile".to_string(), "video".to_string()]))
            .build();

        assert!(
            site.is_ok(),
            "Site with both keywords and kwarray currently passes"
        );

        let site = site.unwrap();
        assert_eq!(site.keywords, Some("tech,ads".to_string()));
        assert_eq!(
            site.kwarray,
            Some(vec!["mobile".to_string(), "video".to_string()])
        );
        // TODO: Per OpenRTB spec, kwarray is mutually exclusive with keywords
        // Should be rejected when both are present
    }

    #[test]
    fn test_site_deserialization_with_both_keyword_formats() {
        // Test deserialization behavior when JSON contains both keyword formats
        let json = r#"{
            "id": "site123",
            "keywords": "tech,ads",
            "kwarray": ["mobile", "video"]
        }"#;

        let result: Result<Site, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Deserialization with both keywords and kwarray currently passes"
        );

        let site = result.unwrap();
        assert_eq!(site.keywords, Some("tech,ads".to_string()));
        assert_eq!(
            site.kwarray,
            Some(vec!["mobile".to_string(), "video".to_string()])
        );
        // TODO: Should deserialization validate mutual exclusivity for keyword fields?
    }

    #[test]
    fn test_site_url_fields() {
        // Spec: Section 3.2.13
        let site = Site::builder()
            .page(Some("https://example.com/articles/tech".to_string()))
            .ref_(Some("https://google.com/search?q=tech".to_string()))
            .search(Some("tech news".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            site.page,
            Some("https://example.com/articles/tech".to_string())
        );
        assert_eq!(
            site.ref_,
            Some("https://google.com/search?q=tech".to_string())
        );
        assert_eq!(site.search, Some("tech news".to_string()));

        let json = serde_json::to_string(&site).unwrap();
        assert!(json.contains("\"page\":\"https://example.com/articles/tech\""));
        assert!(json.contains("\"ref_\":\"https://google.com/search?q=tech\""));
        assert!(json.contains("\"search\":\"tech news\""));
    }

    #[test]
    fn test_site_category_arrays() {
        // Spec: Section 3.2.13
        let site = Site::builder()
            .cat(Some(vec!["IAB1".to_string(), "IAB2".to_string()]))
            .sectioncat(Some(vec!["IAB1-1".to_string()]))
            .pagecat(Some(vec!["IAB2-3".to_string(), "IAB3-1".to_string()]))
            .build()
            .unwrap();

        assert_eq!(site.cat, Some(vec!["IAB1".to_string(), "IAB2".to_string()]));
        assert_eq!(site.sectioncat, Some(vec!["IAB1-1".to_string()]));
        assert_eq!(
            site.pagecat,
            Some(vec!["IAB2-3".to_string(), "IAB3-1".to_string()])
        );
    }

    #[test]
    fn test_site_mobile_flag() {
        // Spec: Section 3.2.13
        let site_mobile = Site::builder().mobile(Some(1)).build().unwrap();
        assert_eq!(site_mobile.mobile, Some(1));

        let site_not_mobile = Site::builder().mobile(Some(0)).build().unwrap();
        assert_eq!(site_not_mobile.mobile, Some(0));

        let site_default = Site::builder().build().unwrap();
        assert_eq!(site_default.mobile, None);
    }

    #[test]
    fn test_site_privacypolicy_flag() {
        // Spec: Section 3.2.13
        let site_with_policy = Site::builder().privacypolicy(Some(1)).build().unwrap();
        assert_eq!(site_with_policy.privacypolicy, Some(1));

        let site_no_policy = Site::builder().privacypolicy(Some(0)).build().unwrap();
        assert_eq!(site_no_policy.privacypolicy, Some(0));
    }

    #[test]
    fn test_site_with_content() {
        // Spec: Section 3.2.13
        let content = Content::builder()
            .id(Some("content-1".to_string()))
            .title(Some("Breaking News".to_string()))
            .language(Some("en".to_string()))
            .build()
            .unwrap();

        let site = Site::builder()
            .id(Some("site-1".to_string()))
            .content(Some(content))
            .build()
            .unwrap();

        assert!(site.content.is_some());
        let c = site.content.as_ref().unwrap();
        assert_eq!(c.id, Some("content-1".to_string()));
        assert_eq!(c.title, Some("Breaking News".to_string()));
        assert_eq!(c.language, Some("en".to_string()));
    }

    #[test]
    fn test_site_ext_field() {
        // Spec: Section 3.2.13
        let site = SiteBuilder::<serde_json::Value>::default()
            .id(Some("site-ext".to_string()))
            .ext(Some(Box::new(serde_json::json!({
                "custom_field": "custom_value"
            }))))
            .build()
            .unwrap();

        assert!(site.ext.is_some());
        let ext = site.ext.as_ref().unwrap();
        assert_eq!(ext["custom_field"], "custom_value");
    }

    #[test]
    fn test_site_roundtrip_all_fields() {
        // Spec: Section 3.2.13
        let publisher = Publisher::builder()
            .id(Some("pub-1".to_string()))
            .build()
            .unwrap();

        let content = Content::builder()
            .id(Some("content-1".to_string()))
            .build()
            .unwrap();

        let site = Site::builder()
            .id(Some("site-all".to_string()))
            .name(Some("All Fields Site".to_string()))
            .domain(Some("allfields.com".to_string()))
            .cattax(2)
            .cat(Some(vec!["IAB1".to_string()]))
            .sectioncat(Some(vec!["IAB1-1".to_string()]))
            .pagecat(Some(vec!["IAB2-1".to_string()]))
            .page(Some("https://allfields.com/page".to_string()))
            .ref_(Some("https://referrer.com".to_string()))
            .search(Some("search query".to_string()))
            .mobile(Some(1))
            .privacypolicy(Some(1))
            .publisher(Some(publisher))
            .content(Some(content))
            .keywords(Some("all,fields".to_string()))
            .inventorypartnerdomain(Some("partner.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&site).unwrap();
        let deserialized: Site = serde_json::from_str(&json).unwrap();

        assert_eq!(site.id, deserialized.id);
        assert_eq!(site.name, deserialized.name);
        assert_eq!(site.domain, deserialized.domain);
        assert_eq!(site.cattax, deserialized.cattax);
        assert_eq!(site.cat, deserialized.cat);
        assert_eq!(site.sectioncat, deserialized.sectioncat);
        assert_eq!(site.pagecat, deserialized.pagecat);
        assert_eq!(site.page, deserialized.page);
        assert_eq!(site.ref_, deserialized.ref_);
        assert_eq!(site.search, deserialized.search);
        assert_eq!(site.mobile, deserialized.mobile);
        assert_eq!(site.privacypolicy, deserialized.privacypolicy);
        assert_eq!(site.publisher, deserialized.publisher);
        assert_eq!(site.content, deserialized.content);
        assert_eq!(site.keywords, deserialized.keywords);
        assert_eq!(
            site.inventorypartnerdomain,
            deserialized.inventorypartnerdomain
        );
    }
}
