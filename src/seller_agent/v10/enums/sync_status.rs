use serde::{Deserialize, Serialize};

/// The current status of a sync operation between seller and ad server.
///
/// This enum tracks the progression of data synchronization to external platforms.
/// All serialization uses snake_case format (e.g., `"pending"` for `Pending`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    /// Sync operation is queued and awaiting execution.
    #[default]
    Pending,

    /// Sync operation is currently in progress.
    Syncing,

    /// Sync operation has completed successfully.
    Synced,

    /// Sync operation failed and did not complete.
    Failed,

    /// Sync data is outdated and needs refresh.
    Stale,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            SyncStatus::Pending,
            SyncStatus::Syncing,
            SyncStatus::Synced,
            SyncStatus::Failed,
            SyncStatus::Stale,
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
        let result: Result<SyncStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            SyncStatus::Pending,
            SyncStatus::Syncing,
            SyncStatus::Synced,
            SyncStatus::Failed,
            SyncStatus::Stale,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: SyncStatus =
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
        let default = SyncStatus::default();
        assert_eq!(default, SyncStatus::Pending, "Default should be Pending");
    }

    /// Seller Agent 1.0 § SyncStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = SyncStatus::Pending;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, SyncStatus::Pending);
    }

    /// Seller Agent 1.0 § SyncStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(SyncStatus::Pending);
        set.insert(SyncStatus::Syncing);
        set.insert(SyncStatus::Synced);
        set.insert(SyncStatus::Failed);
        set.insert(SyncStatus::Stale);

        assert_eq!(set.len(), 5);
        assert!(set.contains(&SyncStatus::Pending));
        assert!(set.contains(&SyncStatus::Stale));
    }

    /// Seller Agent 1.0 § SyncStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(SyncStatus::Pending, SyncStatus::Syncing);
        assert_ne!(SyncStatus::Syncing, SyncStatus::Synced);
        assert_ne!(SyncStatus::Synced, SyncStatus::Failed);
        assert_ne!(SyncStatus::Failed, SyncStatus::Stale);
    }

    /// Seller Agent 1.0 § SyncStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Pending\"", "\"Syncing\""];

        for example in &pascal_case_examples {
            let result: Result<SyncStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § SyncStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (SyncStatus::Pending, "\"pending\""),
            (SyncStatus::Syncing, "\"syncing\""),
            (SyncStatus::Synced, "\"synced\""),
            (SyncStatus::Failed, "\"failed\""),
            (SyncStatus::Stale, "\"stale\""),
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
