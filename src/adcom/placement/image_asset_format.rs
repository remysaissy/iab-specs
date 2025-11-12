use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ImageAssetFormat Object (Section 4.7)
///
/// Image asset constraints for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ImageAssetFormat<Ext: Extension = serde_json::Value> {
    /// Native image asset type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Array of MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Minimum width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Minimum height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,

    /// Width as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Height as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl ImageAssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImageAssetFormatBuilder {
        ImageAssetFormatBuilder::create_empty()
    }
}
