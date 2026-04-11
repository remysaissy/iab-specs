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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
/// ```
/// use iab_specs_openrtb::v26::RefSettings;
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
pub struct RefSettings<Ext: Extension = crate::DefaultExt> {
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
        // Spec: Section 3.2.34
        let ref_settings = RefSettings::builder().build().unwrap();
        let json = serde_json::to_string(&ref_settings).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_ref_settings_serialization_roundtrip() {
        // Spec: Section 3.2.34
        let settings = RefSettings::builder()
            .reftype(Some(1))
            .minint(Some(30))
            .build()
            .unwrap();

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: RefSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_ref_settings_ext_field() {
        // Spec: Section 3.2.34
        let ext = serde_json::json!({"custom_field": "value", "priority": 5});
        let settings = RefSettingsBuilder::<serde_json::Value>::default()
            .reftype(Some(1))
            .ext(Some(Box::new(ext.clone())))
            .build()
            .unwrap();

        assert_eq!(*settings.ext.as_ref().unwrap().as_ref(), ext);

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: RefSettings<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_ref_settings_deserialization_from_json() {
        // Spec: Section 3.2.34
        let json = r#"{"reftype":2,"minint":60}"#;
        let settings: RefSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.reftype, Some(2));
        assert_eq!(settings.minint, Some(60));
    }

    #[test]
    fn test_ref_settings_user_initiated_refresh() {
        // Spec: Section 3.2.34
        let settings = RefSettings::builder()
            .reftype(Some(1))
            .minint(Some(30))
            .build()
            .unwrap();

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"reftype\":1"));
    }

    #[test]
    fn test_ref_settings_auto_refresh() {
        // Spec: Section 3.2.34
        let settings = RefSettings::builder()
            .reftype(Some(2))
            .minint(Some(60))
            .build()
            .unwrap();

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"reftype\":2"));
    }

    #[test]
    fn test_ref_settings_roundtrip_all_fields() {
        // Spec: Section 3.2.34
        let settings = RefSettings::builder()
            .reftype(Some(2))
            .minint(Some(45))
            .build()
            .unwrap();

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: RefSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings.reftype, deserialized.reftype);
        assert_eq!(settings.minint, deserialized.minint);
        assert_eq!(settings, deserialized);
    }
}
