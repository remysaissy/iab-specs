use crate::Extension;
use crate::adcom::media::{DataAsset, ImageAsset, LinkAsset, TitleAsset, VideoAsset};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Asset Object (Section 3.5)
///
/// Container for a native ad component. Must include exactly one asset subtype
/// (title, img, video, or data).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Asset<Ext: Extension = serde_json::Value> {
    /// Asset format ID reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Required flag (1=required, 0=optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<i32>,

    /// Title asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<TitleAsset>>,

    /// Image asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<Box<ImageAsset>>,

    /// Video asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<VideoAsset>>,

    /// Data asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<DataAsset>>,

    /// Link asset for this specific asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<LinkAsset>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Asset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssetBuilder {
        AssetBuilder::create_empty()
    }
}
