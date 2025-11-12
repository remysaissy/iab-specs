use crate::Extension;
use crate::adcom::placement::Companion;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// AudioPlacement Object (Section 4.11)
///
/// Placement details for audio ad formats.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AudioPlacement<Ext: Extension = serde_json::Value> {
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

    /// Feed type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<i32>,

    /// Volume normalization mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvol: Option<i32>,

    /// MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtypes permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<Vec<i32>>,

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

impl AudioPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AudioPlacementBuilder {
        AudioPlacementBuilder::create_empty()
    }
}
