# iab-specs

[![Crates.io](https://img.shields.io/crates/v/iab-specs.svg)](https://crates.io/crates/iab-specs)
[![Documentation](https://docs.rs/iab-specs/badge.svg)](https://docs.rs/iab-specs)
[![License](https://img.shields.io/crates/l/iab-specs.svg)](LICENSE)

An unofficial Rust implementation of various IAB (Interactive Advertising Bureau) specifications.

## Overview

`iab-specs` provides typed Rust data structures for working with IAB advertising specifications. Rather than being just a parser, this library wraps each specification's logic into idiomatic Rust types using `serde` for serialization/deserialization and `FromStr`/`Display` for string conversions.

### Currently Supported Specifications

- **[AdCOM 1.0](https://github.com/InteractiveAdvertisingBureau/AdCOM)** - Advertising Common Object Model (enumerations)
- **[OpenRTB 2.5](https://www.iab.com/wp-content/uploads/2016/03/OpenRTB-API-Specification-Version-2-5-FINAL.pdf)** - Real-Time Bidding protocol
- **[OpenRTB 2.6](https://github.com/InteractiveAdvertisingBureau/openrtb2.x/blob/main/2.6.md)** - Real-Time Bidding protocol with CTV/DOOH support
- **[OpenRTB 3.0](https://github.com/InteractiveAdvertisingBureau/openrtb/blob/main/OpenRTB%20v3.0%20FINAL.md)** - Real-Time Bidding protocol with layered architecture
- **[OpenRTB Native Ads 1.2](https://github.com/InteractiveAdvertisingBureau/Native-Ads/blob/main/OpenRTB-Native-Ads-Specification-Final-1.2.md)** - Native advertising specification
- **[Ads.txt 1.1](https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf)** - Authorized Digital Sellers declaration for websites
- **[App-ads.txt 1.0](https://iabtechlab.com/wp-content/uploads/2019/03/app-ads.txt-v1.0-final-.pdf)** - Authorized Digital Sellers declaration for mobile and CTV apps
- **[Sellers.json 1.0](https://iabtechlab.com/wp-content/uploads/2019/07/Sellers.json_Final.pdf)** - Supply chain transparency

## Installation

Add `iab-specs` to your `Cargo.toml` with the features you need:

```toml
[dependencies]
# Enable all specifications
iab-specs = { version = "0.2", features = ["adcom", "openrtb_25", "openrtb_26", "openrtb_30", "openrtb_native_12", "ads_txt", "app_ads_txt", "sellers_json"] }

# Or enable only what you need
iab-specs = { version = "0.2", features = ["openrtb_30"] }
```

Or use cargo:

```bash
# Enable all specifications
cargo add iab-specs --features adcom,openrtb_25,openrtb_26,openrtb_30,openrtb_native_12,ads_txt,app_ads_txt,sellers_json

# Or enable only what you need
cargo add iab-specs --features openrtb_30
```

## Features

⚠️ **Important**: By default, **no features are enabled**. You must explicitly enable the specifications you need.

The library uses cargo features to enable/disable specifications:

- `adcom` - AdCOM 1.0 support (Advertising Common Object Model enumerations)
- `openrtb_25` - OpenRTB 2.5 support (automatically includes `adcom`)
- `openrtb_26` - OpenRTB 2.6 support (automatically includes `openrtb_25` and `adcom`)
- `openrtb_30` - OpenRTB 3.0 support (automatically includes `adcom`)
- `openrtb_native_12` - OpenRTB Native Ads 1.2 support (automatically includes `adcom`)
- `ads_txt` - Ads.txt 1.1 support
- `app_ads_txt` - App-ads.txt 1.0 support (automatically includes `ads_txt`)
- `sellers_json` - Sellers.json 1.0 support

### Feature Selection Examples

```toml
[dependencies]
# Only AdCOM support
iab-specs = { version = "0.2", features = ["adcom"] }

# Only OpenRTB 2.5 support (automatically includes adcom)
iab-specs = { version = "0.2", features = ["openrtb_25"] }

# Only OpenRTB 2.6 support (automatically includes openrtb_25 and adcom)
iab-specs = { version = "0.2", features = ["openrtb_26"] }

# Only OpenRTB 3.0 support (automatically includes adcom)
iab-specs = { version = "0.2", features = ["openrtb_30"] }

# Only ads.txt support
iab-specs = { version = "0.2", features = ["ads_txt"] }

# Only app-ads.txt support (automatically includes ads_txt)
iab-specs = { version = "0.2", features = ["app_ads_txt"] }

# Only sellers.json support
iab-specs = { version = "0.2", features = ["sellers_json"] }

# OpenRTB 3.0 with ads.txt and sellers.json
iab-specs = { version = "0.2", features = ["openrtb_30", "ads_txt", "sellers_json"] }

# All specifications
iab-specs = { version = "0.2", features = ["adcom", "openrtb_25", "openrtb_26", "openrtb_30", "ads_txt", "app_ads_txt", "sellers_json"] }
```

**Why no default features?**

This design allows you to:
- **Minimize dependencies**: Only include what you actually use
- **Reduce compile time**: Don't compile unused specifications
- **Smaller binary size**: Eliminate unused code from your final binary
- **Explicit dependencies**: Be clear about which IAB specs your project relies on

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
- Default to `serde_json::Value` for maximum flexibility
- Support for custom strongly-typed extensions
- Thread-safe (Send + Sync)
- Serialization/deserialization support

**Types Supporting Extensions:**

The Extension trait is used throughout the crate on many types:
- **AdCOM types**: `Ad`, `Placement`, `DistributionChannel`, `Site`, `App`, `User`, `Device`, `Content`, `Publisher`, `Geo`, `Segment`, `Data`, `Regs`, and many more
- **OpenRTB 2.5/2.6 types**: `BidRequest`, `BidResponse`, `Imp`, `Banner`, `Video`, `Audio`, `Site`, `App`, `Device`, `User`, `Geo`, `Publisher`, `Content`, `Source`, `SeatBid`, `Bid`, and many more
- **OpenRTB 3.0 types**: `Request`, `Response`, `Item`, `Bid`, `SeatBid`, `Source`, `SupplyChain`, `SupplyChainNode`, `Deal`, `Metric`, and many more

**Using default JSON extensions:**

```rust
use iab_specs::adcom::media::Ad;
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {

// Use serde_json::Value for flexible, untyped extensions
let ad = Ad::builder()
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
use iab_specs::openrtb::v26::{Video, Qty, DurFloors};

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

## License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
