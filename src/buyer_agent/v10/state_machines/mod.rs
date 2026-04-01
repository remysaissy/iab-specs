mod deal_state_machine;

pub use deal_state_machine::{
    can_transition_deal, valid_deal_transitions_from, DealTransition, VALID_DEAL_TRANSITIONS,
};
