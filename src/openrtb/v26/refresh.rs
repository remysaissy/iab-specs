use super::RefSettings;
use crate::Extension;
/// OpenRTB 2.6 Refresh Objects
///
/// This module implements the Refresh and RefSettings objects for ad slot refresh configuration.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Refresh Object (Section 3.2.33)
///
/// Details about automatically refreshing ad slots. Contains an array of refresh
/// settings that define the behavior of ad slot refreshes.
///
/// This is commonly used for:
/// - Live streaming content with periodic ad refreshes
/// - Static DOOH displays with rotating ads
/// - Long-form content with intermittent ad updates
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
/// ```
/// use iab_specs::openrtb::v26::{Refresh, RefSettings};
///
/// let refresh = Refresh::builder()
///     .refsettings(Some(vec![
///         RefSettings::builder()
///             .reftype(Some(2))  // Automatic refresh
///             .minint(Some(60))  // Every 60 seconds
///             .build()
///             .unwrap()
///     ]))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Refresh<Ext: Extension = crate::DefaultExt> {
    /// Array of refresh setting objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refsettings: Option<Vec<RefSettings<Ext>>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Refresh {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RefreshBuilder {
        RefreshBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_with_settings() {
        // Spec: Section 3.2.33
        let refresh = Refresh::builder()
            .refsettings(Some(vec![
                RefSettings::builder()
                    .reftype(Some(1))
                    .minint(Some(60))
                    .build()
                    .unwrap(),
                RefSettings::builder()
                    .reftype(Some(2))
                    .minint(Some(120))
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        assert!(refresh.refsettings.is_some());
        assert_eq!(refresh.refsettings.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_refresh_serialization() {
        // Spec: Section 3.2.33
        let refresh = Refresh::builder()
            .refsettings(Some(vec![RefSettings::builder()
                .reftype(Some(2))
                .minint(Some(45))
                .build()
                .unwrap()]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&refresh).unwrap();
        let deserialized: Refresh = serde_json::from_str(&json).unwrap();
        assert_eq!(refresh, deserialized);
    }

    #[test]
    fn test_skip_serializing_none() {
        // Spec: Section 3.2.33
        let refresh = Refresh::builder().build().unwrap();
        let json = serde_json::to_string(&refresh).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_refresh_ext_field() {
        // Spec: Section 3.2.33
        let ext = serde_json::json!({"custom_field": "value", "priority": 5});
        let refresh = RefreshBuilder::<serde_json::Value>::default()
            .refsettings(Some(vec![RefSettings {
                reftype: Some(1),
                minint: Some(30),
                ext: None,
            }]))
            .ext(Some(Box::new(ext.clone())))
            .build()
            .unwrap();

        assert_eq!(*refresh.ext.as_ref().unwrap().as_ref(), ext);

        let json = serde_json::to_string(&refresh).unwrap();
        let deserialized: Refresh<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(refresh, deserialized);
    }

    #[test]
    fn test_refresh_deserialization_from_json() {
        // Spec: Section 3.2.33
        let json = r#"{"refsettings":[{"reftype":1,"minint":30},{"reftype":2,"minint":60}]}"#;
        let refresh: Refresh = serde_json::from_str(json).unwrap();
        assert!(refresh.refsettings.is_some());
        let settings = refresh.refsettings.as_ref().unwrap();
        assert_eq!(settings.len(), 2);
        assert_eq!(settings[0].reftype, Some(1));
        assert_eq!(settings[0].minint, Some(30));
        assert_eq!(settings[1].reftype, Some(2));
        assert_eq!(settings[1].minint, Some(60));
    }

    #[test]
    fn test_refresh_empty_settings() {
        // Spec: Section 3.2.33
        let refresh = Refresh::builder()
            .refsettings(Some(vec![]))
            .build()
            .unwrap();

        assert!(refresh.refsettings.is_some());
        assert_eq!(refresh.refsettings.as_ref().unwrap().len(), 0);

        let json = serde_json::to_string(&refresh).unwrap();
        assert!(json.contains("\"refsettings\":[]"));
    }

    #[test]
    fn test_refresh_roundtrip_all_fields() {
        // Spec: Section 3.2.33
        let refresh = Refresh::builder()
            .refsettings(Some(vec![
                RefSettings::builder()
                    .reftype(Some(1))
                    .minint(Some(30))
                    .build()
                    .unwrap(),
                RefSettings::builder()
                    .reftype(Some(2))
                    .minint(Some(90))
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&refresh).unwrap();
        let deserialized: Refresh = serde_json::from_str(&json).unwrap();

        assert_eq!(refresh.refsettings, deserialized.refsettings);
        assert_eq!(refresh, deserialized);
    }
}
