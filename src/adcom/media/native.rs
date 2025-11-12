use crate::Extension;
use crate::adcom::media::{Asset, LinkAsset};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Native Object (Section 3.4)
///
/// Root container for native ad format containing a default link and array
/// of native ad assets.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Native<Ext: Extension = serde_json::Value> {
    /// Default destination link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<LinkAsset>>,

    /// Array of native ad assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Vec<Asset>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Native {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NativeBuilder {
        NativeBuilder::create_empty()
    }
}
