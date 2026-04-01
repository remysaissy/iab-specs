use serde::{Deserialize, Serialize};

/// A polymorphic JSON-RPC 2.0 request identifier.
///
/// Per the JSON-RPC 2.0 specification, the `id` field can be a string, a number,
/// or null. This enum captures all three variants and uses `#[serde(untagged)]`
/// for transparent serialization/deserialization.
///
/// # Example
///
/// ```
/// use iab_specs::agentic_direct::v21::jsonrpc::JsonRpcId;
///
/// let string_id = JsonRpcId::String("req-1".into());
/// let number_id = JsonRpcId::Number(42);
/// let null_id = JsonRpcId::Null;
///
/// // Serialize
/// assert_eq!(serde_json::to_string(&string_id).unwrap(), r#""req-1""#);
/// assert_eq!(serde_json::to_string(&number_id).unwrap(), "42");
/// assert_eq!(serde_json::to_string(&null_id).unwrap(), "null");
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum JsonRpcId {
    /// A string identifier.
    String(String),
    /// A numeric identifier (integer only per JSON-RPC 2.0).
    Number(i64),
    /// Null identifier — used when the id cannot be determined (e.g., parse errors).
    #[default]
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_string_id() {
        let id: JsonRpcId = serde_json::from_str(r#""req-1""#).unwrap();
        assert_eq!(id, JsonRpcId::String("req-1".into()));
    }

    #[test]
    fn test_deserialize_number_id() {
        let id: JsonRpcId = serde_json::from_str("42").unwrap();
        assert_eq!(id, JsonRpcId::Number(42));
    }

    #[test]
    fn test_deserialize_null_id() {
        let id: JsonRpcId = serde_json::from_str("null").unwrap();
        assert_eq!(id, JsonRpcId::Null);
    }

    #[test]
    fn test_roundtrip_string() {
        let id = JsonRpcId::String("abc-123".into());
        let json = serde_json::to_string(&id).unwrap();
        let parsed: JsonRpcId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_roundtrip_number() {
        let id = JsonRpcId::Number(99);
        let json = serde_json::to_string(&id).unwrap();
        let parsed: JsonRpcId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_roundtrip_null() {
        let id = JsonRpcId::Null;
        let json = serde_json::to_string(&id).unwrap();
        let parsed: JsonRpcId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_default_is_null() {
        assert_eq!(JsonRpcId::default(), JsonRpcId::Null);
    }

    #[test]
    fn test_serialize_string() {
        let json = serde_json::to_string(&JsonRpcId::String("test".into())).unwrap();
        assert_eq!(json, r#""test""#);
    }

    #[test]
    fn test_serialize_number() {
        let json = serde_json::to_string(&JsonRpcId::Number(7)).unwrap();
        assert_eq!(json, "7");
    }

    #[test]
    fn test_serialize_null() {
        let json = serde_json::to_string(&JsonRpcId::Null).unwrap();
        assert_eq!(json, "null");
    }

    #[test]
    fn test_deserialize_negative_number() {
        let id: JsonRpcId = serde_json::from_str("-1").unwrap();
        assert_eq!(id, JsonRpcId::Number(-1));
    }

    #[test]
    fn test_deserialize_zero() {
        let id: JsonRpcId = serde_json::from_str("0").unwrap();
        assert_eq!(id, JsonRpcId::Number(0));
    }

    #[test]
    fn test_deserialize_empty_string() {
        let id: JsonRpcId = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(id, JsonRpcId::String(String::new()));
    }
}
