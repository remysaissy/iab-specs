use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// TitleAsset Object (Section 3.7)
///
/// Text heading for native ad.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct TitleAsset<Ext: Extension = serde_json::Value> {
    /// Title text (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl TitleAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TitleAssetBuilder {
        TitleAssetBuilder::create_empty()
    }
}
