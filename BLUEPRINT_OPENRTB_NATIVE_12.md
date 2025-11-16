# Blueprint: OpenRTB Native Ads 1.2 Implementation

**Date**: 2025-11-16
**Status**: Design Phase
**Specification**: [OpenRTB Native Ads 1.2](https://github.com/InteractiveAdvertisingBureau/Native-Ads/blob/main/OpenRTB-Native-Ads-Specification-Final-1.2.md)

## Executive Summary

This blueprint outlines the implementation of OpenRTB Native Ads 1.2 as a new feature (`openrtb_native_12`) in the iab-specs Rust library. The implementation provides fully typed Rust data structures for native ad request/response objects, leveraging existing AdCOM enumerations to minimize code duplication.

### Key Features
- **Full Type Safety**: All native ad objects as typed Rust structs
- **Builder Pattern**: Ergonomic construction using derive_builder
- **Extension Support**: Generic extension mechanism for custom fields
- **AdCOM Integration**: Reuses existing enumerations (no duplication)
- **JSON Serialization**: Full serde support for OpenRTB integration
- **Comprehensive Testing**: Unit tests with 80%+ coverage target

## 1. Specification Analysis

### 1.1 OpenRTB Native 1.2 Overview

OpenRTB Native Ads 1.2 (March 2017) defines a standardized protocol for trading native advertising formats. Key characteristics:

- **Purpose**: Enable automated trading of native ads across platforms
- **Integration**: Embeds as JSON in OpenRTB 2.x/3.x bid requests
- **Version**: 1.2 (with deprecated fields from 1.0/1.1)
- **Structure**: Request-response protocol with asset-based composition

### 1.2 Core Objects

#### Request Objects (Section 4)

| Object | Purpose | Required Fields | Optional Fields |
|--------|---------|----------------|-----------------|
| **NativeRequest** | Root request | `assets` | `ver`, `context`, `contextsubtype`, `plcmttype`, `plcmtcnt`, `seq`, `aurlsupport`, `durlsupport`, `eventtrackers`, `privacy`, `ext` |
| **Asset** | Individual asset spec | `id` | `required`, `title`, `img`, `video`, `data`, `ext` |
| **Title** | Title text spec | `len` | `ext` |
| **Image** | Image spec | `type` | `w`, `h`, `wmin`, `hmin`, `mimes`, `ext` |
| **Video** | Video spec | `mimes` | `minduration`, `maxduration`, `protocols`, `ext` |
| **Data** | Data element spec | `type` | `len`, `ext` |
| **EventTracker** | Event tracking spec | `event`, `methods` | `ext` |

#### Response Objects (Section 5)

| Object | Purpose | Required Fields | Optional Fields |
|--------|---------|----------------|-----------------|
| **NativeResponse** | Root response | `assets`, `link` | `imptrackers` (deprecated), `jstracker` (deprecated), `eventtrackers`, `privacy`, `assetsurl`, `dcourl`, `ext` |
| **AssetResponse** | Asset content | `id` | `required`, `title`, `img`, `video`, `data`, `link`, `ext` |
| **TitleResponse** | Title text | `text` | `len`, `ext` |
| **ImageResponse** | Image URL | `url` | `w`, `h`, `ext` |
| **VideoResponse** | Video markup | `vasttag` | `ext` |
| **DataResponse** | Data value | `value` | `len`, `ext` |
| **Link** | Click destination | `url` | `clicktrackers`, `fallback`, `ext` |
| **EventTrackerResponse** | Event tracker URL | `event`, `method`, `url` | `customdata`, `ext` |

### 1.3 Enumerations (Section 7)

All required enumerations **already exist in AdCOM**:

| Native 1.2 Enum | AdCOM Enum | Location | Status |
|----------------|-----------|----------|--------|
| Context Types (7.1) | `DisplayContextType` | `src/adcom/enums/display_context_type.rs` | ✅ Exists |
| Placement Types (7.3) | `DisplayPlacementType` | `src/adcom/enums/display_placement_type.rs` | ✅ Exists |
| Data Asset Types (7.4) | `NativeDataAssetType` | `src/adcom/enums/native_data_asset_type.rs` | ✅ Exists |
| Image Asset Types (7.5) | `NativeImageAssetType` | `src/adcom/enums/native_image_asset_type.rs` | ✅ Exists |
| Event Types (7.6) | `EventType` | `src/adcom/enums/event_type.rs` | ✅ Exists |
| Event Tracking Methods (7.7) | `EventTrackingMethod` | `src/adcom/enums/event_tracking_method.rs` | ✅ Exists |

**Result**: No new enumerations needed!

### 1.4 Key Specification Features

#### Multi-Placement Bidding
- `plcmtcnt` > 1 indicates multiple identical placements
- Generalized Second Price auction across placements
- Multiple bids per impression ID allowed

#### Dynamic Creative Optimization (DCO)
- `assetsurl`: URL returning asset JSON
- `dcourl`: Beta feature for dynamic creative retrieval
- Alternative to inline asset responses

#### Event Tracking Evolution
- v1.2: `eventtrackers` array (preferred)
- v1.0/1.1: `imptrackers`, `jstracker` (deprecated)
- Supports both for backward compatibility

## 2. Codebase Analysis

### 2.1 Existing Patterns

The iab-specs codebase follows consistent patterns across all implementations:

#### Struct Pattern
```rust
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ObjectName<Ext: Extension = serde_json::Value> {
    /// Documentation for field
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Type>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ObjectName {
    pub fn builder() -> ObjectNameBuilder {
        ObjectNameBuilder::create_empty()
    }
}
```

#### Testing Pattern
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_creation() { /* ... */ }

    #[test]
    fn test_object_serialization() { /* ... */ }

    #[test]
    fn test_object_deserialization() { /* ... */ }

    #[test]
    fn test_roundtrip() { /* ... */ }
}
```

### 2.2 Module Organization

Current OpenRTB structure:
```
src/openrtb/
├── common/          # Shared objects (SupplyChain, etc.)
├── v25/             # OpenRTB 2.5
│   ├── native.rs    # Simple wrapper (JSON string)
│   ├── banner.rs
│   └── ...
├── v26/             # OpenRTB 2.6 extensions
└── v30/             # OpenRTB 3.0
```

Proposed addition:
```
src/openrtb/
├── native/
│   └── v12/         # OpenRTB Native 1.2
│       ├── mod.rs
│       ├── request.rs   # Request objects
│       └── response.rs  # Response objects
```

### 2.3 Feature Flag Dependencies

Current features:
```toml
adcom = []
openrtb_25 = ["adcom"]
openrtb_26 = ["openrtb_25"]
openrtb_30 = ["adcom"]
```

Proposed addition:
```toml
openrtb_native_12 = ["adcom"]
```

**Rationale**: Native 1.2 is independent of OpenRTB version but requires AdCOM enums.

## 3. Architecture Design

### 3.1 Module Structure

```
src/openrtb/native/v12/
├── mod.rs           # Module organization, re-exports
├── request.rs       # Request objects (7 structs)
└── response.rs      # Response objects (8 structs)
```

### 3.2 Request Objects (request.rs)

#### 3.2.1 NativeRequest
```rust
/// OpenRTB Native 1.2 Request
///
/// Root object for native ad request specification.
///
/// # Example
/// ```
/// use iab_specs::openrtb::native::v12::{NativeRequest, Asset, Title};
///
/// let request = NativeRequest::builder()
///     .ver(Some("1.2".to_string()))
///     .context(Some(1))  // Content-centric
///     .plcmttype(Some(1)) // In-feed
///     .assets(vec![
///         Asset::builder()
///             .id(1)
///             .required(Some(1))
///             .title(Some(Title::builder()
///                 .len(90)
///                 .build()
///                 .unwrap()))
///             .build()
///             .unwrap()
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NativeRequest<Ext: Extension = serde_json::Value> {
    /// Version of the Native Markup. Default: "1.2"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ver: Option<String>,

    /// Context type. Recommended.
    /// Refer to `DisplayContextType` (1=Content, 2=Social, 3=Product)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub context: Option<i32>,

    /// Context subtype. Optional finer-grained context.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub contextsubtype: Option<i32>,

    /// Placement type. Recommended.
    /// Refer to `DisplayPlacementType` (1=Feed, 2=Sidebar, 3=Interstitial, 4=Floating)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub plcmttype: Option<i32>,

    /// Placement count. Number of identical placements in the feed/stream.
    /// Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub plcmtcnt: Option<i32>,

    /// Sequence number. 0 for first ad, 1+ for subsequent.
    /// Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seq: Option<i32>,

    /// Array of asset objects. **Required**
    #[builder(default, setter(into))]
    pub assets: Vec<Asset<Ext>>,

    /// Asset URL support. 0=no, 1=yes.
    /// Indicates support for returning asset objects via URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aurlsupport: Option<i32>,

    /// DCO URL support. 0=no, 1=yes.
    /// Beta feature for dynamic creative optimization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub durlsupport: Option<i32>,

    /// Event trackers. Preferred over deprecated imptrackers/jstracker.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub eventtrackers: Option<Vec<EventTracker<Ext>>>,

    /// Privacy/AdChoices support. 0=no, 1=yes. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub privacy: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}
```

#### 3.2.2 Asset
```rust
/// Native Asset Request
///
/// Represents a single asset in the native ad request.
/// Must contain exactly one of: title, img, video, or data.
///
/// # Example
/// ```
/// use iab_specs::openrtb::native::v12::{Asset, Image};
/// use iab_specs::adcom::enums::NativeImageAssetType;
///
/// let asset = Asset::builder()
///     .id(2)
///     .required(Some(1))
///     .img(Some(Image::builder()
///         .type_(NativeImageAssetType::Main as i32)
///         .w(Some(1200))
///         .h(Some(627))
///         .build()
///         .unwrap()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Asset<Ext: Extension = serde_json::Value> {
    /// Unique asset ID. **Required**
    /// Used to map response assets to request assets.
    #[builder(setter(into))]
    pub id: i32,

    /// Asset required flag. 0=optional, 1=required.
    /// If required=1 and asset not provided, bid may be rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub required: Option<i32>,

    /// Title object. Mutually exclusive with img, video, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<Title<Ext>>,

    /// Image object. Mutually exclusive with title, video, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub img: Option<Image<Ext>>,

    /// Video object. Mutually exclusive with title, img, data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub video: Option<Video<Ext>>,

    /// Data object. Mutually exclusive with title, img, video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<Data<Ext>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}
```

#### 3.2.3 Title, Image, Video, Data, EventTracker

Similar patterns with appropriate fields from specification (see Section 4 of spec).

### 3.3 Response Objects (response.rs)

#### 3.3.1 NativeResponse
```rust
/// OpenRTB Native 1.2 Response
///
/// Root object for native ad response.
///
/// # Example
/// ```
/// use iab_specs::openrtb::native::v12::{NativeResponse, AssetResponse, Link, TitleResponse};
///
/// let response = NativeResponse::builder()
///     .assets(vec![
///         AssetResponse::builder()
///             .id(1)
///             .required(Some(1))
///             .title(Some(TitleResponse::builder()
///                 .text("Amazing Product".to_string())
///                 .build()
///                 .unwrap()))
///             .build()
///             .unwrap()
///     ])
///     .link(Link::builder()
///         .url("https://example.com/product".to_string())
///         .build()
///         .unwrap())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NativeResponse<Ext: Extension = serde_json::Value> {
    /// Version of the Native Markup
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ver: Option<String>,

    /// Array of asset response objects. **Required**
    #[builder(default, setter(into))]
    pub assets: Vec<AssetResponse<Ext>>,

    /// Link object. **Required**
    /// Default destination and click tracking for the ad.
    #[builder(setter(into))]
    pub link: Link<Ext>,

    /// Impression tracking URLs. **Deprecated** - use eventtrackers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub imptrackers: Option<Vec<String>>,

    /// JavaScript tracker. **Deprecated** - use eventtrackers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub jstracker: Option<String>,

    /// Event trackers. Preferred method for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub eventtrackers: Option<Vec<EventTrackerResponse<Ext>>>,

    /// Privacy/AdChoices link URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub privacy: Option<String>,

    /// Asset URL. Alternative to inline assets array.
    /// URL must return JSON with assets array.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub assetsurl: Option<String>,

    /// DCO URL. Beta feature for dynamic creative optimization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dcourl: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}
```

#### 3.3.2 AssetResponse, Link, EventTrackerResponse, etc.

Similar patterns with appropriate fields from specification (see Section 5 of spec).

### 3.4 Integration with OpenRTB

The native objects integrate seamlessly with existing OpenRTB implementations:

#### OpenRTB 2.5/2.6 Integration
```rust
use iab_specs::openrtb::v25::{BidRequest, Imp, Native};
use iab_specs::openrtb::native::v12::NativeRequest;

// Create native request
let native_req = NativeRequest::builder()
    .ver(Some("1.2".to_string()))
    .assets(/* ... */)
    .build()?;

// Serialize to JSON string for OpenRTB Native object
let native_json = serde_json::to_string(&native_req)?;

// Embed in OpenRTB bid request
let bid_request = BidRequest::builder()
    .id("req-123".to_string())
    .imp(vec![
        Imp::builder()
            .id("imp1".to_string())
            .native(Some(Native::builder()
                .request(native_json)
                .ver(Some("1.2".to_string()))
                .build()?))
            .build()?
    ])
    .build()?;
```

#### OpenRTB 3.0 Integration
Similar pattern using OpenRTB 3.0 objects and Item spec field.

## 4. Implementation Tasks

### Phase 1: Core Request Objects (4-6 hours)
**Assignee**: Senior Software Engineer or Sonnet Model

#### Task 1.1: Create module structure
- [ ] Create `src/openrtb/native/` directory
- [ ] Create `src/openrtb/native/v12/` directory
- [ ] Create `mod.rs` with module declaration

**Acceptance Criteria**:
- Module compiles with feature flag
- Proper visibility and re-exports

#### Task 1.2: Implement NativeRequest
- [ ] Define `NativeRequest` struct with all fields
- [ ] Implement builder pattern
- [ ] Add documentation with examples
- [ ] Write unit tests (creation, serialization, deserialization)

**Acceptance Criteria**:
- All fields from spec implemented
- Tests pass with 90%+ coverage
- Documentation includes example

#### Task 1.3: Implement Asset
- [ ] Define `Asset` struct
- [ ] Implement builder pattern
- [ ] Add documentation
- [ ] Write unit tests

**Acceptance Criteria**:
- Properly handles mutually exclusive fields (title/img/video/data)
- Tests validate mutual exclusivity
- Documentation explains constraints

#### Task 1.4: Implement Title, Image, Video, Data
- [ ] Define all 4 structs
- [ ] Implement builder patterns
- [ ] Add documentation
- [ ] Write unit tests for each

**Acceptance Criteria**:
- All spec fields implemented
- Tests validate constraints (e.g., Title.len)
- Documentation includes examples

#### Task 1.5: Implement EventTracker (Request)
- [ ] Define `EventTracker` struct
- [ ] Implement builder pattern
- [ ] Add documentation
- [ ] Write unit tests

**Acceptance Criteria**:
- Proper use of AdCOM enums
- Tests cover all event types
- Documentation references AdCOM

### Phase 2: Core Response Objects (4-6 hours)
**Assignee**: Senior Software Engineer or Sonnet Model

#### Task 2.1: Implement NativeResponse
- [ ] Define `NativeResponse` struct
- [ ] Implement builder pattern
- [ ] Handle deprecated fields (imptrackers, jstracker)
- [ ] Add documentation with deprecation warnings
- [ ] Write unit tests

**Acceptance Criteria**:
- Supports both new and deprecated tracking
- Tests validate backward compatibility
- Documentation warns about deprecations

#### Task 2.2: Implement AssetResponse
- [ ] Define `AssetResponse` struct
- [ ] Implement builder pattern
- [ ] Add documentation
- [ ] Write unit tests

**Acceptance Criteria**:
- Properly maps to request assets via ID
- Tests validate response matching
- Documentation explains mapping

#### Task 2.3: Implement TitleResponse, ImageResponse, VideoResponse, DataResponse
- [ ] Define all 4 response structs
- [ ] Implement builder patterns
- [ ] Add documentation
- [ ] Write unit tests for each

**Acceptance Criteria**:
- All spec fields implemented
- Tests validate constraints
- Documentation includes examples

#### Task 2.4: Implement Link
- [ ] Define `Link` struct
- [ ] Implement builder pattern
- [ ] Add documentation
- [ ] Write unit tests

**Acceptance Criteria**:
- Supports click tracking arrays
- Tests validate URL handling
- Documentation explains click tracking

#### Task 2.5: Implement EventTrackerResponse
- [ ] Define `EventTrackerResponse` struct
- [ ] Implement builder pattern
- [ ] Add documentation
- [ ] Write unit tests

**Acceptance Criteria**:
- Proper use of AdCOM enums
- Tests cover all tracking methods
- Documentation references AdCOM

### Phase 3: Integration Tests (2-3 hours)
**Assignee**: Senior Software Engineer or Sonnet Model

#### Task 3.1: Request-Response Round-trip Tests
- [ ] Test complete request serialization
- [ ] Test complete response deserialization
- [ ] Test request-response matching
- [ ] Test multi-placement scenarios

**Acceptance Criteria**:
- Round-trip preserves all data
- Tests cover edge cases
- Multi-placement logic validated

#### Task 3.2: OpenRTB Integration Tests
- [ ] Test embedding in OpenRTB 2.5 Native
- [ ] Test embedding in OpenRTB 2.6 Native
- [ ] Test embedding in OpenRTB 3.0 spec
- [ ] Test JSON string conversion

**Acceptance Criteria**:
- Integration works seamlessly
- Tests use realistic examples
- Documentation shows integration

#### Task 3.3: DCO and Alternative Delivery Tests
- [ ] Test assetsurl delivery
- [ ] Test dcourl delivery
- [ ] Test aurlsupport/durlsupport flags

**Acceptance Criteria**:
- Alternative delivery modes validated
- Tests cover spec requirements
- Documentation explains usage

### Phase 4: Documentation and Examples (2-3 hours)
**Assignee**: Senior Software Engineer or Sonnet Model

#### Task 4.1: Module Documentation
- [ ] Add comprehensive module-level docs
- [ ] Document request flow
- [ ] Document response flow
- [ ] Add architecture overview

**Acceptance Criteria**:
- Docs explain native ad concepts
- Examples show common use cases
- Architecture diagram included

#### Task 4.2: Example Code
- [ ] Create example: SSP native request
- [ ] Create example: DSP native response
- [ ] Create example: OpenRTB 2.5 integration
- [ ] Create example: OpenRTB 3.0 integration

**Acceptance Criteria**:
- Examples compile and run
- Examples show realistic scenarios
- Examples documented inline

#### Task 4.3: README Updates
- [ ] Add native 1.2 to feature list
- [ ] Add native 1.2 usage example
- [ ] Update roadmap
- [ ] Update feature flag documentation

**Acceptance Criteria**:
- README clearly describes feature
- Examples easy to copy-paste
- Feature flags documented

### Phase 5: Testing and Coverage (2-3 hours)
**Assignee**: Senior Software Engineer or Haiku Model

#### Task 5.1: Unit Test Coverage
- [ ] Ensure 90%+ line coverage for request.rs
- [ ] Ensure 90%+ line coverage for response.rs
- [ ] Add edge case tests
- [ ] Add error case tests

**Acceptance Criteria**:
- Coverage meets or exceeds 90%
- All public APIs tested
- Error paths validated

#### Task 5.2: Integration Test Coverage
- [ ] Test all enum values
- [ ] Test all optional field combinations
- [ ] Test extension mechanism
- [ ] Test builder validation

**Acceptance Criteria**:
- Comprehensive enum coverage
- Optional field combinations validated
- Extensions work correctly

#### Task 5.3: Benchmark Tests (Optional)
- [ ] Create serialization benchmarks
- [ ] Create deserialization benchmarks
- [ ] Compare with JSON string approach

**Acceptance Criteria**:
- Benchmarks runnable via cargo
- Performance acceptable
- Results documented

## 5. File Structure

### 5.1 New Files

```
src/openrtb/native/
├── mod.rs                    (~50 lines)
└── v12/
    ├── mod.rs                (~200 lines - docs, re-exports, integration tests)
    ├── request.rs            (~600 lines - 6 structs with docs and tests)
    └── response.rs           (~700 lines - 8 structs with docs and tests)
```

**Total New Code**: ~1,550 lines

### 5.2 Modified Files

```
src/openrtb/mod.rs            (+2 lines - add native module)
src/lib.rs                    (+3 lines - add native docs)
Cargo.toml                    (+1 line - add feature flag)
README.md                     (+30 lines - usage examples)
```

## 6. Testing Strategy

### 6.1 Unit Tests (Per-Object)
- Object creation via builder
- Field validation
- Serialization to JSON
- Deserialization from JSON
- Round-trip (serialize → deserialize)
- Extension support

### 6.2 Integration Tests
- Complete request creation
- Complete response parsing
- Request-response matching
- Multi-asset scenarios
- Multi-placement scenarios
- OpenRTB 2.5 integration
- OpenRTB 3.0 integration

### 6.3 Edge Cases
- Empty arrays
- Null/None fields
- Invalid enum values
- Missing required fields
- Oversized strings
- Deprecated field handling

### 6.4 Coverage Targets
- Line coverage: 90%+
- Branch coverage: 85%+
- Function coverage: 100%

## 7. Documentation Requirements

### 7.1 Code Documentation
- Module-level overview
- Struct documentation with examples
- Field documentation with spec references
- Builder usage examples
- Integration examples

### 7.2 README Updates
- Feature description
- Installation instructions
- Basic usage example
- Advanced usage example
- Migration notes (if any)

### 7.3 API Documentation
- rustdoc comments on all public items
- Examples in doc comments
- Links to spec sections
- Cross-references to AdCOM enums

## 8. Dependencies and Prerequisites

### 8.1 Rust Crates (Already in Cargo.toml)
- `serde` - Serialization framework
- `serde_json` - JSON support
- `serde_with` - Serde utilities
- `serde_repr` - Enum serialization
- `derive_builder` - Builder pattern
- `thiserror` - Error handling

### 8.2 Existing Code Dependencies
- `crate::Extension` - Extension trait
- `crate::Error` - Error type
- `crate::adcom::enums::*` - All native-related enums

### 8.3 Feature Flags
```toml
[features]
openrtb_native_12 = ["adcom"]
```

## 9. Risk Assessment

### 9.1 Low Risk Items ✅
- **Enum Reuse**: All enums exist in AdCOM (no conflicts)
- **Pattern Consistency**: Follows established codebase patterns
- **Type Safety**: Rust's type system prevents errors
- **Testing**: Comprehensive test strategy defined

### 9.2 Medium Risk Items ⚠️
- **Mutual Exclusivity**: Asset must have exactly one of title/img/video/data
  - *Mitigation*: Document constraint, add validation tests
- **Deprecated Fields**: Must support both old and new tracking
  - *Mitigation*: Clear documentation, backward compatibility tests
- **Multi-Placement**: Complex auction logic
  - *Mitigation*: Document spec requirements, add integration tests

### 9.3 High Risk Items ❌
- None identified

## 10. Success Criteria

### 10.1 Functional Requirements
- ✅ All request objects implemented per spec
- ✅ All response objects implemented per spec
- ✅ All fields correctly typed
- ✅ Builder pattern works ergonomically
- ✅ JSON serialization/deserialization works
- ✅ Integration with OpenRTB 2.5/2.6/3.0 works

### 10.2 Quality Requirements
- ✅ 90%+ test coverage
- ✅ All public APIs documented
- ✅ Examples compile and run
- ✅ No clippy warnings
- ✅ Passes CI/CD pipeline

### 10.3 Performance Requirements
- ✅ Serialization performance acceptable (< 10μs per object)
- ✅ Deserialization performance acceptable (< 20μs per object)
- ✅ No significant binary size increase (< 100KB)

## 11. Timeline Estimate

| Phase | Duration | Dependencies | Assignee |
|-------|----------|-------------|----------|
| Phase 1: Request Objects | 4-6 hours | None | Sonnet/Senior Dev |
| Phase 2: Response Objects | 4-6 hours | Phase 1 | Sonnet/Senior Dev |
| Phase 3: Integration Tests | 2-3 hours | Phase 1, 2 | Sonnet/Senior Dev |
| Phase 4: Documentation | 2-3 hours | Phase 1, 2, 3 | Sonnet/Senior Dev |
| Phase 5: Testing & Coverage | 2-3 hours | Phase 1, 2, 3 | Haiku/Senior Dev |

**Total Estimated Time**: 14-21 hours

**Parallelization Opportunities**:
- Phases 1 and 2 can be partially parallelized (different files)
- Phase 4 can start when Phase 1 is complete
- Phase 5 can overlap with Phases 3 and 4

## 12. Code Style Guidelines

### 12.1 Naming Conventions
- Structs: PascalCase (e.g., `NativeRequest`)
- Fields: snake_case (e.g., `plcmttype`)
- Builders: StructNameBuilder (auto-generated)
- Enums: PascalCase values (e.g., `ContentCentric`)

### 12.2 Documentation Style
```rust
/// Brief one-line description
///
/// Longer description paragraph explaining purpose and usage.
///
/// # Example
/// ```
/// use iab_specs::openrtb::native::v12::NativeRequest;
///
/// let request = NativeRequest::builder()
///     .ver(Some("1.2".to_string()))
///     .build()
///     .unwrap();
/// ```
pub struct StructName { /* ... */ }
```

### 12.3 Field Documentation
```rust
/// Field description.
/// Refer to `EnumName` enumeration for values.
/// Default: value
#[serde(skip_serializing_if = "Option::is_none")]
#[builder(default)]
pub field: Option<Type>,
```

## 13. Rollout Strategy

### 13.1 Development
1. Create feature branch: `feat/openrtb-native-12`
2. Implement Phase 1 (request objects)
3. Implement Phase 2 (response objects)
4. Implement Phase 3 (integration tests)
5. Implement Phase 4 (documentation)
6. Implement Phase 5 (coverage)

### 13.2 Review
1. Self-review checklist:
   - All tests pass
   - Coverage meets targets
   - Documentation complete
   - Examples work
   - No clippy warnings
2. Code review by maintainer
3. Address review feedback

### 13.3 Release
1. Merge to main
2. Update CHANGELOG.md
3. Create git tag (e.g., v0.3.0)
4. Publish to crates.io
5. Update documentation site

## 14. Future Enhancements

### 14.1 Potential Extensions
- **Native 1.3**: When IAB releases next version
- **Validation Helpers**: Utility functions to validate requests/responses
- **Builder Macros**: Simplify complex nested structures
- **Serde Formats**: Support additional serialization formats
- **Performance**: SIMD optimizations for JSON parsing

### 14.2 API Stability
- **Stable**: Core request/response structures (follow semver)
- **Unstable**: Extension mechanisms (may evolve)
- **Deprecated**: Support removed in major version bumps only

## 15. References

### 15.1 Specifications
- [OpenRTB Native 1.2 Final](https://github.com/InteractiveAdvertisingBureau/Native-Ads/blob/main/OpenRTB-Native-Ads-Specification-Final-1.2.md)
- [Implementation Notes](https://github.com/InteractiveAdvertisingBureau/Native-Ads/blob/main/implementation.md)
- [AdCOM 1.0](https://github.com/InteractiveAdvertisingBureau/AdCOM)
- [OpenRTB 2.5](https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf)
- [OpenRTB 3.0](https://github.com/InteractiveAdvertisingBureau/openrtb/blob/main/OpenRTB%20v3.0%20FINAL.md)

### 15.2 Codebase References
- Pattern: `src/openrtb/v25/banner.rs`
- Enums: `src/adcom/enums/`
- Testing: `src/openrtb/v30/mod.rs` (integration tests)
- Documentation: `src/lib.rs` (examples)

## 16. Glossary

| Term | Definition |
|------|------------|
| **Native Ad** | Ad format that matches the form and function of the platform it appears on |
| **Asset** | Individual component of a native ad (title, image, etc.) |
| **DCO** | Dynamic Creative Optimization - serving personalized ad creatives |
| **Multi-Placement** | Multiple identical ad slots in a single request |
| **Event Tracker** | Mechanism for tracking ad events (impressions, clicks) |
| **AdCOM** | Advertising Common Object Model - shared enumerations |
| **OpenRTB** | Open Real-Time Bidding protocol |

---

## Approval

**Prepared by**: Claude (Sonnet)
**Review Required**: Project Maintainer
**Status**: Ready for Implementation

**Next Steps**:
1. Review and approve blueprint
2. Assign tasks to developers
3. Begin Phase 1 implementation
4. Track progress against timeline
