# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2026-04-11

### 🚀 Features

- Add support for ads.txt 1.1 and sellers.json 1.0
- Added features ads_txt and sellers_json. default feature uses both.
- Add app-ads.txt v1.0 support for mobile and CTV applications
- Disable default features and add comprehensive feature testing
- *(adcom)* Implement AdCOM 1.0 Enumerations and Context Objects
- *(openrtb)* Add OpenRTB 2.5/2.6 infrastructure with SupplyChain support
- *(openrtb)* Add OpenRTB 2.5 core bid objects
- *(openrtb)* Add OpenRTB 2.5 impression and media objects
- Add OpenRTB 2.5 context objects
- Add OpenRTB 2.5 user and device objects
- Add OpenRTB 2.5 source and regs objects
- Add feature flag support to coverage.sh script
- Complete OpenRTB 2.6 implementation with CTV and DOOH support
- Add complete OpenRTB 3.0 implementation
- Add shell scripts for CI operations
- Add Extension struct and refactor codebase to use it.
- Add OpenRTB Native Ads 1.2 specification support
- Add Agentic RTB Framework 1.0 specification support
- Add proto feature for protobuf transport support
- *(agentic_direct)* Add feature flag and module skeleton
- *(agentic_direct)* Add OpenDirect status enums
- *(agentic_direct)* Add A2A protocol enums
- *(agentic_direct)* Add JSON-RPC 2.0 message types
- *(agentic_direct)* Add Organization entity
- *(agentic_direct)* Add Account, Product, and Creative entities
- *(agentic_direct)* Add Order entity with state machine
- *(agentic_direct)* Add Line and Assignment entities with state machine
- *(agentic_direct)* Add A2A Agent Card, Task, and state machine
- *(agentic_direct)* Add integration tests, doc examples, and README update
- *(buyer_agent)* Add feature flag and module skeleton
- *(buyer_agent)* Add DealStatus and CampaignStatus enums
- *(buyer_agent)* Add strategy and booking enums
- *(buyer_agent)* Add CampaignBrief and CampaignAllocation models
- *(buyer_agent)* Add UCPEmbedding and AudiencePlan models
- *(buyer_agent)* Add NegotiationStrategy and NegotiationOffer models
- *(buyer_agent)* Add BookingJob and BookingRecommendation models
- *(buyer_agent)* Add Deal Lifecycle state machine
- *(buyer_agent)* Add Campaign Lifecycle state machine
- *(buyer_agent)* Add integration tests, doc examples, and README update
- *(agentic_direct)* Add ChangeRequest and Placement entities
- *(agentic_direct)* Add A2AMessage and task history support
- *(agentic_direct)* Add MCPTool type
- *(agentic_direct)* Add integration tests and update README
- *(seller_agent)* Add feature flag and module skeleton
- *(seller_agent)* Add proposal, pricing, and media kit enums
- *(seller_agent)* Add change request, execution, and distribution enums
- *(seller_agent)* Add TieredPricing, PricingTier, and RateCard models
- *(seller_agent)* Add MediaKit and Package models
- *(seller_agent)* Add NegotiationConfig and NegotiationRound
- *(seller_agent)* Add Proposal, ProposalRevision, and ProposalItem
- *(seller_agent)* Add ChangeRequest model
- *(seller_agent)* Add ExecutionOrder model
- *(seller_agent)* Add DealDistribution and DspIntegration models
- *(seller_agent)* Add Seller Order Lifecycle state machine
- *(seller_agent)* Add integration tests, doc examples, and README update
- *(seller_agent)* Add PricingRule and VolumeDiscount models
- *(seller_agent)* Add Creative and Assignment models
- *(seller_agent)* Add Product and InventorySegment models
- *(seller_agent)* Add PublicPackageView and AuthenticatedPackageView models
- *(seller_agent)* Add Organization and Account extensions
- *(agentic_audience)* Add feature flag and module skeleton
- *(agentic_audience)* Add model type enums
- *(agentic_audience)* Add EmbeddingType, TemporalScope, and CompositionType enums
- *(agentic_audience)* Add signal taxonomy enums
- *(agentic_audience)* Add EmbeddingModel and EmbeddingContext
- *(agentic_audience)* Add EmbeddingEnvelope struct
- *(agentic_audience)* Add EmbeddingSegmentExt for OpenRTB integration
- *(agentic_audience)* Add Campaign Scoring models (CampaignHead, ScoringRequest, ScoringResponse, CampaignScore)
- *(agentic_audience)* Add integration tests, doc examples, and README update
- *(buyer_agent)* Add ChannelBrief model
- *(buyer_agent)* Add BookedLine model
- *(buyer_agent)* Add BuyerIdentity model
- *(buyer_agent)* Add LinearTVParams model
- *(buyer_agent)* Add UCPModelDescriptor and UCPConsent models
- *(buyer_agent)* Add V2 type aliases and updated integration tests
- *(registry_agent)* Add feature flag and module skeleton
- *(registry_agent)* Add VerificationStatus enum
- *(registry_agent)* Add TrustLevel, AgentType enums and Trust state machine
- *(registry_agent)* Add RegistrySearchFilter and SearchResult
- *(registry_agent)* Add RegisteredAgent, RegistrySource, and AgentTrustInfo
- *(registry_agent)* Add integration tests, doc examples, and README update
- *(scripts)* Update bump-version.sh for workspace with full automation
- *(scripts)* Add publish.sh for ordered crates.io publishing

### 🐛 Bug Fixes

- Add categories and authors to Cargo.toml
- Ads.txt parsing error when cert_id is not present.
- Limit the length of the output in case of an unexpected field.
- Rename badly named unit tests.
- Fix parsing of systems with non-ascii characters.
- Fix formatting.
- *(ci)* Remove json/proto feature combinations from CI matrix
- *(seller_agent)* Register product module in models/mod.rs
- *(seller_agent)* Add serde default to product segments field
- *(seller_agent)* Fix product doc test string type mismatches
- *(agentic_audience)* Remove unused CompositionType import in scoring_request tests
- Add missing READMEs and update umbrella metadata per final audit
- *(scripts)* Update inter-crate dependency versions on version bump
- *(scripts)* Use dynamic cargo target dir and force-add Cargo.lock

### 🚜 Refactor

- Fix format issue.
- Include openrtb_native_12 in github action test matrix.
- Removes now useless file.
- Enforce consistent module re-export convention
- Remove json/proto features, default extensions to Vec<u8>
- *(workspace)* Scaffold workspace directory structure
- *(core)* Extract shared Extension trait, errors, and macros into iab-specs-core
- *(workspace)* Migrate adcom, ads_txt, sellers_json, artb to sub-crates (partial Wave 2)
- *(workspace)* Fix artb doc tests and remove old artb source
- *(agentic_direct)* Extract agentic_direct into sub-crate
- *(agentic_audience)* Extract agentic_audience into iab-specs-agentic_audience sub-crate
- *(openrtb_native)* Extract openrtb_native into iab-specs-openrtb_native sub-crate
- *(openrtb)* Extract openrtb into iab-specs-openrtb sub-crate
- *(app_ads_txt)* Extract app_ads_txt into iab-specs-app_ads_txt sub-crate
- *(buyer_agent)* Extract buyer_agent into iab-specs-buyer_agent sub-crate
- *(seller_agent)* Extract seller_agent into iab-specs-seller_agent sub-crate
- *(registry_agent)* Extract registry_agent into iab-specs-registry_agent sub-crate
- *(umbrella)* Wire iab-specs umbrella re-exports for all sub-crates
- *(scripts)* Update dev scripts for workspace support

### 📚 Documentation

- Enhance README for crates.io with examples and clarity
- Improve documentation clarity and examples
- Enhance Extension trait documentation and standardize builder pattern
- Update generic parameter defaults to reference DefaultExt
- Add commit signing and conventional commits guidelines
- Replace gitsign references with GitHub signing documentation
- *(plans)* Add AAMP specification work plans
- Update README and CONTRIBUTING for workspace structure

### 🎨 Styling

- *(agentic_direct)* Fix rustfmt formatting for CI compliance
- *(buyer_agent)* Fix rustfmt formatting for CI compliance
- Fix rustfmt formatting for CI compliance
- *(lib)* Alphabetize agentic_audience module declaration
- *(buyer_agent)* Apply rustfmt formatting fixes
- *(registry_agent)* Apply rustfmt formatting fixes
- Fix cargo fmt formatting across workspace

### 🧪 Testing

- Improve code coverage to >80% for all files
- Add comprehensive test coverage for app-ads.txt implementation
- Add comprehensive tests improving coverage.
- *(registry_agent_10)* Harden test coverage for Registry Agent 1.0
- *(agentic_direct_21)* Harden test coverage for Agentic Direct 2.1
- *(adcom)* Harden test coverage for AdCOM 1.0
- *(ads_txt)* Harden test coverage for Ads.txt 1.1
- *(ads_txt)* Harden test coverage for Ads.txt 1.1
- *(sellers_json)* Harden test coverage for Sellers.json 1.0
- *(app_ads_txt)* Harden test coverage for App-ads.txt 1.0
- *(agentic_audience_10)* Harden test coverage for Agentic Audience v1.0
- *(openrtb_25)* Harden test coverage for OpenRTB 2.5
- *(openrtb_26)* Harden test coverage for OpenRTB 2.6
- *(openrtb_30)* Harden test coverage for OpenRTB 3.0
- *(openrtb_native_12)* Harden test coverage for OpenRTB Native Ads 1.2
- *(artb_10)* Harden test coverage for Agentic RTB 1.0
- *(buyer_agent_10)* Harden test coverage for Buyer Agent 1.0
- *(seller_agent_10)* Harden test coverage for Seller Agent 1.0

### ⚙️ Miscellaneous Tasks

- Add CI tooling and enhance developer experience
- Set MSRV to 1.70 and use edition 2021 for compatibility
- Add development environment initialization script
- Add verify-signatures and conventional-commits workflows
- Remove gitsign verification workflow and add test summary job
- Remove .sisyphus planning files
- Add missing agentic feature gates to test matrix
- Update workflow for cargo workspace structure

<!-- generated by git-cliff -->
