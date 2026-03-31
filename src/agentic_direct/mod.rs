//! Agentic Direct — OpenDirect v2.1 + A2A Protocol
//!
//! This module implements the Agentic Direct specification, which combines
//! OpenDirect v2.1 with autonomous-to-autonomous (A2A) agent communication
//! and JSON-RPC message routing.
//!
//! # Supported Versions
//!
//! - [`v21`] - Agentic Direct v2.1 (OpenDirect v2.1 + A2A Protocol)
//!
//! # Feature Flags
//!
//! - `agentic_direct_21` - Enable Agentic Direct v2.1 support (includes `serde_json`)
//!
//! # Overview
//!
//! Agentic Direct enables direct, agent-to-agent communication for programmatic
//! advertising transactions. It extends OpenDirect v2.1 with A2A protocol support
//! and JSON-RPC 2.0 message routing.

pub mod v21;
