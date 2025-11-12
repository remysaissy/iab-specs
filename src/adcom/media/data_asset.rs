use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DataAsset Object (Section 3.10)
///
/// Data/text content for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DataAsset<Ext: Extension = serde_json::Value> {
    /// Formatted string of data (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Type of data asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DataAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataAssetBuilder {
        DataAssetBuilder::create_empty()
    }
}
