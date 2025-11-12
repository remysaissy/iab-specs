use crate::Extension;
use crate::adcom::placement::Companion;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// VideoPlacement Object (Section 4.10)
///
/// Placement details for video ad formats.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct VideoPlacement<Ext: Extension = serde_json::Value> {
    /// Video placement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptype: Option<i32>,

    /// Ad position on screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    /// Start delay in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,

    /// Skippable flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,

    /// Skip offset in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipmin: Option<i32>,

    /// Skip button visible after this duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipafter: Option<i32>,

    /// Playback method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playmethod: Option<i32>,

    /// Playback cessation mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playend: Option<i32>,

    /// Click type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clktype: Option<i32>,

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

    /// Placement unit identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,

    /// Minimum duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mindur: Option<i32>,

    /// Maximum duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdur: Option<i32>,

    /// Maximum extended duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxext: Option<i32>,

    /// Minimum bit rate in Kbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,

    /// Maximum bit rate in Kbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,

    /// Delivery methods supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<i32>>,

    /// Maximum ad sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i32>,

    /// Video linearity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<i32>,

    /// Boxing allowed flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxing: Option<i32>,

    /// Array of companion ad objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp: Option<Vec<Companion>>,

    /// Array of companion types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comptype: Option<Vec<i32>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl VideoPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoPlacementBuilder {
        VideoPlacementBuilder::create_empty()
    }
}
