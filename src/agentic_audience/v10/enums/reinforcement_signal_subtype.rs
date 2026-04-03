use serde::{Deserialize, Serialize};

/// Subtypes of reinforcement signals derived from user action history and feedback.
///
/// Reinforcement signals capture the outcomes and responses to past advertising exposure,
/// including engagement metrics, conversion events, attribution data, and explicit user feedback.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReinforcementSignalSubtype {
    /// Signals from user engagement with ads and content interactions.
    #[default]
    Engagement,
    /// Signals derived from conversion events and purchase behaviors.
    Conversion,
    /// Signals from multi-touch attribution and conversion path tracking.
    Attribution,
    /// Signals from explicit user feedback and preference signals.
    Feedback,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("engagement", ReinforcementSignalSubtype::Engagement),
            ("conversion", ReinforcementSignalSubtype::Conversion),
            ("attribution", ReinforcementSignalSubtype::Attribution),
            ("feedback", ReinforcementSignalSubtype::Feedback),
        ];

        for (json_str, expected) in values {
            let result: ReinforcementSignalSubtype =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<ReinforcementSignalSubtype, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            ReinforcementSignalSubtype::Engagement,
            ReinforcementSignalSubtype::Conversion,
            ReinforcementSignalSubtype::Attribution,
            ReinforcementSignalSubtype::Feedback,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ReinforcementSignalSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = ReinforcementSignalSubtype::default();
        assert_eq!(
            default,
            ReinforcementSignalSubtype::Engagement,
            "Default should be Engagement"
        );
    }
}
