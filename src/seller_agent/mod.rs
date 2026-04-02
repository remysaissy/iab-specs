//! Seller Agent Framework
//!
//! This module implements the Seller Agent framework for autonomous seller-side
//! agents participating in programmatic advertising workflows with inventory
//! management and yield optimization capabilities.
//!
//! # Supported Versions
//!
//! - [`v10`] - Seller Agent 1.0 (2025)
//!
//! # Overview
//!
//! The Seller Agent framework provides a structured interface for autonomous agents
//! to manage seller-side inventory, optimize pricing strategies, and execute
//! programmatic workflows through order management and state machines.
//!
//! # Feature Flags
//!
//! - `seller_agent_10` - Enable Seller Agent 1.0 support (requires `agentic_direct_21`)

#[cfg(feature = "seller_agent_10")]
pub mod v10;
