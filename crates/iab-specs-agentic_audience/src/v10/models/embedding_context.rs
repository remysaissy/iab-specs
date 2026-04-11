use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Contextual metadata for embeddings.
///
/// Provides contextual information about the content being embedded, such as URL,
/// page title, keywords, language, and content hash for deduplication.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EmbeddingContext<Ext: Extension = crate::DefaultExt> {
    /// URL of the content being embedded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub url: Option<String>,

    /// Page title or headline of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub page_title: Option<String>,

    /// Keywords associated with the content.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub keywords: Vec<String>,

    /// Language code of the content (e.g., "en", "fr", "de").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub language: Option<String>,

    /// Hash of the content for deduplication and tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub content_hash: Option<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl EmbeddingContext {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EmbeddingContextBuilder {
        EmbeddingContextBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_context_creation() {
        let context = EmbeddingContext::builder()
            .url("https://example.com/article")
            .page_title("Example Article")
            .keywords(vec!["ai".to_string(), "embeddings".to_string()])
            .language("en")
            .content_hash("abc123")
            .build()
            .unwrap();

        assert_eq!(context.url, Some("https://example.com/article".to_string()));
        assert_eq!(context.page_title, Some("Example Article".to_string()));
        assert_eq!(context.keywords.len(), 2);
        assert_eq!(context.language, Some("en".to_string()));
        assert_eq!(context.content_hash, Some("abc123".to_string()));
    }

    #[test]
    fn test_embedding_context_serialization() {
        let context = EmbeddingContext::builder()
            .url("https://example.com/news")
            .page_title("Breaking News")
            .keywords(vec!["tech".to_string(), "ai".to_string()])
            .language("en")
            .build()
            .unwrap();

        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains("\"url\":\"https://example.com/news\""));
        assert!(json.contains("\"page_title\":\"Breaking News\""));
        assert!(json.contains("\"keywords\":["));
        assert!(json.contains("\"language\":\"en\""));
    }

    #[test]
    fn test_embedding_context_deserialization() {
        let json = r#"{"url":"https://example.com/test","page_title":"Test Page","keywords":["keyword1","keyword2"],"language":"fr"}"#;
        let context: EmbeddingContext = serde_json::from_str(json).unwrap();

        assert_eq!(context.url, Some("https://example.com/test".to_string()));
        assert_eq!(context.page_title, Some("Test Page".to_string()));
        assert_eq!(context.keywords.len(), 2);
        assert_eq!(context.language, Some("fr".to_string()));
    }

    #[test]
    fn test_embedding_context_roundtrip() {
        let context = EmbeddingContext::builder()
            .url("https://example.com/roundtrip")
            .page_title("Roundtrip Test")
            .keywords(vec!["test".to_string(), "roundtrip".to_string()])
            .language("de")
            .content_hash("xyz789")
            .build()
            .unwrap();

        let json = serde_json::to_string(&context).unwrap();
        let parsed: EmbeddingContext = serde_json::from_str(&json).unwrap();
        assert_eq!(context, parsed);
    }

    #[test]
    fn test_embedding_context_default() {
        let context = EmbeddingContext::builder().build().unwrap();

        assert!(context.url.is_none());
        assert!(context.page_title.is_none());
        assert!(context.keywords.is_empty());
        assert!(context.language.is_none());
        assert!(context.content_hash.is_none());
        assert!(context.ext.is_none());
    }

    #[test]
    fn test_embedding_context_empty_keywords_omitted() {
        // Spec: empty keywords vec should be omitted from JSON via skip_serializing_if
        let context = EmbeddingContext::builder()
            .url("https://example.com")
            .build()
            .unwrap();
        let json = serde_json::to_string(&context).unwrap();
        assert!(
            !json.contains("\"keywords\""),
            "Empty keywords should be omitted from JSON"
        );
    }
}
