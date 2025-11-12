use crate::Extension;
use crate::adcom::context::{Content, Publisher};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// App Object (Section 7.2)
///
/// Distribution channel for mobile/tablet application advertising.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct App<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique app identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// App name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// App domain (e.g., "example.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Category taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Section categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Page categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// App version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// App store bundle/package name (e.g., "com.example.app")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// Privacy policy flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// Paid app flag (0=free, 1=paid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// App store URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl App {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AppBuilder {
        AppBuilder::create_empty()
    }
}
