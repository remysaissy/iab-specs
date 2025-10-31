//! Supply Chain Object
//!
//! This object represents both a whole chain of transactions and individual nodes in that chain.
//! Used to support ads.txt and sellers.json transparency initiatives.
//!
//! Reference: <https://github.com/InteractiveAdvertisingBureau/openrtb/blob/master/supplychainobject.md>

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Supply chain object.
///
/// A supply chain node is a participant in the advertising supply chain, typically representing
/// an advertiser, publisher, or intermediary. The supply chain object is a complete chain
/// tracing the path from the publisher back to the advertiser.
///
/// ## Example
///
/// ```
/// use iab_specs::openrtb::common::{SupplyChain, SupplyChainNode};
/// use std::str::FromStr;
///
/// let json = r#"{
///     "complete": 1,
///     "nodes": [{
///         "asi": "example.com",
///         "sid": "12345",
///         "hp": 1
///     }],
///     "ver": "1.0"
/// }"#;
///
/// let schain = SupplyChain::from_str(json).unwrap();
/// assert_eq!(schain.complete, Some(1));
/// assert_eq!(schain.nodes.len(), 1);
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct SupplyChain {
    /// Flag indicating whether the chain contains all nodes involved in the transaction
    /// leading back to the owner of the site, app or other medium of the inventory.
    ///
    /// - 0 = no or unknown
    /// - 1 = yes
    ///
    /// Implementers SHOULD ensure that any value is accurate.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<u8>,

    /// Array of SupplyChainNode objects in the order of the chain. In a complete supply chain,
    /// the first node represents the initial advertising system and seller ID involved in the
    /// transaction, i.e. the owner of the site, app, or other medium.
    ///
    /// In a complete supply chain, the last node represents the entity sending this bid request.
    #[builder(setter(into))]
    pub nodes: Vec<SupplyChainNode>,

    /// Version of the supply chain specification in use, in the format of "major.minor".
    /// For example, for version 1.0 of the spec, use the string "1.0".
    #[builder(default = "Some(\"1.0\".to_string())")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// Placeholder for advertising-system specific extensions to this object.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

impl SupplyChain {
    pub fn builder() -> SupplyChainBuilder {
        SupplyChainBuilder::default()
    }
}

impl Display for SupplyChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "<Serialize error: {e}>"),
        }
    }
}

impl FromStr for SupplyChain {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<SupplyChain>(content).map_err(|e| e.into())
    }
}

/// Supply chain node.
///
/// This object represents a single node in a supply chain. Each node represents a participant
/// in the advertising supply chain, typically representing an intermediary or the original
/// publisher.
///
/// ## Example
///
/// ```
/// use iab_specs::openrtb::common::SupplyChainNode;
///
/// let node = SupplyChainNode::builder()
///     .asi("exchange.example.com".to_string())
///     .sid("pub-12345".to_string())
///     .hp(1)
///     .build()
///     .unwrap();
///
/// assert_eq!(node.asi, "exchange.example.com");
/// assert_eq!(node.sid, "pub-12345");
/// assert_eq!(node.hp, 1);
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct SupplyChainNode {
    /// The canonical domain name of the SSP, Exchange, Header Wrapper, etc system that bidders
    /// connect to. This may be the operational domain of the system, if that is different than
    /// the parent corporate domain, to facilitate WHOIS and reverse IP lookups to establish
    /// clear ownership of the delegate system.
    ///
    /// This should be the same value as used to identify sellers in an ads.txt file if one
    /// exists.
    #[builder(setter(into))]
    pub asi: String,

    /// The identifier associated with the seller or reseller account within the advertising
    /// system. This must contain the same value used in transactions (i.e. OpenRTB bid requests)
    /// in the publisher.id field or app.publisher.id field.
    ///
    /// This should be the same value as used to identify sellers in an ads.txt file if one exists.
    #[builder(setter(into))]
    pub sid: String,

    /// Indicates whether this node provides the inventory in the transaction.
    ///
    /// - 0 = no
    /// - 1 = yes
    ///
    /// In a complete supply chain, only one SupplyChainNode should have this value set to 1.
    /// In an incomplete supply chain, this value should be set to 1 on the node that is closest
    /// to the actual provider of inventory. For version 1.0 of SupplyChain, this should always
    /// default to 0.
    #[builder(default)]
    pub hp: u8,

    /// The OpenRTB RequestId of the request as issued by this seller.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,

    /// The name of the company (the legal entity) that is paid for inventory transacted
    /// under the given seller_id. This value is optional and should not be included if it
    /// exists in the advertising system's sellers.json file.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The business domain name of the entity represented by this node. This value is optional
    /// and should not be included if it exists in the advertising system's sellers.json file.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Placeholder for advertising-system specific extensions to this object.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

impl SupplyChainNode {
    pub fn builder() -> SupplyChainNodeBuilder {
        SupplyChainNodeBuilder::default()
    }
}

impl Display for SupplyChainNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "<Serialize error: {e}>"),
        }
    }
}

impl FromStr for SupplyChainNode {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<SupplyChainNode>(content).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_chain_minimal() {
        let json = r#"{
            "nodes": [{
                "asi": "example.com",
                "sid": "12345",
                "hp": 1
            }]
        }"#;

        let schain = SupplyChain::from_str(json);
        assert!(schain.is_ok());

        let schain = schain.unwrap();
        assert_eq!(schain.nodes.len(), 1);
        assert_eq!(schain.nodes[0].asi, "example.com");
    }

    #[test]
    fn test_supply_chain_complete() {
        let json = r#"{
            "complete": 1,
            "nodes": [
                {
                    "asi": "publisher.example.com",
                    "sid": "pub-001",
                    "hp": 1,
                    "name": "Publisher Example",
                    "domain": "publisher.com"
                },
                {
                    "asi": "exchange.example.com",
                    "sid": "exch-002",
                    "hp": 0,
                    "rid": "req-abc-123"
                }
            ],
            "ver": "1.0"
        }"#;

        let schain = SupplyChain::from_str(json);
        assert!(schain.is_ok());

        let schain = schain.unwrap();
        assert_eq!(schain.complete, Some(1));
        assert_eq!(schain.nodes.len(), 2);
        assert_eq!(schain.ver, Some("1.0".to_string()));

        assert_eq!(schain.nodes[0].hp, 1);
        assert_eq!(schain.nodes[1].hp, 0);
    }

    #[test]
    fn test_supply_chain_missing_required_field() {
        // Missing 'nodes' field
        let json = r#"{"complete": 1}"#;
        let result = SupplyChain::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_supply_chain_node_minimal() {
        let node = SupplyChainNode::builder()
            .asi("example.com".to_string())
            .sid("12345".to_string())
            .build();

        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(node.asi, "example.com");
        assert_eq!(node.sid, "12345");
        assert_eq!(node.hp, 0); // Default value
    }

    #[test]
    fn test_supply_chain_node_complete() {
        let node = SupplyChainNode::builder()
            .asi("exchange.example.com".to_string())
            .sid("pub-67890".to_string())
            .hp(1)
            .rid(Some("req-xyz-789".to_string()))
            .name(Some("Example Publisher".to_string()))
            .domain(Some("example.com".to_string()))
            .build();

        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(node.hp, 1);
        assert_eq!(node.rid, Some("req-xyz-789".to_string()));
        assert_eq!(node.name, Some("Example Publisher".to_string()));
    }

    #[test]
    fn test_supply_chain_node_serialization() {
        let node = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller-123".to_string())
            .hp(1)
            .build()
            .unwrap();

        let json = serde_json::to_string(&node).unwrap();
        let node2: SupplyChainNode = serde_json::from_str(&json).unwrap();

        assert_eq!(node, node2);
    }

    #[test]
    fn test_supply_chain_serialization() {
        let schain = SupplyChain::builder()
            .complete(Some(1))
            .nodes(vec![
                SupplyChainNode::builder()
                    .asi("example.com".to_string())
                    .sid("pub-001".to_string())
                    .hp(1)
                    .build()
                    .unwrap(),
            ])
            .ver(Some("1.0".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&schain).unwrap();
        let schain2: SupplyChain = serde_json::from_str(&json).unwrap();

        assert_eq!(schain, schain2);
    }

    #[test]
    fn test_supply_chain_builder() {
        let result = SupplyChain::builder().nodes(vec![]).build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_supply_chain_node_builder_missing_required() {
        // Missing asi
        let result = SupplyChainNode::builder().sid("12345".to_string()).build();
        assert!(result.is_err());

        // Missing sid
        let result = SupplyChainNode::builder()
            .asi("example.com".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_supply_chain_display() {
        let schain = SupplyChain::builder()
            .nodes(vec![
                SupplyChainNode::builder()
                    .asi("test.com".to_string())
                    .sid("123".to_string())
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let display_str = schain.to_string();
        assert!(display_str.contains("test.com"));
        assert!(display_str.contains("123"));
    }

    #[test]
    fn test_supply_chain_node_display() {
        let node = SupplyChainNode::builder()
            .asi("example.com".to_string())
            .sid("pub-456".to_string())
            .hp(1)
            .build()
            .unwrap();

        let display_str = node.to_string();
        assert!(display_str.contains("example.com"));
        assert!(display_str.contains("pub-456"));
    }

    #[test]
    fn test_supply_chain_with_extension() {
        let ext_value = serde_json::json!({"custom": "value"});

        let schain = SupplyChain::builder()
            .nodes(vec![])
            .ext(Some(ext_value.clone()))
            .build()
            .unwrap();

        assert_eq!(schain.ext, Some(ext_value));
    }

    #[test]
    fn test_supply_chain_node_with_extension() {
        let ext_value = serde_json::json!({"property": "data"});

        let node = SupplyChainNode::builder()
            .asi("test.com".to_string())
            .sid("seller-1".to_string())
            .ext(Some(ext_value.clone()))
            .build()
            .unwrap();

        assert_eq!(node.ext, Some(ext_value));
    }

    #[test]
    fn test_supply_chain_node_hp_values() {
        let json_hp_0 = r#"{"asi":"ex.com","sid":"123","hp":0}"#;
        let node0: SupplyChainNode = serde_json::from_str(json_hp_0).unwrap();
        assert_eq!(node0.hp, 0);

        let json_hp_1 = r#"{"asi":"ex.com","sid":"123","hp":1}"#;
        let node1: SupplyChainNode = serde_json::from_str(json_hp_1).unwrap();
        assert_eq!(node1.hp, 1);
    }

    #[test]
    fn test_supply_chain_complete_values() {
        let json_complete_0 = r#"{"nodes":[],"complete":0}"#;
        let sc0: SupplyChain = serde_json::from_str(json_complete_0).unwrap();
        assert_eq!(sc0.complete, Some(0));

        let json_complete_1 = r#"{"nodes":[],"complete":1}"#;
        let sc1: SupplyChain = serde_json::from_str(json_complete_1).unwrap();
        assert_eq!(sc1.complete, Some(1));
    }

    #[test]
    fn test_supply_chain_empty_nodes() {
        let schain = SupplyChain::builder().nodes(vec![]).build().unwrap();

        assert_eq!(schain.nodes.len(), 0);

        let json = serde_json::to_string(&schain).unwrap();
        assert!(json.contains("\"nodes\":[]"));
    }

    #[test]
    fn test_supply_chain_multiple_nodes() {
        let nodes = vec![
            SupplyChainNode::builder()
                .asi("pub.com".to_string())
                .sid("p1".to_string())
                .hp(1)
                .build()
                .unwrap(),
            SupplyChainNode::builder()
                .asi("ssp.com".to_string())
                .sid("s1".to_string())
                .hp(0)
                .build()
                .unwrap(),
            SupplyChainNode::builder()
                .asi("exch.com".to_string())
                .sid("e1".to_string())
                .hp(0)
                .build()
                .unwrap(),
        ];

        let schain = SupplyChain::builder()
            .complete(Some(1))
            .nodes(nodes)
            .build()
            .unwrap();

        assert_eq!(schain.nodes.len(), 3);
        assert_eq!(schain.nodes[0].hp, 1);
        assert_eq!(schain.nodes[1].hp, 0);
        assert_eq!(schain.nodes[2].hp, 0);
    }
}
