use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DisplayFormat Object (Section 4.3)
///
/// Display creative format constraints including size and expandability.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DisplayFormat<Ext: Extension = serde_json::Value> {
    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Width as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Height as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Directions in which creative can expand
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<i32>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DisplayFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayFormatBuilder {
        DisplayFormatBuilder::create_empty()
    }
}
