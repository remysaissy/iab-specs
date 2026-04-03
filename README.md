# iab-specs

[![Crates.io](https://img.shields.io/crates/v/iab-specs.svg)](https://crates.io/crates/iab-specs)
[![Documentation](https://docs.rs/iab-specs/badge.svg)](https://docs.rs/iab-specs)
[![License](https://img.shields.io/crates/l/iab-specs.svg)](LICENSE)

An unofficial Rust implementation of various IAB (Interactive Advertising Bureau) specifications.

## Overview

`iab-specs` provides typed Rust data structures for working with IAB advertising specifications. Rather than being just a parser, this library wraps each specification's logic into idiomatic Rust types using `serde` for serialization/deserialization.

### Currently Supported Specifications

- **[AdCOM 1.0](https://github.com/InteractiveAdvertisingBureau/AdCOM)** - Advertising Common Object Model (enumerations)
- **[OpenRTB 2.5](https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf)** - Real-Time Bidding protocol
- **[OpenRTB 2.6](https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md)** - Real-Time Bidding protocol with CTV/DOOH support
- **[OpenRTB 3.0](https://github.com/InteractiveAdvertisingBureau/openrtb/blob/main/OpenRTB%20v3.0%20FINAL.md)** - Real-Time Bidding protocol with layered architecture
- **[OpenRTB Native Ads 1.2](https://github.com/InteractiveAdvertisingBureau/Native-Ads/blob/main/OpenRTB-Native-Ads-Specification-Final-1.2.md)** - Native advertising specification
- **[Ads.txt 1.1](https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf)** - Authorized Digital Sellers declaration for websites
- **[App-ads.txt 1.0](https://iabtechlab.com/wp-content/uploads/2019/03/app-ads.txt-v1.0-final-.pdf)** - Authorized Digital Sellers declaration for mobile and CTV apps
- **[Sellers.json 1.0](https://iabtechlab.com/wp-content/uploads/2019/07/Sellers.json_Final.pdf)** - Supply chain transparency
- **[Agentic RTB Framework 1.0](https://github.com/IABTechLab/agentic-rtb-framework)** - Autonomous agent bidstream processing via the OpenRTB Patch Protocol
- **[Agentic Direct 2.1](https://github.com/IABTechLab/agentic-direct)** — OpenDirect v2.1 + A2A Protocol for direct campaign management
- **[Buyer Agent 1.0](https://github.com/IABTechLab/buyer-agent)** — Demand-side campaign planning, UCP embeddings, negotiation, booking workflows, 2 state machines
- **[Seller Agent 1.0](https://github.com/IABTechLab/seller-agent)** — Supply-side inventory management, proposals, tiered pricing, negotiation, order execution, 1 state machine
- **[Agentic Audience v1.0 (Draft)](https://github.com/IABTechLab/agentic-audiences)** - Embedding exchange protocol for audience targeting

## Installation

Add `iab-specs` to your `Cargo.toml` with the features you need:

```toml
[dependencies]
# Enable all specifications
iab-specs = { version = "0.4", features = ["adcom", "openrtb_25", "openrtb_26", "openrtb_30", "openrtb_native_12", "ads_txt", "app_ads_txt", "sellers_json", "artb_10", "agentic_direct_21", "buyer_agent_10", "seller_agent_10", "agentic_audience_10"] }

# Or enable only what you need
iab-specs = { version = "0.4", features = ["openrtb_30"] }
```

Or use cargo:

```bash
# Enable all specifications
cargo add iab-specs --features adcom,openrtb_25,openrtb_26,openrtb_30,openrtb_native_12,ads_txt,app_ads_txt,sellers_json,artb_10,agentic_direct_21,buyer_agent_10,seller_agent_10,agentic_audience_10

# Or enable only what you need
cargo add iab-specs --features openrtb_30
```

## Features

⚠️ **Important**: By default, **no features are enabled**. You must explicitly enable the specifications you need.

The library uses cargo features to enable/disable specifications:

**Specifications:**
- `adcom` - AdCOM 1.0 support (Advertising Common Object Model enumerations)
- `openrtb_25` - OpenRTB 2.5 support (automatically includes `adcom`)
- `openrtb_26` - OpenRTB 2.6 support (automatically includes `openrtb_25` and `adcom`)
- `openrtb_30` - OpenRTB 3.0 support (automatically includes `adcom`)
- `openrtb_native_12` - OpenRTB Native Ads 1.2 support (automatically includes `adcom`)
- `ads_txt` - Ads.txt 1.1 support
- `app_ads_txt` - App-ads.txt 1.0 support (automatically includes `ads_txt`)
- `sellers_json` - Sellers.json 1.0 support (includes `serde_json`)
- `artb_10` - Agentic RTB Framework 1.0 support (autonomous agent bidstream processing)
- `agentic_direct_21` - Agentic Direct 2.1 support (automatically includes `serde_json`)
- `buyer_agent_10` - Buyer Agent 1.0 support (automatically includes `agentic_direct_21` and `serde_json`)
- `seller_agent_10` - Seller Agent 1.0 support (automatically includes `agentic_direct_21` and `serde_json`)
- `agentic_audience_10` - Agentic Audience v1.0 (Draft) support (automatically includes `serde_json`)

### Feature Selection Examples

```toml
[dependencies]
# Only AdCOM support
iab-specs = { version = "0.4", features = ["adcom"] }

# Only OpenRTB 2.5 support (automatically includes adcom)
iab-specs = { version = "0.4", features = ["openrtb_25"] }

# Only OpenRTB 2.6 support (automatically includes openrtb_25 and adcom)
iab-specs = { version = "0.4", features = ["openrtb_26"] }

# Only OpenRTB 3.0 support (automatically includes adcom)
iab-specs = { version = "0.4", features = ["openrtb_30"] }

# Only ads.txt support
iab-specs = { version = "0.4", features = ["ads_txt"] }

# Only app-ads.txt support (automatically includes ads_txt)
iab-specs = { version = "0.4", features = ["app_ads_txt"] }

# Only sellers.json support
iab-specs = { version = "0.4", features = ["sellers_json"] }

# OpenRTB 3.0 with ads.txt and sellers.json
iab-specs = { version = "0.4", features = ["openrtb_30", "ads_txt", "sellers_json"] }

# Only ARTB 1.0 support (autonomous agent bidstream processing)
iab-specs = { version = "0.4", features = ["artb_10"] }

# Only Agentic Direct v2.1 support (agent-to-agent advertising transactions)
iab-specs = { version = "0.4", features = ["agentic_direct_21"] }

# Only Buyer Agent 1.0 support (automatically includes agentic_direct_21)
iab-specs = { version = "0.4", features = ["buyer_agent_10"] }

# Only Seller Agent 1.0 support (automatically includes agentic_direct_21)
iab-specs = { version = "0.4", features = ["seller_agent_10"] }

# Only Agentic Audience v1.0 support (embedding exchange protocol)
iab-specs = { version = "0.4", features = ["agentic_audience_10"] }

# All specifications
iab-specs = { version = "0.4", features = ["adcom", "openrtb_25", "openrtb_26", "openrtb_30", "openrtb_native_12", "ads_txt", "app_ads_txt", "sellers_json", "artb_10", "agentic_direct_21", "buyer_agent_10", "seller_agent_10", "agentic_audience_10"] }
```

**Why no default features?**

This design allows you to:
- **Minimize dependencies**: Only include what you actually use
- **Reduce compile time**: Don't compile unused specifications
- **Smaller binary size**: Eliminate unused code from your final binary
- **Explicit dependencies**: Be clear about which IAB specs your project relies on

### Serialization Formats

All types derive `serde::Serialize` and `serde::Deserialize`, making the library transport-agnostic. You can use any serde-compatible format:

- **JSON** — Use `serde_json` for human-readable JSON serialization
- **Protobuf** — Use `prost` to encode/decode typed messages with `Vec<u8>` extension fields
- **MessagePack** — Use `rmp-serde` for compact binary serialization
- **CBOR** — Use `ciborium` for CBOR encoding
- **Any serde format** — Plug in any serializer/deserializer that works with serde

Extension fields default to `Vec<u8>` (opaque bytes), making the library serde-agnostic out of the box. You can use explicit type parameters like `BidRequest<serde_json::Value>` for JSON extensions, or provide your own types implementing the `Extension` trait.

## Quick Start

### Creating an OpenRTB Bid Request

```rust
use iab_specs::openrtb::v25::{BidRequest, Imp, Banner, Device};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a bid request with a 300x250 banner impression
    let request = BidRequest::builder()
        .id("req-12345".to_string())
        .imp(vec![
            Imp::builder()
                .id("imp1".to_string())
                .banner(Some(Banner::builder()
                    .w(Some(300))
                    .h(Some(250))
                    .build()?))
                .bidfloor(Some(0.50)) // $0.50 CPM floor
                .bidfloorcur(Some("USD".to_string()))
                .build()?
        ])
        .device(Some(Device::builder()
            .ua(Some("Mozilla/5.0...".to_string()))
            .ip(Some("192.168.1.1".to_string()))
            .build()?))
        .tmax(Some(100)) // 100ms timeout
        .build()?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&request)?;
    println!("{}", json);

    // Deserialize from JSON
    let parsed: BidRequest = serde_json::from_str(&json)?;
    assert_eq!(parsed.id, "req-12345");

    Ok(())
}
```

### Using the Builder Pattern

For more ergonomic construction, use the builder pattern:

```rust
use iab_specs::openrtb::v25::{BidRequest, Imp, Banner, Device};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = BidRequest::builder()
        .id("req-12345")
        .imp(vec![
            Imp::builder()
                .id("imp1")
                .banner(Some(Banner::builder()
                    .w(Some(300))
                    .h(Some(250))
                    .build()?))
                .bidfloor(Some(0.50))
                .bidfloorcur(Some("USD".to_string()))
                .build()?
        ])
        .device(Some(Device::builder()
            .ua(Some("Mozilla/5.0...".to_string()))
            .ip(Some("192.168.1.1".to_string()))
            .build()?))
        .tmax(Some(100))
        .build()?;

    Ok(())
}
```

## Usage Examples

### AdCOM

Use standardized enumeration values from the Advertising Common Object Model:

```rust
use iab_specs::adcom::{AuctionType, DeviceType, ApiFramework, Protocol};

// Auction types
let auction = AuctionType::FirstPrice;
assert_eq!(serde_json::to_string(&auction).unwrap(), "1");

// Device types
let device = DeviceType::Phone;
assert_eq!(serde_json::to_string(&device).unwrap(), "4");

// API frameworks
let api = ApiFramework::Mraid3;
assert_eq!(serde_json::to_string(&api).unwrap(), "6");

// Video protocols
let protocol = Protocol::Vast4;
assert_eq!(serde_json::to_string(&protocol).unwrap(), "7");
```

#### Extension Trait

The `Extension` trait provides a flexible mechanism for adding custom fields to IAB specification objects. This is particularly useful for vendor-specific data, internal tracking, or experimental features.

**Key Features:**
- Type-safe extension handling with generics
- Default to `Vec<u8>` (opaque bytes, serde-agnostic)
- Support for custom strongly-typed extensions
- Thread-safe (Send + Sync)
- Format-neutral serde support (JSON, MessagePack, CBOR, protobuf via prost, etc.)

**Types Supporting Extensions:**

The Extension trait is used throughout the crate on many types:
- **AdCOM types**: `Ad`, `Placement`, `DistributionChannel`, `Site`, `App`, `User`, `Device`, `Content`, `Publisher`, `Geo`, `Segment`, `Data`, `Regs`, and many more
- **OpenRTB 2.5/2.6 types**: `BidRequest`, `BidResponse`, `Imp`, `Banner`, `Video`, `Audio`, `Site`, `App`, `Device`, `User`, `Geo`, `Publisher`, `Content`, `Source`, `SeatBid`, `Bid`, and many more
- **OpenRTB 3.0 types**: `Request`, `Response`, `Item`, `Bid`, `SeatBid`, `Source`, `SupplyChain`, `SupplyChainNode`, `Deal`, `Metric`, and many more

**Using JSON extensions** (with explicit `serde_json::Value` type parameter)**:**

```rust
use iab_specs::adcom::media::AdBuilder;
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {

// Use serde_json::Value for flexible, untyped extensions
let ad = AdBuilder::<serde_json::Value>::default()
    .id(Some("ad123".to_string()))
    .ext(Some(Box::new(serde_json::json!({
        "vendor_field": "custom_value",
        "internal_id": 12345,
        "tracking_data": {
            "campaign": "summer_sale"
        }
    }))))
    .build()?;

// Serialize to JSON
let json = serde_json::to_string(&ad)?;
# Ok(())
# }
```

**Using custom typed extensions:**

```rust
use iab_specs::adcom::media::{Ad, AdBuilder};
use serde::{Deserialize, Serialize};
use derive_builder::Builder;
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {

// Define your custom extension type with Builder support
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
struct MyAdExtension {
    campaign_id: String,
    internal_tracking: i64,
    priority: u8,
}

impl MyAdExtension {
    pub fn builder() -> MyAdExtensionBuilder {
        MyAdExtensionBuilder::create_empty()
    }
}

// Create extension using builder pattern
let my_ext = MyAdExtension::builder()
    .campaign_id("camp_123".to_string())
    .internal_tracking(999)
    .priority(5)
    .build()?;

// Use your custom type for compile-time type safety (use AdBuilder with type parameter)
let ad = AdBuilder::<MyAdExtension>::default()
    .id(Some("ad456".to_string()))
    .ext(Some(Box::new(my_ext)))
    .build()?;

// Type-safe access to extension fields
if let Some(ext) = &ad.ext {
    println!("Campaign: {}", ext.campaign_id);
    println!("Priority: {}", ext.priority);
}
# Ok(())
# }
```

**No extensions needed:**

```rust
use iab_specs::adcom::media::Ad;
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {

// Simply omit the ext field when you don't need extensions
let ad = Ad::builder()
    .id(Some("ad789".to_string()))
    .build()?;
# Ok(())
# }
```

For complete documentation and examples, see the [`Extension` trait documentation](https://docs.rs/iab-specs/latest/iab_specs/trait.Extension.html).

### OpenRTB 2.5 and 2.6

OpenRTB 2.5 and 2.6 are fully implemented with complete bid request/response objects.

#### Supply Chain Transparency

```rust
use iab_specs::openrtb::common::{SupplyChain, SupplyChainNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a supply chain for ads.txt/sellers.json transparency
    let supply_chain = SupplyChain::builder()
        .complete(Some(1))
        .ver(Some("1.0".to_string()))
        .nodes(vec![
            SupplyChainNode::builder()
                .asi("example.com".to_string())
                .sid("12345".to_string())
                .hp(1) // Direct seller
                .build()?,
        ])
        .build()?;

    // Include in bid request source
    let source = iab_specs::openrtb::v25::Source {
        schain: Some(supply_chain),
        ..Default::default()
    };
    Ok(())
}
```

#### OpenRTB 2.6 Features

OpenRTB 2.6 adds support for CTV ad pods, DOOH multipliers, and more:

```rust
use iab_specs::openrtb::v25::Video;
use iab_specs::openrtb::v26::{Qty, DurFloors};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CTV ad pod with duration-based pricing
    let video = Video::builder()
        .mimes(vec!["video/mp4".to_string()])
        .minduration(15)
        .maxduration(Some(30))
        .protocols(Some(vec![7])) // VAST 4.0
        // Ad pod configuration
        .podid(Some("pod-123".to_string()))
        .podseq(0) // First ad in pod
        .slotinpod(1) // Guaranteed first position
        // Duration-based floor pricing
        .durfloors(Some(vec![
            DurFloors::builder()
                .minduration(Some(15))
                .maxduration(Some(30))
                .bidfloor(Some(5.00)) // $5 CPM for 15-30s ads
                .bidfloorcur(Some("USD".to_string()))
                .build()?,
        ]))
        .build()?;

    // DOOH impression with multiplier
    let qty = Qty::builder()
        .multiplier(Some(150.0)) // 150 people viewing
        .source(Some("venue_measurement".to_string()))
        .build()?;
    Ok(())
}
```

### OpenRTB 3.0

OpenRTB 3.0 introduces a layered architecture with explicit versioning and supply chain transparency as a first-class feature:

```rust
use iab_specs::openrtb::v30::{Openrtb, Request, Response, Item, Bid, Seatbid};
use iab_specs::openrtb::v30::{Source, SupplyChain, SupplyChainNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a bid request with supply chain transparency
    let request = Openrtb::builder()
        .ver("3.0".to_string())
        .domainspec("adcom".to_string())
        .domainver("1.0".to_string())
        .request(Some(Request::builder()
            .id("req-123".to_string())
            .tmax(Some(100))
            .at(Some(2)) // Second price auction
            .cur(Some(vec!["USD".to_string()]))
            .item(vec![Item::builder()
                .id("item1".to_string())
                .qty(Some(1))
                .flr(Some(1.50)) // Floor price
                .flrcur(Some("USD".to_string()))
                .build()?])
            .source(Some(Source::builder()
                .tid(Some("txn-456".to_string()))
                .schain(Some(SupplyChain::builder()
                    .complete(1)
                    .nodes(vec![
                        SupplyChainNode::builder()
                            .asi("publisher.com".to_string())
                            .sid("pub-123".to_string())
                            .hp(Some(1)) // Payment recipient
                            .build()?,
                        SupplyChainNode::builder()
                            .asi("exchange.com".to_string())
                            .sid("exch-456".to_string())
                            .hp(Some(1))
                            .build()?,
                    ])
                    .ver("1.0".to_string())
                    .build()?))
                .build()?))
            .build()?))
        .response(None)
        .build()?;

    // Create a bid response
    let response = Openrtb::builder()
        .ver("3.0".to_string())
        .domainspec("adcom".to_string())
        .domainver("1.0".to_string())
        .request(None)
        .response(Some(Response::builder()
            .id("req-123".to_string())
            .cur(Some("USD".to_string()))
            .seatbid(vec![Seatbid::builder()
                .seat(Some("seat-1".to_string()))
                .bid(vec![Bid::builder()
                    .id("bid-1".to_string())
                    .item("item1".to_string()) // References item ID
                    .price(2.50)
                    .nurl(Some("https://win.example.com/?price=${AUCTION_PRICE}".to_string()))
                    .build()?])
                .build()?])
            .build()?))
        .build()?;

    Ok(())
}
```

**Key OpenRTB 3.0 Features:**
- Explicit protocol and domain versioning
- Supply chain transparency promoted to core object
- Item-based inventory (replaces Imp)
- Comprehensive tracking URLs (nurl, burl, lurl)
- Package bidding support
- Measurement metrics

**OpenRTB 3.0 Documentation:**
- [Migration Guide](docs/MIGRATION_GUIDE_OPENRTB3.md) - Migrate from 2.x to 3.0
- [Usage Guide](docs/USAGE_GUIDE_OPENRTB3.md) - Complete examples and patterns
- [Best Practices](docs/BEST_PRACTICES_OPENRTB3.md) - Production guidelines

### OpenRTB Native Ads 1.2

Create and parse native ad requests and responses:

```rust
use iab_specs::openrtb::native::v12::{
    NativeRequest, NativeResponse, Asset, AssetResponse,
    Title, TitleResponse, Image, ImageResponse, Data, DataResponse, Link
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a native ad request
    let request = NativeRequest::builder()
        .ver(Some("1.2".to_string()))
        .context(Some(1))      // Content-centric context
        .plcmttype(Some(1))    // In-feed placement
        .assets(vec![
            // Title asset
            Asset::builder()
                .id(1)
                .required(Some(1))
                .title(Some(Title::builder()
                    .len(90)  // Max 90 characters
                    .build()?))
                .build()?,
            // Main image asset
            Asset::builder()
                .id(2)
                .img(Some(Image::builder()
                    .type_(Some(3))   // Main image
                    .w(Some(1200))
                    .h(Some(627))
                    .build()?))
                .build()?,
            // Description data asset
            Asset::builder()
                .id(3)
                .data(Some(Data::builder()
                    .type_(2)         // Description
                    .len(Some(140))
                    .build()?))
                .build()?,
        ])
        .build()?;

    // Parse a native ad response
    let response = NativeResponse::builder()
        .ver(Some("1.2".to_string()))
        .assets(vec![
            AssetResponse::builder()
                .id(1)
                .title(Some(TitleResponse::builder()
                    .text("Amazing Product - Limited Offer!".to_string())
                    .build()?))
                .build()?,
            AssetResponse::builder()
                .id(2)
                .img(Some(ImageResponse::builder()
                    .url("https://cdn.example.com/product.jpg".to_string())
                    .w(Some(1200))
                    .h(Some(627))
                    .build()?))
                .build()?,
            AssetResponse::builder()
                .id(3)
                .data(Some(DataResponse::builder()
                    .value("High-quality product with excellent reviews".to_string())
                    .build()?))
                .build()?,
        ])
        .link(Link::builder()
            .url("https://example.com/product?utm_source=native".to_string())
            .clicktrackers(Some(vec![
                "https://tracker.com/click".to_string(),
            ]))
            .build()?)
        .build()?;

    // Serialize to JSON for embedding in OpenRTB 2.5 request
    let native_json = serde_json::to_string(&request)?;

    Ok(())
}
```

**Integration with OpenRTB 2.5:**

```rust
use iab_specs::openrtb::v25::{BidRequest, Imp, Native};
use iab_specs::openrtb::native::v12::NativeRequest;

// Create native request
let native_req = NativeRequest::builder()
    .ver(Some("1.2".to_string()))
    .assets(/* ... */)
    .build()?;

// Serialize to JSON string
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

**Key Features:**
- Asset-based composition (title, image, video, data)
- Event tracking with impression and click support
- Multi-placement support for feed-based layouts
- DCO (Dynamic Creative Optimization) URL support
- AdCOM integration for standardized enumerations

### Agentic RTB Framework 1.0

Process OpenRTB bidstream with autonomous agents using the ARTB Patch Protocol.

> **Note:** The example below uses `serde_json::Value` for JSON payloads via explicit type parameters.
> The default payload type is `Vec<u8>` (opaque bytes). Use explicit type parameters like
> `RTBRequestBuilder::<serde_json::Value, Vec<u8>>::default()` for JSON payloads.

```rust
use iab_specs::artb::v10::{
    RTBRequestBuilder, RTBResponseBuilder, Mutation, Metadata, Originator,
    Lifecycle, Intent, Operation, IDsPayload, OriginatorType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Orchestrator creates a request for the agent (with JSON payloads)
    let request = RTBRequestBuilder::<serde_json::Value, Vec<u8>>::default()
        .lifecycle(Lifecycle::PublisherBidRequest)
        .id("req-12345".to_string())
        .tmax(Some(100))
        .bid_request(Some(serde_json::json!({
            "id": "auction-1",
            "imp": [{"id": "imp-1"}]
        })))
        .originator(Some(Originator::builder()
            .type_(OriginatorType::Ssp)
            .name("Example SSP")
            .build()?))
        .applicable_intents(vec![
            Intent::ActivateSegments,
            Intent::ActivateDeals,
        ])
        .build()?;

    // Agent processes and returns mutations
    let response = RTBResponseBuilder::<serde_json::Value, Vec<u8>>::default()
        .id("req-12345".to_string())
        .mutations(vec![
            Mutation::builder()
                .intent(Intent::ActivateSegments)
                .op(Operation::Add)
                .path("/user/data/segment".to_string())
                .ids(Some(IDsPayload::builder()
                    .id(vec!["seg-001".to_string(), "seg-002".to_string()])
                    .build()?))
                .build()?,
        ])
        .metadata(Some(Metadata::builder()
            .api_version("1.0")
            .model_version("v0.10.0")
            .build()?))
        .build()?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);

    Ok(())
}
```

**Key ARTB 1.0 Features:**
- Atomic, intent-declared mutations to OpenRTB bid requests and responses
- Segment activation, deal management, bid shading, metrics, and content data
- Agent metadata with API and model versioning
- Extension support for custom agent-specific data

### Agentic Direct v2.1

Autonomous agent-to-agent advertising transactions combining OpenDirect v2.1 with the A2A Protocol.

```rust
use iab_specs::agentic_direct::v21::{
    Order, OrderStatus, can_transition_order,
    AgentCard, Skill, SkillInputMode, AgentCapabilities,
    JsonRpcRequest, JsonRpcResponse, JsonRpcId,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an advertising order
    let order = Order::builder()
        .name("Summer Campaign")
        .account_id("acc-001")
        .publisher_id("pub-001")
        .currency("USD")
        .status(OrderStatus::Draft)
        .budget(Some(50000.0))
        .build()?;

    // Validate state transitions
    assert!(can_transition_order(&OrderStatus::Draft, &OrderStatus::PendingReview));

    // Create an A2A Agent Card for advertising negotiation
    let agent = AgentCard::builder()
        .name("Ad Negotiation Agent")
        .version("1.0.0")
        .protocol_version("0.3.0")
        .url("https://agent.example.com")
        .skills(vec![
            Skill::builder()
                .id("negotiate-order")
                .name("Order Negotiation")
                .input_modes(vec![SkillInputMode::Text, SkillInputMode::Data])
                .build()?,
        ])
        .capabilities(Some(AgentCapabilities::builder()
            .streaming(Some(true))
            .build()?))
        .build()?;

    // Wrap in JSON-RPC for agent communication
    let request = JsonRpcRequest::builder()
        .jsonrpc("2.0")
        .method("agent/negotiate")
        .id(Some(JsonRpcId::String("req-1".into())))
        .params(Some(serde_json::to_value(&order)?))
        .build()?;

    let response = JsonRpcResponse::builder()
        .jsonrpc("2.0")
        .id(JsonRpcId::String("req-1".into()))
        .result(Some(serde_json::json!({"status": "accepted"})))
        .build()?;

    // Serialize — A2A types use camelCase, OpenDirect entities use snake_case
    let agent_json = serde_json::to_string_pretty(&agent)?;
    println!("{}", agent_json);

    Ok(())
}
```

**Key Agentic Direct v2.1 Features:**
- OpenDirect v2.1 entities: Organization, Account, Product, Order, Line, Creative, Assignment, ChangeRequest, Placement
- A2A Protocol: Agent Cards, Skills, Tasks with A2AMessage history and artifact support
- JSON-RPC 2.0 message framing, MCPTool definitions for agent-to-agent communication
- State machines for Order, Line, and Task lifecycle management
- Dual serialization: snake_case for OpenDirect, camelCase for A2A types

### Buyer Agent 1.0

Plan campaigns, negotiate deals, and manage booking workflows with autonomous buyer agents.

```rust
use iab_specs::buyer_agent::v10::models::{
    CampaignBrief, CampaignAllocation, NegotiationStrategy, NegotiationOffer,
    BookingJob, BookingRecommendation, UCPEmbedding, AudiencePlan,
};
use iab_specs::buyer_agent::v10::enums::{CampaignStatus, DealStatus};
use iab_specs::buyer_agent::v10::state_machines::{
    can_transition_campaign, can_transition_deal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a campaign brief
    let brief = CampaignBrief::builder()
        .name("Q2 Brand Awareness Campaign")
        .budget(50000.0)
        .start_date("2026-04-01")
        .end_date("2026-06-30")
        .channels(vec!["display".to_string(), "video".to_string()])
        .build()?;

    // Allocate budget across channels
    let alloc = CampaignAllocation::builder()
        .channel("display")
        .budget_share(0.6)
        .priority(1)
        .rationale("High-volume reach channel")
        .build()?;

    // Negotiate a deal
    let strategy = NegotiationStrategy::builder()
        .target_cpm(2.50)
        .max_cpm(5.00)
        .concession_step(0.25)
        .max_rounds(5)
        .build()?;

    let offer = NegotiationOffer::builder()
        .price(3.50)
        .round(3)
        .from_buyer(true)
        .accepted(Some(true))
        .build()?;

    // Validate state machine transitions
    assert!(can_transition_campaign(
        &CampaignStatus::Initialized,
        &CampaignStatus::BriefReceived
    ));
    assert!(can_transition_deal(
        &DealStatus::Quoted,
        &DealStatus::Negotiating
    ));

    // Approval rejection loops back to research
    assert!(can_transition_campaign(
        &CampaignStatus::AwaitingApproval,
        &CampaignStatus::Researching
    ));

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&brief)?;
    println!("{}", json);

    Ok(())
}
```

**Key Buyer Agent 1.0 Features:**
- Campaign planning with CampaignBrief, CampaignAllocation, and BookingJob entities
- Deal negotiation with NegotiationStrategy and NegotiationOffer types
- UCP embedding support with 384-dimensional vectors and AudiencePlan targeting
- Validated state machines for Campaign (9 states) and Deal (13 states) lifecycles
- Approval rejection loop: AwaitingApproval → Researching for iterative refinement
- Re-exports all Agentic Direct 2.1 types for seamless integration

### Seller Agent 1.0

Manage inventory, generate proposals, negotiate pricing, and execute orders with autonomous seller agents.

```rust
use iab_specs::seller_agent::v10::models::{
    Proposal, ProposalRevision, ProposalItem, TieredPricing, PricingTier,
    NegotiationConfig, NegotiationRound,
};
use iab_specs::seller_agent::v10::enums::{
    ProposalStatus, PricingTierType, SellerOrderStatus, NegotiationStrategyType,
};
use iab_specs::seller_agent::v10::state_machines::can_transition_seller_order;
use iab_specs::agentic_direct::v21::enums::RateType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a proposal
    let proposal = Proposal::builder()
        .id("prop-001")
        .buyer_id("buyer-001")
        .seller_id("seller-001")
        .status(ProposalStatus::Submitted)
        .build()?;

    // Define tiered pricing
    let pricing = TieredPricing::builder()
        .tiers(vec![
            PricingTier::builder()
                .tier_type(PricingTierType::Public)
                .discount_percent(0.0)
                .negotiation_enabled(false)
                .build()?,
            PricingTier::builder()
                .tier_type(PricingTierType::Agency)
                .discount_percent(10.0)
                .negotiation_enabled(true)
                .min_spend(Some(5000.0))
                .build()?,
        ])
        .build()?;

    // Configure negotiation
    let config = NegotiationConfig::builder()
        .max_rounds(5)
        .per_round_concession_cap(0.50)
        .total_concession_cap(2.00)
        .strategy(NegotiationStrategyType::Collaborative)
        .build()?;

    // Validate state machine transitions
    assert!(can_transition_seller_order(
        &SellerOrderStatus::Draft,
        &SellerOrderStatus::Submitted
    ));
    assert!(can_transition_seller_order(
        &SellerOrderStatus::Syncing,
        &SellerOrderStatus::Booked
    ));

    // Pause/resume cycle
    assert!(can_transition_seller_order(
        &SellerOrderStatus::InProgress,
        &SellerOrderStatus::Paused
    ));
    assert!(can_transition_seller_order(
        &SellerOrderStatus::Paused,
        &SellerOrderStatus::InProgress
    ));

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&proposal)?;
    println!("{}", json);

    Ok(())
}
```

**Key Seller Agent 1.0 Features:**
- Proposal management with Proposal, ProposalRevision, and ProposalItem entities
- Tiered pricing with TieredPricing, PricingTier, and RateCard types
- Negotiation configuration with NegotiationConfig and NegotiationRound types
- Inventory packaging with MediaKit and Package types
- Change management with ChangeRequest, ChangeType, and ChangeSeverity
- Order execution with ExecutionOrder, DealDistribution, and DspIntegration
- Validated state machine for SellerOrder (13 states) lifecycle with pause/resume support
- Re-exports all Agentic Direct 2.1 types for seamless integration

### Agentic Audience v1.0

Exchange embeddings for audience targeting using the Agentic Audience protocol.

> ⚠️ **Draft Specification**: Based on Agentic Audience v1.0 Draft. Breaking changes may occur.

```rust
use iab_specs::agentic_audience::v10::{
    EmbeddingEnvelope, EmbeddingModel, EmbeddingContext, Embedding,
    CampaignHead, ScoringRequest, ScoringResponse, CampaignScore,
    EmbeddingSegmentExt,
    ModelType, DistanceMetric, EmbeddingType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an embedding envelope with model and embeddings
    let envelope = EmbeddingEnvelope::builder()
        .model(EmbeddingModel::builder()
            .id("minilm-l6-v2")
            .version("1.0")
            .type_(ModelType::Encoder)
            .dimension(384)
            .metric(DistanceMetric::Cosine)
            .embedding_space_id("sentence-transformers/all-MiniLM-L6-v2")
            .build()?)
        .context(Some(EmbeddingContext::builder()
            .url("https://example.com/article")
            .page_title("AI in Advertising")
            .language("en")
            .build()?))
        .embeddings(vec![
            Embedding::builder()
                .id("emb-001")
                .type_(EmbeddingType::ContextContent)
                .dimension(384)
                .vector(Some(vec![0.1; 384]))
                .build()?,
        ])
        .build()?;

    // Score embeddings against campaigns
    let request = ScoringRequest::builder()
        .embeddings(envelope.embeddings.clone())
        .campaign_ids(Some(vec!["camp-001".to_string()]))
        .build()?;

    let response = ScoringResponse::builder()
        .scores(vec![
            CampaignScore::builder()
                .campaign_id("camp-001")
                .score(0.87)
                .percentile(Some(0.92))
                .build()?,
        ])
        .build()?;

    // OpenRTB bid stream extension for embedding transport
    let segment_ext = EmbeddingSegmentExt::builder()
        .ver("1.0")
        .vector(vec![0.1, 0.2, 0.3, 0.4])
        .model("minilm-l6-v2")
        .dimension(4)
        .type_(EmbeddingType::ContextContent)
        .metric(Some(DistanceMetric::Cosine))
        .build()?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&envelope)?;
    println!("{}", json);

    Ok(())
}
```

**Key Agentic Audience v1.0 Features:**
- Embedding envelope for transport (model descriptor, context, embeddings)
- 24 embedding type classifications across 7 signal categories
- Campaign scoring with head weights, requests, and responses
- OpenRTB bid stream extension (`EmbeddingSegmentExt`) for `user.data.segment.ext`
- Signal taxonomy enums (SignalType, EmbeddingType, ModelType, DistanceMetric, etc.)

### Ads.txt

Parse and generate ads.txt files:

```rust
use iab_specs::ads_txt::{AdsTxt, AdsTxtSystem, SellerRelationType};
use std::str::FromStr;

// Parse an ads.txt file
let ads_txt_content = "google.com, pub-1234567890123456, DIRECT, f08c47fec0942fa0";
let ads_txt = AdsTxt::from_str(ads_txt_content)?;

// Create an ads.txt programmatically
let ads_txt = AdsTxt::builder()
    .contact(Some("adops@example.com".to_string()))
    .owner_domain(Some("example.com".to_string()))
    .systems(vec![
        AdsTxtSystem::builder()
            .domain("google.com".to_string())
            .publisher_account_id("pub-1234567890123456".to_string())
            .account_type(SellerRelationType::Direct)
            .certification_authority_id(Some("f08c47fec0942fa0".to_string()))
            .build()?,
    ])
    .build()?;

// Serialize to string
let output = ads_txt.to_string();
```

### App-ads.txt

Parse and generate app-ads.txt files for mobile and CTV applications:

```rust
use iab_specs::app_ads_txt::{AppAdsTxt, AdsTxtSystem, SellerRelationType};
use std::str::FromStr;

// Parse an app-ads.txt file
let app_ads_content = r#"
contact=monetization@mygame.com
subdomain=games.mygame.com

# Primary ad network
google.com, pub-1234567890123456, DIRECT, f08c47fec0942fa0
# Reseller partners
silverssp.com, 9876, RESELLER, f6578439
"#;
let app_ads = AppAdsTxt::from_str(app_ads_content)?;

// Create an app-ads.txt programmatically
let app_ads = AppAdsTxt::builder()
    .contact(Some("monetization@mygame.com".to_string()))
    .subdomain(Some("games.mygame.com".to_string()))
    .systems(vec![
        AdsTxtSystem::builder()
            .domain("google.com".to_string())
            .publisher_account_id("pub-1234567890123456".to_string())
            .account_type(SellerRelationType::Direct)
            .certification_authority_id(Some("f08c47fec0942fa0".to_string()))
            .build()?,
    ])
    .build()?;

// Serialize to string
let output = app_ads.to_string();
```

**Note on ads.txt 1.1 vs app-ads.txt 1.0:**

App-ads.txt v1.0 is based on an earlier ads.txt specification and does **not** support the ads.txt 1.1 features:
- `OWNERDOMAIN` (not in app-ads.txt v1.0)
- `MANAGERDOMAIN` (not in app-ads.txt v1.0)

Attempting to parse an app-ads.txt file containing these directives will result in an error.

### Sellers.json

Parse and generate sellers.json files:

```rust
use iab_specs::sellers_json::{Sellers, Seller, SellerType, SellersVersion};
use std::str::FromStr;

// Parse a sellers.json file
let sellers_json = r#"{
    "contact_email": "adops@example.com",
    "version": "1.0",
    "sellers": [
        {
            "seller_id": "12345",
            "seller_type": "publisher",
            "name": "Example Publisher",
            "domain": "example.com"
        }
    ]
}"#;
let sellers = Sellers::from_str(sellers_json)?;

// Create sellers.json programmatically
let sellers = Sellers::builder()
    .contact_email(Some("adops@example.com".to_string()))
    .version(SellersVersion::OneZero)
    .sellers(vec![
        Seller::builder()
            .seller_id("12345".to_string())
            .seller_type(SellerType::Publisher)
            .name(Some("Example Publisher".to_string()))
            .domain(Some("example.com".to_string()))
            .build()?,
    ])
    .build()?;

// Serialize to JSON string
let output = serde_json::to_string_pretty(&sellers)?;
```

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/iab-specs).

For usage examples, please refer to the unit tests in the source code. Each module includes comprehensive test cases demonstrating both serialization and deserialization.

## Development

The project includes shell scripts for common development tasks:

### Format Code

Check or fix code formatting with rustfmt:

```bash
./format.sh --check    # Check formatting (used in CI)
./format.sh --fix      # Fix formatting issues
```

### Run Linter

Check code quality with clippy:

```bash
./check.sh --all-features                           # Check all features
./check.sh --no-default-features --features openrtb_30  # Check specific feature
```

### Run Tests

Run tests with configurable features:

```bash
./test.sh                                           # Test with default features
./test.sh --all-features                            # Test all features
./test.sh --no-default-features --features openrtb_30   # Test specific feature
./test.sh --features openrtb_25,ads_txt             # Test multiple features
```

### Generate Coverage

Generate code coverage reports:

```bash
./coverage.sh --html --all-features                 # HTML report (opens in browser)
./coverage.sh --text --all-features                 # Text summary
./coverage.sh --lcov --all-features --check-thresholds  # CI-style with 80% threshold
./coverage.sh --no-default-features --features openrtb_30  # Coverage for specific feature
```

All scripts support `--help` for more options.

## Roadmap

- [x] AdCOM 1.0
- [x] Ads.txt 1.1
- [x] App-ads.txt 1.0
- [x] Sellers.json 1.0
- [x] OpenRTB 2.5
- [x] OpenRTB 2.6
- [x] OpenRTB 3.0
- [x] OpenRTB Native Ads 1.2
- [x] Agentic RTB Framework 1.0
- [x] Agentic Direct 2.1
- [x] Buyer Agent 1.0
- [x] Seller Agent 1.0
- [x] Agentic Audience v1.0 (Draft)
- [ ] Additional IAB specifications (contributions welcome!)

## Contributing

Contributions are welcome! Whether it's:

- Adding new IAB specifications
- Improving existing implementations
- Fixing bugs
- Improving documentation
- Adding examples

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on:
- Development setup
- Running tests locally with `act`
- Code coverage requirements
- Contribution workflow

**Signed commits are required.** See the [Signing Your Commits](CONTRIBUTING.md#signing-your-commits) section in CONTRIBUTING.md for setup instructions using GPG, SSH, or other supported methods.

### Verifying Release Artifacts

Published crate artifacts include [GitHub Artifact Attestations](https://docs.github.com/en/actions/security-for-github-actions/using-artifact-attestations) for supply chain verification:

```bash
gh attestation verify ./iab-specs-*.crate --repo remysaissy/iab-specs
```

## License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
