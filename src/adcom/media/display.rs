use crate::Extension;
use crate::adcom::media::{Banner, Event, Native};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Display Object (Section 3.2)
///
/// Details specific to display ads, supporting multiple creative formats
/// including banner images, native ads, and ad markup.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Display<Ext: Extension = serde_json::Value> {
    /// MIME type of the ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<i32>,

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

    /// Link to privacy policy URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priv_: Option<String>,

    /// Structured banner ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Box<Banner>>,

    /// Structured native ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Box<Native>>,

    /// Ad markup (e.g., HTML, AMPHTML)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Event trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<Event>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Display {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayBuilder {
        DisplayBuilder::create_empty()
    }
}
