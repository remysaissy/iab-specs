use serde::{Deserialize, Serialize};

/// Trust level of an agent in the registry.
///
/// Agents progress through trust levels as they are verified and proven reliable.
/// All serialization uses snake_case format (e.g., `"registered"` for `Registered`).
///
/// ## Trust Progression
///
/// Unknown → Registered → Verified → Preferred
///
/// Any non-Unknown state can transition to Blocked (terminal).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    /// Agent trust level is unknown (not yet registered).
    #[default]
    Unknown,
    /// Agent has been registered in the registry.
    Registered,
    /// Agent has been verified by the registry.
    Verified,
    /// Agent has earned preferred status.
    Preferred,
    /// Agent has been blocked (terminal state).
    Blocked,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            (TrustLevel::Unknown, "unknown"),
            (TrustLevel::Registered, "registered"),
            (TrustLevel::Verified, "verified"),
            (TrustLevel::Preferred, "preferred"),
            (TrustLevel::Blocked, "blocked"),
        ];

        for (variant, expected) in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            let expected_json = format!("\"{}\"", expected);
            assert_eq!(
                serialized, expected_json,
                "Expected {:?} to serialize as {}, got {}",
                variant, expected_json, serialized
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent\"";
        let result: Result<TrustLevel, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid trust level should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            TrustLevel::Unknown,
            TrustLevel::Registered,
            TrustLevel::Verified,
            TrustLevel::Preferred,
            TrustLevel::Blocked,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: TrustLevel =
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
        let default = TrustLevel::default();
        assert_eq!(default, TrustLevel::Unknown, "Default should be Unknown");
    }
}
