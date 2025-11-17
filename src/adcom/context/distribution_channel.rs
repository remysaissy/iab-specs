use crate::Extension;
use crate::adcom::context::{Content, Publisher};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Distribution Channel abstract base
///
/// Abstract base for distribution channel types (Site, App, DOOH).
/// This is the parent object for describing the properties of the medium
/// through which advertising is being offered.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DistributionChannel<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique identifier of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Displayable name of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the distribution channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DistributionChannel {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DistributionChannelBuilder {
        DistributionChannelBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribution_channel_builder() {
        let dc = DistributionChannel::builder()
            .id(Some("dc123".to_string()))
            .name(Some("Premium Content Network".to_string()))
            .build()
            .unwrap();

        assert_eq!(dc.id, Some("dc123".to_string()));
        assert_eq!(dc.name, Some("Premium Content Network".to_string()));
    }

    #[test]
    fn test_distribution_channel_default() {
        let dc = DistributionChannel::builder().build().unwrap();

        assert!(dc.id.is_none());
        assert!(dc.name.is_none());
        assert!(dc.pub_.is_none());
        assert!(dc.content.is_none());
    }

    #[test]
    fn test_distribution_channel_with_publisher() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Media Company".to_string()))
            .domain(Some("mediacompany.com".to_string()))
            .build()
            .unwrap();

        let dc = DistributionChannel::builder()
            .id(Some("dc456".to_string()))
            .pub_(Some(Box::new(publisher)))
            .build()
            .unwrap();

        assert!(dc.pub_.is_some());
        assert_eq!(dc.pub_.as_ref().unwrap().id, Some("pub123".to_string()));
        assert_eq!(
            dc.pub_.as_ref().unwrap().name,
            Some("Media Company".to_string())
        );
    }

    #[test]
    fn test_distribution_channel_with_content() {
        let content = Content::builder()
            .id(Some("content123".to_string()))
            .title(Some("Live Stream".to_string()))
            .livestream(Some(1))
            .build()
            .unwrap();

        let dc = DistributionChannel::builder()
            .id(Some("dc789".to_string()))
            .content(Some(Box::new(content)))
            .build()
            .unwrap();

        assert!(dc.content.is_some());
        assert_eq!(
            dc.content.as_ref().unwrap().id,
            Some("content123".to_string())
        );
        assert_eq!(dc.content.as_ref().unwrap().livestream, Some(1));
    }

    #[test]
    fn test_distribution_channel_serialization() {
        let dc = DistributionChannel::builder()
            .id(Some("dc999".to_string()))
            .name(Some("Video Platform".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&dc).unwrap();
        assert!(json.contains("\"id\":\"dc999\""));
        assert!(json.contains("\"name\":\"Video Platform\""));
    }

    #[test]
    fn test_distribution_channel_deserialization() {
        let json = r#"{"id":"dc111","name":"Streaming Service"}"#;
        let dc: DistributionChannel = serde_json::from_str(json).unwrap();

        assert_eq!(dc.id, Some("dc111".to_string()));
        assert_eq!(dc.name, Some("Streaming Service".to_string()));
    }
}
