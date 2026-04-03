//! Registry Agent v1.0 — Agent Discovery Models
//!
//! Provides data models for agent registration, trust lifecycle management,
//! and discovery/search in the IAB Tech Lab registry.
//!
//! # Architecture
//!
//! The Registry Agent is a thin specification — its unique data models are minimal.
//! Most types are re-exported from [`crate::agentic_direct::v21`] for convenience.
//!
//! ## Unique Types
//!
//! - **Registration**: [`RegisteredAgent`], [`RegistrySource`], [`AgentTrustInfo`]
//! - **Trust Lifecycle**: [`TrustLevel`], [`AgentType`], [`VerificationStatus`]
//! - **State Machine**: [`can_transition_trust`], [`valid_trust_transitions_from`]
//! - **Search/Discovery**: [`RegistrySearchFilter`], [`RegistrySearchResult`]
//!
//! ## Re-exported Types
//!
//! All types from [`crate::agentic_direct::v21`] are re-exported for convenience,
//! including [`AgentCard`], [`Skill`], [`JsonRpcRequest`], [`Order`], etc.

pub mod enums;
pub mod models;
pub mod state_machines;

#[allow(unused_imports)]
pub use enums::*;
#[allow(unused_imports)]
pub use models::*;
#[allow(unused_imports)]
pub use state_machines::*;

// Re-export shared types from agentic_direct for convenience
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::a2a::*;
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::entities::*;
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::enums::*;
#[allow(unused_imports)]
pub use crate::agentic_direct::v21::jsonrpc::*;
