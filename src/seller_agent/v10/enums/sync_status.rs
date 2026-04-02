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
}
