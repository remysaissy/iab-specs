use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A JSON-RPC 2.0 notification message (a request with no `id`).
///
/// Notifications are fire-and-forget: the server MUST NOT reply.
///
/// # Example
///
/// ```
/// use iab_specs::agentic_direct::v21::jsonrpc::JsonRpcNotification;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let notif = JsonRpcNotification::builder()
///     .jsonrpc("2.0")
///     .method("agent/statusUpdate")
///     .params(Some(serde_json::json!({"status": "ready"})))
///     .build()?;
///
/// let json = serde_json::to_string(&notif)?;
/// assert!(!json.contains("\"id\""));
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct JsonRpcNotification<Ext: Extension = crate::DefaultExt> {
    /// Protocol version — always `"2.0"`.
    #[builder(setter(into))]
    pub jsonrpc: String,

    /// The notification method name.
    #[builder(setter(into))]
    pub method: String,

    /// Structured parameter values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub params: Option<serde_json::Value>,

    /// Extension object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl JsonRpcNotification {
    pub fn builder() -> JsonRpcNotificationBuilder {
        JsonRpcNotificationBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notif = JsonRpcNotification::builder()
            .jsonrpc("2.0")
            .method("event/update")
            .params(Some(serde_json::json!({"key": "val"})))
            .build()
            .unwrap();

        assert_eq!(notif.jsonrpc, "2.0");
        assert_eq!(notif.method, "event/update");
        assert!(notif.params.is_some());
        assert!(notif.ext.is_none());
    }

    #[test]
    fn test_notification_has_no_id_in_output() {
        let notif = JsonRpcNotification::builder()
            .jsonrpc("2.0")
            .method("ping")
            .build()
            .unwrap();

        let json = serde_json::to_string(&notif).unwrap();
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_notification_roundtrip() {
        let notif = JsonRpcNotification::builder()
            .jsonrpc("2.0")
            .method("notify/test")
            .params(Some(serde_json::json!([1, 2, 3])))
            .build()
            .unwrap();

        let json = serde_json::to_string(&notif).unwrap();
        let parsed: JsonRpcNotification = serde_json::from_str(&json).unwrap();
        assert_eq!(notif, parsed);
    }

    #[test]
    fn test_notification_serialization_skips_none() {
        let notif = JsonRpcNotification::builder()
            .jsonrpc("2.0")
            .method("m")
            .build()
            .unwrap();

        let json = serde_json::to_string(&notif).unwrap();
        assert!(!json.contains("\"params\""));
        assert!(!json.contains("\"ext\""));
    }

    #[test]
    fn test_notification_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"event/fire","params":{"x":1}}"#;
        let notif: JsonRpcNotification = serde_json::from_str(json).unwrap();

        assert_eq!(notif.jsonrpc, "2.0");
        assert_eq!(notif.method, "event/fire");
        assert!(notif.params.is_some());
    }

    #[test]
    fn test_notification_default() {
        let notif = JsonRpcNotification::builder().build().unwrap();
        assert_eq!(notif.jsonrpc, String::new());
        assert_eq!(notif.method, String::new());
        assert!(notif.params.is_none());
    }
}
