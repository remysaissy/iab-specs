//! Agentic Direct — OpenDirect v2.1 + A2A Protocol
//!
//! This crate implements the Agentic Direct specification, which combines
//! OpenDirect v2.1 with autonomous-to-autonomous (A2A) agent communication
//! and JSON-RPC message routing.
//!
//! # Supported Versions
//!
//! - [`v21`] - Agentic Direct v2.1 (OpenDirect v2.1 + A2A Protocol)
//!
//! # Overview
//!
//! Agentic Direct enables direct, agent-to-agent communication for programmatic
//! advertising transactions. It extends OpenDirect v2.1 with A2A protocol support
//! and JSON-RPC 2.0 message routing.

pub use iab_specs_core::{DefaultExt, Error, Extension, Result};

pub mod v21;
