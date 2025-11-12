use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// EventSpec Object (Section 4.9)
///
/// Event tracking specification for placement-level tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EventSpec<Ext: Extension = serde_json::Value> {
    /// Event type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Array of tracking methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Vec<i32>>,

    /// Array of API frameworks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Array of JavaScript tracker URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jstrk: Option<Vec<String>>,

    /// Array of tracking URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl EventSpec {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EventSpecBuilder {
        EventSpecBuilder::create_empty()
    }
}
