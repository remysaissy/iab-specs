use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Network Object (Section 7.11)
///
/// Details about the distribution network.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Network<Ext: Extension = serde_json::Value> {
    /// Network identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Network domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Network {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NetworkBuilder {
        NetworkBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_builder() {
        let network = Network::builder()
            .id(Some("net123".to_string()))
            .name(Some("Premium Ad Network".to_string()))
            .domain(Some("adnetwork.example.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(network.id, Some("net123".to_string()));
        assert_eq!(network.name, Some("Premium Ad Network".to_string()));
        assert_eq!(network.domain, Some("adnetwork.example.com".to_string()));
    }

    #[test]
    fn test_network_default() {
        let network = Network::builder().build().unwrap();

        assert!(network.id.is_none());
        assert!(network.name.is_none());
        assert!(network.domain.is_none());
    }

    #[test]
    fn test_network_serialization() {
        let network = Network::builder()
            .id(Some("net456".to_string()))
            .name(Some("Video Ad Network".to_string()))
            .domain(Some("videoadnetwork.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&network).unwrap();
        assert!(json.contains("\"id\":\"net456\""));
        assert!(json.contains("\"name\":\"Video Ad Network\""));
        assert!(json.contains("\"domain\":\"videoadnetwork.com\""));
    }

    #[test]
    fn test_network_deserialization() {
        let json = r#"{"id":"net789","name":"Display Network","domain":"display.example.com"}"#;
        let network: Network = serde_json::from_str(json).unwrap();

        assert_eq!(network.id, Some("net789".to_string()));
        assert_eq!(network.name, Some("Display Network".to_string()));
        assert_eq!(network.domain, Some("display.example.com".to_string()));
    }
}
