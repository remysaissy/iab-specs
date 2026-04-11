//! Buyer Agent Framework
//!
//! This crate implements the Buyer Agent framework for autonomous buyer-side
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

pub use iab_specs_core::{DefaultExt, Error, Extension, Result};

pub mod v10;
