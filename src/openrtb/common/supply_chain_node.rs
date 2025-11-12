//! Supply Chain Object
//!
//! This object represents both a whole chain of transactions and individual nodes in that chain.
//! Used to support ads.txt and sellers.json transparency initiatives.
//!
//! Reference: <https://github.com/InteractiveAdvertisingBureau/openrtb/blob/master/supplychainobject.md>

use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Supply chain node.
///
/// This object represents a single node in a supply chain. Each node represents a participant
/// in the advertising supply chain, typically representing an intermediary or the original
/// publisher.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// ## Example
///
/// ```
/// use iab_specs::openrtb::SupplyChainNode;
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
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SupplyChainNode<Ext: Extension = serde_json::Value> {
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
    pub ext: Option<Box<Ext>>,
}

impl SupplyChainNode {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SupplyChainNodeBuilder {
        SupplyChainNodeBuilder::create_empty()
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
    fn test_supply_chain_node_with_extension() {
        let ext_value = Box::new(serde_json::json!({"property": "data"}));

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
}
