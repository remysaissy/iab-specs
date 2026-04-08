use crate::Extension;
use crate::openrtb::common::SupplyChain;
/// OpenRTB 2.5 Source Object
///
/// This module implements the Source object for inventory source transparency.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Source object for inventory source transparency (OpenRTB 2.5 Section 3.2.2)
///
/// A `Source` object describes the nature and behavior of the entity that is the
/// source of the bid request upstream from the exchange. The primary purpose of this
/// object is to define post-auction or upstream decisioning when the exchange itself
/// does not control the final decision.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Source<Ext: Extension = crate::DefaultExt> {
    /// Entity responsible for the final impression sale decision:
    /// - 0 = exchange
    /// - 1 = upstream source
    ///
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fd: Option<i32>,

    /// Transaction ID that must be common across all participants in this bid request
    /// (e.g., potentially multiple exchanges).
    ///
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tid: Option<String>,

    /// Payment ID chain string containing embedded syntax described in the
    /// TAG Payment ID Protocol v1.0.
    ///
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pchain: Option<String>,

    /// Supply chain object for ads.txt and sellers.json transparency.
    /// Provides a complete or partial supply chain of nodes involved in the
    /// transaction leading back to the owner of the site, app, or other medium.
    ///
    /// See SupplyChain object specification for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub schain: Option<SupplyChain>,

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
    use crate::openrtb::common::SupplyChainNode;

    #[test]
    fn test_source_creation() {
        let source = Source::builder()
            .fd(Some(1))
            .tid(Some("transaction123".to_string()))
            .pchain(Some("payment_chain".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.fd, Some(1));
        assert_eq!(source.tid, Some("transaction123".to_string()));
        assert_eq!(source.pchain, Some("payment_chain".to_string()));
    }

    #[test]
    fn test_source_with_supply_chain() {
        let node = SupplyChainNode::builder()
            .asi("example.com".to_string())
            .sid("12345".to_string())
            .hp(1)
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(Some(1))
            .nodes(vec![node])
            .ver(Some("1.0".to_string()))
            .build()
            .unwrap();

        let source = Source::builder()
            .fd(Some(0))
            .tid(Some("trans456".to_string()))
            .schain(Some(schain))
            .build()
            .unwrap();

        assert_eq!(source.fd, Some(0));
        assert!(source.schain.is_some());
        assert_eq!(source.schain.as_ref().unwrap().nodes.len(), 1);
        assert_eq!(source.schain.as_ref().unwrap().nodes[0].asi, "example.com");
    }

    #[test]
    fn test_source_exchange_decision() {
        let source = Source::builder()
            .fd(Some(0)) // Exchange makes final decision
            .tid(Some("trans789".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.fd, Some(0));
    }

    #[test]
    fn test_source_upstream_decision() {
        let source = Source::builder()
            .fd(Some(1)) // Upstream source makes final decision
            .tid(Some("trans101".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.fd, Some(1));
    }

    #[test]
    fn test_source_serialization() {
        let source = Source::builder()
            .fd(Some(1))
            .tid(Some("transaction123".to_string()))
            .pchain(Some("payment_chain".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"fd\":1"));
        assert!(json.contains("\"tid\":\"transaction123\""));
        assert!(json.contains("\"pchain\":\"payment_chain\""));
    }

    #[test]
    fn test_source_deserialization() {
        let json = r#"{"fd":0,"tid":"trans456"}"#;
        let source: Source = serde_json::from_str(json).unwrap();

        assert_eq!(source.fd, Some(0));
        assert_eq!(source.tid, Some("trans456".to_string()));
    }

    #[test]
    fn test_source_empty() {
        let source = Source::builder().build().unwrap();

        assert_eq!(source.fd, None);
        assert_eq!(source.tid, None);
        assert_eq!(source.pchain, None);
        assert_eq!(source.schain, None);
    }

    #[test]
    fn test_source_tid_field() {
        // Spec: Section 3.2.2
        let source = Source::builder()
            .tid(Some("txn-abc-123".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.tid, Some("txn-abc-123".to_string()));
    }

    #[test]
    fn test_source_pchain_field() {
        // Spec: Section 3.2.2
        let source = Source::builder()
            .pchain(Some("tag1:abc/tag2:def".to_string()))
            .build()
            .unwrap();

        assert_eq!(source.pchain, Some("tag1:abc/tag2:def".to_string()));
    }

    #[test]
    fn test_source_ext_with_serde_json_value() {
        // Spec: Section 3.2.2
        let ext = serde_json::json!({"omidpn": "Example OM SDK", "omidpv": "1.0"});

        let source = SourceBuilder::<serde_json::Value>::default()
            .fd(Some(1))
            .ext(Some(Box::new(ext)))
            .build()
            .unwrap();

        assert!(source.ext.is_some());
        assert_eq!(source.ext.as_ref().unwrap()["omidpn"], "Example OM SDK");
    }

    #[test]
    fn test_source_serde_roundtrip_all_fields() {
        // Spec: Section 3.2.2
        let node = SupplyChainNode::builder()
            .asi("exchange.com".to_string())
            .sid("pub-999".to_string())
            .hp(1)
            .build()
            .unwrap();

        let schain = SupplyChain::builder()
            .complete(Some(1))
            .ver(Some("1.0".to_string()))
            .nodes(vec![node])
            .build()
            .unwrap();

        let source = SourceBuilder::<serde_json::Value>::default()
            .fd(Some(1))
            .tid(Some("txn-roundtrip".to_string()))
            .pchain(Some("chain:value".to_string()))
            .schain(Some(schain))
            .ext(Some(Box::new(serde_json::json!({"key": "val"}))))
            .build()
            .unwrap();

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: Source<serde_json::Value> = serde_json::from_str(&json).unwrap();

        assert_eq!(source.fd, deserialized.fd);
        assert_eq!(source.tid, deserialized.tid);
        assert_eq!(source.pchain, deserialized.pchain);
        assert!(deserialized.schain.is_some());
        assert_eq!(
            source.schain.as_ref().unwrap().nodes[0].asi,
            deserialized.schain.as_ref().unwrap().nodes[0].asi
        );
        assert_eq!(source.ext, deserialized.ext);
    }
}
