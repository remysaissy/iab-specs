use super::id::JsonRpcId;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A JSON-RPC 2.0 request message.
///
/// # Example
///
/// ```
/// use iab_specs_agentic_direct::v21::jsonrpc::{JsonRpcRequest, JsonRpcId};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let request = JsonRpcRequest::builder()
///     .jsonrpc("2.0")
///     .method("agent/negotiate")
///     .id(Some(JsonRpcId::String("req-1".into())))
///     .params(Some(serde_json::json!({"deal_id": "d-100"})))
///     .build()?;
///
/// assert_eq!(request.method, "agent/negotiate");
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct JsonRpcRequest<Ext: Extension = crate::DefaultExt> {
    /// Protocol version — always `"2.0"`.
    #[builder(setter(into))]
    pub jsonrpc: String,

    /// The method to invoke.
    #[builder(setter(into))]
    pub method: String,

    /// Structured parameter values for the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub params: Option<serde_json::Value>,

    /// Request identifier — correlates request to response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<JsonRpcId>,

    /// Extension object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl JsonRpcRequest {
    pub fn builder() -> JsonRpcRequestBuilder {
        JsonRpcRequestBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let req = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("test/method")
            .id(Some(JsonRpcId::String("r1".into())))
            .build()
            .unwrap();

        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "test/method");
        assert_eq!(req.id, Some(JsonRpcId::String("r1".into())));
        assert!(req.params.is_none());
        assert!(req.ext.is_none());
    }

    #[test]
    fn test_request_with_params() {
        let req = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("agent/bid")
            .params(Some(serde_json::json!({"price": 1.5})))
            .id(Some(JsonRpcId::Number(1)))
            .build()
            .unwrap();

        assert!(req.params.is_some());
    }

    #[test]
    fn test_request_roundtrip_string_id() {
        let req = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("test")
            .id(Some(JsonRpcId::String("abc".into())))
            .params(Some(serde_json::json!({"key": "value"})))
            .build()
            .unwrap();

        let json = serde_json::to_string(&req).unwrap();
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, parsed);
    }

    #[test]
    fn test_request_roundtrip_numeric_id() {
        let req = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("test")
            .id(Some(JsonRpcId::Number(42)))
            .build()
            .unwrap();

        let json = serde_json::to_string(&req).unwrap();
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, parsed);
    }

    #[test]
    fn test_request_roundtrip_no_id() {
        let req = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("notify/something")
            .build()
            .unwrap();

        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("\"id\""));
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, parsed);
    }

    #[test]
    fn test_request_serialization_skips_none_fields() {
        let req = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("m")
            .build()
            .unwrap();

        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("\"params\""));
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"ext\""));
    }

    #[test]
    fn test_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"test","id":"req-5","params":{"a":1}}"#;
        let req: JsonRpcRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "test");
        assert_eq!(req.id, Some(JsonRpcId::String("req-5".into())));
        assert!(req.params.is_some());
    }

    #[test]
    fn test_request_default() {
        let req = JsonRpcRequest::builder().build().unwrap();
        assert_eq!(req.jsonrpc, String::new());
        assert_eq!(req.method, String::new());
        assert!(req.id.is_none());
        assert!(req.params.is_none());
    }
}
