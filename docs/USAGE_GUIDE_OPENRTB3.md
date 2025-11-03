# OpenRTB 3.0 Usage Guide

Complete guide to using OpenRTB 3.0 in the `iab-specs` library.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Core Concepts](#core-concepts)
- [Creating Bid Requests](#creating-bid-requests)
- [Processing Bid Responses](#processing-bid-responses)
- [Supply Chain Transparency](#supply-chain-transparency)
- [Private Marketplace (PMP) Deals](#private-marketplace-pmp-deals)
- [Package Bidding](#package-bidding)
- [Advanced Topics](#advanced-topics)
- [Performance Tips](#performance-tips)

---

## Getting Started

### Installation

Add `iab-specs` to your `Cargo.toml`:

```toml
[dependencies]
iab-specs = { version = "0.2", features = ["openrtb_3"] }
```

### Quick Example

```rust
use iab_specs::openrtb::v3::{Openrtb, Request, Item};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple bid request
    let openrtb = Openrtb {
        ver: "3.0".to_string(),
        domainspec: "adcom".to_string(),
        domainver: "1.0".to_string(),
        request: Some(Request {
            id: "req-123".to_string(),
            tmax: Some(100),
            cur: Some(vec!["USD".to_string()]),
            item: vec![Item {
                id: "item1".to_string(),
                qty: Some(1),
                flr: Some(1.50),
                flrcur: Some("USD".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        response: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&openrtb)?;
    println!("{}", json);

    Ok(())
}
```

---

## Core Concepts

### The Openrtb Container

OpenRTB 3.0 uses a wrapper container that holds either a request or response:

```rust
pub struct Openrtb {
    pub ver: String,          // Protocol version: "3.0"
    pub domainspec: String,   // Domain spec: "adcom"
    pub domainver: String,    // Domain version: "1.0"
    pub request: Option<Request>,
    pub response: Option<Response>,
}
```

**Key Points**:
- Always set `ver`, `domainspec`, and `domainver`
- Set either `request` OR `response`, never both
- This structure enables explicit version tracking

### Items vs Impressions

OpenRTB 3.0 replaces "impressions" (Imp) with "items" (Item):

```rust
pub struct Item {
    pub id: String,              // Required: unique ID
    pub qty: Option<i32>,        // Quantity (usually 1)
    pub seq: Option<i32>,        // Sequence for prioritization
    pub flr: Option<f64>,        // Floor price
    pub flrcur: Option<String>,  // Floor currency
    pub deal: Option<Vec<Deal>>, // Private deals
    pub metric: Option<Vec<Metric>>, // Measurements
    pub spec: Option<Value>,     // AdCOM placement spec
    // ... more fields
}
```

**Why "Item"?**
- More flexible: can represent multiple impressions
- Supports quantity-based bidding
- Cleaner terminology

---

## Creating Bid Requests

### Display Banner Request

```rust
use iab_specs::openrtb::v3::{Openrtb, Request, Item};
use iab_specs::openrtb::v3::spec::{DisplayPlacement, DisplayFormat};

let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: Some(Request {
        id: "req-banner-001".to_string(),
        test: Some(0),
        tmax: Some(100),
        at: Some(2), // Second price auction
        cur: Some(vec!["USD".to_string()]),
        item: vec![Item {
            id: "item1".to_string(),
            qty: Some(1),
            flr: Some(1.50),
            flrcur: Some("USD".to_string()),
            spec: Some(serde_json::json!({
                "placement": DisplayPlacement {
                    pos: Some(1), // Above the fold
                    instl: Some(0), // Not interstitial
                    topframe: Some(1), // Top frame
                    w: Some(300),
                    h: Some(250),
                    displayfmt: Some(vec![
                        DisplayFormat {
                            w: Some(300),
                            h: Some(250),
                            ..Default::default()
                        }
                    ]),
                    ..Default::default()
                }
            })),
            ..Default::default()
        }],
        ..Default::default()
    }),
    response: None,
};
```

### Video Request

```rust
use iab_specs::openrtb::v3::spec::VideoPlacement;

let item = Item {
    id: "item1".to_string(),
    qty: Some(1),
    flr: Some(5.00),
    flrcur: Some("USD".to_string()),
    spec: Some(serde_json::json!({
        "placement": VideoPlacement {
            ptype: Some(1), // In-stream
            pos: Some(1), // Above the fold
            w: Some(640),
            h: Some(480),
            mindur: Some(15), // Min 15 seconds
            maxdur: Some(30), // Max 30 seconds
            skip: Some(1), // Skippable
            skipmin: Some(15), // Skip after 15 seconds
            skipafter: Some(5), // Skip button after 5 seconds
            playmethod: Some(vec![1]), // Auto-play sound on
            playend: Some(1), // Play to completion
            mime: Some(vec![
                "video/mp4".to_string(),
                "video/webm".to_string()
            ]),
            protocol: Some(vec![2, 3, 5, 6]), // VAST 2.0, 3.0, DAAST 1.0
            ..Default::default()
        }
    })),
    ..Default::default()
};
```

### Audio Request

```rust
use iab_specs::openrtb::v3::spec::AudioPlacement;

let item = Item {
    id: "item1".to_string(),
    spec: Some(serde_json::json!({
        "placement": AudioPlacement {
            mindur: Some(15),
            maxdur: Some(60),
            protocol: Some(vec![2, 3, 5, 6]),
            feed: Some(1), // Music service
            nvol: Some(4), // Volume normalization
            mime: Some(vec![
                "audio/mp4".to_string(),
                "audio/mpeg".to_string()
            ]),
            stitched: Some(0), // Not stitched
            ..Default::default()
        }
    })),
    ..Default::default()
};
```

### Multi-Item Request

Request multiple ad placements in one request:

```rust
let request = Request {
    id: "req-multi-001".to_string(),
    tmax: Some(120),
    at: Some(2),
    cur: Some(vec!["USD".to_string()]),
    item: vec![
        // Header banner
        Item {
            id: "item1".to_string(),
            qty: Some(1),
            seq: Some(1), // Priority sequence
            flr: Some(2.00),
            flrcur: Some("USD".to_string()),
            spec: Some(/* 728x90 banner */),
            ..Default::default()
        },
        // Sidebar banner
        Item {
            id: "item2".to_string(),
            qty: Some(1),
            seq: Some(2),
            flr: Some(1.50),
            flrcur: Some("USD".to_string()),
            spec: Some(/* 160x600 banner */),
            ..Default::default()
        },
        // Content banner
        Item {
            id: "item3".to_string(),
            qty: Some(1),
            seq: Some(3),
            flr: Some(1.00),
            flrcur: Some("USD".to_string()),
            spec: Some(/* 300x250 banner */),
            ..Default::default()
        },
    ],
    ..Default::default()
};
```

---

## Processing Bid Responses

### Creating a Basic Response

```rust
use iab_specs::openrtb::v3::{Openrtb, Response, Seatbid, Bid};

let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: None,
    response: Some(Response {
        id: "req-123".to_string(), // Match request ID
        bidid: Some("bid-resp-456".to_string()),
        cur: Some("USD".to_string()),
        seatbid: vec![
            Seatbid {
                seat: Some("seat-dsp-1".to_string()),
                bid: vec![
                    Bid {
                        id: "bid-1".to_string(),
                        item: "item1".to_string(), // Match item ID
                        price: 2.50,
                        cid: Some("campaign-789".to_string()),
                        adomain: Some(vec!["advertiser.com".to_string()]),
                        cat: Some(vec!["IAB1".to_string()]),
                        nurl: Some("https://win.dsp.com/?price=${AUCTION_PRICE}".to_string()),
                        burl: Some("https://bill.dsp.com/".to_string()),
                        lurl: Some("https://loss.dsp.com/?reason=${AUCTION_LOSS}".to_string()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ],
        ..Default::default()
    }),
};
```

### Tracking URLs

OpenRTB 3.0 supports three tracking URLs with macro substitution:

```rust
Bid {
    id: "bid-1".to_string(),
    item: "item1".to_string(),
    price: 5.50,

    // Win notice - called when bid wins
    nurl: Some("https://win.example.com/?price=${AUCTION_PRICE}&bidid=${AUCTION_BID_ID}".to_string()),

    // Billing notice - called when ad is rendered
    burl: Some("https://bill.example.com/?bidid=${AUCTION_BID_ID}".to_string()),

    // Loss notice - called when bid loses
    lurl: Some("https://loss.example.com/?reason=${AUCTION_LOSS}&price=${AUCTION_PRICE}".to_string()),

    ..Default::default()
}
```

**Available Macros**:
- `${AUCTION_PRICE}` - Clearing price
- `${AUCTION_BID_ID}` - Bid ID
- `${AUCTION_IMP_ID}` - Item ID
- `${AUCTION_SEAT_ID}` - Seat ID
- `${AUCTION_AD_ID}` - Creative ID
- `${AUCTION_LOSS}` - Loss reason code
- `${AUCTION_CURRENCY}` - Currency

### No-Bid Response

When not bidding, send an empty response with a reason code:

```rust
Response {
    id: "req-123".to_string(),
    nbr: Some(8), // No-bid reason: Unmatched User
    seatbid: vec![], // Empty bid array
    ..Default::default()
}
```

**Common No-Bid Reason Codes**:
- `0` - Unknown
- `1` - Technical Error
- `2` - Invalid Request
- `3` - Known Web Spider
- `4` - Suspected Non-Human Traffic
- `5` - Cloud/Data Center/Proxy IP
- `6` - Unsupported Device
- `7` - Blocked Publisher
- `8` - Unmatched User
- `100+` - Exchange-specific

### Multi-Seat Response

Multiple DSPs can respond with different bids:

```rust
Response {
    id: "req-123".to_string(),
    cur: Some("USD".to_string()),
    seatbid: vec![
        // Seat 1 - DSP A
        Seatbid {
            seat: Some("seat-dsp-a".to_string()),
            bid: vec![
                Bid {
                    id: "bid-a-1".to_string(),
                    item: "item1".to_string(),
                    price: 5.00,
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        // Seat 2 - DSP B
        Seatbid {
            seat: Some("seat-dsp-b".to_string()),
            bid: vec![
                Bid {
                    id: "bid-b-1".to_string(),
                    item: "item1".to_string(),
                    price: 4.50,
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
}
```

---

## Supply Chain Transparency

Supply Chain Object 1.0 enables transparency in programmatic advertising:

```rust
use iab_specs::openrtb::v3::{Source, SupplyChain, SupplyChainNode};

let request = Request {
    id: "req-schain-001".to_string(),
    item: vec![/* ... */],
    source: Some(Source {
        tid: Some("txn-abc123".to_string()), // Transaction ID
        ts: Some(1609459200000), // Timestamp
        schain: Some(SupplyChain {
            complete: 1, // 1 = complete chain, 0 = incomplete
            nodes: vec![
                // Node 1: Publisher
                SupplyChainNode {
                    asi: "publisher.com".to_string(), // Seller identifier
                    sid: "pub-123".to_string(), // Seller account ID
                    hp: Some(1), // Payment recipient (1 = yes, 0 = no)
                    name: Some("Premium Publisher".to_string()),
                    domain: Some("publisher.com".to_string()),
                    ..Default::default()
                },
                // Node 2: Exchange
                SupplyChainNode {
                    asi: "exchange.com".to_string(),
                    sid: "exch-456".to_string(),
                    hp: Some(1),
                    rid: Some("req-schain-001".to_string()), // Request ID
                    ..Default::default()
                },
            ],
            ver: "1.0".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }),
    ..Default::default()
};
```

**Key Fields**:
- `complete`: Whether the chain is complete (1) or truncated (0)
- `asi`: Advertising system identifier (domain)
- `sid`: Seller identifier within that system
- `hp`: Payment recipient flag (1 = receives payment, 0 = intermediary)
- `rid`: Request ID at this hop

---

## Private Marketplace (PMP) Deals

### Creating Deal Requests

```rust
use iab_specs::openrtb::v3::Deal;

let item = Item {
    id: "item1".to_string(),
    qty: Some(1),
    flr: Some(2.00), // Open auction floor
    flrcur: Some("USD".to_string()),
    private: Some(1), // 1 = deals only, 0 = deals + open auction
    deal: Some(vec![
        // Premium deal 1
        Deal {
            id: "deal-premium-001".to_string(),
            flr: Some(10.00), // Deal floor price
            flrcur: Some("USD".to_string()),
            at: Some(3), // Auction type: 3 = fixed price
            wseat: Some(vec!["premium-seat-1".to_string()]), // Whitelisted seats
            wadomain: Some(vec!["premium-advertiser.com".to_string()]), // Whitelisted domains
            ..Default::default()
        },
        // Premium deal 2
        Deal {
            id: "deal-premium-002".to_string(),
            flr: Some(8.50),
            flrcur: Some("USD".to_string()),
            at: Some(3),
            wseat: Some(vec!["premium-seat-2".to_string()]),
            ..Default::default()
        },
    ]),
    ..Default::default()
};
```

### Bidding on Deals

```rust
Bid {
    id: "bid-deal-1".to_string(),
    item: "item1".to_string(),
    deal: Some("deal-premium-001".to_string()), // Reference deal ID
    price: 10.00, // Must meet or exceed deal floor
    cid: Some("premium-campaign-123".to_string()),
    adomain: Some(vec!["premium-advertiser.com".to_string()]),
    ..Default::default()
}
```

**Important Notes**:
- If `private = 1`, ONLY deal bids are accepted
- If `private = 0` or `None`, both deal and open auction bids accepted
- Deal bids must reference a valid `deal.id` from the request
- Deal price must meet the deal's floor price

---

## Package Bidding

Package bidding allows bidding on multiple items as a bundle (all-or-nothing):

### Creating Package Bids

```rust
Response {
    id: "req-multi-001".to_string(),
    cur: Some("USD".to_string()),
    seatbid: vec![
        Seatbid {
            seat: Some("seat-package-dsp".to_string()),
            package: Some(1), // 1 = all or nothing, 0 = partial OK
            bid: vec![
                // Bid for item 1
                Bid {
                    id: "bid-pkg-1".to_string(),
                    item: "item1".to_string(),
                    price: 2.00,
                    cid: Some("package-campaign-111".to_string()),
                    ..Default::default()
                },
                // Bid for item 2
                Bid {
                    id: "bid-pkg-2".to_string(),
                    item: "item2".to_string(),
                    price: 2.50,
                    cid: Some("package-campaign-111".to_string()),
                    ..Default::default()
                },
                // Bid for item 3
                Bid {
                    id: "bid-pkg-3".to_string(),
                    item: "item3".to_string(),
                    price: 1.50,
                    cid: Some("package-campaign-111".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    ],
    ..Default::default()
}
```

**Behavior**:
- If `package = 1`: ALL bids in the seatbid must win, or NONE win
- If `package = 0` or `None`: Bids compete independently
- Useful for homepage takeovers, synchronized campaigns, etc.

---

## Advanced Topics

### Adding Measurement Metrics

```rust
use iab_specs::openrtb::v3::Metric;

let item = Item {
    id: "item1".to_string(),
    metric: Some(vec![
        Metric {
            typ: "viewability".to_string(),
            val: 0.75, // 75% viewability
            vendor: Some("iab.com".to_string()),
            ..Default::default()
        },
        Metric {
            typ: "completion".to_string(),
            val: 0.85, // 85% completion rate
            vendor: Some("vendor.com".to_string()),
            ..Default::default()
        },
        Metric {
            typ: "attention".to_string(),
            val: 0.65, // 65% attention score
            vendor: Some("attention-vendor.com".to_string()),
            ..Default::default()
        },
    ]),
    ..Default::default()
};
```

### Using Builder Pattern

All major types support the builder pattern:

```rust
use iab_specs::openrtb::v3::{OpenrtbBuilder, RequestBuilder, ItemBuilder};

let openrtb = OpenrtbBuilder::default()
    .ver("3.0".to_string())
    .domainspec("adcom".to_string())
    .domainver("1.0".to_string())
    .request(Some(
        RequestBuilder::default()
            .id("req-123".to_string())
            .tmax(Some(100))
            .at(Some(2))
            .cur(Some(vec!["USD".to_string()]))
            .item(vec![
                ItemBuilder::default()
                    .id("item1".to_string())
                    .qty(Some(1))
                    .flr(Some(1.50))
                    .flrcur(Some("USD".to_string()))
                    .build()?
            ])
            .build()?
    ))
    .build()?;
```

### Multi-Currency Support

```rust
Request {
    id: "req-multi-cur".to_string(),
    cur: Some(vec![
        "USD".to_string(),
        "EUR".to_string(),
        "GBP".to_string(),
    ]),
    item: vec![Item {
        id: "item1".to_string(),
        flr: Some(5.00),
        flrcur: Some("EUR".to_string()), // Floor in EUR
        ..Default::default()
    }],
    ..Default::default()
}
```

---

## Performance Tips

### 1. Reuse Structures

```rust
// Bad: Creating new structures every time
for i in 0..1000 {
    let request = Request { /* ... */ };
    // process request
}

// Good: Reuse and modify
let mut request = Request::default();
for i in 0..1000 {
    request.id = format!("req-{}", i);
    // process request
}
```

### 2. Pre-allocate Vectors

```rust
// Pre-allocate capacity
let mut items = Vec::with_capacity(10);
for i in 0..10 {
    items.push(Item { /* ... */ });
}
```

### 3. Use References for Reading

```rust
// Good: Use references
fn process_request(request: &Request) {
    println!("Processing: {}", request.id);
}

// Avoid: Taking ownership unnecessarily
fn process_request_bad(request: Request) {
    println!("Processing: {}", request.id);
}
```

### 4. Batch Serialization

```rust
// Serialize multiple requests efficiently
let requests: Vec<Openrtb> = vec![/* ... */];
let json = serde_json::to_string(&requests)?;
```

---

## Complete Examples

See the `examples/` directory for complete working examples:

- `openrtb3_ssp.rs` - SSP creating requests (4 scenarios)
- `openrtb3_dsp.rs` - DSP processing and responding (5 scenarios)

Run examples:
```bash
cargo run --example openrtb3_ssp --features openrtb_3
cargo run --example openrtb3_dsp --features openrtb_3
```

---

## Additional Resources

- [API Documentation](https://docs.rs/iab-specs)
- [Migration Guide](./MIGRATION_GUIDE_OPENRTB3.md)
- [Best Practices](./BEST_PRACTICES_OPENRTB3.md)
- [OpenRTB 3.0 Specification](https://github.com/InteractiveAdvertisingBureau/openrtb)

---

*Last Updated: 2025-11-03*
*Version: 1.0*
