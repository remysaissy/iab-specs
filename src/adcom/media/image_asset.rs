use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ImageAsset Object (Section 3.8)
///
/// Image specification for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ImageAsset<Ext: Extension = serde_json::Value> {
    /// Image URL (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Native image asset type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl ImageAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImageAssetBuilder {
        ImageAssetBuilder::create_empty()
    }
}
