use crate::Extension;
use crate::adcom::placement::DisplayPlacement;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Companion Object (Section 4.12)
///
/// Companion ad specification for video/audio ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Companion<Ext: Extension = serde_json::Value> {
    /// Companion ad identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Companion type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Display placement for companion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<DisplayPlacement>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Companion {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CompanionBuilder {
        CompanionBuilder::create_empty()
    }
}
