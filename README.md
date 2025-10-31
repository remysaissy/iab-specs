# iab-specs

[![Crates.io](https://img.shields.io/crates/v/iab-specs.svg)](https://crates.io/crates/iab-specs)
[![Documentation](https://docs.rs/iab-specs/badge.svg)](https://docs.rs/iab-specs)
[![License](https://img.shields.io/crates/l/iab-specs.svg)](LICENSE)

An unofficial Rust implementation of various IAB (Interactive Advertising Bureau) specifications.

## Overview

`iab-specs` provides typed Rust data structures for working with IAB advertising specifications. Rather than being just a parser, this library wraps each specification's logic into idiomatic Rust types using `serde` for serialization/deserialization and `FromStr`/`Display` for string conversions.

### Currently Supported Specifications

- **[Ads.txt 1.1](https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf)** - Authorized Digital Sellers declaration for websites
- **[App-ads.txt 1.0](https://iabtechlab.com/wp-content/uploads/2019/03/app-ads.txt-v1.0-final-.pdf)** - Authorized Digital Sellers declaration for mobile and CTV apps
- **[Sellers.json 1.0](https://iabtechlab.com/wp-content/uploads/2019/07/Sellers.json_Final.pdf)** - Supply chain transparency

## Installation

Add `iab-specs` to your `Cargo.toml` with the features you need:

```toml
[dependencies]
# Enable all specifications
iab-specs = { version = "0.1", features = ["ads_txt", "app_ads_txt", "sellers_json"] }

# Or enable only what you need
iab-specs = { version = "0.1", features = ["ads_txt"] }
```

Or use cargo:

```bash
# Enable all specifications
cargo add iab-specs --features ads_txt,app_ads_txt,sellers_json

# Or enable only what you need
cargo add iab-specs --features ads_txt
```

## Features

⚠️ **Important**: By default, **no features are enabled**. You must explicitly enable the specifications you need.

The library uses cargo features to enable/disable specifications:

- `ads_txt` - Ads.txt 1.1 support
- `app_ads_txt` - App-ads.txt 1.0 support (automatically includes `ads_txt`)
- `sellers_json` - Sellers.json 1.0 support

### Feature Selection Examples

```toml
[dependencies]
# Only ads.txt support
iab-specs = { version = "0.1", features = ["ads_txt"] }

# Only app-ads.txt support (automatically includes ads_txt)
iab-specs = { version = "0.1", features = ["app_ads_txt"] }

# Only sellers.json support
iab-specs = { version = "0.1", features = ["sellers_json"] }

# Ads.txt and sellers.json (no app-ads.txt)
iab-specs = { version = "0.1", features = ["ads_txt", "sellers_json"] }

# All specifications
iab-specs = { version = "0.1", features = ["ads_txt", "app_ads_txt", "sellers_json"] }
```

**Why no default features?**

This design allows you to:
- **Minimize dependencies**: Only include what you actually use
- **Reduce compile time**: Don't compile unused specifications
- **Smaller binary size**: Eliminate unused code from your final binary
- **Explicit dependencies**: Be clear about which IAB specs your project relies on

## Usage Examples

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

## Roadmap

- [x] Ads.txt 1.1
- [x] App-ads.txt 1.0
- [x] Sellers.json 1.0
- [ ] OpenRTB 2.5
- [ ] OpenRTB 2.6
- [ ] OpenRTB 3.0
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
