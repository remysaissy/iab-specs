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
