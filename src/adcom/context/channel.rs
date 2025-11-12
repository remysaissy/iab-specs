use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Channel Object (Section 7.12)
///
/// Details about the distribution channel.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Channel<Ext: Extension = serde_json::Value> {
    /// Channel identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Channel domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Channel {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ChannelBuilder {
        ChannelBuilder::create_empty()
    }
}
