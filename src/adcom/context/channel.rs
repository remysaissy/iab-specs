use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Channel Object (Section 7.12)
///
/// Details about the distribution channel.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Channel<Ext: Extension = serde_json::Value> {
    /// Channel identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Channel domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Channel {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ChannelBuilder {
        ChannelBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_builder() {
        let channel = Channel::builder()
            .id(Some("ch123".to_string()))
            .name(Some("Premium Video Channel".to_string()))
            .domain(Some("video.example.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(channel.id, Some("ch123".to_string()));
        assert_eq!(channel.name, Some("Premium Video Channel".to_string()));
        assert_eq!(channel.domain, Some("video.example.com".to_string()));
    }

    #[test]
    fn test_channel_default() {
        let channel = Channel::builder().build().unwrap();

        assert!(channel.id.is_none());
        assert!(channel.name.is_none());
        assert!(channel.domain.is_none());
    }

    #[test]
    fn test_channel_serialization() {
        let channel = Channel::builder()
            .id(Some("ch456".to_string()))
            .name(Some("News Channel".to_string()))
            .domain(Some("news.example.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&channel).unwrap();
        assert!(json.contains("\"id\":\"ch456\""));
        assert!(json.contains("\"name\":\"News Channel\""));
        assert!(json.contains("\"domain\":\"news.example.com\""));
    }

    #[test]
    fn test_channel_deserialization() {
        let json = r#"{"id":"ch789","name":"Sports Channel","domain":"sports.example.com"}"#;
        let channel: Channel = serde_json::from_str(json).unwrap();

        assert_eq!(channel.id, Some("ch789".to_string()));
        assert_eq!(channel.name, Some("Sports Channel".to_string()));
        assert_eq!(channel.domain, Some("sports.example.com".to_string()));
    }
}
