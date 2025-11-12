use crate::Extension;
use crate::adcom::context::BrandVersion;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// UserAgent Object (Section 7.5)
///
/// Structured user agent information per User-Agent Client Hints.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct UserAgent<Ext: Extension = serde_json::Value> {
    /// Browser marketing name array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<Vec<BrandVersion>>,

    /// Platform/OS name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<BrandVersion>>,

    /// Mobile device flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Platform architecture (e.g., "x86", "arm")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Platform bitness (e.g., "64")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,

    /// Device model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Source of user agent data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl UserAgent {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> UserAgentBuilder {
        UserAgentBuilder::create_empty()
    }
}
