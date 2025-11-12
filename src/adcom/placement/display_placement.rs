use crate::Extension;
use crate::adcom::placement::{DisplayFormat, EventSpec, NativeFormat};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DisplayPlacement Object (Section 4.2)
///
/// Placement details for display ad formats including banner and native.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DisplayPlacement<Ext: Extension = serde_json::Value> {
    /// Ad position on screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    /// Interstitial flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instl: Option<i32>,

    /// Top frame flag (1=iframe, 0=top frame)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topframe: Option<i32>,

    /// Array of iframe busters supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifrbust: Option<Vec<String>>,

    /// Click type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clktype: Option<i32>,

    /// AMPHTML creative support flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ampren: Option<i32>,

    /// Display placement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptype: Option<i32>,

    /// Display context type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,

    /// MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtypes permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<Vec<i32>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Array of display format specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayfmt: Option<Vec<DisplayFormat>>,

    /// Native format specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nativefmt: Option<Box<NativeFormat>>,

    /// Event tracking specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<EventSpec>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DisplayPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayPlacementBuilder {
        DisplayPlacementBuilder::create_empty()
    }
}
