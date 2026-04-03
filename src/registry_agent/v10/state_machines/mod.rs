//! Registry Agent v1.0 state machines

pub mod trust_state_machine;

pub use trust_state_machine::{
    TrustTransition, TrustTransitionBuilder, VALID_TRUST_TRANSITIONS, can_transition_trust,
    valid_trust_transitions_from,
};
