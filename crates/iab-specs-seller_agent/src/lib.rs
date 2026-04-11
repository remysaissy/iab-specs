//! Seller Agent Framework
//!
//! This crate implements the Seller Agent framework for autonomous seller-side
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

pub use iab_specs_core::{DefaultExt, Error, Extension, Result};

pub mod v10;
