use crate::Extension;
use crate::adcom::media::{Banner, Event, Native};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Display Object (Section 3.2)
///
/// Details specific to display ads, supporting multiple creative formats
/// including banner images, native ads, and ad markup.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Display<Ext: Extension = serde_json::Value> {
    /// MIME type of the ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<i32>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Width as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Height as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Link to privacy policy URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priv_: Option<String>,

    /// Structured banner ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Box<Banner>>,

    /// Structured native ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Box<Native>>,

    /// Ad markup (e.g., HTML, AMPHTML)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Event trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<Event>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Display {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayBuilder {
        DisplayBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_builder() {
        let display = Display::builder()
            .mime(Some("image/jpeg".to_string()))
            .w(Some(300))
            .h(Some(250))
            .ctype(Some(1))
            .build()
            .unwrap();

        assert_eq!(display.mime, Some("image/jpeg".to_string()));
        assert_eq!(display.w, Some(300));
        assert_eq!(display.h, Some(250));
        assert_eq!(display.ctype, Some(1));
    }

    #[test]
    fn test_display_default() {
        let display = Display::builder().build().unwrap();

        assert!(display.mime.is_none());
        assert!(display.w.is_none());
        assert!(display.h.is_none());
        assert!(display.banner.is_none());
        assert!(display.native.is_none());
    }

    #[test]
    fn test_display_with_ratio() {
        let display = Display::builder()
            .wratio(Some(16))
            .hratio(Some(9))
            .build()
            .unwrap();

        assert_eq!(display.wratio, Some(16));
        assert_eq!(display.hratio, Some(9));
    }

    #[test]
    fn test_display_with_api() {
        let display = Display::builder()
            .mime(Some("text/html".to_string()))
            .api(Some(vec![3, 5, 6]))
            .build()
            .unwrap();

        assert_eq!(display.api, Some(vec![3, 5, 6]));
    }

    #[test]
    fn test_display_serialization() {
        let display = Display::builder()
            .mime(Some("image/png".to_string()))
            .w(Some(728))
            .h(Some(90))
            .build()
            .unwrap();

        let json = serde_json::to_string(&display).unwrap();
        assert!(json.contains("\"mime\":\"image/png\""));
        assert!(json.contains("\"w\":728"));
        assert!(json.contains("\"h\":90"));
    }

    #[test]
    fn test_display_deserialization() {
        let json = r#"{"mime":"image/gif","w":160,"h":600,"ctype":2}"#;
        let display: Display = serde_json::from_str(json).unwrap();

        assert_eq!(display.mime, Some("image/gif".to_string()));
        assert_eq!(display.w, Some(160));
        assert_eq!(display.h, Some(600));
        assert_eq!(display.ctype, Some(2));
    }

    #[test]
    fn test_display_with_adm() {
        let display = Display::builder()
            .adm(Some(
                "<a href='https://example.com'><img src='ad.jpg'/></a>".to_string(),
            ))
            .build()
            .unwrap();

        assert!(display.adm.is_some());
        assert!(display.adm.as_ref().unwrap().contains("<img"));
    }
}
