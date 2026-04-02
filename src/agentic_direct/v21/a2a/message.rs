use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(rename_all = "camelCase")]
pub struct A2AMessage {
    #[builder(setter(into))]
    pub role: String,

    #[builder(setter(into))]
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub timestamp: Option<String>,
}

impl A2AMessage {
    pub fn builder() -> A2AMessageBuilder {
        A2AMessageBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2a_message_creation() {
        let message = A2AMessage::builder()
            .role("user")
            .content("Hello")
            .timestamp("2025-03-31T12:00:00Z")
            .build()
            .unwrap();

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello");
        assert_eq!(message.timestamp, Some("2025-03-31T12:00:00Z".to_string()));
    }

    #[test]
    fn test_a2a_message_serialization() {
        let message = A2AMessage::builder()
            .role("agent")
            .content("Processing request")
            .timestamp("2025-03-31T12:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("\"role\":\"agent\""));
        assert!(json.contains("\"content\":\"Processing request\""));
        assert!(json.contains("\"timestamp\":\"2025-03-31T12:00:00Z\""));
        assert!(!json.contains("time_stamp"));
    }

    #[test]
    fn test_a2a_message_deserialization() {
        let json =
            r#"{"role":"user","content":"Need an update","timestamp":"2025-03-31T13:00:00Z"}"#;
        let message: A2AMessage = serde_json::from_str(json).unwrap();

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Need an update");
        assert_eq!(message.timestamp, Some("2025-03-31T13:00:00Z".to_string()));
    }

    #[test]
    fn test_a2a_message_roundtrip() {
        let message = A2AMessage::builder()
            .role("agent")
            .content("Done")
            .timestamp("2025-03-31T14:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&message).unwrap();
        let parsed: A2AMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(message, parsed);
    }

    #[test]
    fn test_a2a_message_default() {
        let message = A2AMessage::builder().build().unwrap();

        assert_eq!(message.role, String::new());
        assert_eq!(message.content, String::new());
        assert!(message.timestamp.is_none());
    }

    #[test]
    fn test_a2a_message_malformed_json_rejected() {
        let json = r#"{"role": 42, "content": false}"#;
        let result: Result<A2AMessage, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Wrong field types should fail");

        let json = r#"{not valid"#;
        let result: Result<A2AMessage, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
