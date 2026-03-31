# Registry Agent — Agent Discovery Models (registry_agent_10)

## TL;DR

> **Quick Summary**: Implement the Registry Agent specification as a new `registry_agent_10` feature, providing Rust data models for agent registration, trust lifecycle, and discovery/search. This is a **thin specification** — the Registry Agent is mostly a wrapper around an external MCP server, so its unique data models are minimal. Depends on `agentic_direct_21` for Agent Card and A2A types.
> 
> **Deliverables**:
> - New `src/registry_agent/v10/` module
> - AgentRegistration + RegistryMetadata
> - Trust Lifecycle enums + state machine (TrustLevel, VerificationStatus)
> - Search/Discovery models (RegistrySearchFilter, RegistrySearchResult, RegistryEntry)
> - Feature flag `registry_agent_10 = ["agentic_direct_21"]`
> 
> **Estimated Effort**: Quick
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: T1 → T2/T3 → T4/T5 → T6

---

## Prerequisites

> ⚠️ **This plan MUST be executed AFTER `.sisyphus/plans/agentic-direct.md`.**
> The `agentic_direct_21` feature, module, and shared types (AgentCard, Skill, ProtocolType, etc.) must already exist before this plan begins.

---

## Context

### Research Findings
- Registry Agent is a TypeScript ADK agent wrapping a Tech Lab registry MCP server
- Its only unique data models are: registration, trust lifecycle, and search/filter types
- Agent Card, Skills, and A2A types are already in `agentic_direct_21`
- Trust lifecycle: Unknown → Registered → Approved → Preferred, with Suspended/Revoked states

---

## Work Objectives

### Must Have
- AgentRegistration with embedded AgentCard (re-exported from agentic_direct_21)
- TrustLevel enum (6 states) + VerificationStatus enum (5 states)
- Trust Lifecycle state machine with transition validation
- RegistrySearchFilter, RegistrySearchResult, RegistryEntry
- Re-exports of A2A types from agentic_direct_21

### Must NOT Have (Guardrails)
- No MCP client/server implementation
- No registry database or storage
- No search algorithm implementation
- No trust scoring logic
- No ADK (Agent Development Kit) runtime

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
cargo llvm-cov --no-default-features --features registry_agent_10 --fail-under-lines 80
```

### Commit Gating Policy (MANDATORY — NO EXCEPTIONS)

**No commit may be created unless ALL of the following pass:**

1. `cargo test --no-default-features --features registry_agent_10` — **ALL tests pass** (zero failures)
2. `cargo clippy --no-default-features --features registry_agent_10 -- -D warnings` — **zero warnings**
3. For the FINAL task only: additionally run `cargo llvm-cov --no-default-features --features registry_agent_10 --fail-under-lines 80` or `./coverage.sh --check-thresholds`

**If any test fails or coverage is below 80%, the commit MUST NOT proceed.** Fix the issue first, then re-run.

---

## Execution Strategy

```
Wave 1: Task 1 — Cargo.toml + skeleton [quick]
Wave 2 (parallel): Task 2 — Trust enums + state machine [deep], Task 3 — VerificationStatus enum [quick]
Wave 3 (parallel): Task 4 — AgentRegistration + RegistryMetadata [quick], Task 5 — Search/Discovery models [quick]
Wave 4: Task 6 — Integration tests + README update [unspecified-high]
Wave FINAL: F1-F4 — 4 parallel review agents
```

---

## TODOs

- [ ] 1. Feature Flag + Module Skeleton

  **What to do**:
  - Add `registry_agent_10 = ["agentic_direct_21"]` to `Cargo.toml`
  - Create `src/registry_agent/mod.rs`, `src/registry_agent/v10/mod.rs` with submodules + re-exports from agentic_direct
  - Add `#[cfg(feature = "registry_agent_10")] pub mod registry_agent;` to `src/lib.rs`
  - Verify: `cargo check --no-default-features --features registry_agent_10`

  **Recommended Agent Profile**: `quick`

  **QA Scenarios:**
  ```
  Scenario: Feature compiles with agentic_direct dependency
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo check --no-default-features --features registry_agent_10`
      2. Assert exit code 0, zero warnings
    Expected Result: Compiles successfully, auto-includes agentic_direct_21
    Evidence: .sisyphus/evidence/task-1-registry-skeleton.txt
  ```

  **Commit**: `feat(registry_agent): add feature flag and module skeleton`

- [ ] 2. TrustLevel Enum + Trust State Machine

  **What to do**:
  - **TrustLevel** (6 variants): `Unknown`, `Registered`, `Approved`, `Preferred`, `Suspended`, `Revoked`
  - String serialization, `#[serde(rename_all = "snake_case")]`
  - **Trust State Machine** in `trust_state_machine.rs`:
    - `const VALID_TRUST_TRANSITIONS: &[(TrustLevel, TrustLevel)]`:
      - Unknown→Registered
      - Registered→Approved, Registered→Suspended, Registered→Revoked
      - Approved→Preferred, Approved→Suspended, Approved→Revoked
      - Preferred→Suspended, Preferred→Revoked
      - Suspended→Registered (reinstatement)
    - `pub fn can_transition_trust(from: &TrustLevel, to: &TrustLevel) -> bool`
    - `pub fn valid_trust_transitions_from(level: &TrustLevel) -> Vec<TrustLevel>`
    - **TrustTransition** struct: `from`, `to`, `timestamp`, `reason`, `verified_by`
  - Test all valid/invalid transitions. Revoked is terminal (no outgoing).

  **Recommended Agent Profile**: `deep`

  **QA Scenarios:**
  ```
  Scenario: Trust progression happy path
    Tool: Bash (cargo test)
    Steps:
      1. Test: Unknown→Registered→Approved→Preferred
      2. Assert all return true
    Expected Result: Trust escalation path works
    Evidence: .sisyphus/evidence/task-2-trust-happy.txt

  Scenario: Revoked is terminal
    Tool: Bash (cargo test)
    Steps:
      1. Call valid_trust_transitions_from(Revoked)
      2. Assert empty Vec
    Expected Result: No transitions from Revoked
    Evidence: .sisyphus/evidence/task-2-trust-terminal.txt

  Scenario: Suspension and reinstatement
    Tool: Bash (cargo test)
    Steps:
      1. Test: Approved→Suspended (valid)
      2. Test: Suspended→Registered (reinstatement, valid)
      3. Test: Suspended→Approved (invalid — must go through Registered)
    Expected Result: Reinstatement only goes to Registered
    Evidence: .sisyphus/evidence/task-2-trust-reinstate.txt
  ```

  **Commit**: `feat(registry_agent): add TrustLevel enum and Trust state machine`

- [ ] 3. VerificationStatus Enum

  **What to do**:
  - **VerificationStatus** (5 variants): `Unverified`, `Pending`, `Verified`, `Failed`, `Expired`
  - String serialization, standard 4 tests

  **Recommended Agent Profile**: `quick`

  **QA Scenarios:**
  ```
  Scenario: VerificationStatus roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Serialize VerificationStatus::Verified → assert "verified"
      2. Deserialize "expired" → assert VerificationStatus::Expired
    Expected Result: All variants roundtrip as snake_case strings
    Evidence: .sisyphus/evidence/task-3-verification-status.txt
  ```

  **Commit**: `feat(registry_agent): add VerificationStatus enum`

- [ ] 4. AgentRegistration + RegistryMetadata

  **What to do**:
  - **AgentRegistration**: `agent_card: crate::agentic_direct::v21::AgentCard<Ext>` (re-export, required), `registry_id: Option<String>`, `registered_at: Option<String>`, `trust_level: TrustLevel`, `verification_status: VerificationStatus`, `metadata: Option<RegistryMetadata>`, `ext: Option<Box<Ext>>`
  - **RegistryMetadata**: `registry_name: Option<String>`, `registry_url: Option<String>`, `last_verified_at: Option<String>`, `verification_method: Option<String>`
  - Builder, serde, Extension support, standard 5 tests

  **Recommended Agent Profile**: `quick`

  **QA Scenarios:**
  ```
  Scenario: AgentRegistration with embedded AgentCard
    Tool: Bash (cargo test)
    Steps:
      1. Create AgentCard (from agentic_direct), wrap in AgentRegistration
      2. Set trust_level = Registered, verification_status = Verified
      3. Serialize/deserialize roundtrip
    Expected Result: Nested AgentCard roundtrips through registration
    Evidence: .sisyphus/evidence/task-4-registration.txt
  ```

  **Commit**: `feat(registry_agent): add AgentRegistration and RegistryMetadata`

- [ ] 5. Search/Discovery Models

  **What to do**:
  - **RegistrySearchFilter**: `query: Option<String>`, `skill_tags: Vec<String>`, `protocol_types: Vec<crate::agentic_direct::v21::ProtocolType>` (re-export), `trust_levels: Vec<TrustLevel>`, `max_results: Option<i32>`, `ext: Option<Box<Ext>>`
  - **RegistrySearchResult**: `agents: Vec<AgentRegistration<Ext>>`, `total_count: i64` (required), `has_more: bool` (required), `ext: Option<Box<Ext>>`
  - **RegistryEntry**: `id: String` (required), `agent_registration: AgentRegistration<Ext>`, `score: Option<f64>`, `matched_skills: Vec<String>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard tests

  **Recommended Agent Profile**: `quick`

  **QA Scenarios:**
  ```
  Scenario: Search filter with trust level filtering
    Tool: Bash (cargo test)
    Steps:
      1. Create RegistrySearchFilter with trust_levels = [Approved, Preferred]
      2. Serialize/deserialize roundtrip
    Expected Result: Filter roundtrips with enum vec
    Evidence: .sisyphus/evidence/task-5-search-filter.txt
  ```

  **Commit**: `feat(registry_agent): add RegistrySearchFilter, SearchResult, and Entry`

- [ ] 6. Integration Tests + Doc Examples + README Update

  **What to do**:
  - Integration tests: registration flow, trust escalation, search filtering
  - Doc examples in module docs
  - Update README.md
  - Final: `cargo test --all-features && cargo clippy --all-features -- -D warnings`

  **Recommended Agent Profile**: `unspecified-high`

  **QA Scenarios:**
  ```
  Scenario: Full verification suite
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo test --all-features`
      2. Run `cargo clippy --all-features -- -D warnings`
      3. Run `cargo test --doc --no-default-features --features registry_agent_10`
      4. Assert all pass
    Expected Result: Complete clean build + test + lint
    Evidence: .sisyphus/evidence/task-6-registry-full-verification.txt
  ```

  **Commit**: `feat(registry_agent): add integration tests, doc examples, and README update`

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
> cargo test --no-default-features --features registry_agent_10 && \
> cargo clippy --no-default-features --features registry_agent_10 -- -D warnings
> ```
> **The FINAL commit** (Task 6) MUST additionally pass:
> ```bash
> cargo test --all-features && \
> cargo clippy --all-features -- -D warnings && \
> cargo llvm-cov --no-default-features --features registry_agent_10 --fail-under-lines 80
> ```
> **If any test fails, any clippy warning fires, or coverage is below 80%, the commit MUST NOT be created.**

| Task | Message | Pre-commit |
|------|---------|------------|
| 1 | `feat(registry_agent): add feature flag and module skeleton` | `cargo check --no-default-features --features registry_agent_10 && cargo check --all-features` |
| 2 | `feat(registry_agent): add TrustLevel enum and Trust state machine` | `cargo test --no-default-features --features registry_agent_10 && cargo clippy --no-default-features --features registry_agent_10 -- -D warnings` |
| 3 | `feat(registry_agent): add VerificationStatus enum` | `cargo test --no-default-features --features registry_agent_10 && cargo clippy --no-default-features --features registry_agent_10 -- -D warnings` |
| 4 | `feat(registry_agent): add AgentRegistration and RegistryMetadata` | `cargo test --no-default-features --features registry_agent_10 && cargo clippy --no-default-features --features registry_agent_10 -- -D warnings` |
| 5 | `feat(registry_agent): add RegistrySearchFilter, SearchResult, and Entry` | `cargo test --no-default-features --features registry_agent_10 && cargo clippy --no-default-features --features registry_agent_10 -- -D warnings` |
| 6 | `feat(registry_agent): add integration tests, doc examples, README` | `cargo test --all-features && cargo clippy --all-features -- -D warnings && cargo llvm-cov --no-default-features --features registry_agent_10 --fail-under-lines 80` |

---

## Success Criteria

```bash
cargo check --no-default-features --features registry_agent_10
cargo test --no-default-features --features registry_agent_10
cargo clippy --no-default-features --features registry_agent_10 -- -D warnings
cargo check --all-features
cargo llvm-cov --no-default-features --features registry_agent_10 --fail-under-lines 80
```

- [ ] All registry-unique types present
- [ ] AgentCard re-exported from agentic_direct_21
- [ ] Trust Lifecycle state machine (6 states) with invalid transition tests
- [ ] String-based enums, Extension trait
- [ ] ≥80% line coverage (enforced by `cargo llvm-cov --fail-under-lines 80`)
- [ ] Every enum has invalid value rejection tests
- [ ] Trust state machine has invalid transition tests (at least 1 per state)
