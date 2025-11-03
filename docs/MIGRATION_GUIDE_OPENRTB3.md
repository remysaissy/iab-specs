# OpenRTB 2.x → 3.0 Migration Guide

This guide helps you migrate from OpenRTB 2.5/2.6 to OpenRTB 3.0 in the `iab-specs` library.

---

## Table of Contents

- [Overview](#overview)
- [Key Architectural Changes](#key-architectural-changes)
- [Breaking Changes](#breaking-changes)
- [Step-by-Step Migration](#step-by-step-migration)
- [API Mapping Reference](#api-mapping-reference)
- [Common Migration Patterns](#common-migration-patterns)
- [Troubleshooting](#troubleshooting)

---

## Overview

OpenRTB 3.0 represents a fundamental redesign of the protocol with a four-layer architecture:

1. **Transport Layer**: HTTP-based (no change)
2. **Format Layer**: JSON (no change)
3. **Transaction Layer**: New protocol objects (major changes)
4. **Domain Layer**: AdCOM 1.0 integration (new)

**Migration Complexity**: Medium to High
**Estimated Effort**: 2-4 weeks depending on codebase size
**Backward Compatibility**: OpenRTB 2.x and 3.0 can coexist

---

## Key Architectural Changes

### 1. Root Container

**OpenRTB 2.x**:
```rust
use iab_specs::openrtb::v25::BidRequest;
use iab_specs::openrtb::v25::BidResponse;

let request = BidRequest { /* ... */ };
let response = BidResponse { /* ... */ };
```

**OpenRTB 3.0**:
```rust
use iab_specs::openrtb::v3::{Openrtb, Request, Response};

// Single root container with versioning
let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: Some(Request { /* ... */ }),
    response: None,
};
```

**Why**: OpenRTB 3.0 introduces explicit version tracking for both protocol and domain layers.

---

### 2. Inventory Unit: Imp → Item

**OpenRTB 2.x**:
```rust
use iab_specs::openrtb::v25::{BidRequest, Imp};

let request = BidRequest {
    id: "req-123".to_string(),
    imp: vec![
        Imp {
            id: "imp1".to_string(),
            banner: Some(Banner { /* ... */ }),
            bidfloor: Some(1.50),
            bidfloorcur: Some("USD".to_string()),
            ..Default::default()
        }
    ],
    ..Default::default()
};
```

**OpenRTB 3.0**:
```rust
use iab_specs::openrtb::v3::{Request, Item};

let request = Request {
    id: "req-123".to_string(),
    item: vec![
        Item {
            id: "item1".to_string(),
            qty: Some(1),
            flr: Some(1.50),
            flrcur: Some("USD".to_string()),
            spec: Some(/* AdCOM placement spec */),
            ..Default::default()
        }
    ],
    ..Default::default()
};
```

**Changes**:
- `Imp` → `Item`
- `bidfloor` → `flr`
- `bidfloorcur` → `flrcur`
- `banner`/`video`/`audio` → `spec` (AdCOM placement reference)

---

### 3. Supply Chain Transparency

**OpenRTB 2.x** (extension):
```rust
// Supply chain was in ext.schain
let request = BidRequest {
    ext: Some(json!({
        "schain": {
            "complete": 1,
            "nodes": [/* ... */],
            "ver": "1.0"
        }
    })),
    ..Default::default()
};
```

**OpenRTB 3.0** (core field):
```rust
use iab_specs::openrtb::v3::{Source, SupplyChain, SupplyChainNode};

let request = Request {
    source: Some(Source {
        schain: Some(SupplyChain {
            complete: 1,
            nodes: vec![
                SupplyChainNode {
                    asi: "publisher.com".to_string(),
                    sid: "pub-123".to_string(),
                    hp: Some(1),
                    ..Default::default()
                }
            ],
            ver: "1.0".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }),
    ..Default::default()
};
```

**Why**: Supply chain is now a first-class object, no longer an extension.

---

## Breaking Changes

### Field Renames

| OpenRTB 2.x | OpenRTB 3.0 | Notes |
|-------------|-------------|-------|
| `BidRequest` | `Request` | Inside `Openrtb` wrapper |
| `BidResponse` | `Response` | Inside `Openrtb` wrapper |
| `Imp` | `Item` | Inventory unit |
| `bidfloor` | `flr` | Floor price |
| `bidfloorcur` | `flrcur` | Floor currency |
| `pmp` | `deal` (on Item) | Private marketplace |

### Structural Changes

1. **Root object**: All requests/responses wrapped in `Openrtb` container
2. **Media specs**: Banner/Video/Audio moved to `spec` field (AdCOM)
3. **Context objects**: Site/App/User moved to `context` field (AdCOM)
4. **Supply chain**: Promoted from extension to core `source.schain`

### Removed Fields

- `imp.secure`: Security handled differently in AdCOM
- `imp.pmp`: Replaced by `item.deal` array
- Direct banner/video/audio objects: Now in `spec`

---

## Step-by-Step Migration

### Step 1: Add OpenRTB 3.0 Feature

```toml
[dependencies]
iab-specs = { version = "0.2", features = ["openrtb_3"] }
```

### Step 2: Update Imports

**Before**:
```rust
use iab_specs::openrtb::v25::{BidRequest, BidResponse, Imp, Banner};
```

**After**:
```rust
use iab_specs::openrtb::v3::{Openrtb, Request, Response, Item};
use iab_specs::openrtb::v3::spec::{DisplayPlacement, DisplayFormat};
```

### Step 3: Migrate Request Creation

**Before (v2.5)**:
```rust
let request = BidRequest {
    id: "req-123".to_string(),
    imp: vec![
        Imp {
            id: "imp1".to_string(),
            banner: Some(Banner {
                w: Some(300),
                h: Some(250),
                pos: Some(1),
                ..Default::default()
            }),
            bidfloor: Some(1.50),
            bidfloorcur: Some("USD".to_string()),
            ..Default::default()
        }
    ],
    site: Some(Site { /* ... */ }),
    ..Default::default()
};
```

**After (v3.0)**:
```rust
let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: Some(Request {
        id: "req-123".to_string(),
        item: vec![
            Item {
                id: "item1".to_string(),
                qty: Some(1),
                flr: Some(1.50),
                flrcur: Some("USD".to_string()),
                spec: Some(serde_json::json!({
                    "placement": DisplayPlacement {
                        pos: Some(1),
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
            }
        ],
        // context field for Site/App/Device/User
        ..Default::default()
    }),
    response: None,
};
```

### Step 4: Migrate Response Handling

**Before (v2.5)**:
```rust
let response = BidResponse {
    id: "req-123".to_string(),
    seatbid: vec![
        SeatBid {
            bid: vec![
                Bid {
                    id: "bid-1".to_string(),
                    impid: "imp1".to_string(),
                    price: 2.50,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ],
    ..Default::default()
};
```

**After (v3.0)**:
```rust
let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: None,
    response: Some(Response {
        id: "req-123".to_string(),
        cur: Some("USD".to_string()),
        seatbid: vec![
            Seatbid {
                seat: Some("seat-1".to_string()),
                bid: vec![
                    Bid {
                        id: "bid-1".to_string(),
                        item: "item1".to_string(), // Changed from impid
                        price: 2.50,
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

**Key Change**: `bid.impid` → `bid.item`

### Step 5: Migrate Supply Chain

**Before (v2.5)**:
```rust
let request = BidRequest {
    ext: Some(json!({
        "schain": {
            "complete": 1,
            "nodes": [{
                "asi": "example.com",
                "sid": "seller123",
                "hp": 1
            }],
            "ver": "1.0"
        }
    })),
    ..Default::default()
};
```

**After (v3.0)**:
```rust
use iab_specs::openrtb::v3::{Source, SupplyChain, SupplyChainNode};

let request = Request {
    source: Some(Source {
        schain: Some(SupplyChain {
            complete: 1,
            nodes: vec![
                SupplyChainNode {
                    asi: "example.com".to_string(),
                    sid: "seller123".to_string(),
                    hp: Some(1),
                    ..Default::default()
                }
            ],
            ver: "1.0".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }),
    ..Default::default()
};
```

---

## API Mapping Reference

### Request-Side Mapping

| v2.5/2.6 Path | v3.0 Path | Type Change |
|---------------|-----------|-------------|
| `BidRequest` | `Openrtb.request` | Wrapped |
| `BidRequest.id` | `Request.id` | Same |
| `BidRequest.imp` | `Request.item` | `Imp` → `Item` |
| `BidRequest.site` | `Request.context` | AdCOM |
| `BidRequest.app` | `Request.context` | AdCOM |
| `BidRequest.device` | `Request.context` | AdCOM |
| `BidRequest.user` | `Request.context` | AdCOM |
| `BidRequest.at` | `Request.at` | Same |
| `BidRequest.tmax` | `Request.tmax` | Same |
| `BidRequest.cur` | `Request.cur` | Same |
| `Imp.id` | `Item.id` | Same |
| `Imp.banner` | `Item.spec` | AdCOM placement |
| `Imp.video` | `Item.spec` | AdCOM placement |
| `Imp.audio` | `Item.spec` | AdCOM placement |
| `Imp.bidfloor` | `Item.flr` | Renamed |
| `Imp.bidfloorcur` | `Item.flrcur` | Renamed |
| `Imp.pmp` | `Item.deal` | Changed structure |
| `ext.schain` | `source.schain` | Promoted to core |

### Response-Side Mapping

| v2.5/2.6 Path | v3.0 Path | Type Change |
|---------------|-----------|-------------|
| `BidResponse` | `Openrtb.response` | Wrapped |
| `BidResponse.id` | `Response.id` | Same |
| `BidResponse.seatbid` | `Response.seatbid` | Same |
| `BidResponse.bidid` | `Response.bidid` | Same |
| `BidResponse.cur` | `Response.cur` | Same |
| `BidResponse.nbr` | `Response.nbr` | Same |
| `SeatBid` | `Seatbid` | Case change |
| `SeatBid.bid` | `Seatbid.bid` | Same |
| `SeatBid.seat` | `Seatbid.seat` | Same |
| `Bid.id` | `Bid.id` | Same |
| `Bid.impid` | `Bid.item` | Renamed |
| `Bid.price` | `Bid.price` | Same |
| `Bid.adm` | `Bid.media` | AdCOM media |
| `Bid.nurl` | `Bid.nurl` | Same |
| `Bid.burl` | `Bid.burl` | Same |
| `Bid.lurl` | `Bid.lurl` | Same |

---

## Common Migration Patterns

### Pattern 1: Display Banner Request

**v2.5**:
```rust
use iab_specs::openrtb::v25::{BidRequest, Imp, Banner};

BidRequest {
    id: "req-1".to_string(),
    imp: vec![Imp {
        id: "imp1".to_string(),
        banner: Some(Banner {
            w: Some(300),
            h: Some(250),
            ..Default::default()
        }),
        bidfloor: Some(1.50),
        ..Default::default()
    }],
    ..Default::default()
}
```

**v3.0**:
```rust
use iab_specs::openrtb::v3::{Openrtb, Request, Item};
use iab_specs::openrtb::v3::spec::{DisplayPlacement, DisplayFormat};

Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: Some(Request {
        id: "req-1".to_string(),
        item: vec![Item {
            id: "item1".to_string(),
            flr: Some(1.50),
            flrcur: Some("USD".to_string()),
            spec: Some(serde_json::json!({
                "placement": DisplayPlacement {
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
}
```

### Pattern 2: Video Request

**v2.5**:
```rust
imp: vec![Imp {
    id: "imp1".to_string(),
    video: Some(Video {
        mimes: vec!["video/mp4".to_string()],
        minduration: Some(15),
        maxduration: Some(30),
        protocols: Some(vec![2, 3, 5, 6]),
        w: Some(640),
        h: Some(480),
        ..Default::default()
    }),
    ..Default::default()
}]
```

**v3.0**:
```rust
use iab_specs::openrtb::v3::spec::VideoPlacement;

item: vec![Item {
    id: "item1".to_string(),
    spec: Some(serde_json::json!({
        "placement": VideoPlacement {
            ptype: Some(1), // In-stream
            w: Some(640),
            h: Some(480),
            mindur: Some(15),
            maxdur: Some(30),
            mime: Some(vec!["video/mp4".to_string()]),
            protocol: Some(vec![2, 3, 5, 6]),
            ..Default::default()
        }
    })),
    ..Default::default()
}]
```

### Pattern 3: Deal/PMP Request

**v2.5**:
```rust
imp: vec![Imp {
    id: "imp1".to_string(),
    pmp: Some(Pmp {
        private_auction: Some(1),
        deals: Some(vec![
            Deal {
                id: "deal-1".to_string(),
                bidfloor: Some(10.00),
                wseat: Some(vec!["seat1".to_string()]),
                ..Default::default()
            }
        ]),
        ..Default::default()
    }),
    ..Default::default()
}]
```

**v3.0**:
```rust
use iab_specs::openrtb::v3::Deal;

item: vec![Item {
    id: "item1".to_string(),
    private: Some(1),
    deal: Some(vec![
        Deal {
            id: "deal-1".to_string(),
            flr: Some(10.00),
            flrcur: Some("USD".to_string()),
            at: Some(3), // Fixed price
            wseat: Some(vec!["seat1".to_string()]),
            ..Default::default()
        }
    ]),
    ..Default::default()
}]
```

### Pattern 4: Bid Response

**v2.5**:
```rust
BidResponse {
    id: "req-1".to_string(),
    seatbid: vec![SeatBid {
        bid: vec![Bid {
            id: "bid-1".to_string(),
            impid: "imp1".to_string(),
            price: 5.50,
            adm: Some("<creative markup>".to_string()),
            nurl: Some("https://win.example.com/".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    }],
    ..Default::default()
}
```

**v3.0**:
```rust
Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: None,
    response: Some(Response {
        id: "req-1".to_string(),
        cur: Some("USD".to_string()),
        seatbid: vec![Seatbid {
            seat: Some("seat-1".to_string()),
            bid: vec![Bid {
                id: "bid-1".to_string(),
                item: "item1".to_string(), // Changed from impid
                price: 5.50,
                media: Some(/* AdCOM media object */),
                nurl: Some("https://win.example.com/".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    }),
}
```

---

## Troubleshooting

### Issue 1: Missing `Openrtb` wrapper

**Error**:
```
error[E0308]: mismatched types
expected struct `Openrtb`
found struct `Request`
```

**Solution**: Wrap your `Request` or `Response` in an `Openrtb` container:
```rust
let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    request: Some(your_request),
    response: None,
};
```

### Issue 2: Cannot find `Imp` type

**Error**:
```
error[E0432]: unresolved import `iab_specs::openrtb::v3::Imp`
```

**Solution**: Replace `Imp` with `Item`:
```rust
use iab_specs::openrtb::v3::Item;
```

### Issue 3: `bidfloor` field not found

**Error**:
```
error[E0609]: no field `bidfloor` on type `Item`
```

**Solution**: Use `flr` and `flrcur`:
```rust
Item {
    flr: Some(1.50),
    flrcur: Some("USD".to_string()),
    ..Default::default()
}
```

### Issue 4: Supply chain in wrong location

**Error**: Supply chain not being serialized

**Solution**: Move from extension to `source.schain`:
```rust
Request {
    source: Some(Source {
        schain: Some(SupplyChain { /* ... */ }),
        ..Default::default()
    }),
    ..Default::default()
}
```

### Issue 5: `impid` field not found on Bid

**Error**:
```
error[E0609]: no field `impid` on type `Bid`
```

**Solution**: Use `item` instead:
```rust
Bid {
    item: "item1".to_string(), // Not impid
    ..Default::default()
}
```

---

## Additional Resources

- [OpenRTB 3.0 Specification](https://github.com/InteractiveAdvertisingBureau/openrtb/blob/main/OpenRTB%20v3.0%20FINAL.md)
- [Usage Guide](./USAGE_GUIDE_OPENRTB3.md)
- [Best Practices](./BEST_PRACTICES_OPENRTB3.md)
- [Examples](../examples/)
  - `openrtb3_ssp.rs` - SSP integration examples
  - `openrtb3_dsp.rs` - DSP integration examples

---

## Need Help?

- **API Documentation**: Run `cargo doc --open --features openrtb_3`
- **Examples**: Check `examples/openrtb3_*.rs` for complete working code
- **Issues**: Report at [GitHub Issues](https://github.com/remysaissy/iab-specs/issues)

---

*Last Updated: 2025-11-03*
*Version: 1.0*
