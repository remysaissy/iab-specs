use serde::{Deserialize, Serialize};

/// Subtypes of contextual signals based on the environment and context of ad delivery.
///
/// Contextual signals derive from the content, temporal aspects, geographic location,
/// device environment, and session characteristics where the impression occurs.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContextualSignalSubtype {
    /// Signals derived from the editorial or content context of the placement.
    #[default]
    Content,
    /// Signals based on temporal characteristics such as time of day or session duration.
    Temporal,
    /// Signals related to geographic location and regional attributes.
    Geospatial,
    /// Signals from device environment including OS, browser, and hardware attributes.
    DeviceEnvironment,
    /// Signals from the current user session and session behavior.
    Session,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("content", ContextualSignalSubtype::Content),
            ("temporal", ContextualSignalSubtype::Temporal),
            ("geospatial", ContextualSignalSubtype::Geospatial),
            (
                "device_environment",
                ContextualSignalSubtype::DeviceEnvironment,
            ),
            ("session", ContextualSignalSubtype::Session),
        ];

        for (json_str, expected) in values {
            let result: ContextualSignalSubtype =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<ContextualSignalSubtype, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            ContextualSignalSubtype::Content,
            ContextualSignalSubtype::Temporal,
            ContextualSignalSubtype::Geospatial,
            ContextualSignalSubtype::DeviceEnvironment,
            ContextualSignalSubtype::Session,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ContextualSignalSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = ContextualSignalSubtype::default();
        assert_eq!(
            default,
            ContextualSignalSubtype::Content,
            "Default should be Content"
        );
    }

    #[test]
    fn test_integer_value_rejected() {
        // Spec: Agentic Audience v1.0 — enums are string-serialized, integers must be rejected
        let result: Result<ContextualSignalSubtype, _> = serde_json::from_str("42");
        assert!(result.is_err(), "Integer value should be rejected");
    }
}
