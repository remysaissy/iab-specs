use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Video Object (Section 3.12)
///
/// Details specific to video ads including format, duration, and delivery.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Video<Ext: Extension = serde_json::Value> {
    /// MIME types supported (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<i32>>,

    /// Creative subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<i32>,

    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<i32>,

    /// Ad markup (VAST document)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Video protocols
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Bit rate in Kbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,

    /// Timestamp when creative was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<i64>,

    /// Timestamp of last modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<i64>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Video {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoBuilder {
        VideoBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_builder() {
        let video = Video::builder()
            .mimes(Some(vec!["video/mp4".to_string()]))
            .dur(Some(30))
            .w(Some(1920))
            .h(Some(1080))
            .build()
            .unwrap();

        assert_eq!(video.mimes, Some(vec!["video/mp4".to_string()]));
        assert_eq!(video.dur, Some(30));
        assert_eq!(video.w, Some(1920));
        assert_eq!(video.h, Some(1080));
    }

    #[test]
    fn test_video_default() {
        let video = Video::builder().build().unwrap();

        assert!(video.mimes.is_none());
        assert!(video.dur.is_none());
        assert!(video.w.is_none());
        assert!(video.h.is_none());
    }

    #[test]
    fn test_video_with_protocols() {
        let video = Video::builder()
            .mimes(Some(vec!["video/mp4".to_string()]))
            .protocols(Some(vec![2, 3, 5, 6]))
            .build()
            .unwrap();

        assert_eq!(video.protocols, Some(vec![2, 3, 5, 6]));
    }

    #[test]
    fn test_video_with_bitrate() {
        let video = Video::builder()
            .mimes(Some(vec!["video/webm".to_string()]))
            .bitrate(Some(2500))
            .build()
            .unwrap();

        assert_eq!(video.bitrate, Some(2500));
    }

    #[test]
    fn test_video_serialization() {
        let video = Video::builder()
            .mimes(Some(vec![
                "video/mp4".to_string(),
                "video/webm".to_string(),
            ]))
            .dur(Some(15))
            .w(Some(640))
            .h(Some(480))
            .build()
            .unwrap();

        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("\"mimes\":[\"video/mp4\",\"video/webm\"]"));
        assert!(json.contains("\"dur\":15"));
        assert!(json.contains("\"w\":640"));
    }

    #[test]
    fn test_video_deserialization() {
        let json = r#"{"mimes":["video/mp4"],"dur":30,"w":1280,"h":720}"#;
        let video: Video = serde_json::from_str(json).unwrap();

        assert_eq!(video.mimes, Some(vec!["video/mp4".to_string()]));
        assert_eq!(video.dur, Some(30));
        assert_eq!(video.w, Some(1280));
        assert_eq!(video.h, Some(720));
    }

    #[test]
    fn test_video_with_vast() {
        let video = Video::builder()
            .mimes(Some(vec!["video/mp4".to_string()]))
            .adm(Some("<VAST version=\"3.0\">...</VAST>".to_string()))
            .build()
            .unwrap();

        assert!(video.adm.is_some());
        assert!(video.adm.as_ref().unwrap().contains("<VAST"));
    }
}
