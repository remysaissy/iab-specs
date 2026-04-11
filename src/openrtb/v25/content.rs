use super::producer::Producer;
use super::Data;
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
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `NetworkExt` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `ChannelExt` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, NetworkExt: Extension, ChannelExt: Extension",
    deserialize = "Ext: Extension, NetworkExt: Extension, ChannelExt: Extension"
))]
pub struct Content<
    Ext: Extension = crate::DefaultExt,
    NetworkExt: Extension = crate::DefaultExt,
    ChannelExt: Extension = crate::DefaultExt,
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

    #[test]
    fn test_content_media_fields() {
        // Spec: Section 3.2.16
        let content = Content::builder()
            .episode(Some(5))
            .title(Some("Search Committee".to_string()))
            .series(Some("The Office".to_string()))
            .season(Some("Season 3".to_string()))
            .build()
            .unwrap();

        assert_eq!(content.episode, Some(5));
        assert_eq!(content.title, Some("Search Committee".to_string()));
        assert_eq!(content.series, Some("The Office".to_string()));
        assert_eq!(content.season, Some("Season 3".to_string()));
    }

    #[test]
    fn test_content_category_fields() {
        // Spec: Section 3.2.16
        let content = Content::builder()
            .cat(Some(vec!["IAB1".to_string(), "IAB1-1".to_string()]))
            .genre(Some("drama".to_string()))
            .language(Some("en".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            content.cat,
            Some(vec!["IAB1".to_string(), "IAB1-1".to_string()])
        );
        assert_eq!(content.genre, Some("drama".to_string()));
        assert_eq!(content.language, Some("en".to_string()));
    }

    #[test]
    fn test_content_livestream_flag() {
        // Spec: Section 3.2.16
        let live = Content::builder().livestream(Some(1)).build().unwrap();
        assert_eq!(live.livestream, Some(1));

        let not_live = Content::builder().livestream(Some(0)).build().unwrap();
        assert_eq!(not_live.livestream, Some(0));

        let default = Content::builder().build().unwrap();
        assert_eq!(default.livestream, None);
    }

    #[test]
    fn test_content_len_field() {
        // Spec: Section 3.2.16
        let content = Content::builder().len(Some(1800)).build().unwrap();

        assert_eq!(content.len, Some(1800));
    }

    #[test]
    fn test_content_ext_field() {
        // Spec: Section 3.2.16
        let content = ContentBuilder::<serde_json::Value>::default()
            .id(Some("content-ext".to_string()))
            .ext(Some(Box::new(serde_json::json!({
                "content_rating_source": "MPAA"
            }))))
            .build()
            .unwrap();

        assert!(content.ext.is_some());
        assert_eq!(
            content.ext.as_ref().unwrap()["content_rating_source"],
            "MPAA"
        );
    }

    #[test]
    fn test_content_roundtrip_all_fields() {
        // Spec: Section 3.2.16
        let producer = Producer::builder()
            .id(Some("prod-1".to_string()))
            .name(Some("Studio".to_string()))
            .build()
            .unwrap();

        let content = Content::builder()
            .id(Some("content-all".to_string()))
            .episode(Some(10))
            .title(Some("Finale".to_string()))
            .series(Some("Breaking Bad".to_string()))
            .season(Some("Season 5".to_string()))
            .artist(Some("Vince Gilligan".to_string()))
            .genre(Some("drama".to_string()))
            .gtax(9)
            .album(Some("Soundtrack".to_string()))
            .isrc(Some("USRC17607839".to_string()))
            .producer(Some(producer))
            .url(Some("https://example.com/content".to_string()))
            .cattax(1)
            .cat(Some(vec!["IAB1".to_string()]))
            .prodq(Some(2))
            .context(Some(1))
            .contentrating(Some("TV-MA".to_string()))
            .userrating(Some("4.5".to_string()))
            .qagmediarating(Some(2))
            .keywords(Some("drama,thriller".to_string()))
            .livestream(Some(0))
            .sourcerelationship(Some(1))
            .len(Some(3600))
            .language(Some("en".to_string()))
            .langb(Some("en-US".to_string()))
            .embeddable(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&content).unwrap();
        let deserialized: Content = serde_json::from_str(&json).unwrap();

        assert_eq!(content.id, deserialized.id);
        assert_eq!(content.episode, deserialized.episode);
        assert_eq!(content.title, deserialized.title);
        assert_eq!(content.series, deserialized.series);
        assert_eq!(content.season, deserialized.season);
        assert_eq!(content.artist, deserialized.artist);
        assert_eq!(content.genre, deserialized.genre);
        assert_eq!(content.gtax, deserialized.gtax);
        assert_eq!(content.album, deserialized.album);
        assert_eq!(content.isrc, deserialized.isrc);
        assert_eq!(content.producer, deserialized.producer);
        assert_eq!(content.url, deserialized.url);
        assert_eq!(content.cattax, deserialized.cattax);
        assert_eq!(content.cat, deserialized.cat);
        assert_eq!(content.prodq, deserialized.prodq);
        assert_eq!(content.context, deserialized.context);
        assert_eq!(content.contentrating, deserialized.contentrating);
        assert_eq!(content.userrating, deserialized.userrating);
        assert_eq!(content.qagmediarating, deserialized.qagmediarating);
        assert_eq!(content.keywords, deserialized.keywords);
        assert_eq!(content.livestream, deserialized.livestream);
        assert_eq!(content.sourcerelationship, deserialized.sourcerelationship);
        assert_eq!(content.len, deserialized.len);
        assert_eq!(content.language, deserialized.language);
        assert_eq!(content.langb, deserialized.langb);
        assert_eq!(content.embeddable, deserialized.embeddable);
    }
}
