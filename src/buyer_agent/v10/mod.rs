//! Buyer Agent 1.0 Specification
//!
//! This module implements the Buyer Agent 1.0 specification for autonomous
//! buyer-side campaign management and optimization in programmatic advertising.
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Protocol enumerations and buyer-agent-specific state definitions
//! - [`models`] - Core buyer-agent campaign and optimization models
//! - [`state_machines`] - Validated state transitions for buyer-agent workflows
//!
//! # Shared Types
//!
//! This module re-exports core types from Agentic Direct 2.1 for convenience:
//! - Organization, Account, Product, Order, Line, Creative, Assignment
//! - Enumerations and state machines (OrderStatus, LineStatus, etc.)
//! - A2A Protocol types (AgentCard, Skill, A2ATask, etc.)
//! - JSON-RPC transport layer (JsonRpcRequest, JsonRpcResponse, etc.)
//!
//! # Quick Start
//!
//! ## Creating a Campaign Brief and Allocation
//!
//! ```rust
//! #[cfg(feature = "buyer_agent_10")]
//! {
//! use iab_specs::buyer_agent::v10::models::{CampaignBrief, CampaignAllocation};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Define a campaign brief with objectives and budget
//! let brief = CampaignBrief::builder()
//!     .name("Q2 Brand Awareness Campaign")
//!     .objectives(vec!["Increase brand awareness".to_string(), "Drive conversions".to_string()])
//!     .budget(50000.0)
//!     .start_date("2026-04-01")
//!     .end_date("2026-06-30")
//!     .channels(vec!["display".to_string(), "video".to_string()])
//!     .target_audience(Some(serde_json::json!({
//!         "age_range": "25-54",
//!         "interests": ["tech", "finance"]
//!     })))
//!     .build()?;
//!
//! // Allocate budget across channels
//! let display_alloc = CampaignAllocation::builder()
//!     .channel("display")
//!     .budget_share(0.6)
//!     .priority(1)
//!     .rationale("High-volume reach channel")
//!     .build()?;
//!
//! let video_alloc = CampaignAllocation::builder()
//!     .channel("video")
//!     .budget_share(0.4)
//!     .priority(2)
//!     .rationale("Premium engagement format")
//!     .build()?;
//!
//! // Serialize to JSON
//! let json = serde_json::to_string_pretty(&brief)?;
//! assert!(json.contains("\"name\":\"Q2 Brand Awareness Campaign\""));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## Deal Negotiation
//!
//! ```rust
//! #[cfg(feature = "buyer_agent_10")]
//! {
//! use iab_specs::buyer_agent::v10::models::{NegotiationStrategy, NegotiationOffer};
//! use iab_specs::buyer_agent::v10::enums::DealStatus;
//! use iab_specs::buyer_agent::v10::state_machines::can_transition_deal;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Define a negotiation strategy
//! let strategy = NegotiationStrategy::builder()
//!     .target_cpm(2.50)
//!     .max_cpm(5.00)
//!     .concession_step(0.25)
//!     .max_rounds(5)
//!     .build()?;
//!
//! // Simulate a negotiation round
//! let buyer_offer = NegotiationOffer::builder()
//!     .price(2.50)
//!     .round(1)
//!     .from_buyer(true)
//!     .accepted(Some(false))
//!     .counter_price(Some(4.00))
//!     .build()?;
//!
//! let seller_counter = NegotiationOffer::builder()
//!     .price(4.00)
//!     .round(2)
//!     .from_buyer(false)
//!     .accepted(Some(false))
//!     .counter_price(Some(3.00))
//!     .build()?;
//!
//! // Validate deal transitions
//! assert!(can_transition_deal(&DealStatus::Quoted, &DealStatus::Negotiating));
//! assert!(can_transition_deal(&DealStatus::Negotiating, &DealStatus::Accepted));
//! assert!(!can_transition_deal(&DealStatus::Quoted, &DealStatus::Completed));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## State Machine Transitions
//!
//! ```rust
//! #[cfg(feature = "buyer_agent_10")]
//! {
//! use iab_specs::buyer_agent::v10::enums::{CampaignStatus, DealStatus};
//! use iab_specs::buyer_agent::v10::state_machines::{
//!     can_transition_campaign, can_transition_deal,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Campaign lifecycle: Initialized → BriefReceived → ... → Completed
//! assert!(can_transition_campaign(&CampaignStatus::Initialized, &CampaignStatus::BriefReceived));
//! assert!(can_transition_campaign(&CampaignStatus::BriefReceived, &CampaignStatus::BudgetAllocated));
//! assert!(can_transition_campaign(&CampaignStatus::BudgetAllocated, &CampaignStatus::Researching));
//! assert!(can_transition_campaign(&CampaignStatus::Researching, &CampaignStatus::AwaitingApproval));
//! assert!(can_transition_campaign(&CampaignStatus::ExecutingBookings, &CampaignStatus::Completed));
//!
//! // Approval rejection loops back to research
//! assert!(can_transition_campaign(&CampaignStatus::AwaitingApproval, &CampaignStatus::Researching));
//!
//! // Invalid transitions are rejected
//! assert!(!can_transition_campaign(&CampaignStatus::Initialized, &CampaignStatus::Completed));
//! assert!(!can_transition_campaign(&CampaignStatus::Completed, &CampaignStatus::Initialized));
//!
//! // Deal lifecycle: Quoted → Negotiating → Accepted → ... → Completed
//! assert!(can_transition_deal(&DealStatus::Quoted, &DealStatus::Negotiating));
//! assert!(can_transition_deal(&DealStatus::Negotiating, &DealStatus::Accepted));
//! assert!(can_transition_deal(&DealStatus::Accepted, &DealStatus::Booking));
//! assert!(can_transition_deal(&DealStatus::Booking, &DealStatus::Booked));
//! assert!(can_transition_deal(&DealStatus::Booked, &DealStatus::Delivering));
//! assert!(can_transition_deal(&DealStatus::Delivering, &DealStatus::Completed));
//!
//! // Terminal states cannot transition
//! assert!(!can_transition_deal(&DealStatus::Completed, &DealStatus::Quoted));
//! assert!(!can_transition_deal(&DealStatus::Cancelled, &DealStatus::Quoted));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! # Specification Reference
//!
//! This implementation follows the Buyer Agent specification for demand-side
//! autonomous campaign planning, UCP embeddings, negotiation, and booking
//! workflows. It extends the [Agentic Direct](https://github.com/IABTechLab/agentic-direct)
//! specification with buyer-specific types and state machines.

pub mod enums;
pub mod models;
pub mod state_machines;

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
    use crate::buyer_agent::v10::enums::{ApprovalStatus, CampaignStatus, ChannelType, DealStatus};
    use crate::buyer_agent::v10::models::{
        AudiencePlan, BookingJob, BookingRecommendation, BuyerIdentity, CampaignAllocation,
        CampaignBrief, NegotiationOffer, NegotiationStrategy, UCPEmbedding,
    };
    use crate::buyer_agent::v10::state_machines::{
        can_transition_campaign, can_transition_deal, CampaignTransition, DealTransition,
    };

    #[test]
    fn test_complete_campaign_workflow() {
        // 1. CampaignBrief — define campaign goals and budget
        let brief = CampaignBrief::builder()
            .name("Q2 Multi-Channel Campaign")
            .objectives(vec![
                "Increase brand awareness".to_string(),
                "Drive conversions".to_string(),
            ])
            .budget(100000.0)
            .start_date("2026-04-01")
            .end_date("2026-06-30")
            .channels(vec!["display".to_string(), "video".to_string()])
            .target_audience(Some(serde_json::json!({
                "age_range": "25-54",
                "interests": ["tech", "finance"],
                "location": "US"
            })))
            .kpis(Some(serde_json::json!({
                "impression_target": 2000000,
                "cpa": 5.0,
                "roas": 3.0
            })))
            .constraints(Some(serde_json::json!({
                "max_cpc": 2.0,
                "min_viewability": 0.5
            })))
            .build()
            .unwrap();

        assert_eq!(brief.name, "Q2 Multi-Channel Campaign");
        assert_eq!(brief.budget, 100000.0);
        assert_eq!(brief.channels.len(), 2);

        // 2. CampaignAllocations — distribute budget across channels
        let display_alloc = CampaignAllocation::builder()
            .channel("display")
            .budget_share(0.6)
            .priority(1)
            .rationale("High volume reach channel")
            .build()
            .unwrap();

        let video_alloc = CampaignAllocation::builder()
            .channel("video")
            .budget_share(0.4)
            .priority(2)
            .rationale("Premium engagement format")
            .build()
            .unwrap();

        assert_eq!(display_alloc.budget_share + video_alloc.budget_share, 1.0);

        // 3. BookingRecommendations — research phase produces recommendations
        let rec1 = BookingRecommendation::builder()
            .seller_name("Premium Exchange")
            .product_id("prod-display-001")
            .price(2.50)
            .impressions(500000)
            .channel("display")
            .rationale("High viewability, strong brand safety")
            .build()
            .unwrap();

        let rec2 = BookingRecommendation::builder()
            .seller_name("Video Network")
            .product_id("prod-video-001")
            .price(8.00)
            .impressions(200000)
            .channel("video")
            .rationale("Premium pre-roll inventory")
            .build()
            .unwrap();

        assert_eq!(rec1.seller_name, "Premium Exchange");
        assert_eq!(rec2.channel, Some("video".to_string()));

        // 4. BookingJob — tie it all together with status transitions
        let booking = BookingJob::builder()
            .id("job-001")
            .campaign_brief_id("brief-q2-001")
            .status(CampaignStatus::Researching)
            .allocations(vec![display_alloc.clone(), video_alloc.clone()])
            .recommendations(vec![
                serde_json::json!({"seller": "Premium Exchange", "price": 2.50}),
                serde_json::json!({"seller": "Video Network", "price": 8.00}),
            ])
            .build()
            .unwrap();

        assert_eq!(booking.status, CampaignStatus::Researching);
        assert_eq!(booking.allocations.len(), 2);
        assert_eq!(booking.recommendations.len(), 2);

        // 5. Verify all entities serialize/deserialize correctly
        let brief_json = serde_json::to_string(&brief).unwrap();
        let parsed_brief: CampaignBrief = serde_json::from_str(&brief_json).unwrap();
        assert_eq!(brief, parsed_brief);

        let alloc_json = serde_json::to_string(&display_alloc).unwrap();
        let parsed_alloc: CampaignAllocation = serde_json::from_str(&alloc_json).unwrap();
        assert_eq!(display_alloc, parsed_alloc);

        let rec_json = serde_json::to_string(&rec1).unwrap();
        let parsed_rec: BookingRecommendation = serde_json::from_str(&rec_json).unwrap();
        assert_eq!(rec1, parsed_rec);

        let booking_json = serde_json::to_string(&booking).unwrap();
        let parsed_booking: BookingJob = serde_json::from_str(&booking_json).unwrap();
        assert_eq!(booking, parsed_booking);
    }

    #[test]
    fn test_deal_negotiation_flow() {
        // 1. Define a negotiation strategy
        let strategy = NegotiationStrategy::builder()
            .target_cpm(2.50)
            .max_cpm(5.00)
            .concession_step(0.25)
            .max_rounds(5)
            .build()
            .unwrap();

        assert_eq!(strategy.target_cpm, 2.50);
        assert_eq!(strategy.max_cpm, 5.00);

        // 2. Simulate negotiation rounds
        // Round 1: Buyer opens at target CPM
        let offer1 = NegotiationOffer::builder()
            .price(2.50)
            .round(1)
            .from_buyer(true)
            .accepted(Some(false))
            .counter_price(Some(4.50))
            .build()
            .unwrap();

        // Round 2: Seller counters
        let offer2 = NegotiationOffer::builder()
            .price(4.50)
            .round(2)
            .from_buyer(false)
            .accepted(Some(false))
            .counter_price(Some(3.00))
            .build()
            .unwrap();

        // Round 3: Buyer concedes
        let offer3 = NegotiationOffer::builder()
            .price(3.00)
            .round(3)
            .from_buyer(true)
            .accepted(Some(false))
            .counter_price(Some(3.50))
            .build()
            .unwrap();

        // Round 4: Final agreement
        let offer4 = NegotiationOffer::builder()
            .price(3.50)
            .round(4)
            .from_buyer(false)
            .accepted(Some(true))
            .build()
            .unwrap();

        assert!(offer4.accepted.unwrap());
        assert!(offer4.counter_price.is_none());

        // 3. Validate deal state transitions through the full lifecycle
        // Quoted → Negotiating → Accepted → Booking → Booked → Delivering → Completed
        let deal_happy_path = [
            (DealStatus::Quoted, DealStatus::Negotiating),
            (DealStatus::Negotiating, DealStatus::Accepted),
            (DealStatus::Accepted, DealStatus::Booking),
            (DealStatus::Booking, DealStatus::Booked),
            (DealStatus::Booked, DealStatus::Delivering),
            (DealStatus::Delivering, DealStatus::Completed),
        ];

        let mut current = DealStatus::Quoted;
        for (from, to) in &deal_happy_path {
            assert_eq!(&current, from);
            assert!(
                can_transition_deal(from, to),
                "Transition {:?} → {:?} should be valid",
                from,
                to
            );
            current = *to;
        }
        assert_eq!(current, DealStatus::Completed);

        // 4. Verify roundtrip serialization of all offers
        let offers = [&offer1, &offer2, &offer3, &offer4];
        for offer in &offers {
            let json = serde_json::to_string(offer).unwrap();
            let parsed: NegotiationOffer = serde_json::from_str(&json).unwrap();
            assert_eq!(*offer, &parsed);
        }

        let strategy_json = serde_json::to_string(&strategy).unwrap();
        let parsed_strategy: NegotiationStrategy = serde_json::from_str(&strategy_json).unwrap();
        assert_eq!(strategy, parsed_strategy);

        // 5. Verify DealTransition roundtrip
        let transition = DealTransition::builder()
            .from(DealStatus::Negotiating)
            .to(DealStatus::Accepted)
            .timestamp("2026-04-15T14:30:00Z")
            .reason("Both parties agreed on $3.50 CPM")
            .actor("buyer-agent-001")
            .build()
            .unwrap();

        let t_json = serde_json::to_string(&transition).unwrap();
        let parsed_t: DealTransition = serde_json::from_str(&t_json).unwrap();
        assert_eq!(transition, parsed_t);
    }

    #[test]
    fn test_ucp_embedding_and_audience_plan() {
        // 1. Create a 384-dimensional UCP embedding
        let vector: Vec<f32> = (0..384).map(|i| (i as f32) * 0.001).collect();
        let embedding = UCPEmbedding::builder()
            .vector(vector.clone())
            .model_descriptor("sentence-transformers/all-MiniLM-L6-v2")
            .dimension(384)
            .consent("opt-in")
            .ttl(Some(86400))
            .build()
            .unwrap();

        assert_eq!(embedding.vector.len(), 384);
        assert_eq!(embedding.dimension, 384);
        assert_eq!(embedding.consent, Some("opt-in".to_string()));
        assert_eq!(embedding.ttl, Some(86400));

        // 2. Create an audience plan using the embedding
        let plan = AudiencePlan::builder()
            .query_embedding(vector.clone())
            .coverage_estimates(Some(serde_json::json!({
                "reach": 2000000,
                "frequency": 4,
                "cpm": 3.25,
                "estimated_impressions": 8000000
            })))
            .targeting_criteria(Some(serde_json::json!({
                "age_range": "25-54",
                "interests": ["tech", "finance", "sports"],
                "location": "US",
                "behaviors": {
                    "e_commerce_shoppers": true,
                    "video_viewers": true
                }
            })))
            .build()
            .unwrap();

        assert_eq!(plan.query_embedding.len(), 384);
        assert!(plan.coverage_estimates.is_some());
        assert!(plan.targeting_criteria.is_some());

        // 3. Verify roundtrip serialization preserves 384-dim vector precision
        let emb_json = serde_json::to_string(&embedding).unwrap();
        let parsed_emb: UCPEmbedding = serde_json::from_str(&emb_json).unwrap();
        assert_eq!(parsed_emb.vector.len(), 384);
        assert_eq!(parsed_emb.dimension, 384);
        assert_eq!(
            parsed_emb.model_descriptor,
            "sentence-transformers/all-MiniLM-L6-v2"
        );
        for (i, (original, parsed_val)) in vector.iter().zip(parsed_emb.vector.iter()).enumerate() {
            assert!(
                (original - parsed_val).abs() < 1e-6,
                "Mismatch at index {}: {} vs {}",
                i,
                original,
                parsed_val
            );
        }

        let plan_json = serde_json::to_string(&plan).unwrap();
        let parsed_plan: AudiencePlan = serde_json::from_str(&plan_json).unwrap();
        assert_eq!(parsed_plan.query_embedding.len(), 384);
        assert_eq!(parsed_plan.coverage_estimates, plan.coverage_estimates);
        assert_eq!(parsed_plan.targeting_criteria, plan.targeting_criteria);
    }

    #[test]
    fn test_campaign_lifecycle_with_approval_rejection_loop() {
        // Happy path: Initialized → BriefReceived → BudgetAllocated → Researching →
        //   AwaitingApproval → (rejection) → Researching → AwaitingApproval →
        //   ExecutingBookings → Completed
        let transitions = [
            (
                CampaignStatus::Initialized,
                CampaignStatus::BriefReceived,
                "Campaign brief submitted",
            ),
            (
                CampaignStatus::BriefReceived,
                CampaignStatus::BudgetAllocated,
                "Budget allocated across channels",
            ),
            (
                CampaignStatus::BudgetAllocated,
                CampaignStatus::Researching,
                "Research phase started",
            ),
            (
                CampaignStatus::Researching,
                CampaignStatus::AwaitingApproval,
                "Research complete, awaiting approval",
            ),
            // Approval rejected → loop back to research
            (
                CampaignStatus::AwaitingApproval,
                CampaignStatus::Researching,
                "Reviewer requested deeper analysis",
            ),
            // Second pass through research
            (
                CampaignStatus::Researching,
                CampaignStatus::AwaitingApproval,
                "Updated research complete",
            ),
            // Approved → execute
            (
                CampaignStatus::AwaitingApproval,
                CampaignStatus::ExecutingBookings,
                "Approved by media director",
            ),
            (
                CampaignStatus::ExecutingBookings,
                CampaignStatus::Completed,
                "All bookings executed successfully",
            ),
        ];

        let mut current = CampaignStatus::Initialized;
        for (from, to, reason) in &transitions {
            assert_eq!(&current, from, "Expected current state to be {:?}", from);
            assert!(
                can_transition_campaign(from, to),
                "Transition {:?} → {:?} ({}) should be valid",
                from,
                to,
                reason
            );
            current = *to;
        }
        assert_eq!(current, CampaignStatus::Completed);

        // Terminal state: no further transitions allowed
        assert!(!can_transition_campaign(
            &CampaignStatus::Completed,
            &CampaignStatus::Initialized
        ));
        assert!(!can_transition_campaign(
            &CampaignStatus::Completed,
            &CampaignStatus::Cancelled
        ));

        // Verify CampaignTransition record roundtrip
        let transition = CampaignTransition::builder()
            .from(CampaignStatus::AwaitingApproval)
            .to(CampaignStatus::Researching)
            .timestamp("2026-04-10T09:00:00Z")
            .reason("Reviewer requested deeper analysis on video inventory")
            .actor("media-director-001")
            .build()
            .unwrap();

        let json = serde_json::to_string(&transition).unwrap();
        let parsed: CampaignTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(transition, parsed);
    }

    #[test]
    fn test_booking_job_with_status_transitions() {
        // Create a booking job and walk it through status transitions
        let mut status = CampaignStatus::Initialized;

        // Build the booking job
        let booking = BookingJob::builder()
            .id("job-lifecycle-001")
            .campaign_brief_id("brief-lifecycle")
            .status(status)
            .allocations(vec![
                CampaignAllocation::builder()
                    .channel("display")
                    .budget_share(0.5)
                    .priority(1)
                    .build()
                    .unwrap(),
                CampaignAllocation::builder()
                    .channel("video")
                    .budget_share(0.5)
                    .priority(2)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(booking.status, CampaignStatus::Initialized);

        // Walk through the lifecycle transitions
        let lifecycle = [
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated,
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings,
            CampaignStatus::Completed,
        ];

        for next in &lifecycle {
            assert!(
                can_transition_campaign(&status, next),
                "Should transition from {:?} to {:?}",
                status,
                next
            );
            status = *next;
        }

        // Create final approved booking
        let approved_booking = BookingJob::builder()
            .id("job-lifecycle-001")
            .campaign_brief_id("brief-lifecycle")
            .status(CampaignStatus::Completed)
            .allocations(booking.allocations.clone())
            .approved(Some(true))
            .approved_by("media-director")
            .approved_at("2026-04-20T16:00:00Z")
            .build()
            .unwrap();

        assert_eq!(approved_booking.approved, Some(true));
        assert_eq!(approved_booking.status, CampaignStatus::Completed);

        // Roundtrip
        let json = serde_json::to_string(&approved_booking).unwrap();
        let parsed: BookingJob = serde_json::from_str(&json).unwrap();
        assert_eq!(approved_booking, parsed);
    }

    #[test]
    fn test_buyer_identity_for_tiered_pricing() {
        // BuyerIdentity enables context-aware pricing lookups
        let identity = BuyerIdentity::builder()
            .seat_id("seat-premium-001")
            .agency_id("agency-global-media")
            .advertiser_id("advertiser-tech-giant")
            .build()
            .unwrap();

        assert_eq!(identity.seat_id, Some("seat-premium-001".to_string()));
        assert_eq!(identity.agency_id, Some("agency-global-media".to_string()));
        assert_eq!(
            identity.advertiser_id,
            Some("advertiser-tech-giant".to_string())
        );

        // Verify roundtrip serialization
        let json = serde_json::to_string(&identity).unwrap();
        let parsed: BuyerIdentity = serde_json::from_str(&json).unwrap();
        assert_eq!(identity, parsed);

        // Verify skip_serializing_if — None fields are not in JSON
        let partial_identity = BuyerIdentity::builder()
            .seat_id("seat-123")
            .build()
            .unwrap();

        let partial_json = serde_json::to_string(&partial_identity).unwrap();
        assert!(partial_json.contains("\"seat_id\""));
        assert!(!partial_json.contains("agency_id"));
        assert!(!partial_json.contains("advertiser_id"));
    }

    #[test]
    fn test_enum_serialization_roundtrip() {
        // DealStatus
        let deal_variants = [
            DealStatus::Quoted,
            DealStatus::Negotiating,
            DealStatus::Accepted,
            DealStatus::Booking,
            DealStatus::Booked,
            DealStatus::Delivering,
            DealStatus::Completed,
            DealStatus::Cancelled,
            DealStatus::Rejected,
            DealStatus::Expired,
            DealStatus::Failed,
            DealStatus::MakegoodPending,
            DealStatus::PartiallyCanceled,
        ];
        for variant in &deal_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: DealStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // CampaignStatus
        let campaign_variants = [
            CampaignStatus::Initialized,
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated,
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings,
            CampaignStatus::Completed,
            CampaignStatus::Failed,
            CampaignStatus::Cancelled,
        ];
        for variant in &campaign_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: CampaignStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // ApprovalStatus
        let approval_variants = [
            ApprovalStatus::Pending,
            ApprovalStatus::Approved,
            ApprovalStatus::Rejected,
        ];
        for variant in &approval_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: ApprovalStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // ChannelType
        let channel_variants = [
            ChannelType::Display,
            ChannelType::Video,
            ChannelType::Ctv,
            ChannelType::Mobile,
            ChannelType::Audio,
            ChannelType::Dooh,
            ChannelType::Native,
        ];
        for variant in &channel_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: ChannelType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }
    }
}
