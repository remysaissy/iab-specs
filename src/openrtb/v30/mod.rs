mod bid;
mod deal;
mod item;
mod metric;
/// OpenRTB 3.0 Protocol Implementation
///
/// This module implements the OpenRTB 3.0 specification, which represents a fundamental
/// redesign of the OpenRTB protocol with a layered architecture approach.
///
/// # Architecture Overview
///
/// OpenRTB 3.0 introduces a four-layer architecture:
///
/// ## 1. Transport Layer
/// Defines HTTP-based request/response mechanism (not implemented in this library -
/// handled by your HTTP client).
///
/// ## 2. Format Layer
/// JSON encoding/decoding provided by this module via `serde`.
///
/// ## 3. Transaction Layer
/// The core protocol objects implemented here:
/// - [`Openrtb`] - Root container with versioning
/// - [`Request`] - Bid request object
/// - [`Response`] - Bid response object
///
/// ## 4. Domain Layer
/// Domain objects from [AdCOM 1.0](crate::adcom) (context, placement specs, etc.)
///
/// # Key Differences from OpenRTB 2.x
///
/// | Feature | OpenRTB 2.x | OpenRTB 3.0 |
/// |---------|-------------|-------------|
/// | **Root Object** | `BidRequest`/`BidResponse` | [`Openrtb`] wrapper |
/// | **Inventory Unit** | `Imp` | [`Item`] |
/// | **Domain Objects** | Inline in protocol | AdCOM references |
/// | **Supply Chain** | Extension object | Core field |
/// | **Versioning** | Protocol only | Protocol + Domain versions |
///
/// # Feature Flag
///
/// This module is available when the `openrtb_30` feature is enabled:
///
/// ```toml
/// [dependencies]
/// iab-specs = { version = "0.2", features = ["openrtb_30"] }
/// ```
///
/// # Quick Start
///
/// ## Creating a Bid Request
///
/// ```rust
/// use iab_specs::openrtb::v30::{Openrtb, Request, Item};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let openrtb = Openrtb::builder()
///     .ver("3.0".to_string())
///     .domainspec("adcom".to_string())
///     .domainver("1.0".to_string())
///     .request(Some(Request::builder()
///         .id("bid-request-123".to_string())
///         .tmax(Some(100))
///         .item(vec![
///             Item::builder()
///                 .id("item1".to_string())
///                 .qty(Some(1))
///                 .build()
///                 .unwrap()
///         ])
///         .build()
///         .unwrap()
///     ))
///     .build()
///     .unwrap();
///
/// // Serialize to JSON
/// let json = serde_json::to_string(&openrtb)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Parsing a Bid Response
///
/// ```rust
/// use iab_specs::openrtb::v30::{Openrtb, Response};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let json = r#"{
///     "ver": "3.0",
///     "domainspec": "adcom",
///     "domainver": "1.0",
///     "response": {
///         "id": "bid-response-123",
///         "seatbid": []
///     }
/// }"#;
///
/// let openrtb: Openrtb = serde_json::from_str(json)?;
/// assert_eq!(openrtb.ver, "3.0");
/// # Ok(())
/// # }
/// ```
///
/// # Module Structure
///
/// - [`openrtb`](Openrtb) - Root container with versioning
/// - [`request`](Request) - Bid request object
/// - [`response`](Response) - Bid response object
/// - [`item`](Item) - Inventory/impression item
/// - [`deal`](Deal) - Deal terms
/// - [`source`](Source) - Source and supply chain
/// - [`metric`](Metric) - Metric specifications
/// - [`seatbid`](Seatbid) - Seat bid object
/// - [`bid`](Bid) - Individual bid
///
/// # Specification Compliance
///
/// This implementation follows the official [OpenRTB 3.0 specification](https://github.com/InteractiveAdvertisingBureau/openrtb/blob/main/OpenRTB%20v3.0%20FINAL.md)
/// and integrates with [AdCOM 1.0](crate::adcom) for domain objects.
///
/// # Examples
///
/// For complete examples, see:
/// - SSP integration example: `examples/openrtb3_ssp.rs`
/// - DSP integration example: `examples/openrtb3_dsp.rs`
mod openrtb;
mod request;
mod response;
mod seat_bid;
mod source;
pub mod spec;

pub use bid::Bid;
pub use deal::Deal;
pub use item::Item;
pub use metric::Metric;
pub use openrtb::Openrtb;
pub use request::Request;
pub use response::Response;
pub use seat_bid::SeatBid;
pub use source::{Source, SupplyChain, SupplyChainNode};
pub use spec::{AudioPlacement, DisplayFormat, DisplayPlacement, VideoPlacement};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_request_response_cycle() {
        // Create a complete bid request
        let node = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller1".to_string())
            .hp(Some(1))
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(1)
            .nodes(vec![node])
            .ver("1.0".to_string())
            .build()
            .unwrap();

        let source = Source::builder()
            .tid(Some("txn-456".to_string()))
            .schain(Some(schain))
            .build()
            .unwrap();

        let item1 = Item::builder()
            .id("item1".to_string())
            .qty(Some(1))
            .flr(Some(1.50))
            .flrcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let item2 = Item::builder()
            .id("item2".to_string())
            .qty(Some(1))
            .flr(Some(2.00))
            .flrcur(Some("USD".to_string()))
            .build()
            .unwrap();

        let req = Request::builder()
            .id("req-123".to_string())
            .test(Some(0))
            .tmax(Some(100))
            .at(Some(2))
            .cur(Some(vec!["USD".to_string()]))
            .item(vec![item1, item2])
            .source(Some(source))
            .build()
            .unwrap();

        let request = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .request(Some(req))
            .build()
            .unwrap();

        // Serialize the request to JSON
        let request_json = serde_json::to_string(&request).unwrap();
        assert!(request_json.contains("\"ver\":\"3.0\""));
        assert!(request_json.contains("\"id\":\"req-123\""));

        // Create a bid response
        let bid1 = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(2.50)
            .nurl(Some("https://win.example.com/".to_string()))
            .build()
            .unwrap();

        let bid2 = Bid::builder()
            .id("bid-2".to_string())
            .item("item2".to_string())
            .price(3.00)
            .nurl(Some("https://win.example.com/".to_string()))
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-abc".to_string()))
            .bid(vec![bid1, bid2])
            .build()
            .unwrap();

        let resp = Response::builder()
            .id("req-123".to_string())
            .bidid(Some("bid-resp-789".to_string()))
            .cur(Some("USD".to_string()))
            .seatbid(vec![seatbid])
            .build()
            .unwrap();

        let response = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .response(Some(resp))
            .build()
            .unwrap();

        // Serialize the response to JSON
        let response_json = serde_json::to_string(&response).unwrap();
        assert!(response_json.contains("\"ver\":\"3.0\""));
        assert!(response_json.contains("\"id\":\"req-123\""));

        // Verify response matches request
        let req_id = request.request.as_ref().unwrap().id.clone();
        let resp_id = response.response.as_ref().unwrap().id.clone();
        assert_eq!(req_id, resp_id);
    }

    #[test]
    fn test_request_with_deals() {
        let deal1 = Deal::builder()
            .id("deal-1".to_string())
            .flr(Some(5.00))
            .at(Some(3)) // Fixed price
            .wseat(Some(vec!["seat1".to_string()]))
            .build()
            .unwrap();

        let deal2 = Deal::builder()
            .id("deal-2".to_string())
            .flr(Some(4.50))
            .at(Some(3))
            .wseat(Some(vec!["seat2".to_string()]))
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item1".to_string())
            .deal(Some(vec![deal1, deal2]))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-deals".to_string())
            .item(vec![item])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, "req-deals");
        assert_eq!(parsed.item[0].deal.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_response_with_package_bid() {
        let bid1 = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(5.00)
            .build()
            .unwrap();

        let bid2 = Bid::builder()
            .id("bid-2".to_string())
            .item("item2".to_string())
            .price(5.00)
            .build()
            .unwrap();

        let seatbid = SeatBid::builder()
            .seat(Some("seat-package".to_string()))
            .package(Some(1))
            .bid(vec![bid1, bid2])
            .build()
            .unwrap();

        let response = Response::builder()
            .id("resp-package".to_string())
            .cur(Some("USD".to_string()))
            .seatbid(vec![seatbid])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: Response = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.seatbid[0].package, Some(1));
        assert_eq!(parsed.seatbid[0].bid.len(), 2);
    }

    #[test]
    fn test_supply_chain_transparency() {
        let node1 = SupplyChainNode::builder()
            .asi("publisher.com".to_string())
            .sid("pub-123".to_string())
            .hp(Some(1))
            .name(Some("Publisher Name".to_string()))
            .build()
            .unwrap();

        let node2 = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("exch-456".to_string())
            .hp(Some(1))
            .rid(Some("req-789".to_string()))
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(1)
            .nodes(vec![node1, node2])
            .ver("1.0".to_string())
            .build()
            .unwrap();

        let source = Source::builder()
            .tid(Some("txn-abc".to_string()))
            .schain(Some(schain))
            .build()
            .unwrap();

        let item = Item::builder().id("item1".to_string()).build().unwrap();

        let request = Request::builder()
            .id("req-schain".to_string())
            .item(vec![item])
            .source(Some(source))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();

        let schain = parsed.source.unwrap().schain.unwrap();
        assert_eq!(schain.complete, 1);
        assert_eq!(schain.nodes.len(), 2);
        assert_eq!(schain.nodes[0].asi, "publisher.com");
        assert_eq!(schain.nodes[1].asi, "exchange.com");
    }

    #[test]
    fn test_no_bid_response() {
        let response = Response::builder()
            .id("req-nobid".to_string())
            .nbr(Some(8))
            .seatbid(vec![])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: Response = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.nbr, Some(8));
        assert_eq!(parsed.seatbid.len(), 0);
    }

    #[test]
    fn test_multi_currency_request() {
        use spec::{DisplayFormat, DisplayPlacement};

        let display_format = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        let display_placement = DisplayPlacement::builder()
            .pos(Some(1))
            .w(Some(300))
            .h(Some(250))
            .displayfmt(Some(vec![display_format]))
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item1".to_string())
            .flr(Some(5.00))
            .flrcur(Some("EUR".to_string()))
            .spec(Some(Box::new(serde_json::json!({
                "placement": display_placement
            }))))
            .build()
            .unwrap();

        let request_inner = Request::builder()
            .id("req-multicurrency".to_string())
            .cur(Some(vec![
                "USD".to_string(),
                "EUR".to_string(),
                "GBP".to_string(),
            ]))
            .item(vec![item])
            .build()
            .unwrap();

        let request = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .request(Some(request_inner))
            .response(None)
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Openrtb = serde_json::from_str(&json).unwrap();

        let req = parsed.request.unwrap();
        assert_eq!(req.cur.as_ref().unwrap().len(), 3);
        assert_eq!(req.item[0].flrcur.as_ref().unwrap(), "EUR");
    }

    #[test]
    fn test_video_placement_integration() {
        use spec::VideoPlacement;

        let video_placement = VideoPlacement::builder()
            .ptype(Some(1))
            .pos(Some(1))
            .w(Some(640))
            .h(Some(480))
            .mindur(Some(15))
            .maxdur(Some(30))
            .skip(Some(1))
            .skipmin(Some(15))
            .skipafter(Some(5))
            .playmethod(Some(vec![1]))
            .mime(Some(vec!["video/mp4".to_string()]))
            .protocol(Some(vec![2, 3, 5, 6]))
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item1".to_string())
            .spec(Some(Box::new(serde_json::json!({
                "placement": video_placement
            }))))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-video".to_string())
            .item(vec![item])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();

        assert!(parsed.item[0].spec.is_some());
    }

    #[test]
    fn test_audio_placement_integration() {
        use spec::AudioPlacement;

        let audio_placement = AudioPlacement::builder()
            .mindur(Some(15))
            .maxdur(Some(60))
            .protocol(Some(vec![2, 3, 5, 6]))
            .feed(Some(1))
            .mime(Some(vec!["audio/mp4".to_string()]))
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item1".to_string())
            .spec(Some(Box::new(serde_json::json!({
                "placement": audio_placement
            }))))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-audio".to_string())
            .item(vec![item])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();

        assert!(parsed.item[0].spec.is_some());
    }

    #[test]
    fn test_request_with_metrics() {
        let metric1 = Metric::builder()
            .type_("viewability".to_string())
            .val(0.75)
            .vendor(Some("iab.com".to_string()))
            .build()
            .unwrap();

        let metric2 = Metric::builder()
            .type_("completion".to_string())
            .val(0.85)
            .vendor(Some("vendor.com".to_string()))
            .build()
            .unwrap();

        let metric3 = Metric::builder()
            .type_("attention".to_string())
            .val(0.65)
            .vendor(Some("attention-vendor.com".to_string()))
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item1".to_string())
            .metric(Some(vec![metric1, metric2, metric3]))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-metrics".to_string())
            .item(vec![item])
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();

        let metrics = parsed.item[0].metric.as_ref().unwrap();
        assert_eq!(metrics.len(), 3);
        assert_eq!(metrics[0].type_, "viewability");
        assert_eq!(metrics[1].type_, "completion");
        assert_eq!(metrics[2].type_, "attention");
    }

    #[test]
    fn test_multiple_seat_bids() {
        let bid1_1 = Bid::builder()
            .id("bid-1-1".to_string())
            .item("item1".to_string())
            .price(5.00)
            .build()
            .unwrap();

        let seatbid1 = SeatBid::builder()
            .seat(Some("seat-1".to_string()))
            .bid(vec![bid1_1])
            .build()
            .unwrap();

        let bid2_1 = Bid::builder()
            .id("bid-2-1".to_string())
            .item("item1".to_string())
            .price(4.50)
            .build()
            .unwrap();

        let seatbid2 = SeatBid::builder()
            .seat(Some("seat-2".to_string()))
            .bid(vec![bid2_1])
            .build()
            .unwrap();

        let bid3_1 = Bid::builder()
            .id("bid-3-1".to_string())
            .item("item1".to_string())
            .price(6.00)
            .build()
            .unwrap();

        let bid3_2 = Bid::builder()
            .id("bid-3-2".to_string())
            .item("item2".to_string())
            .price(3.50)
            .build()
            .unwrap();

        let seatbid3 = SeatBid::builder()
            .seat(Some("seat-3".to_string()))
            .bid(vec![bid3_1, bid3_2])
            .build()
            .unwrap();

        let response = Response::builder()
            .id("resp-multi-seat".to_string())
            .cur(Some("USD".to_string()))
            .seatbid(vec![seatbid1, seatbid2, seatbid3])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: Response = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.seatbid.len(), 3);
        assert_eq!(parsed.seatbid[0].bid.len(), 1);
        assert_eq!(parsed.seatbid[1].bid.len(), 1);
        assert_eq!(parsed.seatbid[2].bid.len(), 2);
    }

    #[test]
    fn test_minimal_request() {
        let item = Item::builder().id("item1".to_string()).build().unwrap();

        let request_inner = Request::builder()
            .id("req-minimal".to_string())
            .item(vec![item])
            .build()
            .unwrap();

        let request = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .request(Some(request_inner))
            .response(None)
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: Openrtb = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ver, "3.0");
        assert!(parsed.request.is_some());
        let req = parsed.request.unwrap();
        assert_eq!(req.id, "req-minimal");
        assert_eq!(req.item.len(), 1);
    }

    #[test]
    fn test_auction_types() {
        for (at_value, description) in
            &[(1, "First price"), (2, "Second price"), (3, "Fixed price")]
        {
            let item = Item::builder().id("item1".to_string()).build().unwrap();

            let request = Request::builder()
                .id(format!("req-at-{}", at_value))
                .at(Some(*at_value))
                .item(vec![item])
                .build()
                .unwrap();

            let json = serde_json::to_string(&request).unwrap();
            let parsed: Request = serde_json::from_str(&json).unwrap();

            assert_eq!(parsed.at, Some(*at_value), "Failed for {}", description);
        }
    }

    #[test]
    fn test_bid_with_all_tracking_urls() {
        let bid = Bid::builder()
            .id("bid-1".to_string())
            .item("item1".to_string())
            .price(5.50)
            .nurl(Some(
                "https://win.example.com/?price=${AUCTION_PRICE}".to_string(),
            ))
            .burl(Some("https://bill.example.com/".to_string()))
            .lurl(Some(
                "https://loss.example.com/?reason=${AUCTION_LOSS}".to_string(),
            ))
            .adomain(Some(vec!["advertiser.com".to_string()]))
            .cat(Some(vec!["IAB1".to_string(), "IAB2-1".to_string()]))
            .build()
            .unwrap();

        let seatbid = SeatBid::builder().bid(vec![bid]).build().unwrap();

        let response = Response::builder()
            .id("resp-tracking".to_string())
            .seatbid(vec![seatbid])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: Response = serde_json::from_str(&json).unwrap();

        let bid = &parsed.seatbid[0].bid[0];
        assert!(bid.nurl.is_some());
        assert!(bid.burl.is_some());
        assert!(bid.lurl.is_some());
        assert!(bid.nurl.as_ref().unwrap().contains("${AUCTION_PRICE}"));
        assert!(bid.lurl.as_ref().unwrap().contains("${AUCTION_LOSS}"));
    }

    #[test]
    fn test_roundtrip_with_complex_structure() {
        let deal = Deal::builder()
            .id("deal-1".to_string())
            .flr(Some(10.00))
            .at(Some(3))
            .build()
            .unwrap();

        let metric = Metric::builder()
            .type_("viewability".to_string())
            .val(0.80)
            .build()
            .unwrap();

        let item = Item::builder()
            .id("item1".to_string())
            .qty(Some(1))
            .seq(Some(1))
            .flr(Some(2.50))
            .flrcur(Some("USD".to_string()))
            .deal(Some(vec![deal]))
            .metric(Some(vec![metric]))
            .build()
            .unwrap();

        let node = SupplyChainNode::builder()
            .asi("pub.com".to_string())
            .sid("seller1".to_string())
            .hp(Some(1))
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(1)
            .nodes(vec![node])
            .ver("1.0".to_string())
            .build()
            .unwrap();

        let source = Source::builder()
            .tid(Some("txn-roundtrip".to_string()))
            .ts(Some(1609459200000))
            .schain(Some(schain))
            .build()
            .unwrap();

        let request = Request::builder()
            .id("req-complex-roundtrip".to_string())
            .test(Some(0))
            .tmax(Some(120))
            .at(Some(2))
            .cur(Some(vec!["USD".to_string(), "EUR".to_string()]))
            .item(vec![item])
            .source(Some(source))
            .build()
            .unwrap();

        let original = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .request(Some(request))
            .response(None)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Openrtb = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ver, original.ver);
        assert_eq!(parsed.domainspec, original.domainspec);

        let orig_req = original.request.as_ref().unwrap();
        let parsed_req = parsed.request.as_ref().unwrap();

        assert_eq!(parsed_req.id, orig_req.id);
        assert_eq!(parsed_req.at, orig_req.at);
        assert_eq!(parsed_req.cur, orig_req.cur);
        assert_eq!(parsed_req.item.len(), orig_req.item.len());
        assert!(parsed_req.source.is_some());
        assert!(parsed_req.source.as_ref().unwrap().schain.is_some());
    }
}
