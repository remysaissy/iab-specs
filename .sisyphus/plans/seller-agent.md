# Seller Agent — Supply-Side Agent Models (seller_agent_10)

## TL;DR

> **Quick Summary**: Implement the Seller Agent specification as a new `seller_agent_10` feature, providing Rust data models for supply-side inventory management, proposals, tiered pricing, negotiation, media kits, change requests, execution orders, deal distribution, and a full Order Lifecycle state machine (12 states, 21 transitions). Depends on `agentic_direct_21` for shared OpenDirect entities.
> 
> **Deliverables**:
> - New `src/seller_agent/v10/` module
> - Proposal models (Proposal, ProposalRevision, ProposalItem)
> - Pricing models (TieredPricing, PricingTier, RateCard)
> - Negotiation models (NegotiationConfig, NegotiationRound)
> - Media Kit models (MediaKit, Package)
> - Change Request models with severity classification
> - Execution models (ExecutionOrder, ad server types)
> - Deal Distribution models (DspIntegration)
> - Seller Order Lifecycle state machine (12 states, 21 transitions)
> - Feature flag `seller_agent_10 = ["agentic_direct_21"]`
> - Full TDD with ≥80% coverage
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 6 waves
> **Critical Path**: T1 → T2/T3 → T4-T7 → T8-T10 → T11 → T12

---

## Prerequisites

> ⚠️ **This plan MUST be executed AFTER `.sisyphus/plans/agentic-direct.md`.**
> The `agentic_direct_21` feature, module, and all shared types (Account, Order, Line, Product, Creative, RateType, etc.) must already exist in the codebase before this plan begins.

---

## Context

### Original Request
Implement each of the 6 AAMP specifications as Rust data model modules in the `iab-specs` crate.

### Interview Summary
- Seller Agent depends on `agentic_direct_21` for shared types (Product, Order, etc.)
- ALL data models including application-level types
- Full state machines, string enums, TDD, derive_builder always

### Research Findings
- Reference impl is Python/FastAPI with CrewAI 3-level agent hierarchy
- 82 API endpoints, 41 MCP tools — we implement only the DATA MODELS
- Order Lifecycle has 12 states with 21 transitions
- Tiered pricing with 4 buyer tiers (Public, Seat, Agency, Advertiser)
- Change request severity: Minor (auto-approve), Material (review), Critical (senior review)

---

## Work Objectives

### Must Have
- Proposal + ProposalRevision + ProposalItem with status tracking
- TieredPricing + PricingTier + RateCard
- NegotiationConfig + NegotiationRound with strategy types
- MediaKit + Package
- ChangeRequest with ChangeType, ChangeSeverity, ChangeRequestStatus
- ExecutionOrder with AdServerType and SyncStatus
- DealDistribution + DspIntegration
- Seller Order Lifecycle state machine (12 states, 21 transitions)
- Re-exports from `agentic_direct_21`, string enums, Extension trait

### Must NOT Have (Guardrails)
- No PricingRulesEngine logic — only data models
- No NegotiationEngine strategy execution
- No ad server adapter implementations (GAM, FreeWheel)
- No FastAPI server or MCP server
- No event bus or storage layer
- No CrewAI agent hierarchy
- No type duplication from `agentic_direct_21`

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — TDD, `cargo test`, ≥80% coverage.

### TDD Workflow (MANDATORY for every task)

Every task that introduces new types or enums MUST follow this workflow:

1. **RED**: Write tests FIRST that define the expected behavior. Tests MUST fail before implementation.
   - For structs: creation, serialization, deserialization, roundtrip, default (5 tests minimum)
   - For enums: all_valid_values, invalid_value_rejected, serialization_roundtrip, default_value (4 tests minimum)
   - For state machines: every valid transition, at least 1 invalid transition per state, terminal states have no exits (exhaustive coverage)
2. **GREEN**: Implement the minimum code to make tests pass.
3. **REFACTOR**: Clean up while keeping all tests green.

### Invalid/Negative Test Requirement (MANDATORY)

Every type MUST have both valid AND invalid tests:

- **Enums**: Test that invalid/unknown string values are REJECTED by serde (e.g., `"nonexistent_status"` → `Err`)
- **State machines**: Test that INVALID transitions return `false` — at least 1 invalid transition test per state
- **Structs with required fields**: Test that building without required fields produces an error
- **Serialization**: Test that malformed JSON is rejected during deserialization

### Coverage Gate (MANDATORY)

Coverage MUST be checked before the final task's commit and MUST meet ≥80% threshold:

```bash
cargo llvm-cov --no-default-features --features seller_agent_10 --fail-under-lines 80
```

### Commit Gating Policy (MANDATORY — NO EXCEPTIONS)

**No commit may be created unless ALL of the following pass:**

1. `cargo test --no-default-features --features seller_agent_10` — **ALL tests pass** (zero failures)
2. `cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` — **zero warnings**
3. For the FINAL task only: additionally run `cargo llvm-cov --no-default-features --features seller_agent_10 --fail-under-lines 80` or `./coverage.sh --check-thresholds`

**If any test fails or coverage is below 80%, the commit MUST NOT proceed.** Fix the issue first, then re-run.

---

## Execution Strategy

```
Wave 1: Task 1 — Cargo.toml + skeleton [quick]

Wave 2 (Enums — all parallel):
├── Task 2: Proposal + Pricing enums (ProposalStatus, PricingTierType, PackageType) [quick]
├── Task 3: Change + Execution + Distribution enums (ChangeType, ChangeSeverity, AdServerType, SyncStatus, etc.) [quick]

Wave 3 (Core models — all parallel):
├── Task 4: Proposal + ProposalRevision + ProposalItem [unspecified-high]
├── Task 5: TieredPricing + PricingTier + RateCard [quick]
├── Task 6: NegotiationConfig + NegotiationRound [quick]
├── Task 7: MediaKit + Package [quick]

Wave 4 (Secondary models — all parallel):
├── Task 8: ChangeRequest [quick]
├── Task 9: ExecutionOrder [quick]
├── Task 10: DealDistribution + DspIntegration [quick]

Wave 5: Task 11 — Seller Order Lifecycle state machine [deep]

Wave 6: Task 12 — Integration tests + doc examples + README update [unspecified-high]

Wave FINAL: F1-F4 — 4 parallel review agents
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 2-12 | 1 |
| 2 | 1 | 4-7, 11 | 2 |
| 3 | 1 | 8-10, 11 | 2 |
| 4 | 2 | 12 | 3 |
| 5 | 2 | 12 | 3 |
| 6 | 2 | 12 | 3 |
| 7 | 2 | 12 | 3 |
| 8 | 3 | 12 | 4 |
| 9 | 3 | 12 | 4 |
| 10 | 3 | 12 | 4 |
| 11 | 2, 3 | 12 | 5 |
| 12 | 4-11 | F1-F4 | 6 |

---

## TODOs

- [ ] 1. Feature Flag + Module Skeleton

  **What to do**:
  - Add `seller_agent_10 = ["agentic_direct_21"]` to `Cargo.toml`
  - Create `src/seller_agent/mod.rs`, `src/seller_agent/v10/mod.rs` with submodules
  - Add `#[cfg(feature = "seller_agent_10")] pub mod seller_agent;` to `src/lib.rs`
  - Re-export shared types from `crate::agentic_direct::v21`
  - Verify: `cargo check --no-default-features --features seller_agent_10`

  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 1 — Blocks all — Blocked By: None

  **References**: `src/artb/mod.rs`, `Cargo.toml:38-39`

  **QA Scenarios:**
  ```
  Scenario: Feature compiles with agentic_direct dependency
    Tool: Bash
    Steps: cargo check --no-default-features --features seller_agent_10
    Expected Result: Compiles, zero warnings
    Evidence: .sisyphus/evidence/task-1-seller-skeleton.txt
  ```

  **Commit**: `feat(seller_agent): add feature flag and module skeleton`
  - Pre-commit: `cargo check --no-default-features --features seller_agent_10 && cargo check --all-features`

- [ ] 2. Proposal + Pricing + Media Kit Enums

  **What to do**:
  - **ProposalStatus**: `Draft`, `Submitted`, `UnderReview`, `Countered`, `Accepted`, `Rejected`, `Expired`, `Withdrawn`
  - **PricingTierType**: `Public`, `Seat`, `Agency`, `Advertiser`
  - **PackageType**: `Curated`, `Dynamic`
  - **NegotiationStrategyType**: `Aggressive`, `Standard`, `Collaborative`, `Premium`
  - String serialization, `#[serde(rename_all = "snake_case")]`, standard 4 tests per enum

  **QA Scenarios:**
  ```
  Scenario: ProposalStatus roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Serialize ProposalStatus::UnderReview → assert "under_review"
      2. Deserialize "under_review" → assert ProposalStatus::UnderReview
    Expected Result: String-based roundtrip works
    Evidence: .sisyphus/evidence/task-2-proposal-status.txt
  ```

  **Commit**: `feat(seller_agent): add proposal, pricing, and media kit enums`

- [ ] 3. Change Request + Execution + Distribution Enums

  **What to do**:
  - **ChangeType**: `DateShift`, `ImpressionAdjustment`, `PriceChange`, `Cancellation`, `CreativeSwap`
  - **ChangeSeverity**: `Minor`, `Material`, `Critical`
  - **ChangeRequestStatus**: `Pending`, `Approved`, `Rejected`, `Applied`
  - **AdServerType**: `GoogleAdManager`, `FreeWheel`, `Csv`, `Custom`
  - **SyncStatus**: `Pending`, `Syncing`, `Synced`, `Failed`, `Stale`
  - **DistributionStatus**: `Pending`, `Sent`, `Confirmed`, `Rejected`, `Expired`
  - **SellerOrderStatus** (13 variants): `Draft`, `Submitted`, `PendingApproval`, `Approved`, `Rejected`, `InProgress`, `Syncing`, `Booked`, `Paused`, `Completed`, `Failed`, `Cancelled`, `Expired`
  - String serialization, standard tests

  **QA Scenarios:**
  ```
  Scenario: ChangeSeverity enum roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Serialize ChangeSeverity::Critical → assert "critical"
      2. Serialize AdServerType::GoogleAdManager → assert "google_ad_manager"
      3. Deserialize all variants back, assert equality
    Expected Result: All enums serialize as snake_case strings
    Evidence: .sisyphus/evidence/task-3-change-enums.txt
  ```

  **Commit**: `feat(seller_agent): add change request, execution, and distribution enums`

- [ ] 4. Proposal + ProposalRevision + ProposalItem

  **What to do**:
  - **Proposal**: `id: Option<String>`, `buyer_id: String` (required), `seller_id: String` (required), `status: ProposalStatus`, `current_revision_id: Option<String>`, `created_at: Option<String>`, `updated_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **ProposalRevision**: `id: Option<String>`, `proposal_id: String` (required), `revision_number: i32` (required), `items: Vec<ProposalItem>`, `total_budget: Option<f64>`, `notes: Option<String>`, `created_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **ProposalItem**: `product_id: String` (required), `quantity: i64` (required), `rate: f64` (required), `rate_type: crate::agentic_direct::v21::RateType` (re-export), `start_date: String` (required), `end_date: String` (required), `ext: Option<Box<Ext>>`
  - Extension support, builder, standard tests + test with multiple revision items

  **Recommended Agent Profile**: `unspecified-high`

  **QA Scenarios:**
  ```
  Scenario: Proposal with multiple revision items
    Tool: Bash (cargo test)
    Steps:
      1. Create Proposal → ProposalRevision with 3 ProposalItems
      2. Serialize/deserialize roundtrip
      3. Verify nested structure preserved
    Expected Result: Nested proposal hierarchy roundtrips
    Evidence: .sisyphus/evidence/task-4-proposal.txt
  ```

  **Commit**: `feat(seller_agent): add Proposal, ProposalRevision, and ProposalItem`

- [ ] 5. TieredPricing + PricingTier + RateCard

  **What to do**:
  - **TieredPricing**: `tiers: Vec<PricingTier>`, `ext: Option<Box<Ext>>`
  - **PricingTier**: `tier_type: PricingTierType` (required), `discount_percent: f64` (required), `negotiation_enabled: bool` (required), `min_spend: Option<f64>`, `ext: Option<Box<Ext>>`
  - **RateCard**: `product_id: String` (required), `base_cpm: f64` (required), `floor_cpm: f64` (required), `ceiling_cpm: Option<f64>`, `currency: String` (required), `ext: Option<Box<Ext>>`
  - Standard tests including tier ordering test

  **QA Scenarios:**
  ```
  Scenario: TieredPricing with 4 tiers
    Tool: Bash (cargo test)
    Steps:
      1. Create TieredPricing with 4 PricingTiers (Public 0%, Seat 5%, Agency 10%, Advertiser 15%)
      2. Serialize/deserialize roundtrip
      3. Assert tier order and discount values preserved
    Expected Result: Nested tier array roundtrips correctly
    Evidence: .sisyphus/evidence/task-5-tiered-pricing.txt
  ```

  **Commit**: `feat(seller_agent): add TieredPricing, PricingTier, and RateCard`

- [ ] 6. NegotiationConfig + NegotiationRound

  **What to do**:
  - **NegotiationConfig**: `max_rounds: i32` (required), `per_round_concession_cap: f64` (required), `total_concession_cap: f64` (required), `strategy: NegotiationStrategyType`, `ext: Option<Box<Ext>>`
  - **NegotiationRound**: `round_number: i32` (required), `buyer_price: f64` (required), `seller_price: f64` (required), `concession: f64` (required), `accepted: bool`, `ext: Option<Box<Ext>>`
  - Standard tests

  **QA Scenarios:**
  ```
  Scenario: NegotiationRound roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Create NegotiationRound with round_number=3, buyer_price=4.50, seller_price=5.00, concession=0.50, accepted=false
      2. Serialize/deserialize roundtrip
    Expected Result: All f64 fields preserved
    Evidence: .sisyphus/evidence/task-6-negotiation-round.txt
  ```

  **Commit**: `feat(seller_agent): add NegotiationConfig and NegotiationRound`

- [ ] 7. MediaKit + Package

  **What to do**:
  - **MediaKit**: `publisher_id: String` (required), `packages: Vec<Package>`, `updated_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **Package**: `id: Option<String>`, `name: String` (required), `description: Option<String>`, `product_ids: Vec<String>`, `bundle_price: Option<f64>`, `package_type: PackageType`, `ext: Option<Box<Ext>>`
  - Standard tests

  **QA Scenarios:**
  ```
  Scenario: MediaKit with packages
    Tool: Bash (cargo test)
    Steps:
      1. Create MediaKit with publisher_id and 2 Packages (Curated, Dynamic)
      2. Serialize/deserialize roundtrip
    Expected Result: MediaKit with nested packages roundtrips
    Evidence: .sisyphus/evidence/task-7-media-kit.txt
  ```

  **Commit**: `feat(seller_agent): add MediaKit and Package`

- [ ] 8. ChangeRequest

  **What to do**:
  - **ChangeRequest**: `id: Option<String>`, `order_id: String` (required), `change_type: ChangeType` (required), `severity: ChangeSeverity` (required), `description: String` (required), `requested_changes: serde_json::Value` (required), `status: ChangeRequestStatus`, `reviewer: Option<String>`, `reviewed_at: Option<String>`, `ext: Option<Box<Ext>>`
  - Extension support, standard tests including severity-based test

  **QA Scenarios:**
  ```
  Scenario: ChangeRequest with Critical severity and JSON changes
    Tool: Bash (cargo test)
    Steps:
      1. Create ChangeRequest with severity=Critical, requested_changes=json!({"price_change": 25.0})
      2. Serialize/deserialize roundtrip
    Expected Result: Severity and JSON changes preserved
    Evidence: .sisyphus/evidence/task-8-change-request.txt
  ```

  **Commit**: `feat(seller_agent): add ChangeRequest model`

- [ ] 9. ExecutionOrder

  **What to do**:
  - **ExecutionOrder**: `id: Option<String>`, `order_id: String` (required), `ad_server_type: AdServerType` (required), `ad_server_order_id: Option<String>`, `sync_status: SyncStatus`, `last_synced_at: Option<String>`, `error_message: Option<String>`, `ext: Option<Box<Ext>>`
  - Standard tests

  **QA Scenarios:**
  ```
  Scenario: ExecutionOrder with sync status
    Tool: Bash (cargo test)
    Steps:
      1. Create ExecutionOrder with ad_server_type=GoogleAdManager, sync_status=Syncing
      2. Serialize → assert "google_ad_manager" and "syncing" strings
      3. Deserialize back, assert equality
    Expected Result: Enum fields serialize as strings
    Evidence: .sisyphus/evidence/task-9-execution-order.txt
  ```

  **Commit**: `feat(seller_agent): add ExecutionOrder model`

- [ ] 10. DealDistribution + DspIntegration

  **What to do**:
  - **DealDistribution**: `deal_id: String` (required), `buyer_seats: Vec<String>`, `dsp_integrations: Vec<DspIntegration>`, `distributed_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **DspIntegration**: `dsp_name: String` (required), `seat_id: String` (required), `deal_id_at_dsp: Option<String>`, `status: DistributionStatus`, `ext: Option<Box<Ext>>`
  - Standard tests

  **QA Scenarios:**
  ```
  Scenario: DealDistribution with DSP integrations
    Tool: Bash (cargo test)
    Steps:
      1. Create DealDistribution with 2 DspIntegrations (PubMatic, IndexExchange)
      2. Serialize/deserialize roundtrip
    Expected Result: Nested DSP integrations roundtrip
    Evidence: .sisyphus/evidence/task-10-deal-distribution.txt
  ```

  **Commit**: `feat(seller_agent): add DealDistribution and DspIntegration models`

- [ ] 11. Seller Order Lifecycle State Machine

  **What to do**:
  - Create `src/seller_agent/v10/state_machines/seller_order_state_machine.rs`
  - `const VALID_SELLER_ORDER_TRANSITIONS: &[(SellerOrderStatus, SellerOrderStatus)]` — explicit transition pairs (enumerate all, do NOT use "any→X" shorthand):
    - Draft→Submitted, Draft→Cancelled
    - Submitted→PendingApproval, Submitted→Rejected, Submitted→Cancelled
    - PendingApproval→Approved, PendingApproval→Rejected, PendingApproval→Cancelled
    - Approved→InProgress, Approved→Cancelled
    - Rejected→Cancelled
    - InProgress→Syncing, InProgress→Paused, InProgress→Cancelled, InProgress→Expired
    - Syncing→Booked, Syncing→Failed, Syncing→Cancelled
    - Booked→InProgress, Booked→Paused, Booked→Completed, Booked→Cancelled
    - Paused→InProgress, Paused→Cancelled
    - Failed→InProgress, Failed→Cancelled
  - `pub fn can_transition_seller_order(from: &SellerOrderStatus, to: &SellerOrderStatus) -> bool`
  - `pub fn valid_seller_order_transitions_from(state: &SellerOrderStatus) -> Vec<SellerOrderStatus>`
  - **SellerOrderTransition** struct: `from`, `to`, `timestamp`, `reason`, `actor`, `audit_note`
  - Test all 21 valid transitions + invalid transitions + terminal state tests

  **Recommended Agent Profile**: `deep`

  **Acceptance Criteria**:
  - [ ] All 21 valid transitions return true
  - [ ] Terminal states (Completed, Cancelled, Expired) have no outgoing transitions
  - [ ] Syncing→Failed→InProgress (retry path) works

  **QA Scenarios:**
  ```
  Scenario: Full seller order happy path
    Tool: Bash (cargo test)
    Steps:
      1. Test: Draft→Submitted→PendingApproval→Approved→InProgress→Syncing→Booked→Completed
      2. Assert each returns true
    Expected Result: Full lifecycle path succeeds
    Evidence: .sisyphus/evidence/task-11-seller-order-happy.txt

  Scenario: Retry after sync failure
    Tool: Bash (cargo test)
    Steps:
      1. Test: Syncing→Failed (valid)
      2. Test: Failed→InProgress (retry, valid)
      3. Test: Failed→Completed (invalid)
    Expected Result: Retry path works, skip-ahead blocked
    Evidence: .sisyphus/evidence/task-11-seller-order-retry.txt
  ```

  **Commit**: `feat(seller_agent): add Seller Order Lifecycle state machine`

- [ ] 12. Integration Tests + Doc Examples + README Update

  **What to do**:
  - Integration tests: proposal flow, pricing tier selection, negotiation rounds, order lifecycle
  - Doc examples in module docs
  - Update README.md
  - Final: `cargo test --all-features && cargo clippy --all-features -- -D warnings`

  **QA Scenarios:**
  ```
  Scenario: Full verification suite
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo test --all-features`
      2. Run `cargo clippy --all-features -- -D warnings`
      3. Run `cargo test --doc --no-default-features --features seller_agent_10`
      4. Assert all pass with zero warnings/failures
    Expected Result: Complete clean build + test + lint
    Evidence: .sisyphus/evidence/task-12-seller-full-verification.txt
  ```

  **Commit**: `feat(seller_agent): add integration tests, doc examples, and README update`

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
- [ ] F2. **Code Quality Review** — `unspecified-high`
- [ ] F3. **Real Manual QA** — `unspecified-high`
- [ ] F4. **Scope Fidelity Check** — `deep`

---

## Commit Strategy

> **EVERY commit** in this plan MUST pass the following before being created:
> ```bash
> cargo test --no-default-features --features seller_agent_10 && \
> cargo clippy --no-default-features --features seller_agent_10 -- -D warnings
> ```
> **The FINAL commit** (Task 12) MUST additionally pass:
> ```bash
> cargo test --all-features && \
> cargo clippy --all-features -- -D warnings && \
> cargo llvm-cov --no-default-features --features seller_agent_10 --fail-under-lines 80
> ```
> **If any test fails, any clippy warning fires, or coverage is below 80%, the commit MUST NOT be created.** Fix first, then re-run.

| Task | Message | Pre-commit |
|------|---------|------------|
| 1 | `feat(seller_agent): add feature flag and module skeleton` | `cargo check --no-default-features --features seller_agent_10 && cargo check --all-features` |
| 2 | `feat(seller_agent): add proposal, pricing, and media kit enums` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 3 | `feat(seller_agent): add change request, execution, and distribution enums` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 4 | `feat(seller_agent): add Proposal, ProposalRevision, and ProposalItem` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 5 | `feat(seller_agent): add TieredPricing, PricingTier, and RateCard` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 6 | `feat(seller_agent): add NegotiationConfig and NegotiationRound` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 7 | `feat(seller_agent): add MediaKit and Package` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 8 | `feat(seller_agent): add ChangeRequest model` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 9 | `feat(seller_agent): add ExecutionOrder model` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 10 | `feat(seller_agent): add DealDistribution and DspIntegration` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 11 | `feat(seller_agent): add Seller Order Lifecycle state machine` | `cargo test --no-default-features --features seller_agent_10 && cargo clippy --no-default-features --features seller_agent_10 -- -D warnings` |
| 12 | `feat(seller_agent): add integration tests, doc examples, README` | `cargo test --all-features && cargo clippy --all-features -- -D warnings && cargo llvm-cov --no-default-features --features seller_agent_10 --fail-under-lines 80` |

---

## Success Criteria

```bash
cargo check --no-default-features --features seller_agent_10
cargo test --no-default-features --features seller_agent_10
cargo clippy --no-default-features --features seller_agent_10 -- -D warnings
cargo test --doc --no-default-features --features seller_agent_10
cargo check --all-features
cargo llvm-cov --no-default-features --features seller_agent_10 --fail-under-lines 80
```

- [ ] All seller-agent-unique types present
- [ ] Shared types re-exported from agentic_direct_21
- [ ] Seller Order state machine (13 states, full transition validation)
- [ ] String-based enums, Extension trait
- [ ] ≥80% line coverage (enforced by `cargo llvm-cov --fail-under-lines 80`)
- [ ] Every enum has invalid value rejection tests
- [ ] Every state machine has invalid transition tests (at least 1 per state)
