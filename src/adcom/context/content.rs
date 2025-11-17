use crate::Extension;
use crate::adcom::context::Producer;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Content Object
///
/// Details about the content within which an ad will be displayed.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Content<Ext: Extension = serde_json::Value> {
    /// Unique content identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Episode number for episodic content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,

    /// Content title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Content series
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// Content season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    /// Artist credited with the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// Genre(s) of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// Album to which the content belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// International Standard Recording Code (ISRC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    /// URL of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Content categories using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Production quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<i32>,

    /// Content context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,

    /// Content rating (e.g., MPAA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<String>,

    /// User rating of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userrating: Option<String>,

    /// Media rating per IQG guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// 1 = content is live, 0 = not live
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestream: Option<i32>,

    /// 1 = src relationship is direct, 0 = indirect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srcrel: Option<i32>,

    /// Length of content in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Content language using ISO-639-1-alpha-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// 1 = content is embedded, 0 = not embedded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<i32>,

    /// Producer details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Box<Producer>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Content {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ContentBuilder {
        ContentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_builder() {
        let content = Content::builder()
            .id(Some("content123".to_string()))
            .title(Some("Breaking News".to_string()))
            .url(Some("https://example.com/video".to_string()))
            .language(Some("en".to_string()))
            .len(Some(300))
            .build()
            .unwrap();

        assert_eq!(content.id, Some("content123".to_string()));
        assert_eq!(content.title, Some("Breaking News".to_string()));
        assert_eq!(content.url, Some("https://example.com/video".to_string()));
        assert_eq!(content.language, Some("en".to_string()));
        assert_eq!(content.len, Some(300));
    }

    #[test]
    fn test_content_default() {
        let content = Content::builder().build().unwrap();

        assert!(content.id.is_none());
        assert!(content.title.is_none());
        assert!(content.url.is_none());
        assert!(content.producer.is_none());
        assert!(content.cat.is_none());
    }

    #[test]
    fn test_content_with_producer() {
        let producer = Producer::builder()
            .id(Some("prod123".to_string()))
            .name(Some("ABC Studios".to_string()))
            .build()
            .unwrap();

        let content = Content::builder()
            .id(Some("content456".to_string()))
            .title(Some("Documentary".to_string()))
            .producer(Some(Box::new(producer)))
            .build()
            .unwrap();

        assert!(content.producer.is_some());
        assert_eq!(
            content.producer.as_ref().unwrap().id,
            Some("prod123".to_string())
        );
        assert_eq!(
            content.producer.as_ref().unwrap().name,
            Some("ABC Studios".to_string())
        );
    }

    #[test]
    fn test_content_serialization() {
        let content = Content::builder()
            .id(Some("content789".to_string()))
            .title(Some("Action Movie".to_string()))
            .genre(Some("Action".to_string()))
            .len(Some(7200))
            .livestream(Some(0))
            .build()
            .unwrap();

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"id\":\"content789\""));
        assert!(json.contains("\"title\":\"Action Movie\""));
        assert!(json.contains("\"genre\":\"Action\""));
        assert!(json.contains("\"len\":7200"));
    }

    #[test]
    fn test_content_deserialization() {
        let json = r#"{"id":"content999","title":"Sports Event","livestream":1,"len":10800}"#;
        let content: Content = serde_json::from_str(json).unwrap();

        assert_eq!(content.id, Some("content999".to_string()));
        assert_eq!(content.title, Some("Sports Event".to_string()));
        assert_eq!(content.livestream, Some(1));
        assert_eq!(content.len, Some(10800));
    }

    #[test]
    fn test_content_with_media_fields() {
        let content = Content::builder()
            .id(Some("content555".to_string()))
            .series(Some("Breaking Bad".to_string()))
            .season(Some("S05".to_string()))
            .episode(Some(16))
            .title(Some("Felina".to_string()))
            .artist(Some("Vince Gilligan".to_string()))
            .contentrating(Some("TV-MA".to_string()))
            .build()
            .unwrap();

        assert_eq!(content.series, Some("Breaking Bad".to_string()));
        assert_eq!(content.season, Some("S05".to_string()));
        assert_eq!(content.episode, Some(16));
        assert_eq!(content.title, Some("Felina".to_string()));
        assert_eq!(content.contentrating, Some("TV-MA".to_string()));
    }

    #[test]
    fn test_content_with_categories() {
        let content = Content::builder()
            .id(Some("content111".to_string()))
            .title(Some("Tech Tutorial".to_string()))
            .cat(Some(vec!["IAB19".to_string(), "IAB19-6".to_string()]))
            .cattax(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            content.cat,
            Some(vec!["IAB19".to_string(), "IAB19-6".to_string()])
        );
        assert_eq!(content.cattax, Some(1));
    }
}
