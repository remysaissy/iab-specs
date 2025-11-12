use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Network Object (Section 7.11)
///
/// Details about the distribution network.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Network<Ext: Extension = serde_json::Value> {
    /// Network identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Network domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Network {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NetworkBuilder {
        NetworkBuilder::create_empty()
    }
}
