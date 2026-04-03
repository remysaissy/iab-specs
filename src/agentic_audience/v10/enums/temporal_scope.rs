use serde::{Deserialize, Serialize};

/// Temporal scope defining the time window for audience signal evaluation.
///
/// This enum specifies how long audience signals are valid and relevant for targeting decisions:
/// - `Persistent`: Signals remain valid indefinitely (e.g., customer status).
/// - `Session`: Signals are valid only within the current user session.
/// - `RealTime`: Signals are evaluated in real-time context (e.g., current behavior).
/// - `Retrospective`: Signals are based on historical data (e.g., past 30 days).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum TemporalScope {
    /// Signals remain valid indefinitely until explicitly invalidated or updated.
    #[default]
    Persistent,
    /// Signals are valid only within the current user session.
    Session,
    /// Signals are evaluated in real-time context at request time.
    RealTime,
    /// Signals are based on historical data from past time periods.
    Retrospective,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("persistent", TemporalScope::Persistent),
            ("session", TemporalScope::Session),
            ("real_time", TemporalScope::RealTime),
            ("retrospective", TemporalScope::Retrospective),
        ];

        for (json_str, expected) in values {
            let result: TemporalScope = serde_json::from_str(&format!("\"{}\"", json_str))
                .expect(&format!("Failed to deserialize: {}", json_str));
            assert_eq!(
                result, expected,
                "Failed for value: {} (expected {:?}, got {:?})",
                json_str, expected, result
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_temporal_scope\"";
        let result: Result<TemporalScope, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            TemporalScope::Persistent,
            TemporalScope::Session,
            TemporalScope::RealTime,
            TemporalScope::Retrospective,
        ];

        for original in values {
            let json = serde_json::to_string(&original)
                .expect(&format!("Failed to serialize: {:?}", original));
            let deserialized: TemporalScope =
                serde_json::from_str(&json).expect(&format!("Failed to deserialize: {}", json));
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = TemporalScope::default();
        assert_eq!(
            default,
            TemporalScope::Persistent,
            "Default should be Persistent"
        );
    }
}
