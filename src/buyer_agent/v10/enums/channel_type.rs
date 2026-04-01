use serde::{Deserialize, Serialize};

/// The media channel type for programmatic advertising placements.
///
/// Defines the advertising medium where inventory is available (display, video, mobile, etc.).
/// All serialization uses snake_case format (e.g., `"ctv"` for `Ctv`, `"dooh"` for `Dooh`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    /// Display advertising (banners, rectangles, etc.)
    #[default]
    Display,

    /// Video advertising
    Video,

    /// Connected TV advertising
    Ctv,

    /// Mobile advertising
    Mobile,

    /// Audio advertising
    Audio,

    /// Digital Out-of-Home advertising
    Dooh,

    /// Native advertising
    Native,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            ChannelType::Display,
            ChannelType::Video,
            ChannelType::Ctv,
            ChannelType::Mobile,
            ChannelType::Audio,
            ChannelType::Dooh,
            ChannelType::Native,
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
        let json = "\"nonexistent_channel\"";
        let result: Result<ChannelType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid channel should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            ChannelType::Display,
            ChannelType::Video,
            ChannelType::Ctv,
            ChannelType::Mobile,
            ChannelType::Audio,
            ChannelType::Dooh,
            ChannelType::Native,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: ChannelType =
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
        let default = ChannelType::default();
        assert_eq!(default, ChannelType::Display, "Default should be Display");
    }
}
