use super::Data;
use super::producer::Producer;
use crate::Extension;
/// OpenRTB 2.5 Content Object
///
/// This module implements the Content object for content metadata.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Default category taxonomy (1 = IAB Content Category Taxonomy 1.0)
fn default_cattax() -> i32 {
    1
}

/// Default genre taxonomy (9 = User-defined genre taxonomy)
fn default_gtax() -> i32 {
    9
}

/// Content object describing content context (OpenRTB 2.5 Section 3.2.16)
///
/// A `Content` object describes the content in which the ad will appear.
/// This is particularly useful for video and audio, as well as for syndicated content.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `NetworkExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `ChannelExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, NetworkExt: Extension, ChannelExt: Extension",
    deserialize = "Ext: Extension, NetworkExt: Extension, ChannelExt: Extension"
))]
pub struct Content<
    Ext: Extension = serde_json::Value,
    NetworkExt: Extension = serde_json::Value,
    ChannelExt: Extension = serde_json::Value,
> {
    /// ID uniquely identifying the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Episode number.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub episode: Option<i32>,

    /// Content title.
    /// Video Examples: "Search Committee" (television), "A New Hope" (movie).
    /// Non-Video Example: "Why an Antarctic Glacier Is Melting So Quickly" (Time magazine article title).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<String>,

    /// Content series.
    /// Video Examples: "The Office" (television), "Star Wars" (movie).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub series: Option<String>,

    /// Content season (e.g., "Season 3").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub season: Option<String>,

    /// Artist credited with the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub artist: Option<String>,

    /// Genre that best describes the content (e.g., rock, pop, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub genre: Option<String>,

    /// Genre taxonomy.
    /// Default is 9 (user-defined genre taxonomy).
    /// Refer to AdCOM `GenreTaxonomy` enumeration.
    #[serde(default = "default_gtax")]
    #[builder(default = "default_gtax()")]
    pub gtax: i32,

    /// Array of genre IDs corresponding to the genre taxonomy.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub genres: Option<Vec<String>>,

    /// Album to which the content belongs; typically for audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub album: Option<String>,

    /// International Standard Recording Code conforming to ISO-3901.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub isrc: Option<String>,

    /// Details about the content Producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub producer: Option<Producer>,

    /// URL of the content, for buy-side contextualization or review.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub url: Option<String>,

    /// The taxonomy in use for cat attribute.
    /// Default is 1 (IAB Content Category Taxonomy 1.0).
    /// Refer to AdCOM `CategoryTaxonomy` enumeration.
    #[serde(default = "default_cattax")]
    #[builder(default = "default_cattax()")]
    pub cattax: i32,

    /// Array of IAB content categories that describe the content.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Production quality.
    /// Refer to AdCOM `ProductionQuality` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub prodq: Option<i32>,

    /// Type of content (game, video, text, etc.).
    /// Refer to AdCOM `ContentContext` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub context: Option<i32>,

    /// Content rating (e.g., MPAA).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub contentrating: Option<String>,

    /// User rating of the content (e.g., number of stars, likes, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub userrating: Option<String>,

    /// Media rating per IQG guidelines.
    /// Refer to AdCOM `QagMediaRating` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub qagmediarating: Option<i32>,

    /// Comma-separated list of keywords describing the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub keywords: Option<String>,

    /// Array of keywords describing the content.
    /// Mutually exclusive with `keywords` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub kwarray: Option<Vec<String>>,

    /// Indicates whether the content is live:
    /// - 0 = not live
    /// - 1 = content is live (e.g., stream, live blog)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub livestream: Option<i32>,

    /// Source relationship:
    /// - 0 = indirect
    /// - 1 = direct
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sourcerelationship: Option<i32>,

    /// Length of content in seconds; appropriate for video or audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub len: Option<i32>,

    /// Content language using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub language: Option<String>,

    /// Content language using IETF BCP 47.
    /// OpenRTB 2.6+ field for more detailed language specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub langb: Option<String>,

    /// Indicator of whether the content is embeddable (e.g., an embeddable video player):
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub embeddable: Option<i32>,

    /// Additional content data. Each Data object represents a different data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<Vec<Data>>,

    /// Network object representing the content network.
    /// Uses placeholder until Network is implemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub network: Option<Box<NetworkExt>>,

    /// Channel object representing the content channel.
    /// Uses placeholder until Channel is implemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub channel: Option<Box<ChannelExt>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Content {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ContentBuilder {
        ContentBuilder::create_empty()
    }
}

impl<Ext: Extension, NetworkExt: Extension, ChannelExt: Extension> Default
    for Content<Ext, NetworkExt, ChannelExt>
{
    fn default() -> Self {
        Self {
            id: None,
            episode: None,
            title: None,
            series: None,
            season: None,
            artist: None,
            genre: None,
            gtax: default_gtax(),
            genres: None,
            album: None,
            isrc: None,
            producer: None,
            url: None,
            cattax: default_cattax(),
            cat: None,
            prodq: None,
            context: None,
            contentrating: None,
            userrating: None,
            qagmediarating: None,
            keywords: None,
            kwarray: None,
            livestream: None,
            sourcerelationship: None,
            len: None,
            language: None,
            langb: None,
            embeddable: None,
            data: None,
            network: None,
            channel: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_creation() {
        let content = Content::builder()
            .id(Some("content123".to_string()))
            .title(Some("Great Movie".to_string()))
            .series(Some("Movie Series".to_string()))
            .url(Some("https://example.com/content".to_string()))
            .build()
            .unwrap();

        assert_eq!(content.id, Some("content123".to_string()));
        assert_eq!(content.title, Some("Great Movie".to_string()));
        assert_eq!(content.series, Some("Movie Series".to_string()));
        assert_eq!(content.cattax, 1); // Default value
        assert_eq!(content.gtax, 9); // Default value
    }

    #[test]
    fn test_content_with_producer() {
        let producer = Producer::builder()
            .id(Some("prod123".to_string()))
            .name(Some("Warner Bros".to_string()))
            .build()
            .unwrap();

        let content = Content::builder()
            .id(Some("content456".to_string()))
            .producer(Some(producer))
            .build()
            .unwrap();

        assert!(content.producer.is_some());
        assert_eq!(
            content.producer.as_ref().unwrap().id,
            Some("prod123".to_string())
        );
    }

    #[test]
    fn test_content_serialization() {
        let content = Content::builder()
            .id(Some("content123".to_string()))
            .title(Some("Great Movie".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"id\":\"content123\""));
        assert!(json.contains("\"title\":\"Great Movie\""));
    }

    #[test]
    fn test_content_deserialization() {
        let json = r#"{"id":"content123","title":"Great Movie"}"#;
        let content: Content = serde_json::from_str(json).unwrap();

        assert_eq!(content.id, Some("content123".to_string()));
        assert_eq!(content.title, Some("Great Movie".to_string()));
    }
}
