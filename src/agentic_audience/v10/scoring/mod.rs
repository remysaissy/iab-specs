//! Campaign scoring types (head, request, response, score) and OpenRTB bid stream extension.

mod campaign_head;
mod campaign_score;
mod embedding_segment_ext;
mod scoring_request;
mod scoring_response;

pub use campaign_head::{CampaignHead, CampaignHeadBuilder};
pub use campaign_score::{CampaignScore, CampaignScoreBuilder};
pub use embedding_segment_ext::{EmbeddingSegmentExt, EmbeddingSegmentExtBuilder};
pub use scoring_request::{ScoringRequest, ScoringRequestBuilder};
pub use scoring_response::{ScoringResponse, ScoringResponseBuilder};
