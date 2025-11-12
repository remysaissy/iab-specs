use crate::Extension;
use crate::adcom::placement::{AudioPlacement, DisplayPlacement, VideoPlacement};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Placement Object (Section 4.1)
///
/// Abstract base for placement specifications defining ad slot characteristics.
/// Subtype objects include DisplayPlacement, VideoPlacement, and AudioPlacement.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Placement<Ext: Extension = serde_json::Value> {
    /// Placement identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Placement name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Placement description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// HTTPS only flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,

    /// Array of blocked advertiser categories using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,

    /// Taxonomy used for bcat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Array of blocked advertiser domains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddr: Option<Vec<String>>,

    /// Array of blocked creative attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,

    /// Workflow language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<String>>,

    /// Display placement details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<DisplayPlacement>>,

    /// Video placement details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<VideoPlacement>>,

    /// Audio placement details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<AudioPlacement>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Placement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PlacementBuilder {
        PlacementBuilder::create_empty()
    }
}
