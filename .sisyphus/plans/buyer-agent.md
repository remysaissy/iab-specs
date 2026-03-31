# Buyer Agent — Demand-Side Agent Models (buyer_agent_10)

## TL;DR

> **Quick Summary**: Implement the Buyer Agent specification as a new `buyer_agent_10` feature, providing Rust data models for demand-side campaign planning, UCP embeddings, negotiation strategies, booking workflows, and two full state machines (Deal Lifecycle — 12 states, Campaign Lifecycle — 9 states). Depends on `agentic_direct_21` for shared OpenDirect entities.
> 
> **Deliverables**:
> - New `src/buyer_agent/v10/` module
> - Campaign models (CampaignBrief, CampaignAllocation)
> - UCP models (UCPEmbedding, AudiencePlan)
> - Negotiation models (NegotiationStrategy, NegotiationOffer)
> - Booking models (BookingJob, BookingRecommendation)
> - Deal Lifecycle state machine (12 states)
> - Campaign Lifecycle state machine (9 states)
> - Feature flag `buyer_agent_10 = ["agentic_direct_21"]`
> - Full TDD with ≥80% coverage
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 5 waves
> **Critical Path**: T1 → T2/T3 → T4-T7 → T8/T9 → T10

---

## Prerequisites

> ⚠️ **This plan MUST be executed AFTER `.sisyphus/plans/agentic-direct.md`.**
> The `agentic_direct_21` feature, module, and all shared types (Account, Order, Line, Product, Creative, AgentCard, etc.) must already exist in the codebase before this plan begins. Task 1 of this plan adds `buyer_agent_10 = ["agentic_direct_21"]` to Cargo.toml, which requires `agentic_direct_21` to already be defined there.

---

## Context

### Original Request
Implement each of the 6 AAMP specifications as Rust data model modules in the `iab-specs` crate.

### Interview Summary
- Buyer Agent depends on `agentic_direct_21` for shared OpenDirect types (Account, Order, Line, Product, Creative)
- ALL data models including application-level types (CampaignBrief, NegotiationStrategy, etc.)
- Full state machines with transition validation and side-effect types
- String-based enum serialization
- TDD approach

### Research Findings
- Reference impl is Python/Pydantic (CrewAI agent hierarchy)
- 3-level agent hierarchy is runtime — we only implement data models
- Deal Lifecycle has 12 states including Linear TV-specific states (MakegoodPending, PartiallyCanceled)
- Campaign Lifecycle has 9 states with mandatory human approval gate

---

## Work Objectives

### Core Objective
Implement demand-side data models for campaign planning, audience targeting, negotiation, and booking workflows as idiomatic Rust types.

### Must Have
- CampaignBrief with budget, objectives, target audience, KPIs
- UCPEmbedding and AudiencePlan for audience targeting
- NegotiationStrategy and NegotiationOffer for deal negotiation
- BookingJob and BookingRecommendation for booking workflows
- Deal Lifecycle state machine (12 states, full transition validation)
- Campaign Lifecycle state machine (9 states, full transition validation)
- Re-exports of shared types from `agentic_direct_21`
- Extension trait on all extensible types
- String-based enum serialization

### Must NOT Have (Guardrails)
- No CrewAI agent hierarchy or agent logic
- No LLM integration or AI planning logic
- No HTTP client for seller discovery
- No runtime booking execution
- No type duplication — re-export from `agentic_direct_21`
- No `serde_repr` integer enums
- No `unsafe` code

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: TDD (RED-GREEN-REFACTOR)
- **Framework**: `cargo test`

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
cargo llvm-cov --no-default-features --features buyer_agent_10 --fail-under-lines 80
```

### Commit Gating Policy (MANDATORY — NO EXCEPTIONS)

**No commit may be created unless ALL of the following pass:**

1. `cargo test --no-default-features --features buyer_agent_10` — **ALL tests pass** (zero failures)
2. `cargo clippy --no-default-features --features buyer_agent_10 -- -D warnings` — **zero warnings**
3. For the FINAL task only: additionally run `cargo llvm-cov --no-default-features --features buyer_agent_10 --fail-under-lines 80` or `./coverage.sh --check-thresholds`

**If any test fails or coverage is below 80%, the commit MUST NOT proceed.** Fix the issue first, then re-run.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation):
├── Task 1: Cargo.toml + feature flag + module skeleton [quick]

Wave 2 (Enums — all parallel):
├── Task 2: DealStatus + CampaignStatus enums [quick]
├── Task 3: Strategy/booking enums [quick]

Wave 3 (Core models — all parallel):
├── Task 4: CampaignBrief + CampaignAllocation [quick]
├── Task 5: UCPEmbedding + AudiencePlan [quick]
├── Task 6: NegotiationStrategy + NegotiationOffer [quick]
├── Task 7: BookingJob + BookingRecommendation [quick]

Wave 4 (State machines — parallel):
├── Task 8: Deal Lifecycle state machine [deep]
├── Task 9: Campaign Lifecycle state machine [deep]

Wave 5 (Integration):
├── Task 10: Integration tests + doc examples + README update [unspecified-high]

Wave FINAL:
├── F1-F4: 4 parallel review agents
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 2-10 | 1 |
| 2 | 1 | 8, 9 | 2 |
| 3 | 1 | 7 | 2 |
| 4 | 1 | 10 | 3 |
| 5 | 1 | 10 | 3 |
| 6 | 1 | 10 | 3 |
| 7 | 1, 3 | 10 | 3 |
| 8 | 2 | 10 | 4 |
| 9 | 2 | 10 | 4 |
| 10 | 4-9 | F1-F4 | 5 |

---

## TODOs

- [ ] 1. Feature Flag + Module Skeleton

  **What to do**:
  - Add `buyer_agent_10 = ["agentic_direct_21"]` to `Cargo.toml` features
  - Create `src/buyer_agent/mod.rs` with `pub mod v10;`
  - Create `src/buyer_agent/v10/mod.rs` with submodule declarations (`pub mod enums;`, `pub mod models;`, `pub mod state_machines;`) and re-exports of shared types from `crate::agentic_direct::v21::*`
  - Create empty submodule `mod.rs` files
  - Add `#[cfg(feature = "buyer_agent_10")] pub mod buyer_agent;` to `src/lib.rs`
  - Verify: `cargo check --no-default-features --features buyer_agent_10`

  **Must NOT do**: Do not add types yet — only skeleton

  **Recommended Agent Profile**: `quick`

  **Parallelization**: Wave 1 (solo) — Blocks all others — Blocked By: None

  **References**:
  - `src/artb/mod.rs` — Module root pattern
  - `src/artb/v10/mod.rs` — Submodule organization + re-exports
  - `Cargo.toml:38-39` — Feature dependency pattern (`openrtb_25 = ["adcom"]`)

  **Acceptance Criteria**:
  - [ ] `cargo check --no-default-features --features buyer_agent_10` compiles
  - [ ] `cargo check --all-features` — no conflicts
  - [ ] Feature auto-includes `agentic_direct_21`

  **QA Scenarios:**
  ```
  Scenario: Feature compiles and includes agentic_direct
    Tool: Bash (cargo)
    Steps:
      1. cargo check --no-default-features --features buyer_agent_10
      2. Assert exit code 0
    Expected Result: Compiles with zero warnings
    Evidence: .sisyphus/evidence/task-1-buyer-skeleton.txt
  ```

  **Commit**: `feat(buyer_agent): add feature flag and module skeleton`

- [ ] 2. DealStatus + CampaignStatus Enums

  **What to do**:
  - **DealStatus** (13 variants): `Quoted`, `Negotiating`, `Accepted`, `Booking`, `Booked`, `Delivering`, `Completed`, `Cancelled`, `Rejected`, `Expired`, `Failed`, `MakegoodPending`, `PartiallyCanceled`
  - **CampaignStatus** (9 variants): `Initialized`, `BriefReceived`, `BudgetAllocated`, `Researching`, `AwaitingApproval`, `ExecutingBookings`, `Completed`, `Failed`, `Cancelled`
  - String serialization with `#[serde(rename_all = "snake_case")]`
  - Derive: `Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default`
  - Standard 4 tests per enum

  **References**: `src/ads_txt/seller_relation_type.rs` — string enum pattern

  **Acceptance Criteria**:
  - [ ] Both enums serialize as snake_case strings
  - [ ] All variants roundtrip correctly

  **QA Scenarios:**
  ```
  Scenario: DealStatus MakegoodPending serializes correctly
    Tool: Bash (cargo test)
    Steps:
      1. Serialize MakegoodPending
      2. Assert output is "makegood_pending"
      3. Deserialize "makegood_pending" back
    Expected Result: Roundtrips correctly as snake_case string
    Evidence: .sisyphus/evidence/task-2-deal-status.txt
  ```

  **Commit**: `feat(buyer_agent): add DealStatus and CampaignStatus enums`

- [ ] 3. Strategy and Booking Enums

  **What to do**:
  - **ApprovalStatus** enum: `Pending`, `Approved`, `Rejected`
  - **ChannelType** enum: `Display`, `Video`, `Ctv`, `Mobile`, `Audio`, `Dooh`, `Native`
  - String serialization, standard tests

  **QA Scenarios:**
  ```
  Scenario: ChannelType enum roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Serialize ChannelType::Ctv
      2. Assert output is "ctv"
      3. Deserialize "ctv" back, assert ChannelType::Ctv
    Expected Result: All channel types roundtrip as snake_case strings
    Evidence: .sisyphus/evidence/task-3-channel-type.txt
  ```

  **Commit**: `feat(buyer_agent): add strategy and booking enums`

- [ ] 4. CampaignBrief + CampaignAllocation Models

  **What to do**:
  - **CampaignBrief**: `name: String` (required), `objectives: Vec<String>`, `budget: f64` (required), `start_date: String` (required), `end_date: String` (required), `target_audience: Option<serde_json::Value>`, `kpis: Option<serde_json::Value>`, `channels: Vec<String>`, `constraints: Option<serde_json::Value>`, `ext: Option<Box<Ext>>`
  - **CampaignAllocation**: `channel: String` (required), `budget_share: f64` (required), `priority: i32`, `rationale: Option<String>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard 5 tests per struct

  **References**: `src/artb/v10/rtb_request.rs` — struct with serde_json::Value fields

  **Acceptance Criteria**:
  - [ ] CampaignBrief with arbitrary JSON in target_audience, kpis, constraints
  - [ ] CampaignAllocation with budget_share as f64

  **QA Scenarios:**
  ```
  Scenario: CampaignBrief with targeting JSON
    Tool: Bash (cargo test)
    Steps:
      1. Create CampaignBrief with target_audience = json!({"age": "25-54", "interests": ["sports"]})
      2. Serialize/deserialize roundtrip
      3. Assert targeting blob preserved
    Expected Result: Arbitrary JSON preserved through serde
    Evidence: .sisyphus/evidence/task-4-campaign-brief.txt
  ```

  **Commit**: `feat(buyer_agent): add CampaignBrief and CampaignAllocation models`

- [ ] 5. UCPEmbedding + AudiencePlan Models

  **What to do**:
  - **UCPEmbedding**: `vector: Vec<f32>` (required), `model_descriptor: String` (required), `dimension: i32` (required), `consent: Option<String>`, `ttl: Option<i64>`, `ext: Option<Box<Ext>>`
  - **AudiencePlan**: `query_embedding: Vec<f32>` (required), `coverage_estimates: Option<serde_json::Value>`, `targeting_criteria: Option<serde_json::Value>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard tests
  - Include test with realistic 384-dimensional embedding vector

  **Acceptance Criteria**:
  - [ ] UCPEmbedding handles Vec<f32> with 384+ dimensions
  - [ ] Serialization roundtrip preserves floating point precision

  **QA Scenarios:**
  ```
  Scenario: UCPEmbedding with 384-dim vector
    Tool: Bash (cargo test)
    Steps:
      1. Create UCPEmbedding with 384-element Vec<f32>
      2. Serialize to JSON, verify vector is JSON array of numbers
      3. Deserialize back, assert equality
    Expected Result: Large float vectors roundtrip correctly
    Evidence: .sisyphus/evidence/task-5-ucp-embedding.txt
  ```

  **Commit**: `feat(buyer_agent): add UCPEmbedding and AudiencePlan models`

- [ ] 6. NegotiationStrategy + NegotiationOffer Models

  **What to do**:
  - **NegotiationStrategy**: `target_cpm: f64` (required), `max_cpm: f64` (required), `concession_step: f64` (required), `max_rounds: i32` (required), `ext: Option<Box<Ext>>`
  - **NegotiationOffer**: `price: f64` (required), `round: i32` (required), `from_buyer: bool` (required), `accepted: Option<bool>`, `counter_price: Option<f64>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard tests

  **QA Scenarios:**
  ```
  Scenario: NegotiationStrategy roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Create NegotiationStrategy with target_cpm=2.50, max_cpm=5.00, concession_step=0.25, max_rounds=5
      2. Serialize to JSON, deserialize back, assert equality
    Expected Result: All f64 fields preserve precision through roundtrip
    Evidence: .sisyphus/evidence/task-6-negotiation.txt
  ```

  **Commit**: `feat(buyer_agent): add NegotiationStrategy and NegotiationOffer models`

- [ ] 7. BookingJob + BookingRecommendation Models

  **What to do**:
  - **BookingJob**: `id: Option<String>`, `campaign_brief_id: String` (required), `status: CampaignStatus`, `allocations: Vec<CampaignAllocation>`, `recommendations: Vec<serde_json::Value>`, `approved: Option<bool>`, `approved_by: Option<String>`, `approved_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **BookingRecommendation**: `seller_name: String` (required), `product_id: String` (required), `price: f64` (required), `impressions: i64` (required), `rationale: Option<String>`, `channel: Option<String>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard tests

  **QA Scenarios:**
  ```
  Scenario: BookingJob with recommendations
    Tool: Bash (cargo test)
    Steps:
      1. Create BookingJob with status=Researching, 2 allocations, approved=None
      2. Serialize/deserialize roundtrip
      3. Assert status serializes as "researching" (string)
    Expected Result: BookingJob with nested allocations roundtrips
    Evidence: .sisyphus/evidence/task-7-booking-job.txt
  ```

  **Commit**: `feat(buyer_agent): add BookingJob and BookingRecommendation models`

- [ ] 8. Deal Lifecycle State Machine

  **What to do**:
  - Create `src/buyer_agent/v10/state_machines/deal_state_machine.rs`
  - `const VALID_DEAL_TRANSITIONS: &[(DealStatus, DealStatus)]` — all valid transitions:
    - Quoted→Negotiating, Quoted→Accepted, Quoted→Rejected, Quoted→Expired
    - Negotiating→Accepted, Negotiating→Rejected, Negotiating→Cancelled
    - Accepted→Booking
    - Booking→Booked, Booking→Failed
    - Booked→Delivering
    - Delivering→Completed, Delivering→MakegoodPending, Delivering→PartiallyCanceled
    - MakegoodPending→Delivering, MakegoodPending→Cancelled
    - any→Cancelled (except terminal states)
  - `pub fn can_transition_deal(from: &DealStatus, to: &DealStatus) -> bool`
  - `pub fn valid_deal_transitions_from(state: &DealStatus) -> Vec<DealStatus>`
  - **DealTransition**: `from: DealStatus`, `to: DealStatus`, `timestamp: Option<String>`, `reason: Option<String>`, `actor: Option<String>`
  - Test every valid transition and at least one invalid per state
  - Test terminal states (Completed, Cancelled, Rejected, Expired) have no outgoing transitions

  **Recommended Agent Profile**: `deep`

  **Acceptance Criteria**:
  - [ ] All valid transitions return true from `can_transition_deal`
  - [ ] All invalid transitions return false
  - [ ] Terminal states (Completed, Cancelled, Rejected, Expired) have empty valid_transitions_from

  **QA Scenarios:**
  ```
  Scenario: Deal lifecycle happy path
    Tool: Bash (cargo test)
    Steps:
      1. Test full chain: Quoted→Negotiating→Accepted→Booking→Booked→Delivering→Completed
      2. Assert each step returns true
    Expected Result: Full happy path succeeds
    Evidence: .sisyphus/evidence/task-8-deal-happy-path.txt

  Scenario: Terminal states have no exits
    Tool: Bash (cargo test)
    Steps:
      1. Call valid_deal_transitions_from(Completed)
      2. Assert empty Vec
      3. Repeat for Cancelled, Rejected, Expired
    Expected Result: No transitions from terminal states
    Evidence: .sisyphus/evidence/task-8-deal-terminal.txt
  ```

  **Commit**: `feat(buyer_agent): add Deal Lifecycle state machine`

- [ ] 9. Campaign Lifecycle State Machine

  **What to do**:
  - Create `src/buyer_agent/v10/state_machines/campaign_state_machine.rs`
  - `const VALID_CAMPAIGN_TRANSITIONS: &[(CampaignStatus, CampaignStatus)]`:
    - Initialized→BriefReceived
    - BriefReceived→BudgetAllocated
    - BudgetAllocated→Researching
    - Researching→AwaitingApproval
    - AwaitingApproval→ExecutingBookings, AwaitingApproval→Researching (rejection → re-research)
    - ExecutingBookings→Completed, ExecutingBookings→Failed
    - any→Cancelled (except terminal states)
  - `pub fn can_transition_campaign(from: &CampaignStatus, to: &CampaignStatus) -> bool`
  - `pub fn valid_campaign_transitions_from(state: &CampaignStatus) -> Vec<CampaignStatus>`
  - **CampaignTransition** struct
  - Exhaustive tests

  **Recommended Agent Profile**: `deep`

  **QA Scenarios:**
  ```
  Scenario: Campaign lifecycle happy path
    Tool: Bash (cargo test)
    Steps:
      1. Test full chain: Initialized→BriefReceived→BudgetAllocated→Researching→AwaitingApproval→ExecutingBookings→Completed
      2. Assert each step returns true
    Expected Result: Full happy path succeeds
    Evidence: .sisyphus/evidence/task-9-campaign-happy.txt

  Scenario: Approval rejection loops back to research
    Tool: Bash (cargo test)
    Steps:
      1. Test: AwaitingApproval→Researching (valid — rejection re-research)
      2. Test: AwaitingApproval→Completed (invalid — can't skip execution)
    Expected Result: Rejection path works, skip-ahead blocked
    Evidence: .sisyphus/evidence/task-9-campaign-rejection.txt
  ```

  **Commit**: `feat(buyer_agent): add Campaign Lifecycle state machine`

- [ ] 10. Integration Tests + Doc Examples + README Update

  **What to do**:
  - Integration tests in `src/buyer_agent/v10/mod.rs`:
    - Complete campaign workflow: CampaignBrief → allocations → recommendations → approval → booking
    - Deal negotiation flow: Strategy → offers → acceptance → state transitions
    - UCP embedding creation + audience plan
  - Doc examples in module docs
  - Update README.md: add Buyer Agent to supported specs, usage example, feature list, roadmap
  - Final verification: `cargo test --all-features && cargo clippy --all-features -- -D warnings`

  **QA Scenarios:**
  ```
  Scenario: Full verification suite
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo test --all-features`
      2. Run `cargo clippy --all-features -- -D warnings`
      3. Run `cargo test --doc --no-default-features --features buyer_agent_10`
      4. Assert all pass with zero warnings/failures
    Expected Result: Complete clean build + test + lint
    Evidence: .sisyphus/evidence/task-10-buyer-full-verification.txt
  ```

  **Commit**: `feat(buyer_agent): add integration tests, doc examples, and README update`

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
- [ ] F2. **Code Quality Review** — `unspecified-high`
- [ ] F3. **Real Manual QA** — `unspecified-high`
- [ ] F4. **Scope Fidelity Check** — `deep`

---

## Commit Strategy

| Task | Message | Pre-commit |
|------|---------|------------|
| 1 | `feat(buyer_agent): add feature flag and module skeleton` | `cargo check --no-default-features --features buyer_agent_10` |
| 2 | `feat(buyer_agent): add DealStatus and CampaignStatus enums` | `cargo test --no-default-features --features buyer_agent_10` |
| 3 | `feat(buyer_agent): add strategy and booking enums` | `cargo test --no-default-features --features buyer_agent_10` |
| 4 | `feat(buyer_agent): add CampaignBrief and CampaignAllocation` | `cargo test --no-default-features --features buyer_agent_10` |
| 5 | `feat(buyer_agent): add UCPEmbedding and AudiencePlan` | `cargo test --no-default-features --features buyer_agent_10` |
| 6 | `feat(buyer_agent): add NegotiationStrategy and NegotiationOffer` | `cargo test --no-default-features --features buyer_agent_10` |
| 7 | `feat(buyer_agent): add BookingJob and BookingRecommendation` | `cargo test --no-default-features --features buyer_agent_10` |
| 8 | `feat(buyer_agent): add Deal Lifecycle state machine` | `cargo test --no-default-features --features buyer_agent_10` |
| 9 | `feat(buyer_agent): add Campaign Lifecycle state machine` | `cargo test --no-default-features --features buyer_agent_10` |
| 10 | `feat(buyer_agent): add integration tests and README update` | `cargo test --all-features && cargo clippy --all-features -- -D warnings && cargo llvm-cov --no-default-features --features buyer_agent_10 --fail-under-lines 80` |

---

## Success Criteria

### Verification Commands
```bash
cargo check --no-default-features --features buyer_agent_10
cargo test --no-default-features --features buyer_agent_10
cargo clippy --no-default-features --features buyer_agent_10 -- -D warnings
cargo test --doc --no-default-features --features buyer_agent_10
cargo check --all-features
```

### Final Checklist
- [ ] All buyer-agent-unique types present with builder + serde
- [ ] Shared types re-exported from agentic_direct_21
- [ ] Deal Lifecycle state machine (13 states, full validation)
- [ ] Campaign Lifecycle state machine (9 states, full validation)
- [ ] String-based enum serialization
- [ ] Extension trait on extensible types
- [ ] ≥80% line coverage (enforced by `cargo llvm-cov --fail-under-lines 80`)
- [ ] Every enum has invalid value rejection tests (unknown strings → `Err`)
- [ ] Every state machine has invalid transition tests (at least 1 per state)
- [ ] Every struct has malformed JSON rejection tests
