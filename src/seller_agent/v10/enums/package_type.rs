use serde::{Deserialize, Serialize};

/// The type of media package offered.
///
/// Package types categorize how media inventory is packaged for sale.
/// All serialization uses snake_case format (e.g., `"dynamic"` for `Dynamic`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum PackageType {
    /// Curated media package with handpicked inventory.
    #[default]
    Curated,

    /// Dynamic media package with programmatic inventory.
    Dynamic,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [PackageType::Curated, PackageType::Dynamic];

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
        let json = "\"nonexistent_type\"";
        let result: Result<PackageType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid type should fail deserialization");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [PackageType::Curated, PackageType::Dynamic];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: PackageType =
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
        let default = PackageType::default();
        assert_eq!(default, PackageType::Curated, "Default should be Curated");
    }
}
