use crate::Extension;
use crate::adcom::placement::{DataAssetFormat, ImageAssetFormat, TitleAssetFormat};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// AssetFormat Object (Section 4.5)
///
/// Native asset format specifications defining requirements for title, image, or data assets.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AssetFormat<Ext: Extension = serde_json::Value> {
    /// Asset format identifier (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Required flag (1=required, 0=optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<i32>,

    /// Title asset format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<TitleAssetFormat>>,

    /// Image asset format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<Box<ImageAssetFormat>>,

    /// Data asset format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<DataAssetFormat>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl AssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssetFormatBuilder {
        AssetFormatBuilder::create_empty()
    }
}
