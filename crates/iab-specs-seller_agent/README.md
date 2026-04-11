# iab-specs-seller_agent

Seller Agent 1.0 support for the [iab-specs](https://crates.io/crates/iab-specs) ecosystem.

## Overview

Implements the Seller Agent 1.0 specification for autonomous supply-side inventory management, proposal generation, tiered pricing, negotiation, and order execution in programmatic advertising.

- **Proposal management** — Proposal, ProposalRevision, ProposalItem
- **Tiered pricing** — TieredPricing, PricingTier, RateCard, PricingRule, VolumeDiscount
- **Negotiation** — NegotiationConfig, NegotiationRound
- **Inventory packaging** — MediaKit, Package, PublicPackageView, AuthenticatedPackageView
- **Order execution** — ExecutionOrder, DealDistribution, DspIntegration
- **Change management** — ChangeRequest, ChangeType, ChangeSeverity
- **State machine** — SellerOrder lifecycle (13 states) with pause/resume support
- **Re-exports** — All Agentic Direct 2.1 types for seamless integration

## License

Apache-2.0
