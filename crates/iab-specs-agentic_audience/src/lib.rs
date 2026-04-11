//! Agentic Audience — Embedding Exchange Protocol
//!
//! This crate implements the Agentic Audience specification for agent-to-agent
//! audience targeting via embedding exchange.
//!
//! # Supported Versions
//!
//! - [`v10`] - Agentic Audience v1.0 (Draft)
//!
//! # Overview
//!
//! Agentic Audience enables embedding-based audience targeting between autonomous
//! agents. It defines transport envelopes, scoring protocols, and OpenRTB bid
//! stream extensions for embedding exchange.

pub use iab_specs_core::{DefaultExt, Error, Extension, Result};

pub mod v10;
