use crate::Extension;
use crate::adcom::media::LinkAsset;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Banner Object (Section 3.3)
///
/// Basic banner creative containing image and link assets.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Banner<Ext: Extension = serde_json::Value> {
    /// Image URL (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,

    /// Destination link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<LinkAsset>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Banner {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BannerBuilder {
        BannerBuilder::create_empty()
    }
}
