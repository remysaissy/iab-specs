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

    /// Buyer Agent 1.0 § ChannelType — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = ChannelType::Video;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, ChannelType::Video);
    }

    /// Buyer Agent 1.0 § ChannelType — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ChannelType::Display);
        set.insert(ChannelType::Video);
        set.insert(ChannelType::Ctv);
        set.insert(ChannelType::Mobile);
        set.insert(ChannelType::Audio);
        set.insert(ChannelType::Dooh);
        set.insert(ChannelType::Native);

        assert_eq!(set.len(), 7);
        assert!(set.contains(&ChannelType::Display));
        assert!(set.contains(&ChannelType::Dooh));
    }

    /// Buyer Agent 1.0 § ChannelType — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(ChannelType::Display, ChannelType::Video);
        assert_ne!(ChannelType::Video, ChannelType::Ctv);
        assert_ne!(ChannelType::Ctv, ChannelType::Mobile);
        assert_ne!(ChannelType::Mobile, ChannelType::Audio);
        assert_ne!(ChannelType::Audio, ChannelType::Dooh);
        assert_ne!(ChannelType::Dooh, ChannelType::Native);
    }

    /// Buyer Agent 1.0 § ChannelType — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Video\"", "\"ConnectedTv\""];

        for example in &pascal_case_examples {
            let result: Result<ChannelType, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Buyer Agent 1.0 § ChannelType — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (ChannelType::Display, "\"display\""),
            (ChannelType::Video, "\"video\""),
            (ChannelType::Ctv, "\"ctv\""),
            (ChannelType::Mobile, "\"mobile\""),
            (ChannelType::Audio, "\"audio\""),
            (ChannelType::Dooh, "\"dooh\""),
            (ChannelType::Native, "\"native\""),
        ];

        for (variant, expected_json) in &expected {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(
                &json, expected_json,
                "Mismatch for {:?}: got {}, expected {}",
                variant, json, expected_json
            );
        }
    }
}
