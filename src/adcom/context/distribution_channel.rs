use crate::Extension;
use crate::adcom::context::{Content, Publisher};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Distribution Channel abstract base
///
/// Abstract base for distribution channel types (Site, App, DOOH).
/// This is the parent object for describing the properties of the medium
/// through which advertising is being offered.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DistributionChannel<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique identifier of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DistributionChannel {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DistributionChannelBuilder {
        DistributionChannelBuilder::create_empty()
    }
}
