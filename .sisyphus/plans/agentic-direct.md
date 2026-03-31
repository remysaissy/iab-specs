# Agentic Direct — OpenDirect v2.1 + A2A Protocol (agentic_direct_21)

## TL;DR

> **Quick Summary**: Implement the Agentic Direct specification as a new `agentic_direct_21` feature in the `iab-specs` crate, providing Rust data models for OpenDirect v2.1 business entities (Account, Order, Line, Product, Creative, Assignment, Organization) and A2A Protocol v0.3.0 types (Agent Card, Skills, JSON-RPC 2.0 messages). This is the **foundation module** — Buyer Agent, Seller Agent, and Registry Agent all depend on it.
> 
> **Deliverables**:
> - New `src/agentic_direct/` module with `v21/` submodule
> - OpenDirect v2.1 core entities (7 primary types + sub-types)
> - A2A Protocol v0.3.0 types (Agent Card, Skills, Task, JSON-RPC messages)
> - Feature flag `agentic_direct_21` in Cargo.toml (depends on `dep:serde_json`)
> - Error type expansion: `InvalidTransition` variant
> - Full TDD with ≥80% coverage
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: Task 1 → Task 2 → Tasks 3-8 (parallel) → Tasks 9-12 (parallel) → Task 13

---

## Context

### Original Request
Implement each of the 6 AAMP (Agentic Advertising Management Protocols) specifications as Rust data model modules in the `iab-specs` crate, following established codebase patterns.

### Interview Summary
**Key Discussions**:
- Agentic Direct owns all shared types — Buyer/Seller/Registry depend on it
- Feature naming: version-suffixed (`agentic_direct_21`)
- Module path: `src/agentic_direct/v21/` (underlying protocol version)
- String-based enum serialization for AAMP types
- Full state machines with transition validation
- `serde_json` added as optional dependency for JSON-RPC `Value` params
- Always use `derive_builder`, even for small types

**Research Findings**:
- OpenDirect v2.1 schema is defined in `opendirect.json` (OpenAPI 3.0, 3000+ lines)
- Core entities: Account, Order, Line, Product, Creative, Assignment, Organization
- A2A Protocol v0.3.0: Agent Card with Skills, JSON-RPC 2.0 messages, Task states
- Reference impl is TypeScript — we're translating data models to Rust

### Metis Review
**Identified Gaps** (addressed):
- String-based vs integer enums → resolved: strings for AAMP types
- `serde_json` dependency → resolved: added as optional dep for this feature
- Module version naming → resolved: `v21` for OpenDirect version
- Error type expansion → resolved: add `InvalidTransition` variant here
- JSON-RPC `id` polymorphism → addressed: need `JsonRpcId` enum
- Required vs optional fields → addressed: non-Option types for required fields

---

## Work Objectives

### Core Objective
Implement the Agentic Direct specification's data models as idiomatic Rust types, serving as the shared foundation for all AAMP agent specifications.

### Concrete Deliverables
- `src/agentic_direct/mod.rs` — module root with re-exports
- `src/agentic_direct/v21/mod.rs` — v2.1 submodule
- `src/agentic_direct/v21/enums/` — string-based enums (OrderStatus, LineStatus, etc.)
- `src/agentic_direct/v21/entities/` — OpenDirect business entities
- `src/agentic_direct/v21/a2a/` — A2A Protocol types
- `src/agentic_direct/v21/jsonrpc/` — JSON-RPC 2.0 message types
- Updated `Cargo.toml` with `agentic_direct_21` feature
- Updated `src/lib.rs` with conditional module
- Updated `src/errors.rs` with `InvalidTransition` variant

### Definition of Done
- [ ] `cargo check --no-default-features --features agentic_direct_21` passes with zero warnings
- [ ] `cargo test --no-default-features --features agentic_direct_21` — all tests pass
- [ ] `cargo clippy --no-default-features --features agentic_direct_21 -- -D warnings` — clean
- [ ] `cargo test --doc --no-default-features --features agentic_direct_21` — doc examples compile
- [ ] `cargo check --all-features` — no conflicts with existing features
- [ ] ≥80% line coverage for new code

### Must Have
- All 7 OpenDirect core entities with serde + builder
- A2A Agent Card with Skills
- JSON-RPC 2.0 Request/Response/Notification with polymorphic `id`
- String-based enum serialization for all AAMP enums
- Extension trait support on all extensible types
- Comprehensive tests (creation, serialization, deserialization, roundtrip, default)

### Must NOT Have (Guardrails)
- No HTTP server or client implementation
- No MCP server/client runtime
- No LLM integration or agent logic
- No API endpoint definitions or pagination types
- No OAuth implementation (only OAuth scheme data models)
- No `serde_json` as non-optional dependency (must be gated behind feature)
- No integer-based enum serialization (`serde_repr`) for AAMP types
- No type duplication — types defined here, re-exported by dependent features
- No `as any`, `@ts-ignore` equivalent, or `unsafe` code
- No runtime logic beyond state transition validation

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (cargo test, coverage.sh, CI)
- **Automated tests**: TDD (RED-GREEN-REFACTOR)
- **Framework**: `cargo test` (built-in)
- **Each task**: Write failing test → implement → verify pass → refactor

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
# Coverage check command (run before final commit)
cargo llvm-cov --no-default-features --features agentic_direct_21 --fail-under-lines 80
```

If `cargo-llvm-cov` is not available, use:
```bash
./coverage.sh --check-thresholds
```

### Commit Gating Policy (MANDATORY — NO EXCEPTIONS)

**No commit may be created unless ALL of the following pass:**

1. `cargo test --no-default-features --features agentic_direct_21` — **ALL tests pass** (zero failures)
2. `cargo clippy --no-default-features --features agentic_direct_21 -- -D warnings` — **zero warnings**
3. For the FINAL task only: additionally run `cargo llvm-cov --no-default-features --features agentic_direct_21 --fail-under-lines 80` or `./coverage.sh --check-thresholds`

**If any test fails or coverage is below 80%, the commit MUST NOT proceed.** Fix the issue first, then re-run.

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library/Module**: Use Bash (`cargo test`, `cargo check`, `cargo clippy`) — compile, run tests, verify output
- **Serialization**: Use Bash (cargo test) — roundtrip JSON tests built into unit tests

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — scaffolding + shared infrastructure):
├── Task 1: Cargo.toml + feature flag + empty module skeleton + lib.rs + errors.rs [quick]

Wave 2 (Enums — all independent, MAX PARALLEL):
├── Task 2: OpenDirect status enums (OrderStatus, LineStatus, CreativeStatus, etc.) [quick]
├── Task 3: A2A Protocol enums (TaskState, SkillTag, TransportType, etc.) [quick]
├── Task 4: JSON-RPC types (JsonRpcId, JsonRpcError, Request, Response, Notification) [unspecified-high]

Wave 3 (Core entities — depend on enums, MAX PARALLEL):
├── Task 5: Organization entity [quick]
├── Task 6: Account entity [quick]
├── Task 7: Product entity [quick]
├── Task 8: Creative entity [quick]
├── Task 9: Order entity + OrderStatus state machine [deep]
├── Task 10: Line entity + LineStatus state machine [deep]
├── Task 11: Assignment entity [quick]

Wave 4 (A2A + Integration — depend on entities):
├── Task 12: A2A Agent Card + Skill + AgentCapabilities [unspecified-high]
├── Task 13: A2A Task + TaskStatus state machine [deep]
├── Task 14: Integration tests + doc examples + README update [unspecified-high]

Wave FINAL (After ALL tasks — 4 parallel reviews, then user okay):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1    | —         | 2-14   | 1    |
| 2    | 1         | 5-11   | 2    |
| 3    | 1         | 12, 13 | 2    |
| 4    | 1         | 12     | 2    |
| 5    | 2         | 6, 9   | 3    |
| 6    | 2, 5      | 9      | 3    |
| 7    | 2         | 10     | 3    |
| 8    | 2         | 11     | 3    |
| 9    | 2, 5, 6   | 14     | 3    |
| 10   | 2, 7      | 11, 14 | 3    |
| 11   | 2, 8, 10  | 14     | 3    |
| 12   | 3, 4      | 14     | 4    |
| 13   | 3         | 14     | 4    |
| 14   | 9-13      | F1-F4  | 4    |

### Agent Dispatch Summary

- **Wave 1**: **1** — T1 → `quick`
- **Wave 2**: **3** — T2 → `quick`, T3 → `quick`, T4 → `unspecified-high`
- **Wave 3**: **7** — T5-T8, T11 → `quick`, T9-T10 → `deep`
- **Wave 4**: **3** — T12 → `unspecified-high`, T13 → `deep`, T14 → `unspecified-high`
- **FINAL**: **4** — F1 → `oracle`, F2-F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [ ] 1. Feature Flag + Module Skeleton + Error Type Expansion

  **What to do**:
  - Add `agentic_direct_21 = ["dep:serde_json"]` feature to `Cargo.toml`
  - Create `src/agentic_direct/mod.rs` with `pub mod v21;` and module-level doc comments
  - Create `src/agentic_direct/v21/mod.rs` with submodule declarations (`pub mod enums;`, `pub mod entities;`, `pub mod a2a;`, `pub mod jsonrpc;`) and re-exports
  - Create empty `src/agentic_direct/v21/enums/mod.rs`, `src/agentic_direct/v21/entities/mod.rs`, `src/agentic_direct/v21/a2a/mod.rs`, `src/agentic_direct/v21/jsonrpc/mod.rs`
  - Add `#[cfg(feature = "agentic_direct_21")] pub mod agentic_direct;` to `src/lib.rs`
  - Add `InvalidTransition { from: String, to: String }` variant to `Error` enum in `src/errors.rs` (not feature-gated — any AAMP feature may use it)
  - Write test for `InvalidTransition` error variant
  - Verify: `cargo check --no-default-features --features agentic_direct_21` compiles

  **Must NOT do**:
  - Do not add any types yet — only the skeleton
  - Do not make `serde_json` a non-optional dependency

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (solo)
  - **Blocks**: Tasks 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `src/artb/mod.rs` — Module root pattern (1 line: `pub mod v10;`)
  - `src/artb/v10/mod.rs` — Submodule pattern with re-exports and doc comments
  - `src/lib.rs:124-125` — Feature-gated module declaration pattern (`#[cfg(feature = "artb_10")] pub mod artb;`)
  - `src/errors.rs` — Error enum pattern to extend with `InvalidTransition`

  **API/Type References**:
  - `Cargo.toml:32-42` — Feature declaration pattern, dependency gating pattern (`dep:serde_json`)

  **WHY Each Reference Matters**:
  - `artb/mod.rs` shows the exact 1-line module root pattern to follow
  - `artb/v10/mod.rs` shows how to organize submodules, re-exports, and doc comments for a versioned spec
  - `lib.rs` shows the `cfg(feature)` gating pattern
  - `errors.rs` is the shared error type — must be extended carefully to avoid breaking existing features

  **Acceptance Criteria**:

  - [ ] `cargo check --no-default-features --features agentic_direct_21` → compiles, zero warnings
  - [ ] `cargo check --all-features` → compiles, no conflicts with existing features
  - [ ] `cargo test --no-default-features` → existing `InvalidTransition` test passes (in errors.rs)

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Feature flag compiles with empty module
    Tool: Bash (cargo)
    Preconditions: Clean checkout, no prior build artifacts
    Steps:
      1. Run `cargo check --no-default-features --features agentic_direct_21`
      2. Assert exit code 0
      3. Assert stderr contains no warnings
    Expected Result: Compilation succeeds with zero warnings
    Failure Indicators: Non-zero exit code, "warning:" in stderr, "error:" in stderr
    Evidence: .sisyphus/evidence/task-1-feature-compiles.txt

  Scenario: All features still compile together
    Tool: Bash (cargo)
    Preconditions: agentic_direct_21 feature added
    Steps:
      1. Run `cargo check --all-features`
      2. Assert exit code 0
    Expected Result: No feature conflicts
    Failure Indicators: Compilation errors, duplicate symbol errors
    Evidence: .sisyphus/evidence/task-1-all-features.txt

  Scenario: InvalidTransition error works correctly
    Tool: Bash (cargo test)
    Preconditions: Error variant added
    Steps:
      1. Run `cargo test --no-default-features test_invalid_transition`
      2. Assert test passes
    Expected Result: Error variant creates, displays, and matches correctly
    Failure Indicators: Test failure
    Evidence: .sisyphus/evidence/task-1-error-variant.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add feature flag and module skeleton`
  - Files: `Cargo.toml`, `src/lib.rs`, `src/errors.rs`, `src/agentic_direct/mod.rs`, `src/agentic_direct/v21/mod.rs`, `src/agentic_direct/v21/enums/mod.rs`, `src/agentic_direct/v21/entities/mod.rs`, `src/agentic_direct/v21/a2a/mod.rs`, `src/agentic_direct/v21/jsonrpc/mod.rs`
  - Pre-commit: `cargo check --no-default-features --features agentic_direct_21 && cargo check --all-features`

- [ ] 2. OpenDirect Status Enums

  **What to do**:
  - Create string-based enums for all OpenDirect v2.1 statuses using `serde` string serialization (NOT `serde_repr`)
  - **OrderStatus**: `Draft`, `PendingReview`, `Approved`, `InProgress`, `Paused`, `Completed`, `Cancelled`, `Rejected`, `Expired`
  - **LineStatus**: `Draft`, `PendingReview`, `Reserved`, `Booked`, `InProgress`, `Paused`, `Completed`, `Cancelled`, `Rejected`
  - **CreativeStatus**: `Draft`, `PendingApproval`, `Approved`, `Rejected`, `Active`, `Paused`, `Archived`
  - **AssignmentStatus**: `Draft`, `Active`, `Paused`, `Completed`, `Cancelled`
  - **ProductAvailability**: `Available`, `Limited`, `Unavailable`
  - **RateType**: `Cpm`, `Cpc`, `Cpa`, `Flat`, `CpvCompleted`
  - **DeliveryType**: `Guaranteed`, `NonGuaranteed`, `Programmatic`
  - **Currency**: Use `String` (ISO 4217 code), not an enum
  - For each enum: derive `Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default`
  - Use `#[serde(rename_all = "snake_case")]` for consistent serialization
  - Write standard 4 tests per enum: all_valid_values, invalid_value, serialization_roundtrip, default_value
  - Register all enums in `src/agentic_direct/v21/enums/mod.rs` with re-exports

  **Must NOT do**:
  - Do not use `serde_repr` — these are string enums
  - Do not implement state transition logic here — only data model enums

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 4)
  - **Blocks**: Tasks 5, 6, 7, 8, 9, 10, 11
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `src/ads_txt/seller_relation_type.rs` — String-based enum serialization pattern (serde rename, Display/FromStr)
  - `src/artb/v10/enums/intent.rs` — Enum test pattern (all_valid_values, invalid, roundtrip, default)
  - `src/artb/v10/enums/mod.rs` — Enum module re-export pattern

  **External References**:
  - OpenDirect v2.1 schema: `https://github.com/IABTechLab/agentic-direct/blob/main/opendirect.json` — Entity status field definitions

  **WHY Each Reference Matters**:
  - `seller_relation_type.rs` shows how to implement string-based enums with serde (the exact pattern for AAMP)
  - `intent.rs` shows the 4 standard enum tests to replicate
  - `enums/mod.rs` shows the re-export pattern for enum submodules

  **Acceptance Criteria**:

  - [ ] All 7+ enums created with string serialization
  - [ ] `cargo test --no-default-features --features agentic_direct_21` → all enum tests pass
  - [ ] Each enum serializes as lowercase snake_case string (e.g., `"pending_review"`)

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: OrderStatus enum roundtrip
    Tool: Bash (cargo test)
    Preconditions: Enum implemented
    Steps:
      1. Run `cargo test --no-default-features --features agentic_direct_21 test_order_status`
      2. Assert all tests pass
      3. Verify serialization produces strings like "draft", "pending_review", "in_progress"
    Expected Result: All status values roundtrip correctly as strings
    Failure Indicators: Integer serialization output, deserialization failure
    Evidence: .sisyphus/evidence/task-2-order-status-roundtrip.txt

  Scenario: Invalid enum value rejected
    Tool: Bash (cargo test)
    Preconditions: Enum implemented
    Steps:
      1. Run test that attempts to deserialize `"nonexistent_status"` as OrderStatus
      2. Assert deserialization returns Err
    Expected Result: Unknown strings are rejected
    Failure Indicators: Deserialization succeeds with invalid value
    Evidence: .sisyphus/evidence/task-2-invalid-enum.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add OpenDirect status enums`
  - Files: `src/agentic_direct/v21/enums/*.rs`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 3. A2A Protocol Enums

  **What to do**:
  - Create string-based enums for A2A Protocol v0.3.0:
  - **TaskState**: `Working`, `InputRequired`, `Completed`, `Failed`, `Cancelled`
  - **TransportType**: `Http`, `Sse`, `WebSocket`
  - **ProtocolType**: `JsonRpc`, `Mcp`, `Rest`
  - **SecuritySchemeType**: `OAuth2`, `ApiKey`, `Bearer`, `Mtls`
  - **SkillInputMode**: `Text`, `File`, `Data`
  - For each enum: derive `Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default`
  - Use `#[serde(rename_all = "snake_case")]` or `#[serde(rename_all = "lowercase")]` as appropriate per spec
  - Write standard 4 tests per enum
  - Register in `src/agentic_direct/v21/enums/mod.rs`

  **Must NOT do**:
  - Do not use `serde_repr` — these are string enums
  - Do not implement A2A protocol logic — only data types

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 2, 4)
  - **Blocks**: Tasks 12, 13
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `src/ads_txt/seller_relation_type.rs` — String enum serde pattern
  - `src/artb/v10/enums/mod.rs` — Enum module organization

  **External References**:
  - A2A Protocol v0.3.0: Agent Card schema from `https://github.com/IABTechLab/agentic-direct/blob/main/server/src/a2a/agent-card.ts`

  **Acceptance Criteria**:

  - [ ] All A2A enums created with string serialization
  - [ ] `cargo test --no-default-features --features agentic_direct_21` → all enum tests pass

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: TaskState enum roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Run `cargo test --no-default-features --features agentic_direct_21 test_task_state`
      2. Verify "working", "completed", "failed" roundtrip correctly
    Expected Result: All A2A enum values roundtrip as strings
    Evidence: .sisyphus/evidence/task-3-a2a-enums.txt

  Scenario: Unknown transport type rejected
    Tool: Bash (cargo test)
    Steps:
      1. Attempt to deserialize `"grpc"` as TransportType
      2. Assert Err returned
    Expected Result: Unknown values rejected gracefully
    Evidence: .sisyphus/evidence/task-3-invalid-transport.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add A2A protocol enums`
  - Files: `src/agentic_direct/v21/enums/*.rs`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 4. JSON-RPC 2.0 Message Types

  **What to do**:
  - Create JSON-RPC 2.0 types in `src/agentic_direct/v21/jsonrpc/`:
  - **JsonRpcId**: Enum with `String(String)`, `Number(i64)`, `Null` — custom serde for polymorphic deserialization
  - **JsonRpcVersion**: Const `"2.0"` string with serde validation
  - **JsonRpcRequest**: `jsonrpc: String`, `method: String`, `params: Option<serde_json::Value>`, `id: Option<JsonRpcId>`
  - **JsonRpcResponse**: `jsonrpc: String`, `result: Option<serde_json::Value>`, `error: Option<JsonRpcError>`, `id: JsonRpcId`
  - **JsonRpcNotification**: `jsonrpc: String`, `method: String`, `params: Option<serde_json::Value>` (no `id`)
  - **JsonRpcError**: `code: i32`, `message: String`, `data: Option<serde_json::Value>`
  - **Standard error codes**: `ParseError = -32700`, `InvalidRequest = -32600`, `MethodNotFound = -32601`, `InvalidParams = -32602`, `InternalError = -32603`
  - All types: derive Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq
  - Extension support on Request, Response, Error
  - Write comprehensive tests including polymorphic id handling

  **Must NOT do**:
  - Do not implement JSON-RPC transport or routing logic
  - Do not add JSON-RPC method dispatch
  - Do not make `serde_json` a hard crate dependency (it's already gated behind the feature)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Custom serde for polymorphic JsonRpcId requires careful implementation
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 2, 3)
  - **Blocks**: Task 12
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `src/artb/v10/rtb_request.rs:47-94` — Struct with generic Extension, serde bounds, builder pattern
  - `src/sellers_json/sellers_version.rs` — String-based enum with custom serde

  **External References**:
  - JSON-RPC 2.0 spec: `https://www.jsonrpc.org/specification` — Canonical message format
  - A2A communication: `https://github.com/IABTechLab/agentic-direct/blob/main/server/src/a2a/` — How JSON-RPC is used

  **WHY Each Reference Matters**:
  - `rtb_request.rs` shows the exact struct template (Builder, serde bounds, Extension generic)
  - JSON-RPC 2.0 spec defines the exact wire format for id polymorphism

  **Acceptance Criteria**:

  - [ ] JsonRpcId deserializes string, number, and null correctly
  - [ ] JsonRpcRequest, Response, Notification, Error all have serde roundtrip tests
  - [ ] Standard error codes are accessible as constants
  - [ ] `cargo test --no-default-features --features agentic_direct_21` → all JSON-RPC tests pass

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: JSON-RPC request with string id
    Tool: Bash (cargo test)
    Steps:
      1. Deserialize `{"jsonrpc":"2.0","method":"sendMessage","params":{"text":"hello"},"id":"req-1"}`
      2. Assert id is JsonRpcId::String("req-1")
      3. Serialize back, assert JSON matches
    Expected Result: String id roundtrips correctly
    Evidence: .sisyphus/evidence/task-4-jsonrpc-string-id.txt

  Scenario: JSON-RPC request with numeric id
    Tool: Bash (cargo test)
    Steps:
      1. Deserialize `{"jsonrpc":"2.0","method":"getTask","id":42}`
      2. Assert id is JsonRpcId::Number(42)
    Expected Result: Numeric id parsed correctly
    Evidence: .sisyphus/evidence/task-4-jsonrpc-numeric-id.txt

  Scenario: JSON-RPC notification (no id)
    Tool: Bash (cargo test)
    Steps:
      1. Create JsonRpcNotification with method "taskUpdate"
      2. Serialize to JSON
      3. Assert no "id" field in output
    Expected Result: Notifications correctly omit id field
    Evidence: .sisyphus/evidence/task-4-jsonrpc-notification.txt

  Scenario: JSON-RPC error response
    Tool: Bash (cargo test)
    Steps:
      1. Deserialize `{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":"req-1"}`
      2. Assert error code is -32601
      3. Assert result is None
    Expected Result: Error responses parse correctly with standard codes
    Evidence: .sisyphus/evidence/task-4-jsonrpc-error.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add JSON-RPC 2.0 message types`
  - Files: `src/agentic_direct/v21/jsonrpc/*.rs`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 5. Organization Entity

  **What to do**:
  - Create `src/agentic_direct/v21/entities/organization.rs`
  - **Organization**: `id: Option<String>`, `name: String`, `type_: Option<OrganizationType>` (enum: `Advertiser`, `Agency`, `Publisher`, `Exchange`), `address: Option<Address>`, `contacts: Vec<Contact>`, `phone: Option<String>`, `url: Option<String>`, `ext: Option<Box<Ext>>`
  - **Address**: `street: Option<String>`, `city: Option<String>`, `state: Option<String>`, `postal_code: Option<String>`, `country: Option<String>`
  - **Contact**: `name: Option<String>`, `email: Option<String>`, `phone: Option<String>`, `role: Option<String>`
  - **OrganizationType** enum: `Advertiser`, `Agency`, `Publisher`, `Exchange` (string-based)
  - All types: derive Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq
  - Extension support on Organization
  - `name` is non-Option (required field)
  - Use `setter(into, strip_option)` on all `Option<String>` fields
  - Write standard 5 tests per struct + enum tests

  **Must NOT do**:
  - Do not add organization business logic
  - Do not add organization validation beyond serde

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 6-11)
  - **Blocks**: Tasks 6, 9
  - **Blocked By**: Task 2

  **References**:

  **Pattern References**:
  - `src/artb/v10/originator.rs:28-57` — Struct with Extension, builder, optional string fields with `setter(into, strip_option)`

  **External References**:
  - OpenDirect v2.1: `https://github.com/IABTechLab/agentic-direct/blob/main/opendirect.json` — Organization schema definition

  **Acceptance Criteria**:

  - [ ] Organization, Address, Contact, OrganizationType types created
  - [ ] `name` is required (non-Option)
  - [ ] All optional string fields accept `.field("value")` syntax
  - [ ] Serde roundtrip tests pass

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Organization creation with builder
    Tool: Bash (cargo test)
    Steps:
      1. Create Organization with name "IAB Tech Lab", type Advertiser
      2. Serialize to JSON, deserialize back
      3. Assert equality
    Expected Result: Roundtrip preserves all fields
    Evidence: .sisyphus/evidence/task-5-org-roundtrip.txt

  Scenario: Organization with nested Address
    Tool: Bash (cargo test)
    Steps:
      1. Create Organization with full Address (street, city, state, postal, country)
      2. Serialize, verify nested JSON structure
      3. Deserialize, verify equality
    Expected Result: Nested objects serialize/deserialize correctly
    Evidence: .sisyphus/evidence/task-5-org-address.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Organization entity`
  - Files: `src/agentic_direct/v21/entities/organization.rs`, `src/agentic_direct/v21/entities/mod.rs`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 6. Account Entity

  **What to do**:
  - Create `src/agentic_direct/v21/entities/account.rs`
  - **Account**: `id: Option<String>`, `advertiser_id: String` (required), `buyer_id: String` (required), `name: String` (required), `status: Option<AccountStatus>`, `ext: Option<Box<Ext>>`
  - **AccountStatus** enum: `Active`, `Paused`, `Closed` (string-based)
  - Extension support on Account
  - Write standard tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 5, 7-11)
  - **Blocks**: Task 9
  - **Blocked By**: Task 2

  **References**:
  - Pattern: `src/artb/v10/originator.rs` — Struct with required + optional fields
  - External: OpenDirect schema `Account` definition

  **Acceptance Criteria**:
  - [ ] Account with required fields (advertiser_id, buyer_id, name) created
  - [ ] AccountStatus enum with string serialization
  - [ ] Serde roundtrip tests pass

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Account builder enforces required fields
    Tool: Bash (cargo test)
    Steps:
      1. Create Account with all required fields
      2. Serialize/deserialize roundtrip
      3. Verify required fields present in JSON
    Expected Result: Required fields always serialized
    Evidence: .sisyphus/evidence/task-6-account.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Account entity`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 7. Product Entity

  **What to do**:
  - Create `src/agentic_direct/v21/entities/product.rs`
  - **Product**: `id: Option<String>`, `publisher_id: String` (required), `name: String` (required), `description: Option<String>`, `availability: Option<ProductAvailability>`, `base_price: Option<f64>`, `rate_type: Option<RateType>`, `delivery_type: Option<DeliveryType>`, `min_spend: Option<f64>`, `max_spend: Option<f64>`, `currency: Option<String>`, `targeting: Option<serde_json::Value>`, `ad_units: Vec<String>`, `ext: Option<Box<Ext>>`
  - Extension support on Product
  - Write standard tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 10
  - **Blocked By**: Task 2

  **References**:
  - Pattern: `src/artb/v10/rtb_request.rs` — Struct with serde_json::Value field
  - External: OpenDirect schema `Product` definition

  **Acceptance Criteria**:
  - [ ] Product with all fields including `targeting: Option<serde_json::Value>`
  - [ ] Serde roundtrip tests pass including JSON targeting blob

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Product with targeting JSON blob
    Tool: Bash (cargo test)
    Steps:
      1. Create Product with targeting = json!({"geo": "US", "age": "25-54"})
      2. Serialize/deserialize roundtrip
      3. Verify targeting blob preserved exactly
    Expected Result: Arbitrary JSON in targeting field preserved
    Evidence: .sisyphus/evidence/task-7-product.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Product entity`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 8. Creative Entity

  **What to do**:
  - Create `src/agentic_direct/v21/entities/creative.rs`
  - **Creative**: `id: Option<String>`, `name: String` (required), `account_id: String` (required), `status: Option<CreativeStatus>`, `ad_format: Option<String>`, `click_url: Option<String>`, `markup: Option<String>`, `width: Option<i32>`, `height: Option<i32>`, `duration: Option<i32>`, `mime_type: Option<String>`, `ext: Option<Box<Ext>>`
  - Extension support on Creative
  - Write standard tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  - Pattern: `src/artb/v10/originator.rs` — Struct with optional fields
  - External: OpenDirect schema `Creative` definition

  **Acceptance Criteria**:
  - [ ] Creative with all fields and CreativeStatus reference
  - [ ] Serde roundtrip tests pass

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Creative with markup field
    Tool: Bash (cargo test)
    Steps:
      1. Create Creative with HTML markup and dimensions
      2. Serialize/deserialize roundtrip
    Expected Result: HTML markup preserved without escaping issues
    Evidence: .sisyphus/evidence/task-8-creative.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Creative entity`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 9. Order Entity + Order State Machine

  **What to do**:
  - Create `src/agentic_direct/v21/entities/order.rs`
  - **Order**: `id: Option<String>`, `name: String` (required), `account_id: String` (required), `publisher_id: String` (required), `status: OrderStatus`, `currency: String` (required), `budget: Option<f64>`, `start_date: Option<String>`, `end_date: Option<String>`, `advertiser_id: Option<String>`, `brand_id: Option<String>`, `contacts: Vec<Contact>`, `ext: Option<Box<Ext>>`
  - **Order State Machine** in `src/agentic_direct/v21/entities/order_state_machine.rs`:
    - `const VALID_ORDER_TRANSITIONS: &[(OrderStatus, OrderStatus)]` — complete transition map
    - `pub fn can_transition_order(from: &OrderStatus, to: &OrderStatus) -> bool`
    - `pub fn valid_order_transitions_from(state: &OrderStatus) -> Vec<OrderStatus>`
    - **OrderTransition** struct: `from: OrderStatus`, `to: OrderStatus`, `timestamp: Option<String>`, `reason: Option<String>`, `actor: Option<String>`
  - Valid transitions: Draft→PendingReview, PendingReview→Approved, PendingReview→Rejected, Approved→InProgress, InProgress→Paused, Paused→InProgress, InProgress→Completed, any→Cancelled
  - Test every valid transition and at least one invalid per state
  - Test `valid_order_transitions_from()` for each state

  **Must NOT do**:
  - Do not implement async state handlers or event sourcing
  - Do not add order business logic beyond state transitions

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: State machine requires careful transition validation and exhaustive testing
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 14
  - **Blocked By**: Tasks 2, 5, 6

  **References**:

  **Pattern References**:
  - `src/artb/v10/rtb_request.rs` — Struct with required + optional fields, Extension support
  - `src/artb/v10/enums/lifecycle.rs` — Enum pattern (state enums follow this for derives)

  **External References**:
  - Seller Agent order lifecycle: `https://github.com/IABTechLab/seller-agent/blob/main/docs/state-machines/order-lifecycle.md`

  **Acceptance Criteria**:
  - [ ] Order entity with all fields + Extension support
  - [ ] State machine: `VALID_ORDER_TRANSITIONS` const, `can_transition_order()`, `valid_order_transitions_from()`
  - [ ] OrderTransition struct for recording transitions
  - [ ] All valid transitions tested, all invalid transitions tested (at least 1 per state)
  - [ ] `cargo test --no-default-features --features agentic_direct_21` → all state machine tests pass

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Valid order transition Draft → PendingReview
    Tool: Bash (cargo test)
    Steps:
      1. Call can_transition_order(Draft, PendingReview)
      2. Assert returns true
    Expected Result: Valid transition accepted
    Evidence: .sisyphus/evidence/task-9-order-valid-transition.txt

  Scenario: Invalid order transition Draft → Completed
    Tool: Bash (cargo test)
    Steps:
      1. Call can_transition_order(Draft, Completed)
      2. Assert returns false
    Expected Result: Invalid transition rejected
    Evidence: .sisyphus/evidence/task-9-order-invalid-transition.txt

  Scenario: Order entity roundtrip with status
    Tool: Bash (cargo test)
    Steps:
      1. Create Order with status Draft
      2. Serialize to JSON — verify status is "draft" (string, not integer)
      3. Deserialize back, assert equality
    Expected Result: Order with string-based status roundtrips
    Evidence: .sisyphus/evidence/task-9-order-roundtrip.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Order entity with state machine`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 10. Line Entity + Line State Machine

  **What to do**:
  - Create `src/agentic_direct/v21/entities/line.rs`
  - **Line**: `id: Option<String>`, `name: String` (required), `order_id: String` (required), `product_id: String` (required), `status: LineStatus`, `start_date: String` (required), `end_date: String` (required), `rate_type: RateType`, `rate: Option<f64>`, `quantity: i64` (required), `budget: Option<f64>`, `targeting: Option<serde_json::Value>`, `frequency_cap: Option<FrequencyCap>`, `ext: Option<Box<Ext>>`
  - **FrequencyCap**: `count: i32`, `period_seconds: i64`
  - **Line State Machine** in `src/agentic_direct/v21/entities/line_state_machine.rs`:
    - `const VALID_LINE_TRANSITIONS: &[(LineStatus, LineStatus)]`
    - `pub fn can_transition_line(from: &LineStatus, to: &LineStatus) -> bool`
    - `pub fn valid_line_transitions_from(state: &LineStatus) -> Vec<LineStatus>`
    - **LineTransition** struct
  - Valid transitions: Draft→PendingReview, PendingReview→Reserved, PendingReview→Rejected, Reserved→Booked, Booked→InProgress, InProgress→Paused, Paused→InProgress, InProgress→Completed, any→Cancelled

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Tasks 11, 14
  - **Blocked By**: Tasks 2, 7

  **References**:
  - Same patterns as Task 9

  **Acceptance Criteria**:
  - [ ] Line entity with all fields + FrequencyCap sub-type
  - [ ] State machine with full transition validation
  - [ ] All transitions tested

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Line state machine transitions
    Tool: Bash (cargo test)
    Steps:
      1. Test all valid transitions for Line
      2. Test at least one invalid transition per state
    Expected Result: All transitions validated correctly
    Evidence: .sisyphus/evidence/task-10-line-state-machine.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Line entity with state machine`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 11. Assignment Entity

  **What to do**:
  - Create `src/agentic_direct/v21/entities/assignment.rs`
  - **Assignment**: `id: Option<String>`, `creative_id: String` (required), `line_id: String` (required), `status: AssignmentStatus`, `weight: Option<f64>`, `start_date: Option<String>`, `end_date: Option<String>`, `ext: Option<Box<Ext>>`
  - Extension support, standard tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 14
  - **Blocked By**: Tasks 2, 8, 10

  **Acceptance Criteria**:
  - [ ] Assignment entity with required fields and AssignmentStatus
  - [ ] Serde roundtrip tests pass

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Assignment roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Create Assignment linking creative to line
      2. Serialize/deserialize roundtrip
    Expected Result: All fields preserved
    Evidence: .sisyphus/evidence/task-11-assignment.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add Assignment entity`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 12. A2A Agent Card + Skill + Capabilities

  **What to do**:
  - Create `src/agentic_direct/v21/a2a/agent_card.rs`
  - **AgentCard**: `name: String` (required), `description: Option<String>`, `version: String` (required), `protocol_version: String` (required — e.g., "0.3.0"), `url: String` (required), `skills: Vec<Skill>`, `capabilities: Option<AgentCapabilities>`, `additional_interfaces: Vec<AgentInterface>`, `security_schemes: Vec<SecurityScheme>`, `ext: Option<Box<Ext>>`
  - **Skill**: `id: String` (required), `name: String` (required), `description: Option<String>`, `tags: Vec<String>`, `examples: Vec<String>`, `input_modes: Vec<SkillInputMode>`
  - **AgentCapabilities**: `push_notifications: Option<bool>`, `streaming: Option<bool>`, `mcp_integration: Option<bool>`
  - **AgentInterface**: `protocol: ProtocolType`, `version: Option<String>`, `transport: TransportType`, `url: String` (required)
  - **SecurityScheme**: `type_: SecuritySchemeType`, `description: Option<String>`, `flows: Option<serde_json::Value>` (OAuth2 flows are complex — use Value)
  - All types: Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq
  - Extension on AgentCard, Skill
  - Use `#[serde(rename_all = "camelCase")]` to match the A2A JSON format (camelCase in TypeScript source)

  **Must NOT do**:
  - Do not implement agent discovery logic
  - Do not implement HTTP well-known endpoint serving

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Multiple interrelated types, careful serde naming needed
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 13, 14)
  - **Blocks**: Task 14
  - **Blocked By**: Tasks 3, 4

  **References**:

  **Pattern References**:
  - `src/artb/v10/originator.rs` — Struct with Extension + builder + optional fields

  **External References**:
  - A2A Agent Card: `https://github.com/IABTechLab/agentic-direct/blob/main/server/src/a2a/agent-card.ts` — Canonical Agent Card structure

  **Acceptance Criteria**:
  - [ ] AgentCard, Skill, AgentCapabilities, AgentInterface, SecurityScheme all created
  - [ ] camelCase JSON serialization (matching TypeScript source)
  - [ ] All types have serde roundtrip tests
  - [ ] Extension support on AgentCard and Skill

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Agent Card full roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Create AgentCard with name, version, skills, capabilities, interfaces
      2. Serialize to JSON — verify camelCase field names ("protocolVersion", not "protocol_version")
      3. Deserialize back, assert equality
    Expected Result: Full Agent Card roundtrips with correct casing
    Evidence: .sisyphus/evidence/task-12-agent-card.txt

  Scenario: Agent Card with multiple skills
    Tool: Bash (cargo test)
    Steps:
      1. Create AgentCard with 3 skills (campaign-planning, order-creation, inventory-management)
      2. Verify skills array serializes correctly
    Expected Result: Skills array preserved
    Evidence: .sisyphus/evidence/task-12-skills.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add A2A Agent Card and Skill types`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 13. A2A Task + TaskStatus State Machine

  **What to do**:
  - Create `src/agentic_direct/v21/a2a/task.rs`
  - **A2ATask**: `id: String` (required), `state: TaskState`, `message: Option<String>`, `result: Option<serde_json::Value>`, `artifacts: Vec<A2AArtifact>`, `created_at: Option<String>`, `updated_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **A2AArtifact**: `name: String` (required), `description: Option<String>`, `parts: Vec<A2AArtifactPart>`
  - **A2AArtifactPart**: `type_: String`, `content: String`
  - **Task State Machine** in `src/agentic_direct/v21/a2a/task_state_machine.rs`:
    - `const VALID_TASK_TRANSITIONS: &[(TaskState, TaskState)]`
    - `pub fn can_transition_task(from: &TaskState, to: &TaskState) -> bool`
    - `pub fn valid_task_transitions_from(state: &TaskState) -> Vec<TaskState>`
    - **TaskTransition** struct
  - Valid transitions: Working→InputRequired, Working→Completed, Working→Failed, InputRequired→Working, Working→Cancelled, InputRequired→Cancelled

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 14
  - **Blocked By**: Task 3

  **Acceptance Criteria**:
  - [ ] A2ATask, A2AArtifact, A2AArtifactPart types created
  - [ ] TaskState machine with full transition validation
  - [ ] All transitions tested

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Task state machine transitions
    Tool: Bash (cargo test)
    Steps:
      1. Test Working→Completed (valid)
      2. Test Completed→Working (invalid)
      3. Test all valid transitions
    Expected Result: All transitions validated correctly
    Evidence: .sisyphus/evidence/task-13-task-state-machine.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add A2A Task with state machine`
  - Pre-commit: `cargo test --no-default-features --features agentic_direct_21`

- [ ] 14. Integration Tests + Doc Examples + README Update

  **What to do**:
  - Add integration tests in `src/agentic_direct/v21/mod.rs` `#[cfg(test)] mod integration_tests`:
    - Test creating a complete workflow: Organization → Account → Product → Order → Line → Creative → Assignment
    - Test Agent Card creation with full skill set
    - Test JSON-RPC request/response cycle
    - Test state machine transitions across Order, Line, and Task
  - Add doc examples in `src/agentic_direct/v21/mod.rs` module docs:
    - Quick Start example showing entity creation + serialization
    - Agent Card example
    - State machine example
  - Update `README.md`: add Agentic Direct to "Currently Supported Specifications", add usage example, update Roadmap, update feature list
  - Update `Cargo.toml` keywords if needed
  - Run final verification: `cargo test --all-features && cargo clippy --all-features -- -D warnings && cargo test --doc --all-features`

  **Must NOT do**:
  - Do not add examples that require running a server
  - Do not add non-markdown documentation files

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (after Tasks 9-13)
  - **Blocks**: F1-F4
  - **Blocked By**: Tasks 9, 10, 11, 12, 13

  **References**:
  - `src/artb/v10/mod.rs:128-481` — Integration test pattern (complete workflow tests)
  - `README.md:603-668` — ARTB usage example pattern for README

  **Acceptance Criteria**:
  - [ ] Integration tests cover end-to-end entity creation workflow
  - [ ] Doc examples compile: `cargo test --doc --no-default-features --features agentic_direct_21`
  - [ ] README updated with Agentic Direct section
  - [ ] `cargo test --all-features` passes
  - [ ] `cargo clippy --all-features -- -D warnings` clean

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Full verification suite
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo test --all-features`
      2. Run `cargo clippy --all-features -- -D warnings`
      3. Run `cargo test --doc --all-features`
      4. Assert all pass with zero warnings/failures
    Expected Result: Complete clean build + test + lint
    Evidence: .sisyphus/evidence/task-14-full-verification.txt

  Scenario: Feature isolation verification
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo check --no-default-features --features agentic_direct_21`
      2. Run `cargo check --no-default-features --features artb_10`
      3. Assert both compile independently
    Expected Result: Features don't leak dependencies
    Evidence: .sisyphus/evidence/task-14-feature-isolation.txt
  ```

  **Commit**: YES
  - Message: `feat(agentic_direct): add integration tests, doc examples, and README update`
  - Pre-commit: `cargo test --all-features && cargo clippy --all-features -- -D warnings`

---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy --no-default-features --features agentic_direct_21 -- -D warnings` + `cargo test --no-default-features --features agentic_direct_21`. Review all changed files for: empty catches, `unwrap()` in non-test code, unused imports, dead code. Check serde attributes are consistent.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Run `cargo check --all-features` to verify no conflicts. Run `cargo test --doc --no-default-features --features agentic_direct_21`. Create instances of every public type using builder, serialize to JSON, deserialize back, verify equality. Test state machine transitions (valid and invalid). Save evidence.
  Output: `Types [N/N verified] | State Machines [N/N] | Doc Tests [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff. Verify 1:1 match. Check no runtime logic leaked in. Check no type duplication. Verify Extension trait used correctly on all extensible types. Check enum serialization is string-based (not integer).
  Output: `Tasks [N/N compliant] | Guardrails [N/N] | VERDICT`

---

## Commit Strategy

| Task | Commit Message | Files | Pre-commit |
|------|---------------|-------|------------|
| 1 | `feat(agentic_direct): add feature flag and module skeleton` | Cargo.toml, src/lib.rs, src/errors.rs, src/agentic_direct/mod.rs, src/agentic_direct/v21/mod.rs | `cargo check --no-default-features --features agentic_direct_21` |
| 2 | `feat(agentic_direct): add OpenDirect status enums` | src/agentic_direct/v21/enums/*.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 3 | `feat(agentic_direct): add A2A protocol enums` | src/agentic_direct/v21/enums/*.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 4 | `feat(agentic_direct): add JSON-RPC 2.0 message types` | src/agentic_direct/v21/jsonrpc/*.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 5 | `feat(agentic_direct): add Organization entity` | src/agentic_direct/v21/entities/organization.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 6 | `feat(agentic_direct): add Account entity` | src/agentic_direct/v21/entities/account.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 7 | `feat(agentic_direct): add Product entity` | src/agentic_direct/v21/entities/product.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 8 | `feat(agentic_direct): add Creative entity` | src/agentic_direct/v21/entities/creative.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 9 | `feat(agentic_direct): add Order entity with state machine` | src/agentic_direct/v21/entities/order.rs, state machine files | `cargo test --no-default-features --features agentic_direct_21` |
| 10 | `feat(agentic_direct): add Line entity with state machine` | src/agentic_direct/v21/entities/line.rs, state machine files | `cargo test --no-default-features --features agentic_direct_21` |
| 11 | `feat(agentic_direct): add Assignment entity` | src/agentic_direct/v21/entities/assignment.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 12 | `feat(agentic_direct): add A2A Agent Card and Skill types` | src/agentic_direct/v21/a2a/*.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 13 | `feat(agentic_direct): add A2A Task with state machine` | src/agentic_direct/v21/a2a/task.rs | `cargo test --no-default-features --features agentic_direct_21` |
| 14 | `feat(agentic_direct): add integration tests and doc examples` | src/agentic_direct/v21/mod.rs, README.md | `cargo test --all-features && cargo clippy --all-features -- -D warnings && cargo llvm-cov --no-default-features --features agentic_direct_21 --fail-under-lines 80` |

---

## Success Criteria

### Verification Commands
```bash
cargo check --no-default-features --features agentic_direct_21  # Expected: success, zero warnings
cargo test --no-default-features --features agentic_direct_21   # Expected: all tests pass
cargo clippy --no-default-features --features agentic_direct_21 -- -D warnings  # Expected: clean
cargo test --doc --no-default-features --features agentic_direct_21  # Expected: doc examples pass
cargo check --all-features  # Expected: no conflicts
```

### Final Checklist
- [ ] All 7 OpenDirect entities present with builder + serde
- [ ] A2A Agent Card + Skills types present
- [ ] JSON-RPC 2.0 Request/Response/Notification types present
- [ ] All enums use string serialization
- [ ] Extension trait on all extensible types
- [ ] State machines for Order, Line, Task statuses
- [ ] `InvalidTransition` error variant added
- [ ] Feature flag `agentic_direct_21` works in isolation
- [ ] No conflicts with existing features
- [ ] ≥80% line coverage (enforced by `cargo llvm-cov --fail-under-lines 80`)
- [ ] Every enum has invalid value rejection tests (unknown strings → `Err`)
- [ ] Every state machine has invalid transition tests (at least 1 per state)
- [ ] Every struct has malformed JSON rejection tests
