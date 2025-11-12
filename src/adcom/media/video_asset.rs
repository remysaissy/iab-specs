use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// VideoAsset Object (Section 3.9)
///
/// Video specification for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct VideoAsset<Ext: Extension = serde_json::Value> {
    /// Ad markup (VAST document)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl VideoAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoAssetBuilder {
        VideoAssetBuilder::create_empty()
    }
}
