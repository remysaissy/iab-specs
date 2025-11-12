use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Publisher Object
///
/// The publisher of the media in which ads will be displayed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Publisher<Ext: Extension = serde_json::Value> {
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
