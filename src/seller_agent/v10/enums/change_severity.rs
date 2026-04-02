use serde::{Deserialize, Serialize};

/// The severity level of a change request.
///
/// This enum categorizes how impactful a change is to the campaign or order.
/// All serialization uses snake_case format (e.g., `"minor"` for `Minor`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChangeSeverity {
    /// Low impact change with minimal consequences.
    #[default]
    Minor,

    /// Significant change affecting campaign performance or delivery.
    Material,

    /// Critical change that may affect campaign validity or compliance.
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            ChangeSeverity::Minor,
            ChangeSeverity::Material,
            ChangeSeverity::Critical,
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
        let json = "\"nonexistent_severity\"";
        let result: Result<ChangeSeverity, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid severity should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            ChangeSeverity::Minor,
            ChangeSeverity::Material,
            ChangeSeverity::Critical,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: ChangeSeverity =
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
        let default = ChangeSeverity::default();
        assert_eq!(default, ChangeSeverity::Minor, "Default should be Minor");
    }
}
