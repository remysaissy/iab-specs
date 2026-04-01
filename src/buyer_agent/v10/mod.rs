//! Buyer Agent 1.0 Specification
//!
//! This module implements the Buyer Agent 1.0 specification for autonomous
//! buyer-side campaign management and optimization in programmatic advertising.
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Protocol enumerations and buyer-agent-specific state definitions
//! - [`models`] - Core buyer-agent campaign and optimization models
//! - [`state_machines`] - Validated state transitions for buyer-agent workflows
//!
//! # Shared Types
//!
//! This module re-exports core types from Agentic Direct 2.1 for convenience:
//! - Organization, Account, Product, Order, Line, Creative, Assignment
//! - Enumerations and state machines (OrderStatus, LineStatus, etc.)
//! - A2A Protocol types (AgentCard, Skill, A2ATask, etc.)
//! - JSON-RPC transport layer (JsonRpcRequest, JsonRpcResponse, etc.)

pub mod enums;
pub mod models;
pub mod state_machines;

// Re-export shared types from agentic_direct for convenience
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::a2a::*;
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::entities::*;
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::enums::*;
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::jsonrpc::*;
