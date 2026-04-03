//! Registry Agent — Agent Discovery and Trust Lifecycle
//!
//! This module implements the Registry Agent specification, providing data models
//! for agent registration, trust lifecycle management, and discovery/search.
//!
//! # Supported Versions
//!
//! - [`v10`] - Registry Agent v1.0
//!
//! # Feature Flags
//!
//! - `registry_agent_10` - Enable Registry Agent v1.0 support (includes `agentic_direct_21`)
//!
//! # Overview
//!
//! The Registry Agent enables discovery and trust management of autonomous agents
//! in the advertising ecosystem. It wraps the IAB Tech Lab registry and provides
//! typed data models for agent registration, trust escalation, and search/filtering.

#[cfg(feature = "registry_agent_10")]
pub mod v10;
