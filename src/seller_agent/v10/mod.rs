//! Seller Agent 1.0 Specification
//!
//! This module implements the Seller Agent 1.0 specification for autonomous
//! seller-side inventory management and yield optimization in programmatic advertising.
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Protocol enumerations and seller-agent-specific state definitions
//! - [`models`] - Core seller-agent inventory and optimization models
//! - [`state_machines`] - Validated state transitions for seller-agent workflows
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
//! ## Creating a Proposal with ProposalRevision
//!
//! ```rust
//! #[cfg(feature = "seller_agent_10")]
//! {
//! use iab_specs::seller_agent::v10::models::{Proposal, ProposalRevision, ProposalItem};
//! use iab_specs::seller_agent::v10::enums::ProposalStatus;
//! use iab_specs::agentic_direct::v21::enums::RateType;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a proposal from seller to buyer
//! let proposal = Proposal::builder()
//!     .id("prop-001")
//!     .buyer_id("buyer-001")
//!     .seller_id("seller-001")
//!     .status(ProposalStatus::Submitted)
//!     .current_revision_id("rev-001")
//!     .created_at("2026-04-01T00:00:00Z")
//!     .build()?;
//!
//! // Create a revision with line items
//! let revision = ProposalRevision::builder()
//!     .id("rev-001")
//!     .proposal_id("prop-001")
//!     .revision_number(1)
//!     .items(vec![
//!         ProposalItem::builder()
//!             .product_id("prod-display-001")
//!             .quantity(500_000)
//!             .rate(2.50)
//!             .rate_type(RateType::Cpm)
//!             .start_date("2026-04-01")
//!             .end_date("2026-06-30")
//!             .build()?,
//!     ])
//!     .total_budget(Some(12500.0))
//!     .notes("Initial proposal for Q2 display campaign")
//!     .build()?;
//!
//! // Serialize to JSON
//! let json = serde_json::to_string_pretty(&proposal)?;
//! assert!(json.contains("\"buyer_id\":\"buyer-001\""));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## Seller Order State Machine Transitions
//!
//! ```rust
//! #[cfg(feature = "seller_agent_10")]
//! {
//! use iab_specs::seller_agent::v10::enums::SellerOrderStatus;
//! use iab_specs::seller_agent::v10::state_machines::{
//!     can_transition_seller_order, valid_seller_order_transitions_from,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Seller order lifecycle: Draft → Submitted → PendingApproval → Approved →
//! //   InProgress → Syncing → Booked → Completed
//! assert!(can_transition_seller_order(&SellerOrderStatus::Draft, &SellerOrderStatus::Submitted));
//! assert!(can_transition_seller_order(&SellerOrderStatus::Submitted, &SellerOrderStatus::PendingApproval));
//! assert!(can_transition_seller_order(&SellerOrderStatus::PendingApproval, &SellerOrderStatus::Approved));
//! assert!(can_transition_seller_order(&SellerOrderStatus::Approved, &SellerOrderStatus::InProgress));
//! assert!(can_transition_seller_order(&SellerOrderStatus::InProgress, &SellerOrderStatus::Syncing));
//! assert!(can_transition_seller_order(&SellerOrderStatus::Syncing, &SellerOrderStatus::Booked));
//! assert!(can_transition_seller_order(&SellerOrderStatus::Booked, &SellerOrderStatus::Completed));
//!
//! // Invalid transitions are rejected
//! assert!(!can_transition_seller_order(&SellerOrderStatus::Draft, &SellerOrderStatus::Completed));
//! assert!(!can_transition_seller_order(&SellerOrderStatus::Completed, &SellerOrderStatus::Draft));
//!
//! // Pause/resume cycle
//! assert!(can_transition_seller_order(&SellerOrderStatus::InProgress, &SellerOrderStatus::Paused));
//! assert!(can_transition_seller_order(&SellerOrderStatus::Paused, &SellerOrderStatus::InProgress));
//!
//! // Query valid transitions from a state
//! let from_draft = valid_seller_order_transitions_from(&SellerOrderStatus::Draft);
//! assert!(from_draft.contains(&SellerOrderStatus::Submitted));
//! assert!(from_draft.contains(&SellerOrderStatus::Cancelled));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## State Machine Validation
//!
//! ```rust
//! #[cfg(feature = "seller_agent_10")]
//! {
//! use iab_specs::seller_agent::v10::enums::SellerOrderStatus;
//! use iab_specs::seller_agent::v10::state_machines::can_transition_seller_order;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Validate a transition before applying it
//! let current = SellerOrderStatus::Syncing;
//! let desired = SellerOrderStatus::Booked;
//!
//! if can_transition_seller_order(&current, &desired) {
//!     // Safe to apply the transition
//!     assert_eq!(desired, SellerOrderStatus::Booked);
//! }
//!
//! // Terminal states cannot transition
//! assert!(!can_transition_seller_order(&SellerOrderStatus::Completed, &SellerOrderStatus::Draft));
//! assert!(!can_transition_seller_order(&SellerOrderStatus::Cancelled, &SellerOrderStatus::Draft));
//! assert!(!can_transition_seller_order(&SellerOrderStatus::Expired, &SellerOrderStatus::Draft));
//! # Ok(())
//! # }
//! }
//! ```
//!
//! # Specification Reference
//!
//! This implementation follows the Seller Agent specification for supply-side
//! autonomous inventory management, proposal generation, pricing tiers, negotiation,
//! and order execution workflows. It extends the [Agentic Direct](https://github.com/IABTechLab/agentic-direct)
//! specification with seller-specific types and state machines.

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
    use crate::agentic_direct::v21::enums::RateType;
    use crate::seller_agent::v10::enums::{
        AdServerType, ChangeRequestStatus, ChangeSeverity, ChangeType, DistributionStatus,
        NegotiationStrategyType, PackageType, PricingTierType, ProposalStatus, SellerOrderStatus,
        SyncStatus,
    };
    use crate::seller_agent::v10::models::{
        NegotiationConfig, NegotiationRound, PricingTier, Proposal, ProposalItem, ProposalRevision,
        TieredPricing,
    };
    use crate::seller_agent::v10::state_machines::{
        can_transition_seller_order, SellerOrderTransition,
    };

    #[test]
    fn test_complete_proposal_workflow() {
        // 1. Create a Proposal
        let proposal = Proposal::builder()
            .id("prop-001")
            .buyer_id("buyer-agency-xyz")
            .seller_id("seller-pub-abc")
            .status(ProposalStatus::Submitted)
            .current_revision_id("rev-001")
            .created_at("2026-04-01T00:00:00Z")
            .updated_at("2026-04-01T12:00:00Z")
            .build()
            .unwrap();

        assert_eq!(proposal.buyer_id, "buyer-agency-xyz");
        assert_eq!(proposal.seller_id, "seller-pub-abc");
        assert_eq!(proposal.status, ProposalStatus::Submitted);

        // 2. Create a ProposalRevision with 3 ProposalItems
        let items = vec![
            ProposalItem::builder()
                .product_id("prod-display-001")
                .quantity(500_000)
                .rate(2.50)
                .rate_type(RateType::Cpm)
                .start_date("2026-04-01")
                .end_date("2026-04-30")
                .build()
                .unwrap(),
            ProposalItem::builder()
                .product_id("prod-video-002")
                .quantity(200_000)
                .rate(8.00)
                .rate_type(RateType::Cpm)
                .start_date("2026-05-01")
                .end_date("2026-05-31")
                .build()
                .unwrap(),
            ProposalItem::builder()
                .product_id("prod-native-003")
                .quantity(100_000)
                .rate(1.75)
                .rate_type(RateType::Cpc)
                .start_date("2026-06-01")
                .end_date("2026-06-30")
                .build()
                .unwrap(),
        ];

        let revision = ProposalRevision::builder()
            .id("rev-001")
            .proposal_id("prop-001")
            .revision_number(1)
            .items(items)
            .total_budget(Some(25000.0))
            .notes("Q2 multi-format campaign proposal")
            .created_at("2026-04-01T00:00:00Z")
            .build()
            .unwrap();

        assert_eq!(revision.items.len(), 3);
        assert_eq!(revision.revision_number, 1);
        assert_eq!(revision.total_budget, Some(25000.0));

        // 3. Verify serialize/deserialize roundtrip for all entities
        let proposal_json = serde_json::to_string(&proposal).unwrap();
        let parsed_proposal: Proposal = serde_json::from_str(&proposal_json).unwrap();
        assert_eq!(proposal, parsed_proposal);

        let revision_json = serde_json::to_string(&revision).unwrap();
        let parsed_revision: ProposalRevision = serde_json::from_str(&revision_json).unwrap();
        assert_eq!(revision, parsed_revision);

        // Verify individual items roundtrip
        for item in &revision.items {
            let item_json = serde_json::to_string(item).unwrap();
            let parsed_item: ProposalItem = serde_json::from_str(&item_json).unwrap();
            assert_eq!(item, &parsed_item);
        }
    }

    #[test]
    fn test_pricing_tier_selection() {
        // Create TieredPricing with 4 tiers
        let pricing = TieredPricing::builder()
            .tiers(vec![
                PricingTier::builder()
                    .tier_type(PricingTierType::Public)
                    .discount_percent(0.0)
                    .negotiation_enabled(false)
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Seat)
                    .discount_percent(5.0)
                    .negotiation_enabled(true)
                    .min_spend(Some(1000.0))
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Agency)
                    .discount_percent(10.0)
                    .negotiation_enabled(true)
                    .min_spend(Some(5000.0))
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Advertiser)
                    .discount_percent(15.0)
                    .negotiation_enabled(true)
                    .min_spend(Some(10000.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(pricing.tiers.len(), 4);

        // Verify all tier types
        assert_eq!(pricing.tiers[0].tier_type, PricingTierType::Public);
        assert!(!pricing.tiers[0].negotiation_enabled);
        assert_eq!(pricing.tiers[1].tier_type, PricingTierType::Seat);
        assert_eq!(pricing.tiers[1].discount_percent, 5.0);
        assert_eq!(pricing.tiers[2].tier_type, PricingTierType::Agency);
        assert_eq!(pricing.tiers[2].discount_percent, 10.0);
        assert_eq!(pricing.tiers[3].tier_type, PricingTierType::Advertiser);
        assert_eq!(pricing.tiers[3].discount_percent, 15.0);

        // Verify roundtrip
        let json = serde_json::to_string(&pricing).unwrap();
        let parsed: TieredPricing = serde_json::from_str(&json).unwrap();
        assert_eq!(pricing, parsed);

        // Verify individual tier roundtrip
        for tier in &pricing.tiers {
            let tier_json = serde_json::to_string(tier).unwrap();
            let parsed_tier: PricingTier = serde_json::from_str(&tier_json).unwrap();
            assert_eq!(tier, &parsed_tier);
        }
    }

    #[test]
    fn test_negotiation_rounds() {
        // 1. Create a NegotiationConfig
        let config = NegotiationConfig::builder()
            .max_rounds(5)
            .per_round_concession_cap(0.50)
            .total_concession_cap(2.00)
            .strategy(NegotiationStrategyType::Collaborative)
            .build()
            .unwrap();

        assert_eq!(config.max_rounds, 5);
        assert_eq!(config.per_round_concession_cap, 0.50);
        assert_eq!(config.total_concession_cap, 2.00);
        assert_eq!(config.strategy, NegotiationStrategyType::Collaborative);

        // 2. Create 3 NegotiationRounds simulating a multi-round negotiation
        // Round 1: Buyer offers low, seller holds firm
        let round1 = NegotiationRound::builder()
            .round_number(1)
            .buyer_price(2.50)
            .seller_price(4.00)
            .concession(0.0)
            .accepted(false)
            .build()
            .unwrap();

        // Round 2: Buyer raises offer, seller concedes slightly
        let round2 = NegotiationRound::builder()
            .round_number(2)
            .buyer_price(3.00)
            .seller_price(3.50)
            .concession(0.50)
            .accepted(false)
            .build()
            .unwrap();

        // Round 3: Agreement reached
        let round3 = NegotiationRound::builder()
            .round_number(3)
            .buyer_price(3.25)
            .seller_price(3.25)
            .concession(0.25)
            .accepted(true)
            .build()
            .unwrap();

        assert!(!round1.accepted);
        assert!(!round2.accepted);
        assert!(round3.accepted);
        assert_eq!(round3.buyer_price, round3.seller_price);

        // 3. Verify roundtrip for all entities
        let config_json = serde_json::to_string(&config).unwrap();
        let parsed_config: NegotiationConfig = serde_json::from_str(&config_json).unwrap();
        assert_eq!(config, parsed_config);

        let rounds = [&round1, &round2, &round3];
        for round in &rounds {
            let json = serde_json::to_string(round).unwrap();
            let parsed: NegotiationRound = serde_json::from_str(&json).unwrap();
            assert_eq!(*round, &parsed);
        }
    }

    #[test]
    fn test_seller_order_lifecycle() {
        // Happy path: Draft → Submitted → PendingApproval → Approved →
        //   InProgress → Syncing → Booked → Completed
        let happy_path = [
            (
                SellerOrderStatus::Draft,
                SellerOrderStatus::Submitted,
                "Order created and submitted",
            ),
            (
                SellerOrderStatus::Submitted,
                SellerOrderStatus::PendingApproval,
                "Submitted for publisher approval",
            ),
            (
                SellerOrderStatus::PendingApproval,
                SellerOrderStatus::Approved,
                "Publisher approved the order",
            ),
            (
                SellerOrderStatus::Approved,
                SellerOrderStatus::InProgress,
                "Order execution started",
            ),
            (
                SellerOrderStatus::InProgress,
                SellerOrderStatus::Syncing,
                "Syncing to ad server",
            ),
            (
                SellerOrderStatus::Syncing,
                SellerOrderStatus::Booked,
                "Successfully booked in ad server",
            ),
            (
                SellerOrderStatus::Booked,
                SellerOrderStatus::Completed,
                "All scheduled activity completed",
            ),
        ];

        let mut current = SellerOrderStatus::Draft;
        for (from, to, reason) in &happy_path {
            assert_eq!(&current, from, "Expected current state to be {:?}", from);
            assert!(
                can_transition_seller_order(from, to),
                "Transition {:?} → {:?} ({}) should be valid",
                from,
                to,
                reason
            );
            current = *to;
        }
        assert_eq!(current, SellerOrderStatus::Completed);

        // Terminal states: no further transitions
        assert!(!can_transition_seller_order(
            &SellerOrderStatus::Completed,
            &SellerOrderStatus::Draft
        ));
        assert!(!can_transition_seller_order(
            &SellerOrderStatus::Cancelled,
            &SellerOrderStatus::Draft
        ));
        assert!(!can_transition_seller_order(
            &SellerOrderStatus::Expired,
            &SellerOrderStatus::Draft
        ));

        // Verify SellerOrderTransition record roundtrip
        let transition = SellerOrderTransition::builder()
            .from(SellerOrderStatus::PendingApproval)
            .to(SellerOrderStatus::Approved)
            .timestamp("2026-04-10T09:00:00Z")
            .reason("Publisher approved after review")
            .actor("publisher-agent-001")
            .audit_note("Auto-approved: meets all criteria")
            .build()
            .unwrap();

        let json = serde_json::to_string(&transition).unwrap();
        let parsed: SellerOrderTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(transition, parsed);
    }

    #[test]
    fn test_enum_serialization_roundtrip() {
        // ProposalStatus
        let proposal_variants = [
            ProposalStatus::Draft,
            ProposalStatus::Submitted,
            ProposalStatus::UnderReview,
            ProposalStatus::Countered,
            ProposalStatus::Accepted,
            ProposalStatus::Rejected,
            ProposalStatus::Expired,
            ProposalStatus::Withdrawn,
        ];
        for variant in &proposal_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: ProposalStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // PricingTierType
        let tier_variants = [
            PricingTierType::Public,
            PricingTierType::Seat,
            PricingTierType::Agency,
            PricingTierType::Advertiser,
        ];
        for variant in &tier_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: PricingTierType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // PackageType
        let package_variants = [PackageType::Curated, PackageType::Dynamic];
        for variant in &package_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: PackageType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // NegotiationStrategyType
        let strategy_variants = [
            NegotiationStrategyType::Aggressive,
            NegotiationStrategyType::Standard,
            NegotiationStrategyType::Collaborative,
            NegotiationStrategyType::Premium,
        ];
        for variant in &strategy_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: NegotiationStrategyType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // ChangeType
        let change_type_variants = [
            ChangeType::DateShift,
            ChangeType::ImpressionAdjustment,
            ChangeType::PriceChange,
            ChangeType::Cancellation,
            ChangeType::CreativeSwap,
        ];
        for variant in &change_type_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: ChangeType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // ChangeSeverity
        let severity_variants = [
            ChangeSeverity::Minor,
            ChangeSeverity::Material,
            ChangeSeverity::Critical,
        ];
        for variant in &severity_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: ChangeSeverity = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // ChangeRequestStatus
        let cr_status_variants = [
            ChangeRequestStatus::Pending,
            ChangeRequestStatus::Approved,
            ChangeRequestStatus::Rejected,
            ChangeRequestStatus::Applied,
        ];
        for variant in &cr_status_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: ChangeRequestStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // AdServerType
        let ad_server_variants = [
            AdServerType::GoogleAdManager,
            AdServerType::FreeWheel,
            AdServerType::Csv,
            AdServerType::Custom,
        ];
        for variant in &ad_server_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: AdServerType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // SyncStatus
        let sync_variants = [
            SyncStatus::Pending,
            SyncStatus::Syncing,
            SyncStatus::Synced,
            SyncStatus::Failed,
            SyncStatus::Stale,
        ];
        for variant in &sync_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: SyncStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // DistributionStatus
        let dist_variants = [
            DistributionStatus::Pending,
            DistributionStatus::Sent,
            DistributionStatus::Confirmed,
            DistributionStatus::Rejected,
            DistributionStatus::Expired,
        ];
        for variant in &dist_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: DistributionStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }

        // SellerOrderStatus
        let order_variants = [
            SellerOrderStatus::Draft,
            SellerOrderStatus::Submitted,
            SellerOrderStatus::PendingApproval,
            SellerOrderStatus::Approved,
            SellerOrderStatus::Rejected,
            SellerOrderStatus::InProgress,
            SellerOrderStatus::Syncing,
            SellerOrderStatus::Booked,
            SellerOrderStatus::Paused,
            SellerOrderStatus::Completed,
            SellerOrderStatus::Failed,
            SellerOrderStatus::Cancelled,
            SellerOrderStatus::Expired,
        ];
        for variant in &order_variants {
            let json = serde_json::to_string(variant).unwrap();
            let parsed: SellerOrderStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, &parsed);
        }
    }
}
