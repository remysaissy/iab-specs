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
/// # Example
/// ```
/// use iab_specs::openrtb::v26::RefSettings;
///
/// let settings = RefSettings {
///     reftype: Some(1),  // User-initiated refresh
///     minint: Some(30),  // Minimum 30 seconds between refreshes
///     ext: None,
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct RefSettings {
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
    pub ext: Option<serde_json::Value>,
}

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
/// # Example
/// ```
/// use iab_specs::openrtb::v26::{Refresh, RefSettings};
///
/// let refresh = Refresh {
///     refsettings: Some(vec![
///         RefSettings {
///             reftype: Some(2),  // Automatic refresh
///             minint: Some(60),  // Every 60 seconds
///             ext: None,
///         }
///     ]),
///     ext: None,
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Refresh {
    /// Array of refresh setting objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refsettings: Option<Vec<RefSettings>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refsettings_builder() {
        let settings = RefSettingsBuilder::default()
            .reftype(Some(2))
            .minint(Some(30))
            .build()
            .unwrap();

        assert_eq!(settings.reftype, Some(2));
        assert_eq!(settings.minint, Some(30));
    }

    #[test]
    fn test_refresh_with_settings() {
        let refresh = RefreshBuilder::default()
            .refsettings(Some(vec![
                RefSettings {
                    reftype: Some(1),
                    minint: Some(60),
                    ext: None,
                },
                RefSettings {
                    reftype: Some(2),
                    minint: Some(120),
                    ext: None,
                },
            ]))
            .build()
            .unwrap();

        assert!(refresh.refsettings.is_some());
        assert_eq!(refresh.refsettings.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_refresh_serialization() {
        let refresh = Refresh {
            refsettings: Some(vec![RefSettings {
                reftype: Some(2),
                minint: Some(45),
                ext: None,
            }]),
            ext: None,
        };

        let json = serde_json::to_string(&refresh).unwrap();
        let deserialized: Refresh = serde_json::from_str(&json).unwrap();
        assert_eq!(refresh, deserialized);
    }

    #[test]
    fn test_skip_serializing_none() {
        let refresh = Refresh::default();
        let json = serde_json::to_string(&refresh).unwrap();
        assert_eq!(json, "{}");
    }
}
