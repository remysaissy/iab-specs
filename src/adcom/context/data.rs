use crate::Extension;
use crate::adcom::context::Segment;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Data Object (Section 7.8)
///
/// First-party data segment with user information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Data<Ext: Extension = serde_json::Value> {
    /// Vendor-specific data provider identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Data provider name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Array of data segments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<Segment>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Data {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataBuilder {
        DataBuilder::create_empty()
    }
}
