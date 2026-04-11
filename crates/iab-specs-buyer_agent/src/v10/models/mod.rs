mod booked_line;
mod booking;
mod buyer_identity;
mod campaign;
mod channel_brief;
mod linear_tv;
mod negotiation;
mod ucp;
mod ucp_metadata;

pub use booked_line::BookedLine;
pub use booking::{BookingJob, BookingRecommendation};
pub use buyer_identity::BuyerIdentity;
pub use campaign::{CampaignAllocation, CampaignBrief};
pub use channel_brief::ChannelBrief;
pub use linear_tv::LinearTVParams;
pub use negotiation::{NegotiationOffer, NegotiationStrategy};
pub use ucp::{AudiencePlan, UCPEmbedding};
pub use ucp_metadata::{UCPConsent, UCPModelDescriptor};

// V2 type aliases — spec renames for improved naming consistency
/// Alias for [`CampaignAllocation`]. The Buyer Agent spec now calls this "ChannelAllocation".
pub type ChannelAllocation<Ext = crate::DefaultExt> = CampaignAllocation<Ext>;
/// Alias for [`NegotiationOffer`]. The Buyer Agent spec now uses "NegotiationRound".
pub type NegotiationRound<Ext = crate::DefaultExt> = NegotiationOffer<Ext>;
/// Alias for [`BookingJob`]. The Buyer Agent spec now uses "BookingState".
pub type BookingState<Ext = crate::DefaultExt> = BookingJob<Ext>;
/// Alias for [`BookingRecommendation`]. The Buyer Agent spec now uses "ProductRecommendation".
pub type ProductRecommendation<Ext = crate::DefaultExt> = BookingRecommendation<Ext>;
