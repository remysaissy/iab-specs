use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// BrandVersion helper struct for UserAgent
///
/// Brand and version information for user agent client hints.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct BrandVersion {
    /// Brand name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Version numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,
}

impl BrandVersion {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BrandVersionBuilder {
        BrandVersionBuilder::create_empty()
    }
}
