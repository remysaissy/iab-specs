use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Standard JSON-RPC 2.0 error code: parse error (-32700).
pub const PARSE_ERROR: i32 = -32700;
/// Standard JSON-RPC 2.0 error code: invalid request (-32600).
pub const INVALID_REQUEST: i32 = -32600;
/// Standard JSON-RPC 2.0 error code: method not found (-32601).
pub const METHOD_NOT_FOUND: i32 = -32601;
/// Standard JSON-RPC 2.0 error code: invalid params (-32602).
pub const INVALID_PARAMS: i32 = -32602;
/// Standard JSON-RPC 2.0 error code: internal error (-32603).
pub const INTERNAL_ERROR: i32 = -32603;

/// A JSON-RPC 2.0 error object returned in error responses.
///
/// # Example
///
/// ```
/// use iab_specs::agentic_direct::v21::jsonrpc::{JsonRpcError, METHOD_NOT_FOUND};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let error = JsonRpcError::builder()
///     .code(METHOD_NOT_FOUND)
///     .message("Method not found")
///     .build()?;
///
/// assert_eq!(error.code, -32601);
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct JsonRpcError<Ext: Extension = crate::DefaultExt> {
    /// The error code (integer).
    pub code: i32,

    /// A short description of the error.
    #[builder(setter(into))]
    pub message: String,

    /// Additional data about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<serde_json::Value>,

    /// Extension object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl JsonRpcError {
    pub fn builder() -> JsonRpcErrorBuilder {
        JsonRpcErrorBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = JsonRpcError::builder()
            .code(METHOD_NOT_FOUND)
            .message("Method not found")
            .build()
            .unwrap();

        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
        assert!(error.data.is_none());
        assert!(error.ext.is_none());
    }

    #[test]
    fn test_error_with_data() {
        let error = JsonRpcError::builder()
            .code(INVALID_PARAMS)
            .message("Invalid params")
            .data(Some(serde_json::json!({"field": "name"})))
            .build()
            .unwrap();

        assert_eq!(error.code, -32602);
        assert!(error.data.is_some());
    }

    #[test]
    fn test_error_serialization() {
        let error = JsonRpcError::builder()
            .code(PARSE_ERROR)
            .message("Parse error")
            .build()
            .unwrap();

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("\"code\":-32700"));
        assert!(json.contains("\"message\":\"Parse error\""));
        assert!(!json.contains("\"data\""));
    }

    #[test]
    fn test_error_deserialization() {
        let json = r#"{"code":-32603,"message":"Internal error"}"#;
        let error: JsonRpcError = serde_json::from_str(json).unwrap();

        assert_eq!(error.code, INTERNAL_ERROR);
        assert_eq!(error.message, "Internal error");
        assert!(error.data.is_none());
    }

    #[test]
    fn test_error_roundtrip() {
        let error = JsonRpcError::builder()
            .code(INVALID_REQUEST)
            .message("Invalid request")
            .data(Some(serde_json::json!("details")))
            .build()
            .unwrap();

        let json = serde_json::to_string(&error).unwrap();
        let parsed: JsonRpcError = serde_json::from_str(&json).unwrap();
        assert_eq!(error, parsed);
    }

    #[test]
    fn test_error_code_constants() {
        assert_eq!(PARSE_ERROR, -32700);
        assert_eq!(INVALID_REQUEST, -32600);
        assert_eq!(METHOD_NOT_FOUND, -32601);
        assert_eq!(INVALID_PARAMS, -32602);
        assert_eq!(INTERNAL_ERROR, -32603);
    }

    #[test]
    fn test_error_default() {
        let error = JsonRpcError::builder().build().unwrap();
        assert_eq!(error.code, 0);
        assert_eq!(error.message, String::new());
        assert!(error.data.is_none());
    }

    #[test]
    fn test_error_data_not_serialized_when_none() {
        let error = JsonRpcError::builder()
            .code(INTERNAL_ERROR)
            .message("err")
            .build()
            .unwrap();

        let json = serde_json::to_string(&error).unwrap();
        assert!(!json.contains("data"));
        assert!(!json.contains("ext"));
    }
}
