use crate::Extension;
use crate::adcom::placement::AssetFormat;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// NativeFormat Object (Section 4.4)
///
/// Native ad format requirements including required assets and event tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NativeFormat<Ext: Extension = serde_json::Value> {
    /// Array of asset format specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Vec<AssetFormat>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl NativeFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NativeFormatBuilder {
        NativeFormatBuilder::create_empty()
    }
}
