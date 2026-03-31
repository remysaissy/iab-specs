//! Agentic Direct v2.1 Specification
//!
//! This module implements the complete Agentic Direct v2.1 specification,
//! combining OpenDirect v2.1 with autonomous-to-autonomous (A2A) protocol support
//! and JSON-RPC 2.0 message routing for agent-to-agent communication.
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Enumerations for transaction states, roles, and protocol identifiers
//! - [`entities`] - Core data structures for deals, creatives, and agent metadata
//! - [`a2a`] - A2A protocol messages and message exchange patterns
//! - [`jsonrpc`] - JSON-RPC 2.0 message framing and routing

pub mod a2a;
pub mod entities;
pub mod enums;
pub mod jsonrpc;
