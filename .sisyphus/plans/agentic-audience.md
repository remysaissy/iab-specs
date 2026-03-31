# Agentic Audience — Embedding Exchange Protocol (agentic_audience_10)

## TL;DR

> **Quick Summary**: Implement the Agentic Audience specification (formerly UCP — User Context Protocol) as a new `agentic_audience_10` feature, providing Rust data models for the embedding exchange protocol including the embedding envelope, signal taxonomy enums, campaign scoring types, and OpenRTB bid stream extension model. This is a **standalone spec** with no dependency on other AAMP features. Based on Draft v0.1 — clearly documented as unstable.
> 
> **Deliverables**:
> - New `src/agentic_audience/v10/` module
> - Embedding envelope (EmbeddingEnvelope, EmbeddingModel, EmbeddingContext, Embedding)
> - Signal taxonomy enums (SignalType, 6 subtype enums, TemporalScope, CompositionType)
> - Model type enums (ModelType, DistanceMetric, NormalizationType, EmbeddingType)
> - Campaign Scoring models (CampaignHead, ScoringRequest, ScoringResponse, CampaignScore)
> - OpenRTB Extension model (EmbeddingSegmentExt)
> - Feature flag `agentic_audience_10 = ["dep:serde_json"]`
> - Full TDD with ≥80% coverage
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 5 waves
> **Critical Path**: T1 → T2-T4 → T5-T7 → T8/T9 → T10

---

## Context

### Interview Summary
- Standalone spec — no dependency on `agentic_direct_21`
- Draft v0.1 — document instability clearly in module docs
- String-based enum serialization, derive_builder always, TDD

### Research Findings
- Formal JSON schema exists: `embedding_format.schema.json`
- Embedding vectors are `Vec<f32>` or base64-encoded quantized (`quantized_b64: Option<String>`)
- Transport: HTTPS JSON/NDJSON with Content-Type `application/vnd.ucp.embedding+json; v=1`
- OpenRTB integration: embeddings go in `user.data.segment.ext`
- Sidecar pattern for real-time scoring (runtime — we implement data models only)
- Signal taxonomy has 6 top-level types with subtypes, temporal scopes, and composition types

### Metis Review
- Vec<f32> for full-precision, String for base64 quantized — handled via Option fields
- `serde_json` needed for metadata fields → add as feature dependency
- `NormalizationType::None` conflicts with Rust keyword → use `#[serde(rename = "none")]` on a variant named `NoNorm` or similar

---

## Work Objectives

### Core Objective
Implement the Agentic Audience embedding exchange protocol as idiomatic Rust types, enabling embedding transport, taxonomy classification, campaign scoring, and OpenRTB bid stream integration.

### Must Have
- EmbeddingEnvelope with nested EmbeddingModel, EmbeddingContext, Vec<Embedding>
- Complete signal taxonomy (6 signal types × subtypes)
- EmbeddingType enum combining signal type + subtype (23 variants)
- ModelType, DistanceMetric, NormalizationType enums
- TemporalScope and CompositionType enums
- CampaignHead, ScoringRequest, ScoringResponse for scoring workflow
- EmbeddingSegmentExt for OpenRTB `user.data.segment.ext` transport
- Draft v0.1 stability warning in module docs
- Extension trait on extensible types
- String-based enum serialization

### Must NOT Have (Guardrails)
- No vector similarity computation or scoring logic
- No model training or Campaign Head generation
- No NDJSON streaming parser
- No sidecar HTTP server
- No browser tag implementation
- No first-party cookie management
- No base64 encode/decode logic (just store as String)
- No `unsafe` code

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — TDD, `cargo test`, ≥80% coverage.

### TDD Workflow (MANDATORY for every task)

Every task that introduces new types or enums MUST follow this workflow:

1. **RED**: Write tests FIRST that define the expected behavior. Tests MUST fail before implementation.
   - For structs: creation, serialization, deserialization, roundtrip, default (5 tests minimum)
   - For enums: all_valid_values, invalid_value_rejected, serialization_roundtrip, default_value (4 tests minimum)
2. **GREEN**: Implement the minimum code to make tests pass.
3. **REFACTOR**: Clean up while keeping all tests green.

### Invalid/Negative Test Requirement (MANDATORY)

Every type MUST have both valid AND invalid tests:

- **Enums**: Test that invalid/unknown string values are REJECTED by serde (e.g., `"nonexistent_type"` → `Err`)
- **Structs with required fields**: Test that building without required fields produces an error
- **Serialization**: Test that malformed JSON is rejected during deserialization
- **Embeddings**: Test that missing both `vector` and `quantized_b64` is handled (both None)

### Coverage Gate (MANDATORY)

Coverage MUST be checked before the final task's commit and MUST meet ≥80% threshold:

```bash
cargo llvm-cov --no-default-features --features agentic_audience_10 --fail-under-lines 80
```

### Commit Gating Policy (MANDATORY — NO EXCEPTIONS)

**No commit may be created unless ALL of the following pass:**

1. `cargo test --no-default-features --features agentic_audience_10` — **ALL tests pass** (zero failures)
2. `cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` — **zero warnings**
3. For the FINAL task only: additionally run `cargo llvm-cov --no-default-features --features agentic_audience_10 --fail-under-lines 80` or `./coverage.sh --check-thresholds`

**If any test fails or coverage is below 80%, the commit MUST NOT proceed.** Fix the issue first, then re-run.

---

## Execution Strategy

```
Wave 1: Task 1 — Cargo.toml + skeleton [quick]

Wave 2 (Taxonomy enums — all parallel):
├── Task 2: Signal taxonomy enums (SignalType + 6 subtype enums) [quick]
├── Task 3: Model enums (ModelType, DistanceMetric, NormalizationType) [quick]
├── Task 4: EmbeddingType + TemporalScope + CompositionType enums [quick]

Wave 3 (Core models — all parallel):
├── Task 5: EmbeddingModel + EmbeddingContext [quick]
├── Task 6: Embedding struct [unspecified-high]
├── Task 7: EmbeddingEnvelope [quick]

Wave 4 (Scoring + Extension — parallel):
├── Task 8: Campaign Scoring models (CampaignHead, ScoringRequest, ScoringResponse, CampaignScore) [quick]
├── Task 9: EmbeddingSegmentExt (OpenRTB extension) [quick]

Wave 5: Task 10 — Integration tests + doc examples + README update [unspecified-high]

Wave FINAL: F1-F4 — 4 parallel review agents
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 2-10 | 1 |
| 2 | 1 | 6 | 2 |
| 3 | 1 | 5, 6 | 2 |
| 4 | 1 | 6, 9 | 2 |
| 5 | 3 | 7 | 3 |
| 6 | 2, 3, 4 | 7, 8, 9 | 3 |
| 7 | 5, 6 | 10 | 3 |
| 8 | 6 | 10 | 4 |
| 9 | 4, 6 | 10 | 4 |
| 10 | 7-9 | F1-F4 | 5 |

---

## TODOs

- [ ] 1. Feature Flag + Module Skeleton

  **What to do**:
  - Add `agentic_audience_10 = ["dep:serde_json"]` to `Cargo.toml`
  - Create `src/agentic_audience/mod.rs`, `src/agentic_audience/v10/mod.rs` with submodules (`pub mod enums;`, `pub mod models;`, `pub mod scoring;`)
  - Add `#[cfg(feature = "agentic_audience_10")] pub mod agentic_audience;` to `src/lib.rs`
  - Add Draft v0.1 stability warning in module docs: `//! ⚠️ **Draft Specification**: Based on Agentic Audience Draft v0.1. Breaking changes may occur.`
  - Verify: `cargo check --no-default-features --features agentic_audience_10`

  **Recommended Agent Profile**: `quick`
  **QA Scenarios:**
  ```
  Scenario: Feature compiles standalone
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo check --no-default-features --features agentic_audience_10`
      2. Assert exit code 0, zero warnings
    Expected Result: Compiles independently (no agentic_direct dependency)
    Evidence: .sisyphus/evidence/task-1-audience-skeleton.txt
  ```

  **Commit**: `feat(agentic_audience): add feature flag and module skeleton`

- [ ] 2. Signal Taxonomy Enums

  **What to do**:
  - **SignalType** (6 variants): `Identity`, `Contextual`, `Reinforcement`, `Creative`, `Inventory`, `QueryIntent`
  - **IdentitySignalSubtype** (4): `PiiDerived`, `Behavioral`, `Demographic`, `GraphBased`
  - **ContextualSignalSubtype** (5): `Content`, `Temporal`, `Geospatial`, `DeviceEnvironment`, `Session`
  - **ReinforcementSignalSubtype** (4): `Engagement`, `Conversion`, `Attribution`, `Feedback`
  - **CreativeSignalSubtype** (4): `Visual`, `Textual`, `Multimodal`, `CreativePerformance`
  - **InventorySignalSubtype** (3): `Publisher`, `Placement`, `AudienceInventory`
  - **QueryIntentSubtype** (3): `SearchQuery`, `BuyerIntent`, `SellerOffer`
  - String serialization with `#[serde(rename_all = "snake_case")]`, standard 4 tests per enum

  **References**: `src/ads_txt/seller_relation_type.rs` — string enum pattern
  **External**: `https://github.com/IABTechLab/agentic-audiences/blob/main/specs/v1.0/embedding-taxonomy.md`

  **QA Scenarios:**
  ```
  Scenario: SignalType enum roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Serialize SignalType::QueryIntent → assert "query_intent"
      2. Serialize ContextualSignalSubtype::DeviceEnvironment → assert "device_environment"
      3. Deserialize all variants back, assert equality
    Expected Result: All taxonomy enums serialize as snake_case strings
    Evidence: .sisyphus/evidence/task-2-taxonomy-enums.txt
  ```

  **Commit**: `feat(agentic_audience): add signal taxonomy enums`

- [ ] 3. Model Enums

  **What to do**:
  - **ModelType** (3): `Encoder`, `Llm`, `Slm`
  - **DistanceMetric** (4): `Cosine`, `Dot`, `L2`, `Manhattan`
  - **NormalizationType** (4): `L2Norm`, `MinMax`, `ZScore`, `NoNorm` — with `#[serde(rename = "none")]` on `NoNorm` to avoid Rust keyword conflict
  - String serialization, standard tests
  - Test that `NoNorm` serializes as `"none"` (not `"no_norm"`)

  **QA Scenarios:**
  ```
  Scenario: NormalizationType::NoNorm serializes as "none"
    Tool: Bash (cargo test)
    Steps:
      1. Serialize NormalizationType::NoNorm
      2. Assert output is "none" (matching spec), not "no_norm"
    Expected Result: Custom rename works correctly
    Evidence: .sisyphus/evidence/task-3-normalization-none.txt
  ```

  **Commit**: `feat(agentic_audience): add model type enums`

- [ ] 4. EmbeddingType + TemporalScope + CompositionType Enums

  **What to do**:
  - **EmbeddingType** (23 variants combining signal + subtype): `IdentityPii`, `IdentityBehavioral`, `IdentityDemographic`, `IdentityGraph`, `ContextContent`, `ContextTemporal`, `ContextGeospatial`, `ContextDevice`, `ContextSession`, `ReinforcementEngagement`, `ReinforcementConversion`, `ReinforcementAttribution`, `ReinforcementFeedback`, `CreativeVisual`, `CreativeTextual`, `CreativeMultimodal`, `CreativePerformance`, `InventoryPublisher`, `InventoryPlacement`, `InventoryAudience`, `QuerySearch`, `QueryBuyerIntent`, `QuerySellerOffer`
  - **TemporalScope** (4): `Persistent`, `Session`, `RealTime`, `Retrospective`
  - **CompositionType** (5): `Atomic`, `Composite`, `Graph`, `CrossSignalFusion`, `Hierarchical`
  - String serialization, standard tests for all

  **QA Scenarios:**
  ```
  Scenario: EmbeddingType all 23 variants
    Tool: Bash (cargo test)
    Steps:
      1. Iterate all 23 EmbeddingType variants
      2. Serialize each, deserialize back
      3. Assert roundtrip equality for all
    Expected Result: All 23 combined type variants roundtrip
    Evidence: .sisyphus/evidence/task-4-embedding-types.txt
  ```

  **Commit**: `feat(agentic_audience): add EmbeddingType, TemporalScope, and CompositionType enums`

- [ ] 5. EmbeddingModel + EmbeddingContext

  **What to do**:
  - **EmbeddingModel**: `id: String` (required), `version: String` (required), `type_: ModelType` (required, `#[serde(rename = "type")]`), `dimension: i32` (required), `metric: DistanceMetric` (required), `embedding_space_id: String` (required), `normalization: Option<NormalizationType>`, `ext: Option<Box<Ext>>`
  - **EmbeddingContext**: `url: Option<String>`, `page_title: Option<String>`, `keywords: Vec<String>`, `language: Option<String>`, `content_hash: Option<String>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard 5 tests

  **References**: `src/artb/v10/originator.rs` — struct with rename, Extension, builder

  **QA Scenarios:**
  ```
  Scenario: EmbeddingModel with all required fields
    Tool: Bash (cargo test)
    Steps:
      1. Create EmbeddingModel with id="minilm-l6-v2", version="1.0", type_=Encoder, dimension=384, metric=Cosine, embedding_space_id="ucp://spaces/contextual/en-v1"
      2. Serialize to JSON — verify "type" field (not "type_")
      3. Deserialize back, assert equality
    Expected Result: Serde rename works, all fields roundtrip
    Evidence: .sisyphus/evidence/task-5-embedding-model.txt
  ```

  **Commit**: `feat(agentic_audience): add EmbeddingModel and EmbeddingContext`

- [ ] 6. Embedding Struct

  **What to do**:
  - **Embedding**: `id: String` (required), `type_: EmbeddingType` (required, `#[serde(rename = "type")]`), `dimension: i32` (required), `vector: Option<Vec<f32>>`, `quantized_b64: Option<String>`, `metadata: Option<serde_json::Value>`, `ttl: Option<i64>`, `created_at: Option<String>`, `temporal_scope: Option<TemporalScope>`, `composition: Option<CompositionType>`, `ext: Option<Box<Ext>>`
  - NOTE: `vector` and `quantized_b64` are mutually exclusive (one or the other) — document this in rustdoc but don't enforce at type level (leave to runtime validation by consumers)
  - Test with 384-dim Vec<f32> vector
  - Test with base64 quantized string
  - Test roundtrip precision for f32 vectors

  **Recommended Agent Profile**: `unspecified-high` — careful handling of Vec<f32> serialization

  **QA Scenarios:**
  ```
  Scenario: Embedding with full-precision vector
    Tool: Bash (cargo test)
    Steps:
      1. Create Embedding with 384-element Vec<f32>
      2. Serialize to JSON — verify "vector" is JSON array
      3. Deserialize back — assert float values match
    Expected Result: f32 vector roundtrips through JSON
    Evidence: .sisyphus/evidence/task-6-embedding-vector.txt

  Scenario: Embedding with quantized base64
    Tool: Bash (cargo test)
    Steps:
      1. Create Embedding with quantized_b64 = Some("SGVsbG8gV29ybGQ=")
      2. Serialize/deserialize roundtrip
      3. Assert vector is None, quantized_b64 preserved
    Expected Result: Base64 string preserved, vector absent
    Evidence: .sisyphus/evidence/task-6-embedding-quantized.txt
  ```

  **Commit**: `feat(agentic_audience): add Embedding struct`

- [ ] 7. EmbeddingEnvelope

  **What to do**:
  - **EmbeddingEnvelope**: `model: EmbeddingModel` (required), `context: Option<EmbeddingContext>`, `embeddings: Vec<Embedding>` (required, at least one), `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support
  - Test with full envelope (model + context + multiple embeddings)

  **QA Scenarios:**
  ```
  Scenario: Full envelope roundtrip
    Tool: Bash (cargo test)
    Steps:
      1. Create EmbeddingEnvelope with model, context, and 3 embeddings
      2. Serialize to JSON
      3. Verify structure has top-level keys: "model", "context", "embeddings" (matching spec at https://github.com/IABTechLab/agentic-audiences/blob/main/specs/v1.0/schema/embedding_format.schema.json)
      4. Deserialize back, assert equality
    Expected Result: Complete nested envelope roundtrips
    Evidence: .sisyphus/evidence/task-7-envelope.txt
  ```

  **Commit**: `feat(agentic_audience): add EmbeddingEnvelope`

- [ ] 8. Campaign Scoring Models

  **What to do**:
  - **CampaignHead**: `campaign_id: String` (required), `head_weights: Vec<f32>` (required), `dimension: i32` (required), `model_id: String` (required), `created_at: Option<String>`, `ext: Option<Box<Ext>>`
  - **ScoringRequest**: `embeddings: Vec<Embedding>` (required), `campaign_ids: Option<Vec<String>>`, `ext: Option<Box<Ext>>`
  - **ScoringResponse**: `scores: Vec<CampaignScore>`, `ext: Option<Box<Ext>>`
  - **CampaignScore**: `campaign_id: String` (required), `score: f64` (required), `percentile: Option<f64>`, `ext: Option<Box<Ext>>`
  - Builder, serde, Extension support, standard tests

  **QA Scenarios:**
  ```
  Scenario: CampaignHead with head_weights vector
    Tool: Bash (cargo test)
    Steps:
      1. Create CampaignHead with 384-element head_weights Vec<f32>
      2. Serialize/deserialize roundtrip
    Expected Result: Large f32 vector roundtrips through JSON
    Evidence: .sisyphus/evidence/task-8-campaign-head.txt
  ```

  **Commit**: `feat(agentic_audience): add Campaign Scoring models`

- [ ] 9. EmbeddingSegmentExt (OpenRTB Extension)

  **What to do**:
  - **EmbeddingSegmentExt**: `ver: String` (required, e.g., "1.0"), `vector: Vec<f32>` (required), `model: String` (required), `dimension: i32` (required), `type_: EmbeddingType` (required, `#[serde(rename = "type")]`), `metric: Option<DistanceMetric>`, `ttl: Option<i64>`, `ext: Option<Box<Ext>>`
  - This is the type used as `user.data.segment.ext` in OpenRTB bid requests
  - Document in rustdoc how this integrates with OpenRTB (reference `iab_specs::openrtb::v25` if openrtb_25 feature is enabled)
  - Standard tests + test that matches the spec's example JSON structure

  **QA Scenarios:**
  ```
  Scenario: EmbeddingSegmentExt matches spec JSON format
    Tool: Bash (cargo test)
    Steps:
      1. Create EmbeddingSegmentExt matching spec example:
         ver="1.0", vector=[0.15, -0.22, ...], model="minilm-l6-v2", dimension=384, type=ContextContent
      2. Serialize to JSON
      3. Verify field names match spec ("ver", "vector", "model", "dimension", "type")
    Expected Result: JSON output matches spec format
    Evidence: .sisyphus/evidence/task-9-segment-ext.txt
  ```

  **Commit**: `feat(agentic_audience): add EmbeddingSegmentExt for OpenRTB integration`

- [ ] 10. Integration Tests + Doc Examples + README Update

  **What to do**:
  - Integration tests in `src/agentic_audience/v10/mod.rs`:
    - Full data model workflow: create EmbeddingEnvelope with model + context + embeddings → create ScoringRequest referencing those embeddings → create ScoringResponse with mock scores → serialize/deserialize all (NO actual scoring logic — just model construction and serde verification)
    - OpenRTB extension: create EmbeddingSegmentExt matching the spec's bid request example
    - Taxonomy: verify all 23 EmbeddingType variants serialize/deserialize correctly
  - Doc examples with Draft v0.1 warning
  - Update README.md: add Agentic Audience to supported specs, usage example, feature list, roadmap
  - Final: `cargo test --all-features && cargo clippy --all-features -- -D warnings`

  **QA Scenarios:**
  ```
  Scenario: Full verification suite
    Tool: Bash (cargo)
    Steps:
      1. Run `cargo test --all-features`
      2. Run `cargo clippy --all-features -- -D warnings`
      3. Run `cargo test --doc --no-default-features --features agentic_audience_10`
      4. Assert all pass with zero warnings/failures
    Expected Result: Complete clean build + test + lint
    Evidence: .sisyphus/evidence/task-10-audience-full-verification.txt
  ```

  **Commit**: `feat(agentic_audience): add integration tests, doc examples, and README update`

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
> cargo test --no-default-features --features agentic_audience_10 && \
> cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings
> ```
> **The FINAL commit** (Task 10) MUST additionally pass:
> ```bash
> cargo test --all-features && \
> cargo clippy --all-features -- -D warnings && \
> cargo llvm-cov --no-default-features --features agentic_audience_10 --fail-under-lines 80
> ```
> **If any test fails, any clippy warning fires, or coverage is below 80%, the commit MUST NOT be created.**

| Task | Message | Pre-commit |
|------|---------|------------|
| 1 | `feat(agentic_audience): add feature flag and module skeleton` | `cargo check --no-default-features --features agentic_audience_10 && cargo check --all-features` |
| 2 | `feat(agentic_audience): add signal taxonomy enums` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 3 | `feat(agentic_audience): add model type enums` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 4 | `feat(agentic_audience): add EmbeddingType, TemporalScope, CompositionType` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 5 | `feat(agentic_audience): add EmbeddingModel and EmbeddingContext` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 6 | `feat(agentic_audience): add Embedding struct` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 7 | `feat(agentic_audience): add EmbeddingEnvelope` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 8 | `feat(agentic_audience): add Campaign Scoring models` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 9 | `feat(agentic_audience): add EmbeddingSegmentExt` | `cargo test --no-default-features --features agentic_audience_10 && cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings` |
| 10 | `feat(agentic_audience): add integration tests, doc examples, README` | `cargo test --all-features && cargo clippy --all-features -- -D warnings && cargo llvm-cov --no-default-features --features agentic_audience_10 --fail-under-lines 80` |

---

## Success Criteria

```bash
cargo check --no-default-features --features agentic_audience_10
cargo test --no-default-features --features agentic_audience_10
cargo clippy --no-default-features --features agentic_audience_10 -- -D warnings
cargo test --doc --no-default-features --features agentic_audience_10
cargo check --all-features
cargo llvm-cov --no-default-features --features agentic_audience_10 --fail-under-lines 80
```

- [ ] Complete embedding envelope (model + context + embeddings)
- [ ] All 23 EmbeddingType variants + taxonomy enums
- [ ] Campaign scoring types (head, request, response, score)
- [ ] OpenRTB EmbeddingSegmentExt
- [ ] Draft v0.1 documented in module docs
- [ ] Vec<f32> vector roundtrip works
- [ ] String-based enums, Extension trait
- [ ] ≥80% line coverage (enforced by `cargo llvm-cov --fail-under-lines 80`)
- [ ] Every enum has invalid value rejection tests
- [ ] Every struct has malformed JSON rejection tests
