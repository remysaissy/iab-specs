use crate::Extension;
use crate::adcom::placement::Companion;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// VideoPlacement Object (Section 4.10)
///
/// Placement details for video ad formats.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct VideoPlacement<Ext: Extension = serde_json::Value> {
    /// Video placement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptype: Option<i32>,

    /// Ad position on screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    /// Start delay in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,

    /// Skippable flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,

    /// Skip offset in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipmin: Option<i32>,

    /// Skip button visible after this duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipafter: Option<i32>,

    /// Playback method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playmethod: Option<i32>,

    /// Playback cessation mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playend: Option<i32>,

    /// Click type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clktype: Option<i32>,

    /// MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtypes permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<Vec<i32>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Placement unit identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,

    /// Minimum duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mindur: Option<i32>,

    /// Maximum duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdur: Option<i32>,

    /// Maximum extended duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxext: Option<i32>,

    /// Minimum bit rate in Kbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,

    /// Maximum bit rate in Kbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,

    /// Delivery methods supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<i32>>,

    /// Maximum ad sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i32>,

    /// Video linearity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<i32>,

    /// Boxing allowed flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxing: Option<i32>,

    /// Array of companion ad objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp: Option<Vec<Companion>>,

    /// Array of companion types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comptype: Option<Vec<i32>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl VideoPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoPlacementBuilder {
        VideoPlacementBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_placement_builder() {
        let video = VideoPlacement::builder()
            .ptype(Some(1))
            .w(Some(640))
            .h(Some(480))
            .pos(Some(1))
            .build()
            .unwrap();

        assert_eq!(video.ptype, Some(1));
        assert_eq!(video.w, Some(640));
        assert_eq!(video.h, Some(480));
        assert_eq!(video.pos, Some(1));
    }

    #[test]
    fn test_video_placement_default() {
        let video = VideoPlacement::builder().build().unwrap();

        assert!(video.ptype.is_none());
        assert!(video.w.is_none());
        assert!(video.h.is_none());
        assert!(video.mindur.is_none());
        assert!(video.maxdur.is_none());
    }

    #[test]
    fn test_video_placement_skippable() {
        let video = VideoPlacement::builder()
            .skip(Some(1))
            .skipmin(Some(5))
            .skipafter(Some(5))
            .build()
            .unwrap();

        assert_eq!(video.skip, Some(1));
        assert_eq!(video.skipmin, Some(5));
        assert_eq!(video.skipafter, Some(5));
    }

    #[test]
    fn test_video_placement_duration() {
        let video = VideoPlacement::builder()
            .mindur(Some(15))
            .maxdur(Some(30))
            .maxext(Some(60))
            .build()
            .unwrap();

        assert_eq!(video.mindur, Some(15));
        assert_eq!(video.maxdur, Some(30));
        assert_eq!(video.maxext, Some(60));
    }

    #[test]
    fn test_video_placement_bitrate() {
        let video = VideoPlacement::builder()
            .minbitrate(Some(300))
            .maxbitrate(Some(1500))
            .build()
            .unwrap();

        assert_eq!(video.minbitrate, Some(300));
        assert_eq!(video.maxbitrate, Some(1500));
    }

    #[test]
    fn test_video_placement_with_mime_and_api() {
        let video = VideoPlacement::builder()
            .mime(Some(vec![
                "video/mp4".to_string(),
                "video/webm".to_string(),
            ]))
            .api(Some(vec![1, 2, 5]))
            .build()
            .unwrap();

        assert_eq!(
            video.mime,
            Some(vec!["video/mp4".to_string(), "video/webm".to_string()])
        );
        assert_eq!(video.api, Some(vec![1, 2, 5]));
    }

    #[test]
    fn test_video_placement_serialization() {
        let video = VideoPlacement::builder()
            .ptype(Some(1))
            .w(Some(640))
            .h(Some(480))
            .build()
            .unwrap();

        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("\"ptype\":1"));
        assert!(json.contains("\"w\":640"));
        assert!(json.contains("\"h\":480"));
    }

    #[test]
    fn test_video_placement_deserialization() {
        let json = r#"{"ptype":1,"w":640,"h":480,"mindur":15,"maxdur":30}"#;
        let video: VideoPlacement = serde_json::from_str(json).unwrap();

        assert_eq!(video.ptype, Some(1));
        assert_eq!(video.w, Some(640));
        assert_eq!(video.h, Some(480));
        assert_eq!(video.mindur, Some(15));
        assert_eq!(video.maxdur, Some(30));
    }
}
