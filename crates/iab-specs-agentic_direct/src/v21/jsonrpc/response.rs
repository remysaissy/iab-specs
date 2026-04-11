use super::error::JsonRpcError;
use super::id::JsonRpcId;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A JSON-RPC 2.0 response message.
///
/// Contains either a `result` (success) or an `error` (failure), but not both.
/// The `id` field is always present, matching the request it responds to.
///
/// # Example
///
/// ```
/// use iab_specs_agentic_direct::v21::jsonrpc::{JsonRpcResponse, JsonRpcId};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let response = JsonRpcResponse::builder()
///     .jsonrpc("2.0")
///     .id(JsonRpcId::String("req-1".into()))
///     .result(Some(serde_json::json!({"status": "ok"})))
///     .build()?;
///
/// assert!(response.result.is_some());
/// assert!(response.error.is_none());
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct JsonRpcResponse<Ext: Extension = crate::DefaultExt> {
    /// Protocol version — always `"2.0"`.
    #[builder(setter(into))]
    pub jsonrpc: String,

    /// The result of a successful request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub result: Option<serde_json::Value>,

    /// The error object on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub error: Option<JsonRpcError<Ext>>,

    /// Request identifier this response corresponds to.
    pub id: JsonRpcId,

    /// Extension object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl JsonRpcResponse {
    pub fn builder() -> JsonRpcResponseBuilder {
        JsonRpcResponseBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v21::jsonrpc::error::{INTERNAL_ERROR, METHOD_NOT_FOUND};

    #[test]
    fn test_success_response_creation() {
        let resp = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(JsonRpcId::String("r1".into()))
            .result(Some(serde_json::json!({"accepted": true})))
            .build()
            .unwrap();

        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, JsonRpcId::String("r1".into()));
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_error_response_creation() {
        let resp = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(JsonRpcId::Number(7))
            .error(Some(
                JsonRpcError::builder()
                    .code(METHOD_NOT_FOUND)
                    .message("Method not found")
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        assert_eq!(resp.error.as_ref().unwrap().code, -32601);
    }

    #[test]
    fn test_success_response_roundtrip() {
        let resp = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(JsonRpcId::String("abc".into()))
            .result(Some(serde_json::json!(42)))
            .build()
            .unwrap();

        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, parsed);
    }

    #[test]
    fn test_error_response_roundtrip() {
        let resp = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(JsonRpcId::Null)
            .error(Some(
                JsonRpcError::builder()
                    .code(INTERNAL_ERROR)
                    .message("Internal error")
                    .data(Some(serde_json::json!({"trace": "stack"})))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, parsed);
    }

    #[test]
    fn test_response_serialization_skips_none() {
        let resp = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(JsonRpcId::Number(1))
            .result(Some(serde_json::json!("ok")))
            .build()
            .unwrap();

        let json = serde_json::to_string(&resp).unwrap();
        assert!(!json.contains("\"error\""));
        assert!(!json.contains("\"ext\""));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","result":{"val":1},"id":99}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, JsonRpcId::Number(99));
        assert!(resp.result.is_some());
    }

    #[test]
    fn test_response_default() {
        let resp = JsonRpcResponse::builder().build().unwrap();
        assert_eq!(resp.jsonrpc, String::new());
        assert_eq!(resp.id, JsonRpcId::Null);
        assert!(resp.result.is_none());
        assert!(resp.error.is_none());
    }
}
