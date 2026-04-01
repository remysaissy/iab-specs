//! Buyer Agent Framework
//!
//! This module implements the Buyer Agent framework for autonomous buyer-side
//! agents participating in programmatic advertising workflows with real-time
//! campaign management and optimization capabilities.
//!
//! # Supported Versions
//!
//! - [`v10`] - Buyer Agent 1.0 (2025)
//!
//! # Overview
//!
//! The Buyer Agent framework provides a structured interface for autonomous agents
//! to manage buyer-side campaigns, optimize bidding strategies, and execute
//! programmatic workflows through campaign order management and state machines.
//!
//! # Feature Flags
//!
//! - `buyer_agent_10` - Enable Buyer Agent 1.0 support (requires `agentic_direct_21`)

#[cfg(feature = "buyer_agent_10")]
pub mod v10;
