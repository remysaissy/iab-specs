//! Supply Chain Object
//!
//! This object represents both a whole chain of transactions and individual nodes in that chain.
//! Used to support ads.txt and sellers.json transparency initiatives.
//!
//! Reference: <https://github.com/InteractiveAdvertisingBureau/openrtb/blob/master/supplychainobject.md>

use crate::Extension;
use crate::openrtb::SupplyChainNode;
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
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// ## Example
///
/// ```
/// use iab_specs::openrtb::{SupplyChain, SupplyChainNode};
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
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SupplyChain<Ext: Extension = serde_json::Value> {
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
    pub ext: Option<Box<Ext>>,
}

impl SupplyChain {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SupplyChainBuilder {
        SupplyChainBuilder::create_empty()
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
    fn test_supply_chain_with_extension() {
        let ext_value = Box::new(serde_json::json!({"custom": "value"}));

        let schain = SupplyChain::builder()
            .nodes(vec![])
            .ext(Some(ext_value.clone()))
            .build()
            .unwrap();

        assert_eq!(schain.ext, Some(ext_value));
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
