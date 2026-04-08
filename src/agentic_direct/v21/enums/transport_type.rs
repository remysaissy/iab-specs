use serde::{Deserialize, Serialize};

/// Transport protocol type for A2A Protocol communication.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransportType {
    /// HTTP/HTTPS transport.
    #[default]
    Http,
    /// Server-Sent Events (SSE) transport.
    Sse,
    /// WebSocket transport.
    WebSocket,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("http", TransportType::Http),
            ("sse", TransportType::Sse),
            ("web_socket", TransportType::WebSocket),
        ];

        for (json_str, expected) in values {
            let result: TransportType = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<TransportType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            TransportType::Http,
            TransportType::Sse,
            TransportType::WebSocket,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: TransportType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = TransportType::default();
        assert_eq!(default, TransportType::Http, "Default should be Http");
    }

    #[test]
    fn test_case_sensitive_deserialization() {
        // Spec: Agentic Direct 2.1 — snake_case serialization is mandatory
        let invalid = ["\"Http\"", "\"SSE\""];
        for json in &invalid {
            assert!(
                serde_json::from_str::<TransportType>(json).is_err(),
                "{} should be rejected",
                json
            );
        }
    }
}
