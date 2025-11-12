use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DataAssetFormat Object (Section 4.8)
///
/// Data asset constraints for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DataAssetFormat<Ext: Extension = serde_json::Value> {
    /// Data asset type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DataAssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataAssetFormatBuilder {
        DataAssetFormatBuilder::create_empty()
    }
}
