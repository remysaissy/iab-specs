//! JSON-RPC 2.0 message framing and routing

pub mod error;
pub mod id;
pub mod mcp_tool;
pub mod notification;
pub mod request;
pub mod response;

pub use error::{
    JsonRpcError, JsonRpcErrorBuilder, INTERNAL_ERROR, INVALID_PARAMS, INVALID_REQUEST,
    METHOD_NOT_FOUND, PARSE_ERROR,
};
pub use id::JsonRpcId;
pub use mcp_tool::{MCPTool, MCPToolBuilder};
pub use notification::{JsonRpcNotification, JsonRpcNotificationBuilder};
pub use request::{JsonRpcRequest, JsonRpcRequestBuilder};
pub use response::{JsonRpcResponse, JsonRpcResponseBuilder};
