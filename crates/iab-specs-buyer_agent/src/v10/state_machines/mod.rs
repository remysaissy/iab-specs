mod campaign_state_machine;
mod deal_state_machine;

pub use campaign_state_machine::{
    can_transition_campaign, valid_campaign_transitions_from, CampaignTransition,
    VALID_CAMPAIGN_TRANSITIONS,
};
pub use deal_state_machine::{
    can_transition_deal, valid_deal_transitions_from, DealTransition, VALID_DEAL_TRANSITIONS,
};
