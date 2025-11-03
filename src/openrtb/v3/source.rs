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
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::SupplyChainNode;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let node = SupplyChainNode {
///     asi: "exchange.com".to_string(),
///     sid: "seller-123".to_string(),
///     hp: Some(1),
///     rid: Some("request-456".to_string()),
///     name: Some("Publisher Name".to_string()),
///     domain: Some("publisher.com".to_string()),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct SupplyChainNode {
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
    pub ext: Option<serde_json::Value>,
}

/// SupplyChain object (Supply Chain Object 1.0)
///
/// Represents the complete supply chain for an ad request, allowing each
/// participant to identify themselves and their relationship to the request.
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::{SupplyChain, SupplyChainNode};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let schain = SupplyChain {
///     complete: 1,
///     nodes: vec![
///         SupplyChainNode {
///             asi: "exchange.com".to_string(),
///             sid: "seller1".to_string(),
///             hp: Some(1),
///             ..Default::default()
///         },
///         SupplyChainNode {
///             asi: "intermediary.com".to_string(),
///             sid: "seller2".to_string(),
///             hp: Some(1),
///             ..Default::default()
///         },
///     ],
///     ver: "1.0".to_string(),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct SupplyChain {
    /// Indicates whether the chain is complete:
    /// - 0 = incomplete (more nodes exist)
    /// - 1 = complete (all nodes included)
    ///
    /// REQUIRED by the specification.
    pub complete: i32,

    /// Array of supply chain nodes in order from the originating entity
    /// to the final requestor.
    /// REQUIRED (at least one node must be present).
    pub nodes: Vec<SupplyChainNode>,

    /// Version of the supply chain specification.
    /// Current version is "1.0".
    /// REQUIRED by the specification.
    pub ver: String,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

/// Source object (OpenRTB 3.0 Section 3.2.4)
///
/// The `Source` object contains information about the inventory source and
/// the supply chain transparency information.
///
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::{Source, SupplyChain, SupplyChainNode};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let source = Source {
///     tid: Some("transaction-123".to_string()),
///     ts: Some(1609459200),
///     ds: Some("datasource-456".to_string()),
///     dsmap: Some("dsmap-789".to_string()),
///     cert: Some("cert-abc".to_string()),
///     schain: Some(SupplyChain {
///         complete: 1,
///         nodes: vec![
///             SupplyChainNode {
///                 asi: "exchange.com".to_string(),
///                 sid: "seller1".to_string(),
///                 hp: Some(1),
///                 ..Default::default()
///             },
///         ],
///         ver: "1.0".to_string(),
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Source {
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
    pub schain: Option<SupplyChain>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_chain_node_creation() {
        let node = SupplyChainNode {
            asi: "exchange.com".to_string(),
            sid: "seller123".to_string(),
            hp: Some(1),
            rid: Some("req456".to_string()),
            name: Some("Publisher".to_string()),
            domain: Some("publisher.com".to_string()),
            ..Default::default()
        };

        assert_eq!(node.asi, "exchange.com");
        assert_eq!(node.sid, "seller123");
        assert_eq!(node.hp, Some(1));
    }

    #[test]
    fn test_supply_chain_creation() {
        let schain = SupplyChain {
            complete: 1,
            nodes: vec![
                SupplyChainNode {
                    asi: "ssp.com".to_string(),
                    sid: "seller1".to_string(),
                    hp: Some(1),
                    ..Default::default()
                },
                SupplyChainNode {
                    asi: "exchange.com".to_string(),
                    sid: "seller2".to_string(),
                    hp: Some(1),
                    ..Default::default()
                },
            ],
            ver: "1.0".to_string(),
            ..Default::default()
        };

        assert_eq!(schain.complete, 1);
        assert_eq!(schain.nodes.len(), 2);
        assert_eq!(schain.ver, "1.0");
    }

    #[test]
    fn test_source_with_schain() {
        let source = Source {
            tid: Some("txn123".to_string()),
            ts: Some(1609459200000),
            schain: Some(SupplyChain {
                complete: 1,
                nodes: vec![SupplyChainNode {
                    asi: "exchange.com".to_string(),
                    sid: "seller1".to_string(),
                    hp: Some(1),
                    ..Default::default()
                }],
                ver: "1.0".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(source.tid, Some("txn123".to_string()));
        assert!(source.schain.is_some());
        assert_eq!(source.schain.as_ref().unwrap().nodes.len(), 1);
    }

    #[test]
    fn test_source_minimal() {
        let source = Source {
            tid: Some("txn456".to_string()),
            ..Default::default()
        };

        assert_eq!(source.tid, Some("txn456".to_string()));
        assert_eq!(source.schain, None);
    }

    #[test]
    fn test_supply_chain_serialization() {
        let schain = SupplyChain {
            complete: 1,
            nodes: vec![SupplyChainNode {
                asi: "exchange.com".to_string(),
                sid: "seller1".to_string(),
                hp: Some(1),
                ..Default::default()
            }],
            ver: "1.0".to_string(),
            ..Default::default()
        };

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
    fn test_source_builder() {
        let source = SourceBuilder::default()
            .tid(Some("txn123".to_string()))
            .ts(Some(1609459200000))
            .build()
            .unwrap();

        assert_eq!(source.tid, Some("txn123".to_string()));
        assert_eq!(source.ts, Some(1609459200000));
    }

    #[test]
    fn test_supply_chain_incomplete() {
        let schain = SupplyChain {
            complete: 0,
            nodes: vec![SupplyChainNode {
                asi: "exchange.com".to_string(),
                sid: "seller1".to_string(),
                ..Default::default()
            }],
            ver: "1.0".to_string(),
            ..Default::default()
        };

        assert_eq!(schain.complete, 0);
    }

    #[test]
    fn test_supply_chain_node_payment_flag() {
        let node_in_payment = SupplyChainNode {
            asi: "exchange.com".to_string(),
            sid: "seller1".to_string(),
            hp: Some(1),
            ..Default::default()
        };

        let node_not_in_payment = SupplyChainNode {
            asi: "exchange.com".to_string(),
            sid: "seller2".to_string(),
            hp: Some(0),
            ..Default::default()
        };

        assert_eq!(node_in_payment.hp, Some(1));
        assert_eq!(node_not_in_payment.hp, Some(0));
    }

    #[test]
    fn test_source_with_cert() {
        let source = Source {
            tid: Some("txn789".to_string()),
            cert: Some("cert-abc123".to_string()),
            ..Default::default()
        };

        assert_eq!(source.cert, Some("cert-abc123".to_string()));
    }
}
