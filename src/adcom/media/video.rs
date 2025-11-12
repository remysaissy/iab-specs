use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Video Object (Section 3.12)
///
/// Details specific to video ads including format, duration, and delivery.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Video<Ext: Extension = serde_json::Value> {
    /// MIME types supported (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<i32>>,

    /// Creative subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<i32>,

    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<i32>,

    /// Ad markup (VAST document)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Video protocols
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Bit rate in Kbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,

    /// Timestamp when creative was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<i64>,

    /// Timestamp of last modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<i64>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Video {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoBuilder {
        VideoBuilder::create_empty()
    }
}
