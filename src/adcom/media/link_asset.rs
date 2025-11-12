use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// LinkAsset Object (Section 3.6)
///
/// Destination link specification with click tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct LinkAsset<Ext: Extension = serde_json::Value> {
    /// Landing URL of the clickable link (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Fallback URL for deeplink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlfb: Option<String>,

    /// Descriptive text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trkr: Option<Vec<String>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl LinkAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> LinkAssetBuilder {
        LinkAssetBuilder::create_empty()
    }
}
