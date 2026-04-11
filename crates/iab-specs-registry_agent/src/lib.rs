//! Registry Agent Framework
//!
//! This crate implements the Registry Agent framework for agent discovery,
//! trust lifecycle management, and registry search in the advertising
//! agent ecosystem.
//!
//! # Supported Versions
//!
//! - [`v10`] - Registry Agent 1.0 (2025)
//!
//! # Overview
//!
//! The Registry Agent framework provides a structured interface for autonomous agents
//! to register, discover, and establish trust relationships through registry-based
//! lifecycle management and search capabilities.

pub use iab_specs_core::{DefaultExt, Error, Extension, Result};

pub mod v10;
