use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Regs Object (Section 7.10)
///
/// Regulatory conditions in effect.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Regs<Ext: Extension = serde_json::Value> {
    /// COPPA compliance flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coppa: Option<i32>,

    /// GDPR applicability (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Regs {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RegsBuilder {
        RegsBuilder::create_empty()
    }
}
