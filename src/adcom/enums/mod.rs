//! AdCOM (Advertising Common Object Model) Enumerations
//!
//! This module contains standardized enumeration lists from the AdCOM v1.0 specification.
//! These enumerations are used across multiple advertising protocols including OpenRTB 2.x/3.x.
//!
//! AdCOM enumerations provide consistent value definitions for:
//! - Auction mechanics (auction types, no-bid reasons)
//! - Media specifications (video protocols, API frameworks)
//! - Device characteristics (device types, connection types)
//! - Content classification (contexts, quality ratings)
//! - Creative attributes (expandable, skippable, etc.)
//!
//! Reference: <https://github.com/InteractiveAdvertisingBureau/AdCOM>

mod ad_position;
mod agent_type;
mod api_framework;
mod auction_type;
mod audit_status_code;
mod auto_refresh_trigger;
mod banner_ad_type;
mod category_taxonomy;
mod click_type;
mod companion_type;
mod connection_type;
mod content_context;
mod content_delivery_method;
mod content_quality;
mod creative_attribute;
mod creative_subtype_audio_video;
mod creative_subtype_display;
mod device_orientation;
mod device_type;
mod display_context_type;
mod display_placement_type;
mod dooh_multiplier_measurement_source;
mod dooh_venue_taxonomy;
mod event_tracking_method;
mod event_type;
mod expandable_direction;
mod feed_type;
mod id_match_method;
mod local_market_identifier_type;
mod location_service;
mod location_type;
mod loss_reason;
mod native_data_asset_type;
mod native_image_asset_type;
mod no_bid_reason;
mod operating_system;
mod placement_position;
mod playback_cessation_mode;
mod playback_method;
mod pod_deduplication;
mod pod_sequence;
mod production_quality;
mod protocol;
mod qag_media_rating;
mod size_unit;
mod slot_position;
mod start_delay;
mod user_agent_source;
mod video_linearity;
mod video_placement_subtype;
mod video_placement_type;
mod volume_normalization_mode;

pub use ad_position::*;
pub use agent_type::*;
pub use api_framework::*;
pub use auction_type::*;
pub use audit_status_code::*;
pub use auto_refresh_trigger::*;
pub use banner_ad_type::*;
pub use category_taxonomy::*;
pub use click_type::*;
pub use companion_type::*;
pub use connection_type::*;
pub use content_context::*;
pub use content_delivery_method::*;
pub use content_quality::*;
pub use creative_attribute::*;
pub use creative_subtype_audio_video::*;
pub use creative_subtype_display::*;
pub use device_orientation::*;
pub use device_type::*;
pub use display_context_type::*;
pub use display_placement_type::*;
pub use dooh_multiplier_measurement_source::*;
pub use dooh_venue_taxonomy::*;
pub use event_tracking_method::*;
pub use event_type::*;
pub use expandable_direction::*;
pub use feed_type::*;
pub use id_match_method::*;
pub use local_market_identifier_type::*;
pub use location_service::*;
pub use location_type::*;
pub use loss_reason::*;
pub use native_data_asset_type::*;
pub use native_image_asset_type::*;
pub use no_bid_reason::*;
pub use operating_system::*;
pub use placement_position::*;
pub use playback_cessation_mode::*;
pub use playback_method::*;
pub use pod_deduplication::*;
pub use pod_sequence::*;
pub use production_quality::*;
pub use protocol::*;
pub use qag_media_rating::*;
pub use size_unit::*;
pub use slot_position::*;
pub use start_delay::*;
pub use user_agent_source::*;
pub use video_linearity::*;
pub use video_placement_subtype::*;
pub use video_placement_type::*;
pub use volume_normalization_mode::*;
