mod campaign_state_machine;
mod deal_state_machine;

pub use campaign_state_machine::{
    CampaignTransition, VALID_CAMPAIGN_TRANSITIONS, can_transition_campaign,
    valid_campaign_transitions_from,
};
pub use deal_state_machine::{
    DealTransition, VALID_DEAL_TRANSITIONS, can_transition_deal, valid_deal_transitions_from,
};
