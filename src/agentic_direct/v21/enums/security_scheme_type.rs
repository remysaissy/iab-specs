use serde::{Deserialize, Serialize};

/// Security scheme type for A2A Protocol authentication.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum SecuritySchemeType {
    /// OAuth 2.0 authentication.
    #[serde(rename = "oauth2")]
    OAuth2,
    /// API Key authentication.
    ApiKey,
    /// Bearer token authentication.
    #[default]
    Bearer,
    /// Mutual TLS (mTLS) authentication.
    Mtls,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("oauth2", SecuritySchemeType::OAuth2),
            ("api_key", SecuritySchemeType::ApiKey),
            ("bearer", SecuritySchemeType::Bearer),
            ("mtls", SecuritySchemeType::Mtls),
        ];

        for (json_str, expected) in values {
            let result: SecuritySchemeType =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<SecuritySchemeType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            SecuritySchemeType::OAuth2,
            SecuritySchemeType::ApiKey,
            SecuritySchemeType::Bearer,
            SecuritySchemeType::Mtls,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: SecuritySchemeType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = SecuritySchemeType::default();
        assert_eq!(
            default,
            SecuritySchemeType::Bearer,
            "Default should be Bearer"
        );
    }
}
