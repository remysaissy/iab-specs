use crate::Extension;
use crate::adcom::context::Producer;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Content Object
///
/// Details about the content within which an ad will be displayed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Content<Ext: Extension = serde_json::Value> {
    /// Unique content identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Episode number for episodic content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,

    /// Content title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Content series
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// Content season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    /// Artist credited with the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// Genre(s) of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// Album to which the content belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// International Standard Recording Code (ISRC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    /// URL of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Content categories using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Production quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<i32>,

    /// Content context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,

    /// Content rating (e.g., MPAA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<String>,

    /// User rating of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userrating: Option<String>,

    /// Media rating per IQG guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// 1 = content is live, 0 = not live
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestream: Option<i32>,

    /// 1 = src relationship is direct, 0 = indirect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srcrel: Option<i32>,

    /// Length of content in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Content language using ISO-639-1-alpha-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// 1 = content is embedded, 0 = not embedded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<i32>,

    /// Producer details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Box<Producer>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Content {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ContentBuilder {
        ContentBuilder::create_empty()
    }
}
