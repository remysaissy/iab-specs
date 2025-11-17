use crate::Extension;
use crate::adcom::media::{Audio, Audit, Display, Video};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Ad Object (Section 3.1)
///
/// Root structure defining an advertising media instance with rendering instructions.
/// Must include exactly one media subtype object (Display, Video, or Audio).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Ad<Ext: Extension = serde_json::Value> {
    /// Creative identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Advertiser domain for buyer targeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adomain: Option<Vec<String>>,

    /// App bundle/package name for buyer targeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Vec<String>>,

    /// Preview image URL for creative approval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iurl: Option<String>,

    /// Content categories describing the creative using IDs from taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// The taxonomy used for cat attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Language using ISO-639-1-alpha-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Creative attributes array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<Vec<i32>>,

    /// HTTPS creative flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,

    /// Media rating per IQG guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrating: Option<i32>,

    /// Timestamp when ad was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<i64>,

    /// Timestamp of last modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<i64>,

    /// Display ad subtype
    /// Note: Display will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<Display>>,

    /// Video ad subtype
    /// Note: Video will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<Video>>,

    /// Audio ad subtype
    /// Note: Audio will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<Audio>>,

    /// Audit object for quality/safety review
    /// Note: Audit will be updated to generic in future, currently uses serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<Box<Audit>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Ad {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AdBuilder {
        AdBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ad_builder() {
        let ad = Ad::builder()
            .id(Some("ad123".to_string()))
            .adomain(Some(vec!["advertiser.com".to_string()]))
            .lang(Some("en".to_string()))
            .secure(Some(1))
            .build()
            .unwrap();

        assert_eq!(ad.id, Some("ad123".to_string()));
        assert_eq!(ad.adomain, Some(vec!["advertiser.com".to_string()]));
        assert_eq!(ad.lang, Some("en".to_string()));
        assert_eq!(ad.secure, Some(1));
    }

    #[test]
    fn test_ad_default() {
        let ad = Ad::builder().build().unwrap();

        assert!(ad.id.is_none());
        assert!(ad.adomain.is_none());
        assert!(ad.display.is_none());
        assert!(ad.video.is_none());
        assert!(ad.audio.is_none());
    }

    #[test]
    fn test_ad_with_categories() {
        let ad = Ad::builder()
            .id(Some("ad456".to_string()))
            .cat(Some(vec!["IAB1".to_string(), "IAB1-1".to_string()]))
            .cattax(Some(1))
            .build()
            .unwrap();

        assert_eq!(ad.cat, Some(vec!["IAB1".to_string(), "IAB1-1".to_string()]));
        assert_eq!(ad.cattax, Some(1));
    }

    #[test]
    fn test_ad_with_bundle() {
        let ad = Ad::builder()
            .id(Some("ad789".to_string()))
            .bundle(Some(vec!["com.example.app".to_string()]))
            .build()
            .unwrap();

        assert_eq!(ad.bundle, Some(vec!["com.example.app".to_string()]));
    }

    #[test]
    fn test_ad_serialization() {
        let ad = Ad::builder()
            .id(Some("ad999".to_string()))
            .adomain(Some(vec!["example.com".to_string()]))
            .secure(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&ad).unwrap();
        assert!(json.contains("\"id\":\"ad999\""));
        assert!(json.contains("\"adomain\":[\"example.com\"]"));
        assert!(json.contains("\"secure\":1"));
    }

    #[test]
    fn test_ad_deserialization() {
        let json = r#"{"id":"ad111","adomain":["test.com"],"lang":"en","secure":1}"#;
        let ad: Ad = serde_json::from_str(json).unwrap();

        assert_eq!(ad.id, Some("ad111".to_string()));
        assert_eq!(ad.adomain, Some(vec!["test.com".to_string()]));
        assert_eq!(ad.lang, Some("en".to_string()));
        assert_eq!(ad.secure, Some(1));
    }

    #[test]
    fn test_ad_with_timestamps() {
        let ad = Ad::builder()
            .id(Some("ad222".to_string()))
            .init(Some(1234567890))
            .lastmod(Some(1234567900))
            .build()
            .unwrap();

        assert_eq!(ad.init, Some(1234567890));
        assert_eq!(ad.lastmod, Some(1234567900));
    }
}
