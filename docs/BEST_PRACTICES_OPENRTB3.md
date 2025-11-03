# OpenRTB 3.0 Best Practices

Production-ready best practices for using OpenRTB 3.0 in the `iab-specs` library.

---

## Table of Contents

- [General Guidelines](#general-guidelines)
- [Performance Optimization](#performance-optimization)
- [Error Handling](#error-handling)
- [Security](#security)
- [Validation](#validation)
- [Supply Chain Integrity](#supply-chain-integrity)
- [Testing Strategies](#testing-strategies)
- [Monitoring and Logging](#monitoring-and-logging)
- [Common Pitfalls](#common-pitfalls)

---

## General Guidelines

### DO: Use Explicit Version Numbers

```rust
// ✅ GOOD: Explicit versions
let openrtb = Openrtb {
    ver: "3.0".to_string(),
    domainspec: "adcom".to_string(),
    domainver: "1.0".to_string(),
    // ...
};

// ❌ BAD: Hardcoded or missing
let openrtb = Openrtb {
    ver: "3".to_string(), // Too vague
    // ...
};
```

### DO: Validate Request/Response IDs Match

```rust
// ✅ GOOD: Check ID match
fn process_response(request_id: &str, response: &Response) -> Result<(), Error> {
    if response.id != request_id {
        return Err(Error::IdMismatch {
            expected: request_id.to_string(),
            actual: response.id.clone(),
        });
    }
    // Process response
    Ok(())
}

// ❌ BAD: No validation
fn process_response_bad(response: &Response) {
    // Just process without checking
}
```

### DO: Use Timeouts

```rust
// ✅ GOOD: Explicit timeout
Request {
    id: "req-123".to_string(),
    tmax: Some(100), // 100ms timeout
    // ...
}

// ❌ BAD: No timeout or unreasonable value
Request {
    id: "req-123".to_string(),
    tmax: Some(10000), // 10 seconds is too long
    // ...
}
```

**Recommended Timeouts**:
- Display: 50-150ms
- Video: 100-200ms
- Audio: 100-200ms

---

## Performance Optimization

### 1. Reuse Allocations

```rust
// ✅ GOOD: Reuse buffer
let mut buffer = String::with_capacity(4096);
for request in requests {
    buffer.clear();
    serde_json::to_writer(&mut buffer.as_mut_vec(), &request)?;
    send_request(&buffer)?;
}

// ❌ BAD: Allocate every time
for request in requests {
    let json = serde_json::to_string(&request)?; // New allocation each time
    send_request(&json)?;
}
```

### 2. Pre-allocate Vectors

```rust
// ✅ GOOD: Pre-allocate capacity
let mut items = Vec::with_capacity(expected_count);
for i in 0..expected_count {
    items.push(create_item(i));
}

// ❌ BAD: Let vector grow
let mut items = Vec::new();
for i in 0..expected_count {
    items.push(create_item(i)); // Reallocations as it grows
}
```

### 3. Use Streaming for Large Responses

```rust
// ✅ GOOD: Stream large responses
use serde_json::Deserializer;

let reader = std::io::BufReader::new(response_stream);
let deserializer = Deserializer::from_reader(reader);
let openrtb: Openrtb = serde_json::from_reader(deserializer)?;

// ❌ BAD: Load entire response into memory first
let body = response.text()?; // Could be large
let openrtb: Openrtb = serde_json::from_str(&body)?;
```

### 4. Minimize Field Serialization

```rust
// ✅ GOOD: Only set fields you need
Item {
    id: "item1".to_string(),
    flr: Some(1.50),
    flrcur: Some("USD".to_string()),
    ..Default::default() // Skip unneeded fields
}

// ❌ BAD: Setting every field even when not needed
Item {
    id: "item1".to_string(),
    qty: None, // Not needed but explicitly set
    seq: None,
    flr: Some(1.50),
    flrcur: Some("USD".to_string()),
    // ... many more None values
}
```

### 5. Batch Operations

```rust
// ✅ GOOD: Batch multiple operations
let requests: Vec<Openrtb> = create_batch_requests(100);
for chunk in requests.chunks(10) {
    process_batch(chunk)?;
}

// ❌ BAD: Process one at a time
for i in 0..100 {
    let request = create_request(i);
    process_single(request)?; // High overhead
}
```

---

## Error Handling

### DO: Use Comprehensive Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenRtbError {
    #[error("Invalid request ID: {0}")]
    InvalidRequestId(String),

    #[error("Request/Response ID mismatch: expected {expected}, got {actual}")]
    IdMismatch {
        expected: String,
        actual: String,
    },

    #[error("Invalid floor price: {0}")]
    InvalidFloorPrice(f64),

    #[error("Timeout exceeded: {0}ms")]
    Timeout(u64),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// ✅ GOOD: Specific errors
fn validate_request(req: &Request) -> Result<(), OpenRtbError> {
    if req.id.is_empty() {
        return Err(OpenRtbError::InvalidRequestId(req.id.clone()));
    }
    // ...
    Ok(())
}
```

### DO: Validate Before Processing

```rust
// ✅ GOOD: Validate first
fn process_bid_request(openrtb: &Openrtb) -> Result<Response, OpenRtbError> {
    let request = openrtb.request.as_ref()
        .ok_or(OpenRtbError::MissingRequest)?;

    validate_request(request)?; // Validate before processing

    // Process request
    let response = create_response(request)?;
    Ok(response)
}

// ❌ BAD: Process without validation
fn process_bad(openrtb: &Openrtb) -> Response {
    let request = openrtb.request.as_ref().unwrap(); // Can panic!
    create_response(request) // No validation
}
```

### DO: Handle Partial Failures Gracefully

```rust
// ✅ GOOD: Continue on partial failures
fn process_multi_item_request(request: &Request) -> Response {
    let mut bids = Vec::new();

    for item in &request.item {
        match create_bid(item) {
            Ok(bid) => bids.push(bid),
            Err(e) => {
                log::warn!("Failed to create bid for item {}: {}", item.id, e);
                // Continue processing other items
            }
        }
    }

    create_response(request.id.clone(), bids)
}

// ❌ BAD: Fail entirely on first error
fn process_bad(request: &Request) -> Result<Response, Error> {
    let mut bids = Vec::new();

    for item in &request.item {
        bids.push(create_bid(item)?); // Stops on first error
    }

    Ok(create_response(request.id.clone(), bids))
}
```

---

## Security

### 1. Validate Input Thoroughly

```rust
// ✅ GOOD: Comprehensive validation
fn validate_request(req: &Request) -> Result<(), Error> {
    // Check required fields
    if req.id.is_empty() {
        return Err(Error::EmptyRequestId);
    }

    if req.item.is_empty() {
        return Err(Error::NoItems);
    }

    // Validate item IDs are unique
    let mut seen_ids = HashSet::new();
    for item in &req.item {
        if !seen_ids.insert(&item.id) {
            return Err(Error::DuplicateItemId(item.id.clone()));
        }
    }

    // Validate floor prices
    for item in &req.item {
        if let Some(flr) = item.flr {
            if flr < 0.0 || flr > 1000000.0 {
                return Err(Error::InvalidFloorPrice(flr));
            }
        }
    }

    // Validate auction type
    if let Some(at) = req.at {
        if at < 1 || at > 3 {
            return Err(Error::InvalidAuctionType(at));
        }
    }

    Ok(())
}
```

### 2. Sanitize URLs

```rust
// ✅ GOOD: Validate and sanitize URLs
fn validate_tracking_url(url: &str) -> Result<(), Error> {
    let parsed = Url::parse(url)?;

    // Only allow https
    if parsed.scheme() != "https" {
        return Err(Error::InsecureUrl(url.to_string()));
    }

    // Check domain whitelist
    if !is_whitelisted_domain(parsed.host_str()) {
        return Err(Error::BlockedDomain(url.to_string()));
    }

    Ok(())
}

// Apply to all tracking URLs
fn validate_bid(bid: &Bid) -> Result<(), Error> {
    if let Some(ref nurl) = bid.nurl {
        validate_tracking_url(nurl)?;
    }
    if let Some(ref burl) = bid.burl {
        validate_tracking_url(burl)?;
    }
    if let Some(ref lurl) = bid.lurl {
        validate_tracking_url(lurl)?;
    }
    Ok(())
}
```

### 3. Rate Limiting

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

// ✅ GOOD: Rate limit requests
struct RequestHandler {
    semaphore: Arc<Semaphore>,
}

impl RequestHandler {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn handle_request(&self, request: Openrtb) -> Result<Openrtb, Error> {
        let _permit = self.semaphore.acquire().await?;
        // Process request with rate limiting
        process_request(request).await
    }
}
```

### 4. Prevent Injection Attacks

```rust
// ✅ GOOD: Escape special characters in IDs
fn sanitize_id(id: &str) -> String {
    id.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

// Apply to all IDs
fn create_safe_request(unsafe_id: &str) -> Request {
    Request {
        id: sanitize_id(unsafe_id),
        // ...
    }
}
```

---

## Validation

### Required Field Validation

```rust
// ✅ GOOD: Validate all required fields
fn validate_openrtb(openrtb: &Openrtb) -> Result<(), Error> {
    // Version required
    if openrtb.ver.is_empty() {
        return Err(Error::MissingVersion);
    }

    // Must have request or response
    if openrtb.request.is_none() && openrtb.response.is_none() {
        return Err(Error::MissingRequestAndResponse);
    }

    // Validate request if present
    if let Some(ref req) = openrtb.request {
        validate_request(req)?;
    }

    // Validate response if present
    if let Some(ref resp) = openrtb.response {
        validate_response(resp)?;
    }

    Ok(())
}

fn validate_request(req: &Request) -> Result<(), Error> {
    if req.id.is_empty() {
        return Err(Error::EmptyRequestId);
    }

    if req.item.is_empty() {
        return Err(Error::NoItems);
    }

    for item in &req.item {
        if item.id.is_empty() {
            return Err(Error::EmptyItemId);
        }
    }

    Ok(())
}
```

### Business Logic Validation

```rust
// ✅ GOOD: Validate business rules
fn validate_deal_bid(bid: &Bid, item: &Item) -> Result<(), Error> {
    // If bidding on a deal, validate deal exists
    if let Some(ref deal_id) = bid.deal {
        let deals = item.deal.as_ref()
            .ok_or(Error::NoDealsDefined)?;

        let deal = deals.iter()
            .find(|d| &d.id == deal_id)
            .ok_or_else(|| Error::InvalidDealId(deal_id.clone()))?;

        // Bid must meet deal floor price
        if bid.price < deal.flr.unwrap_or(0.0) {
            return Err(Error::BidBelowDealFloor {
                bid_price: bid.price,
                deal_floor: deal.flr.unwrap_or(0.0),
            });
        }

        // Check seat whitelist
        if let Some(ref wseat) = deal.wseat {
            // Verify bid seat is whitelisted
            // (implementation depends on your architecture)
        }
    }

    Ok(())
}
```

---

## Supply Chain Integrity

### DO: Validate Supply Chain Completeness

```rust
// ✅ GOOD: Verify supply chain integrity
fn validate_supply_chain(schain: &SupplyChain) -> Result<(), Error> {
    // Check version
    if schain.ver != "1.0" {
        return Err(Error::UnsupportedSupplyChainVersion(schain.ver.clone()));
    }

    // Must have at least one node
    if schain.nodes.is_empty() {
        return Err(Error::EmptySupplyChain);
    }

    // Validate each node
    for (i, node) in schain.nodes.iter().enumerate() {
        if node.asi.is_empty() {
            return Err(Error::EmptySupplyChainAsi(i));
        }
        if node.sid.is_empty() {
            return Err(Error::EmptySupplyChainSid(i));
        }
    }

    // If complete, verify payment chain
    if schain.complete == 1 {
        let payment_recipients: Vec<_> = schain.nodes.iter()
            .filter(|n| n.hp == Some(1))
            .collect();

        if payment_recipients.is_empty() {
            log::warn!("Complete supply chain with no payment recipients");
        }
    }

    Ok(())
}
```

### DO: Log Supply Chain for Auditing

```rust
// ✅ GOOD: Log supply chain for transparency
fn log_supply_chain(request_id: &str, schain: &SupplyChain) {
    let chain_str: Vec<String> = schain.nodes.iter()
        .map(|node| format!("{}:{} (hp={})",
            node.asi,
            node.sid,
            node.hp.unwrap_or(0)))
        .collect();

    log::info!(
        "Request {} supply chain [complete={}]: {}",
        request_id,
        schain.complete,
        chain_str.join(" -> ")
    );
}
```

---

## Testing Strategies

### 1. Unit Test Core Logic

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_validation() {
        // Valid request
        let request = Request {
            id: "req-123".to_string(),
            item: vec![Item {
                id: "item1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert!(validate_request(&request).is_ok());

        // Invalid: empty ID
        let invalid = Request {
            id: String::new(),
            ..Default::default()
        };
        assert!(validate_request(&invalid).is_err());

        // Invalid: no items
        let invalid = Request {
            id: "req-123".to_string(),
            item: vec![],
            ..Default::default()
        };
        assert!(validate_request(&invalid).is_err());
    }

    #[test]
    fn test_floor_price_validation() {
        assert!(validate_floor_price(1.50).is_ok());
        assert!(validate_floor_price(-1.0).is_err());
        assert!(validate_floor_price(1000001.0).is_err());
    }
}
```

### 2. Integration Test Request/Response Cycle

```rust
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_complete_rtb_cycle() {
        // Create request
        let request = create_test_request();
        let request_json = serde_json::to_string(&request).unwrap();

        // Deserialize request (simulating SSP -> DSP)
        let parsed_request: Openrtb = serde_json::from_str(&request_json).unwrap();

        // Create response
        let response = create_response_for_request(&parsed_request);
        let response_json = serde_json::to_string(&response).unwrap();

        // Deserialize response (simulating DSP -> SSP)
        let parsed_response: Openrtb = serde_json::from_str(&response_json).unwrap();

        // Verify IDs match
        assert_eq!(
            parsed_request.request.unwrap().id,
            parsed_response.response.unwrap().id
        );
    }
}
```

### 3. Fuzz Test Serialization

```rust
#[cfg(test)]
mod fuzz_tests {
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Request {
        fn arbitrary(g: &mut Gen) -> Self {
            Request {
                id: String::arbitrary(g),
                item: vec![Item::arbitrary(g)],
                ..Default::default()
            }
        }
    }

    #[test]
    fn fuzz_serialization_roundtrip() {
        quickcheck::quickcheck(|request: Request| -> bool {
            let json = serde_json::to_string(&request).unwrap();
            let parsed: Request = serde_json::from_str(&json).unwrap();
            parsed.id == request.id
        });
    }
}
```

---

## Monitoring and Logging

### 1. Structured Logging

```rust
// ✅ GOOD: Structured logging with context
fn process_request(openrtb: &Openrtb) -> Result<Openrtb, Error> {
    let request = openrtb.request.as_ref()
        .ok_or(Error::MissingRequest)?;

    log::info!(
        request_id = %request.id,
        items = request.item.len(),
        timeout = ?request.tmax,
        auction_type = ?request.at;
        "Processing bid request"
    );

    let start = std::time::Instant::now();
    let response = handle_request(request)?;
    let duration = start.elapsed();

    log::info!(
        request_id = %request.id,
        bid_count = response.response.as_ref()
            .map(|r| r.seatbid.iter().map(|s| s.bid.len()).sum::<usize>())
            .unwrap_or(0),
        duration_ms = duration.as_millis();
        "Request processed"
    );

    Ok(response)
}
```

### 2. Metrics Collection

```rust
use prometheus::{Counter, Histogram};

lazy_static! {
    static ref REQUEST_COUNTER: Counter = Counter::new(
        "openrtb_requests_total",
        "Total number of OpenRTB requests"
    ).unwrap();

    static ref RESPONSE_TIME: Histogram = Histogram::new(
        "openrtb_response_time_ms",
        "OpenRTB response time in milliseconds"
    ).unwrap();

    static ref BID_COUNTER: Counter = Counter::new(
        "openrtb_bids_total",
        "Total number of bids submitted"
    ).unwrap();
}

// ✅ GOOD: Collect metrics
fn handle_request(request: &Request) -> Result<Response, Error> {
    REQUEST_COUNTER.inc();
    let timer = RESPONSE_TIME.start_timer();

    let response = process_bidding(request)?;

    let bid_count: usize = response.seatbid.iter()
        .map(|s| s.bid.len())
        .sum();
    BID_COUNTER.inc_by(bid_count as f64);

    timer.observe_duration();
    Ok(response)
}
```

---

## Common Pitfalls

### Pitfall 1: Not Checking for None Values

```rust
// ❌ BAD: Assuming values exist
let floor_price = item.flr.unwrap(); // Can panic!

// ✅ GOOD: Handle None safely
let floor_price = item.flr.unwrap_or(0.0);

// ✅ BETTER: Explicit check
if let Some(flr) = item.flr {
    // Use flr
} else {
    // Handle missing floor price
}
```

### Pitfall 2: Ignoring Auction Type

```rust
// ❌ BAD: Ignoring auction type
fn create_bid(item: &Item) -> Bid {
    Bid {
        price: calculate_bid_price(item),
        // ...
    }
}

// ✅ GOOD: Respect auction type
fn create_bid(request: &Request, item: &Item) -> Bid {
    let price = match request.at {
        Some(1) => calculate_first_price_bid(item),  // First price
        Some(2) => calculate_second_price_bid(item), // Second price
        Some(3) => get_fixed_price(item),            // Fixed price
        _ => calculate_default_bid(item),
    };

    Bid { price, /* ... */ }
}
```

### Pitfall 3: Not Validating Item References

```rust
// ❌ BAD: No validation
fn create_response(request: &Request) -> Response {
    Response {
        id: request.id.clone(),
        seatbid: vec![Seatbid {
            bid: vec![Bid {
                item: "item1".to_string(), // Might not exist in request!
                // ...
            }],
            // ...
        }],
        // ...
    }
}

// ✅ GOOD: Validate item exists
fn create_response(request: &Request) -> Result<Response, Error> {
    let item_id = "item1";

    // Verify item exists in request
    if !request.item.iter().any(|i| i.id == item_id) {
        return Err(Error::InvalidItemReference(item_id.to_string()));
    }

    Ok(Response {
        id: request.id.clone(),
        seatbid: vec![Seatbid {
            bid: vec![Bid {
                item: item_id.to_string(),
                // ...
            }],
            // ...
        }],
        // ...
    })
}
```

### Pitfall 4: Forgetting Supply Chain Transparency

```rust
// ❌ BAD: No supply chain
let request = Request {
    id: "req-123".to_string(),
    // Missing source/schain
};

// ✅ GOOD: Always include supply chain
let request = Request {
    id: "req-123".to_string(),
    source: Some(Source {
        tid: Some(generate_transaction_id()),
        schain: Some(build_supply_chain()),
        ..Default::default()
    }),
    // ...
};
```

---

## Additional Resources

- [Usage Guide](./USAGE_GUIDE_OPENRTB3.md)
- [Migration Guide](./MIGRATION_GUIDE_OPENRTB3.md)
- [API Documentation](https://docs.rs/iab-specs)
- [Examples](../examples/)

---

*Last Updated: 2025-11-03*
*Version: 1.0*
