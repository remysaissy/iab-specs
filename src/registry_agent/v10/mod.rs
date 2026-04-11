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
//!
//! # Quick Start
//!
//! ## Agent Registration
//!
//! ```rust
//! #[cfg(feature = "registry_agent_10")]
//! {
//! use iab_specs::registry_agent::v10::{
//!     RegisteredAgent, RegistrySource,
//!     TrustLevel, VerificationStatus,
//!     AgentCard, Skill, SkillInputMode,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an agent card
//! let agent_card = AgentCard::builder()
//!     .name("Ad Optimization Agent")
//!     .version("1.0.0")
//!     .protocol_version("0.3.0")
//!     .url("https://agent.example.com")
//!     .skills(vec![
//!         Skill::builder()
//!             .id("optimize-campaign")
//!             .name("Campaign Optimization")
//!             .input_modes(vec![SkillInputMode::Data])
//!             .build()?,
//!     ])
//!     .build()?;
//!
//! // Register in the registry
//! let registered = RegisteredAgent::builder()
//!     .agent_card(agent_card)
//!     .registry_id("reg-001")
//!     .trust_level(TrustLevel::Registered)
//!     .verification_status(VerificationStatus::Verified)
//!     .source(Some(RegistrySource::builder()
//!         .name("IAB Tech Lab Registry")
//!         .url("https://registry.iabtechlab.com")
//!         .build()?))
//!     .build()?;
//!
//! // Serialize to JSON
//! let json = serde_json::to_string_pretty(&registered)?;
//! assert!(json.contains("\"trust_level\":\"registered\""));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## Trust Lifecycle
//!
//! ```rust
//! #[cfg(feature = "registry_agent_10")]
//! {
//! use iab_specs::registry_agent::v10::{TrustLevel, can_transition_trust, valid_trust_transitions_from};
//!
//! // Trust escalation path
//! assert!(can_transition_trust(&TrustLevel::Unknown, &TrustLevel::Registered));
//! assert!(can_transition_trust(&TrustLevel::Registered, &TrustLevel::Verified));
//! assert!(can_transition_trust(&TrustLevel::Verified, &TrustLevel::Preferred));
//!
//! // Blocking from any non-Unknown state
//! assert!(can_transition_trust(&TrustLevel::Registered, &TrustLevel::Blocked));
//! assert!(can_transition_trust(&TrustLevel::Verified, &TrustLevel::Blocked));
//!
//! // Blocked is terminal
//! assert!(valid_trust_transitions_from(&TrustLevel::Blocked).is_empty());
//! }
//! ```
//!
//! ## Search Filtering
//!
//! ```rust
//! #[cfg(feature = "registry_agent_10")]
//! {
//! use iab_specs::registry_agent::v10::{
//!     RegistrySearchFilter, AgentType, TrustLevel,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let filter = RegistrySearchFilter::builder()
//!     .query("optimization")
//!     .trust_levels(vec![TrustLevel::Verified, TrustLevel::Preferred])
//!     .agent_types(vec![AgentType::Dsp, AgentType::Ssp])
//!     .max_results(Some(10))
//!     .build()?;
//!
//! let json = serde_json::to_string(&filter)?;
//! assert!(json.contains("\"query\":\"optimization\""));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! # Specification Reference
//!
//! This implementation follows the Registry Agent specification for agent
//! discovery, trust lifecycle management, and registry search. It extends
//! the [Agentic Direct](https://github.com/IABTechLab/agentic-direct)
//! specification with registry-specific types and state machines.

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

#[cfg(test)]
mod integration_tests {
    use crate::registry_agent::v10::enums::{AgentType, TrustLevel, VerificationStatus};
    use crate::registry_agent::v10::models::{
        AgentTrustInfo, RegisteredAgent, RegistrySearchFilter, RegistrySearchResult, RegistrySource,
    };
    use crate::registry_agent::v10::state_machines::{
        can_transition_trust, valid_trust_transitions_from, TrustTransition,
    };

    use crate::agentic_direct::v21::a2a::AgentCard;

    #[test]
    fn test_agent_registration_workflow() {
        let agent_card = AgentCard::builder()
            .name("Test DSP Agent")
            .version("2.0.0")
            .protocol_version("0.3.0")
            .url("https://dsp.example.com")
            .build()
            .unwrap();

        let source = RegistrySource::builder()
            .name("IAB Tech Lab")
            .url("https://registry.iabtechlab.com")
            .last_verified_at("2026-04-01T00:00:00Z")
            .build()
            .unwrap();

        let registered = RegisteredAgent::builder()
            .agent_card(agent_card)
            .registry_id("reg-dsp-001")
            .registered_at("2026-03-15T10:00:00Z")
            .trust_level(TrustLevel::Registered)
            .verification_status(VerificationStatus::Verified)
            .source(Some(source))
            .build()
            .unwrap();

        let json = serde_json::to_string(&registered).unwrap();
        let parsed: RegisteredAgent = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.trust_level, TrustLevel::Registered);
        assert_eq!(parsed.agent_card.name, "Test DSP Agent");
        assert_eq!(parsed.registry_id.as_deref(), Some("reg-dsp-001"));
        assert!(parsed.source.is_some());
        assert_eq!(parsed.source.as_ref().unwrap().name, "IAB Tech Lab");
    }

    #[test]
    fn test_trust_escalation_lifecycle() {
        let mut current = TrustLevel::Unknown;

        let path = [
            TrustLevel::Registered,
            TrustLevel::Verified,
            TrustLevel::Preferred,
        ];

        for next in path {
            assert!(
                can_transition_trust(&current, &next),
                "Transition {:?} → {:?} should be valid",
                current,
                next
            );
            current = next;
        }

        assert_eq!(current, TrustLevel::Preferred);
        assert_eq!(
            valid_trust_transitions_from(&current),
            vec![TrustLevel::Blocked]
        );

        assert!(can_transition_trust(&current, &TrustLevel::Blocked));
        current = TrustLevel::Blocked;
        assert!(valid_trust_transitions_from(&current).is_empty());

        let transition = TrustTransition::builder()
            .from(TrustLevel::Verified)
            .to(TrustLevel::Preferred)
            .timestamp("2026-04-15T14:30:00Z")
            .reason("Agent passed extended verification")
            .verified_by("registry-admin-001")
            .build()
            .unwrap();

        let t_json = serde_json::to_string(&transition).unwrap();
        let parsed_t: TrustTransition = serde_json::from_str(&t_json).unwrap();
        assert_eq!(transition, parsed_t);
    }

    #[test]
    fn test_search_filter_and_result_workflow() {
        let filter = RegistrySearchFilter::builder()
            .query("optimization")
            .trust_levels(vec![TrustLevel::Verified, TrustLevel::Preferred])
            .agent_types(vec![AgentType::Dsp, AgentType::Ssp])
            .max_results(Some(10))
            .build()
            .unwrap();

        let filter_json = serde_json::to_string(&filter).unwrap();
        let parsed_filter: RegistrySearchFilter = serde_json::from_str(&filter_json).unwrap();
        assert_eq!(parsed_filter.trust_levels.len(), 2);
        assert_eq!(parsed_filter.agent_types.len(), 2);
        assert_eq!(parsed_filter.max_results, Some(10));

        let agent = RegisteredAgent::builder()
            .agent_card(
                AgentCard::builder()
                    .name("DSP Optimizer")
                    .version("1.0.0")
                    .protocol_version("0.3.0")
                    .url("https://optimizer.example.com")
                    .build()
                    .unwrap(),
            )
            .trust_level(TrustLevel::Verified)
            .verification_status(VerificationStatus::Verified)
            .build()
            .unwrap();

        let result = RegistrySearchResult::builder()
            .agents(vec![agent])
            .total_count(1)
            .has_more(false)
            .build()
            .unwrap();

        let result_json = serde_json::to_string(&result).unwrap();
        let parsed: RegistrySearchResult = serde_json::from_str(&result_json).unwrap();
        assert_eq!(parsed.total_count, 1);
        assert!(!parsed.has_more);
        assert_eq!(parsed.agents[0].agent_card.name, "DSP Optimizer");
    }

    #[test]
    fn test_agent_trust_info_quick_lookup() {
        let info = AgentTrustInfo::builder()
            .agent_url("https://agent.example.com")
            .is_registered(true)
            .trust_level(TrustLevel::Verified)
            .registry_id("reg-001")
            .build()
            .unwrap();

        let json = serde_json::to_string(&info).unwrap();
        let parsed: AgentTrustInfo = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_registered);
        assert_eq!(parsed.trust_level, TrustLevel::Verified);
        assert_eq!(parsed.registry_id.as_deref(), Some("reg-001"));
        assert_eq!(parsed.agent_url, "https://agent.example.com");
    }
}
