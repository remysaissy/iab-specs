use crate::Extension;
/// OpenRTB 2.6 Refresh Objects
///
/// This module implements the Refresh and RefSettings objects for ad slot refresh configuration.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// RefSettings Object (Section 3.2.34)
///
/// Settings that control refresh behavior for continuously displayed ad slots.
/// Specifies parameters like refresh interval and maximum refresh count.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
/// ```
/// use iab_specs::openrtb::v26::RefSettings;
///
/// let settings = RefSettings::builder()
///     .reftype(Some(1))  // User-initiated refresh
///     .minint(Some(30))  // Minimum 30 seconds between refreshes
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct RefSettings<Ext: Extension = serde_json::Value> {
    /// Type of refresh
    /// 1 = User-initiated
    /// 2 = Automatic (time-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reftype: Option<i32>,

    /// Minimum interval between refreshes in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minint: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl RefSettings {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RefSettingsBuilder {
        RefSettingsBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_serializing_none() {
        let ref_settings = RefSettings::builder().build().unwrap();
        let json = serde_json::to_string(&ref_settings).unwrap();
        assert_eq!(json, "{}");
    }
}
