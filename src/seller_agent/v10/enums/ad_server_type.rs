use serde::{Deserialize, Serialize};

/// The type of ad server platform used for campaign delivery and synchronization.
///
/// This enum defines the supported ad server integrations for syncing campaign data.
/// All serialization uses snake_case format (e.g., `"google_ad_manager"` for `GoogleAdManager`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum AdServerType {
    /// Google Ad Manager platform.
    #[default]
    GoogleAdManager,

    /// FreeWheel ad server platform.
    FreeWheel,

    /// CSV file-based ad server format.
    Csv,

    /// Custom or proprietary ad server platform.
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            AdServerType::GoogleAdManager,
            AdServerType::FreeWheel,
            AdServerType::Csv,
            AdServerType::Custom,
        ];

        for variant in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            assert!(
                serialized.starts_with('"') && serialized.ends_with('"'),
                "Serialized value {} should be a JSON string",
                serialized
            );
            let unquoted = &serialized[1..serialized.len() - 1];
            assert!(
                unquoted.chars().all(|c| c.is_lowercase() || c == '_'),
                "Serialized value {} should be snake_case",
                unquoted
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_server\"";
        let result: Result<AdServerType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid server type should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            AdServerType::GoogleAdManager,
            AdServerType::FreeWheel,
            AdServerType::Csv,
            AdServerType::Custom,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: AdServerType =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(
                original, &deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = AdServerType::default();
        assert_eq!(
            default,
            AdServerType::GoogleAdManager,
            "Default should be GoogleAdManager"
        );
    }
}
