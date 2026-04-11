use serde::{Deserialize, Serialize};

/// Subtypes of identity signals used for user identification and verification.
///
/// Identity signals are directly associated with user identification and can be derived
/// from various sources including personally identifiable information, behavioral patterns,
/// demographic data, and social graph relationships.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum IdentitySignalSubtype {
    /// Signals derived from personally identifiable information (PII).
    #[default]
    PiiDerived,
    /// Signals based on observed user behavior patterns and interactions.
    Behavioral,
    /// Signals related to demographic attributes of the user.
    Demographic,
    /// Signals derived from social graph relationships and connections.
    GraphBased,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("pii_derived", IdentitySignalSubtype::PiiDerived),
            ("behavioral", IdentitySignalSubtype::Behavioral),
            ("demographic", IdentitySignalSubtype::Demographic),
            ("graph_based", IdentitySignalSubtype::GraphBased),
        ];

        for (json_str, expected) in values {
            let result: IdentitySignalSubtype =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<IdentitySignalSubtype, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            IdentitySignalSubtype::PiiDerived,
            IdentitySignalSubtype::Behavioral,
            IdentitySignalSubtype::Demographic,
            IdentitySignalSubtype::GraphBased,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: IdentitySignalSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = IdentitySignalSubtype::default();
        assert_eq!(
            default,
            IdentitySignalSubtype::PiiDerived,
            "Default should be PiiDerived"
        );
    }

    #[test]
    fn test_integer_value_rejected() {
        // Spec: Agentic Audience v1.0 — enums are string-serialized, integers must be rejected
        let result: Result<IdentitySignalSubtype, _> = serde_json::from_str("42");
        assert!(result.is_err(), "Integer value should be rejected");
    }
}
