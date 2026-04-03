use serde::{Deserialize, Serialize};

/// Verification status of an agent in the registry.
///
/// Tracks the lifecycle of agent identity verification.
///
/// > ⚠️ **Speculative**: Based on original research — not confirmed in current public repos.
/// > This enum may change or be removed in future versions.
///
/// All serialization uses snake_case format (e.g., `"unverified"` for `Unverified`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    /// Agent has not been verified.
    #[default]
    Unverified,
    /// Verification is in progress.
    Pending,
    /// Agent has been verified.
    Verified,
    /// Verification has failed.
    Failed,
    /// Verification has expired.
    Expired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            VerificationStatus::Unverified,
            VerificationStatus::Pending,
            VerificationStatus::Verified,
            VerificationStatus::Failed,
            VerificationStatus::Expired,
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
        let json = "\"nonexistent_status\"";
        let result: Result<VerificationStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            VerificationStatus::Unverified,
            VerificationStatus::Pending,
            VerificationStatus::Verified,
            VerificationStatus::Failed,
            VerificationStatus::Expired,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: VerificationStatus =
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
        let default = VerificationStatus::default();
        assert_eq!(
            default,
            VerificationStatus::Unverified,
            "Default should be Unverified"
        );
    }
}
