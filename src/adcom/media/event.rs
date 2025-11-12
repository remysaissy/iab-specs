use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Event Object (Section 3.11)
///
/// Tracks advertiser or buyer events for measurement purposes.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Event<Ext: Extension = serde_json::Value> {
    /// Event type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Tracking method (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<i32>,

    /// Array of tracking URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,

    /// Array of JavaScript trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jstrk: Option<Vec<String>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Event {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EventBuilder {
        EventBuilder::create_empty()
    }
}
