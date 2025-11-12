use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Audit Object (Section 3.14)
///
/// Quality and safety review status for creative content.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Audit<Ext: Extension = serde_json::Value> {
    /// Audit status code (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

    /// Correction flag (1=corrected, 0=original)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corr: Option<Vec<String>>,

    /// Organization conducting the audit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<i64>,

    /// Timestamp of last audit modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<i64>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Audit {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AuditBuilder {
        AuditBuilder::create_empty()
    }
}
