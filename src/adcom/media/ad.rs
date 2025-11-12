use crate::Extension;
use crate::adcom::media::{Audio, Audit, Display, Video};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Ad Object (Section 3.1)
///
/// Root structure defining an advertising media instance with rendering instructions.
/// Must include exactly one media subtype object (Display, Video, or Audio).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Ad<Ext: Extension = serde_json::Value> {
    /// Creative identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Advertiser domain for buyer targeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adomain: Option<Vec<String>>,

    /// App bundle/package name for buyer targeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Vec<String>>,

    /// Preview image URL for creative approval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iurl: Option<String>,

    /// Content categories describing the creative using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Language using ISO-639-1-alpha-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Creative attributes array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<Vec<i32>>,

    /// HTTPS creative flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,

    /// Media rating per IQG guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrating: Option<i32>,

    /// Timestamp when ad was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<i64>,

    /// Timestamp of last modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<i64>,

    /// Display ad subtype
    /// Note: Display will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<Display>>,

    /// Video ad subtype
    /// Note: Video will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<Video>>,

    /// Audio ad subtype
    /// Note: Audio will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<Audio>>,

    /// Audit object for quality/safety review
    /// Note: Audit will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<Box<Audit>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Ad {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AdBuilder {
        AdBuilder::create_empty()
    }
}
