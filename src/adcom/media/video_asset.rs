use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// VideoAsset Object (Section 3.9)
///
/// Video specification for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct VideoAsset<Ext: Extension = serde_json::Value> {
    /// Ad markup (VAST document)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl VideoAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoAssetBuilder {
        VideoAssetBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_asset_builder() {
        let video = VideoAsset::builder()
            .adm(Some("<VAST version=\"3.0\"></VAST>".to_string()))
            .curl(Some("https://cdn.example.com/vast.xml".to_string()))
            .build()
            .unwrap();

        assert_eq!(video.adm, Some("<VAST version=\"3.0\"></VAST>".to_string()));
        assert_eq!(
            video.curl,
            Some("https://cdn.example.com/vast.xml".to_string())
        );
    }

    #[test]
    fn test_video_asset_default() {
        let video = VideoAsset::builder().build().unwrap();

        assert!(video.adm.is_none());
        assert!(video.curl.is_none());
    }

    #[test]
    fn test_video_asset_with_adm() {
        let vast = r#"<VAST version="4.0"><Ad></Ad></VAST>"#;
        let video = VideoAsset::builder()
            .adm(Some(vast.to_string()))
            .build()
            .unwrap();

        assert_eq!(video.adm, Some(vast.to_string()));
        assert!(video.curl.is_none());
    }

    #[test]
    fn test_video_asset_with_curl() {
        let video = VideoAsset::builder()
            .curl(Some("https://example.com/video.xml".to_string()))
            .build()
            .unwrap();

        assert!(video.adm.is_none());
        assert_eq!(
            video.curl,
            Some("https://example.com/video.xml".to_string())
        );
    }

    #[test]
    fn test_video_asset_serialization() {
        let video = VideoAsset::builder()
            .curl(Some("https://example.com/vast.xml".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("\"curl\":\"https://example.com/vast.xml\""));
    }

    #[test]
    fn test_video_asset_deserialization() {
        let json = r#"{"adm":"<VAST></VAST>","curl":"https://example.com/video.xml"}"#;
        let video: VideoAsset = serde_json::from_str(json).unwrap();

        assert_eq!(video.adm, Some("<VAST></VAST>".to_string()));
        assert_eq!(
            video.curl,
            Some("https://example.com/video.xml".to_string())
        );
    }
}
