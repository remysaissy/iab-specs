use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct MCPTool {
    #[builder(setter(into))]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    pub input_schema: serde_json::Value,
}

impl MCPTool {
    pub fn builder() -> MCPToolBuilder {
        MCPToolBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_tool_creation() {
        let tool = MCPTool::builder()
            .name("search")
            .description("Searches documents")
            .input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" }
                },
                "required": ["query"]
            }))
            .build()
            .unwrap();

        assert_eq!(tool.name, "search");
        assert_eq!(tool.description, Some("Searches documents".to_string()));
        assert_eq!(tool.input_schema["type"], "object");
    }

    #[test]
    fn test_mcp_tool_serialization() {
        let tool = MCPTool::builder()
            .name("lookup")
            .input_schema(serde_json::json!({"type": "object"}))
            .build()
            .unwrap();

        let json = serde_json::to_string(&tool).unwrap();
        assert!(json.contains("\"name\":\"lookup\""));
        assert!(json.contains("\"input_schema\":{\"type\":\"object\"}"));
        assert!(!json.contains("\"description\""));
    }

    #[test]
    fn test_mcp_tool_deserialization() {
        let json = r#"{"name":"fetch","description":"Fetch content","input_schema":{"type":"object","properties":{"url":{"type":"string"}}}}"#;
        let tool: MCPTool = serde_json::from_str(json).unwrap();

        assert_eq!(tool.name, "fetch");
        assert_eq!(tool.description, Some("Fetch content".to_string()));
        assert_eq!(tool.input_schema["properties"]["url"]["type"], "string");
    }

    #[test]
    fn test_mcp_tool_roundtrip() {
        let tool = MCPTool::builder()
            .name("summarize")
            .description("Summarizes text")
            .input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string" }
                }
            }))
            .build()
            .unwrap();

        let json = serde_json::to_string(&tool).unwrap();
        let parsed: MCPTool = serde_json::from_str(&json).unwrap();
        assert_eq!(tool, parsed);
    }

    #[test]
    fn test_mcp_tool_with_complex_schema() {
        let tool = MCPTool::builder()
            .name("create_order")
            .input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "order_id": { "type": "string" },
                    "budget": { "type": "number", "minimum": 0.0 },
                    "channels": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "metadata": {
                        "type": "object",
                        "additionalProperties": true
                    }
                },
                "required": ["order_id", "budget"],
                "additionalProperties": false
            }))
            .build()
            .unwrap();

        assert_eq!(tool.input_schema["properties"]["budget"]["minimum"], 0.0);
        assert_eq!(
            tool.input_schema["properties"]["channels"]["items"]["type"],
            "string"
        );
        assert_eq!(tool.input_schema["additionalProperties"], false);
    }

    #[test]
    fn test_mcp_tool_malformed_json_rejected() {
        let json = r#"{"name": 123, "input_schema": "not_an_object"}"#;
        let result: Result<MCPTool, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Wrong field types should fail");

        let json = r#"totally broken json"#;
        let result: Result<MCPTool, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
