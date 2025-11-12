use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Segment Object (Section 7.9)
///
/// Specific data segment about a user.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Segment<Ext: Extension = serde_json::Value> {
    /// Segment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Segment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Segment value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Segment {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SegmentBuilder {
        SegmentBuilder::create_empty()
    }
}
