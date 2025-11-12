use crate::Extension;
/// OpenRTB 3.0 Source Object
///
/// This module implements the Source object for supply chain transparency.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// SupplyChainNode object (Supply Chain Object 1.0)
///
/// Represents a single node in the supply chain for an ad request.
/// Each node identifies a participant in the transaction and their relationship.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SupplyChainNode<Ext: Extension = serde_json::Value> {
    /// Advertising system identifier (domain of the system).
    /// REQUIRED by the specification.
    pub asi: String,

    /// Seller ID within the advertising system.
    /// REQUIRED by the specification.
    pub sid: String,

    /// Indicates whether this node is involved in payment flow:
    /// - 0 = not involved in payment
    /// - 1 = involved in payment
    ///
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hp: Option<i32>,

    /// Request ID provided by this node for tracing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rid: Option<String>,

    /// Business name of the entity represented by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// Domain of the entity represented by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<String>,

    /// Extension object for system-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SupplyChainNode {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SupplyChainNodeBuilder {
        SupplyChainNodeBuilder::create_empty()
    }
}

/// SupplyChain object (Supply Chain Object 1.0)
///
/// Represents the complete supply chain for an ad request, allowing each
/// participant to identify themselves and their relationship to the request.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SupplyChain<Ext: Extension = serde_json::Value> {
    /// Indicates whether the chain is complete:
    /// - 0 = incomplete (more nodes exist)
    /// - 1 = complete (all nodes included)
    ///
    /// REQUIRED by the specification.
    pub complete: i32,

    /// Array of supply chain nodes in order from the originating entity
    /// to the final requestor.
    /// REQUIRED (at least one node must be present).
    pub nodes: Vec<SupplyChainNode<Ext>>,

    /// Version of the supply chain specification.
    /// Current version is "1.0".
    /// REQUIRED by the specification.
    pub ver: String,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SupplyChain {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SupplyChainBuilder {
        SupplyChainBuilder::create_empty()
    }
}

/// Source object (OpenRTB 3.0 Section 3.2.4)
///
/// The `Source` object contains information about the inventory source and
/// the supply chain transparency information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Source<Ext: Extension = serde_json::Value> {
    /// Transaction ID that must be common across all participants.
    /// Used for reconciliation and debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tid: Option<String>,

    /// Timestamp when the request originated at the beginning of the supply chain.
    /// Expressed as Unix epoch time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ts: Option<i64>,

    /// Identifier for the data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ds: Option<String>,

    /// Data source mapping identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dsmap: Option<String>,

    /// Certificate or identifier for chain of custody.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cert: Option<String>,

    /// Supply chain object providing transparency into the path from originator to requestor.
    /// Recommended by the specification for transparency.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub schain: Option<SupplyChain<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Source {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SourceBuilder {
        SourceBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_chain_node_creation() {
        let node = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller123".to_string())
            .hp(Some(1))
            .rid(Some("req456".to_string()))
            .name(Some("Publisher".to_string()))
            .domain(Some("publisher.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(node.asi, "exchange.com");
        assert_eq!(node.sid, "seller123");
        assert_eq!(node.hp, Some(1));
    }

    #[test]
    fn test_supply_chain_creation() {
        let node1 = SupplyChainNode::builder()
            .asi("ssp.com".to_string())
            .sid("seller1".to_string())
            .hp(Some(1))
            .build()
            .unwrap();

        let node2 = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller2".to_string())
            .hp(Some(1))
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(1)
            .nodes(vec![node1, node2])
            .ver("1.0".to_string())
            .build()
            .unwrap();

        assert_eq!(schain.complete, 1);
        assert_eq!(schain.nodes.len(), 2);
        assert_eq!(schain.ver, "1.0");
    }

    #[test]
    fn test_source_with_schain() {
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
            .tid(Some("txn123".to_string()))
            .ts(Some(1609459200000))
            .schain(Some(schain))
            .build()
            .unwrap();

        assert_eq!(source.tid, Some("txn123".to_string()));
        assert!(source.schain.is_some());
        assert_eq!(source.schain.as_ref().unwrap().nodes.len(), 1);
    }

    #[test]
    fn test_source_minimal() {
        let source = Source::builder()
            .tid(Some("txn456".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.tid, Some("txn456".to_string()));
        assert_eq!(source.schain, None);
    }

    #[test]
    fn test_supply_chain_serialization() {
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

        let json = serde_json::to_string(&schain).unwrap();
        assert!(json.contains("\"complete\":1"));
        assert!(json.contains("\"ver\":\"1.0\""));
        assert!(json.contains("\"asi\":\"exchange.com\""));
    }

    #[test]
    fn test_supply_chain_deserialization() {
        let json = r#"{
            "complete": 1,
            "nodes": [
                {
                    "asi": "exchange.com",
                    "sid": "seller1",
                    "hp": 1
                }
            ],
            "ver": "1.0"
        }"#;

        let schain: SupplyChain = serde_json::from_str(json).unwrap();
        assert_eq!(schain.complete, 1);
        assert_eq!(schain.nodes.len(), 1);
        assert_eq!(schain.ver, "1.0");
    }

    #[test]
    fn test_supply_chain_incomplete() {
        let node = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller1".to_string())
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(0)
            .nodes(vec![node])
            .ver("1.0".to_string())
            .build()
            .unwrap();

        assert_eq!(schain.complete, 0);
    }

    #[test]
    fn test_supply_chain_node_payment_flag() {
        let node_in_payment = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller1".to_string())
            .hp(Some(1))
            .build()
            .unwrap();

        let node_not_in_payment = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("seller2".to_string())
            .hp(Some(0))
            .build()
            .unwrap();

        assert_eq!(node_in_payment.hp, Some(1));
        assert_eq!(node_not_in_payment.hp, Some(0));
    }

    #[test]
    fn test_source_with_cert() {
        let source = Source::builder()
            .tid(Some("txn789".to_string()))
            .cert(Some("cert-abc123".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.cert, Some("cert-abc123".to_string()));
    }
}
