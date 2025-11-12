use crate::Extension;
/// OpenRTB 3.0 Video Placement Specification
///
/// This module implements the VideoPlacement object for video ad placements.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// VideoPlacement specification (AdCOM 1.0 Section 6.5)
///
/// The `VideoPlacement` object describes a video ad placement including
/// protocols, durations, and playback capabilities.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `CompExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, CompExt: Extension",
    deserialize = "Ext: Extension, CompExt: Extension"
))]
pub struct VideoPlacement<
    Ext: Extension = serde_json::Value,
    CompExt: Extension = serde_json::Value,
> {
    /// Placement subtype:
    /// - 1 = In-stream (default)
    /// - 2 = In-banner
    /// - 3 = In-article
    /// - 4 = In-feed
    /// - 5 = Interstitial/slider/floating
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ptype: Option<i32>,

    /// Placement position on screen.
    /// Refer to AdCOM Placement Position enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pos: Option<i32>,

    /// Placement width in units specified by `unit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Placement height in units specified by `unit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Unit of measurement for `w` and `h`:
    /// - 1 = pixels (default)
    /// - 2 = percentage (viewport)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub unit: Option<i32>,

    /// Minimum video ad duration in seconds.
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mindur: Option<i32>,

    /// Maximum video ad duration in seconds.
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxdur: Option<i32>,

    /// Maximum extended video ad duration if extension is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxext: Option<i32>,

    /// Array of supported video protocols.
    /// Refer to AdCOM Creative Subtypes - Audio/Video enumeration.
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub protocol: Option<Vec<i32>>,

    /// Indicator for skippable video:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub skip: Option<i32>,

    /// Videos of total duration greater than this value can be skippable.
    /// Only applicable if skip=1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub skipmin: Option<i32>,

    /// Number of seconds a video must play before skip control shown.
    /// Only applicable if skip=1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub skipafter: Option<i32>,

    /// Playback methods that may be in use:
    /// - 1 = Auto-play, sound on
    /// - 2 = Auto-play, sound off
    /// - 3 = Click-to-play
    /// - 4 = Mouse-over
    /// - 5 = Entering viewport, sound on
    /// - 6 = Entering viewport, sound off
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub playmethod: Option<Vec<i32>>,

    /// Playback cessation mode:
    /// - 1 = On completion or when terminated by user
    /// - 2 = On leaving viewport or when terminated
    /// - 3 = On leaving viewport, continues until completion
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub playend: Option<i32>,

    /// Click behavior for the creative:
    /// - 1 = Clickable
    /// - 0 = Not clickable
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clktype: Option<i32>,

    /// Array of supported MIME types (e.g., "video/mp4").
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mime: Option<Vec<String>>,

    /// Array of supported APIs.
    /// Refer to AdCOM API Frameworks enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub api: Option<Vec<i32>>,

    /// Array of supported delivery methods:
    /// - 1 = Streaming
    /// - 2 = Progressive
    /// - 3 = Download
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub delivery: Option<Vec<i32>>,

    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minbitr: Option<i32>,

    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxbitr: Option<i32>,

    /// Supported linear modes:
    /// - 1 = Linear (in-stream)
    /// - 2 = Non-linear (overlay)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub linear: Option<i32>,

    /// Indicator for companion ads:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub comp: Option<Vec<Box<CompExt>>>,

    /// Array of supported video companion types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub comptype: Option<Vec<i32>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
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
    fn test_video_placement_creation() {
        let video = VideoPlacement::builder()
            .ptype(Some(1))
            .w(Some(640))
            .h(Some(480))
            .mindur(Some(5))
            .maxdur(Some(30))
            .build()
            .unwrap();

        assert_eq!(video.ptype, Some(1));
        assert_eq!(video.w, Some(640));
        assert_eq!(video.mindur, Some(5));
        assert_eq!(video.maxdur, Some(30));
    }

    #[test]
    fn test_video_placement_skippable() {
        let video = VideoPlacement::builder()
            .skip(Some(1))
            .skipmin(Some(15))
            .skipafter(Some(5))
            .build()
            .unwrap();

        assert_eq!(video.skip, Some(1));
        assert_eq!(video.skipmin, Some(15));
        assert_eq!(video.skipafter, Some(5));
    }

    #[test]
    fn test_video_placement_with_protocols() {
        let video = VideoPlacement::builder()
            .protocol(Some(vec![2, 3, 5, 6]))
            .build()
            .unwrap();

        assert_eq!(video.protocol.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_video_placement_with_mime_types() {
        let video = VideoPlacement::builder()
            .mime(Some(vec![
                "video/mp4".to_string(),
                "video/webm".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(video.mime.as_ref().unwrap().len(), 2);
        assert!(
            video
                .mime
                .as_ref()
                .unwrap()
                .contains(&"video/mp4".to_string())
        );
    }

    #[test]
    fn test_video_placement_playback_methods() {
        let video = VideoPlacement::builder()
            .playmethod(Some(vec![1, 3]))
            .playend(Some(1))
            .build()
            .unwrap();

        assert_eq!(video.playmethod.as_ref().unwrap().len(), 2);
        assert_eq!(video.playend, Some(1));
    }

    #[test]
    fn test_video_placement_bitrate() {
        let video = VideoPlacement::builder()
            .minbitr(Some(300))
            .maxbitr(Some(1500))
            .build()
            .unwrap();

        assert_eq!(video.minbitr, Some(300));
        assert_eq!(video.maxbitr, Some(1500));
    }

    #[test]
    fn test_video_placement_serialization() {
        let video = VideoPlacement::builder()
            .ptype(Some(1))
            .mindur(Some(5))
            .maxdur(Some(30))
            .build()
            .unwrap();

        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("\"ptype\":1"));
        assert!(json.contains("\"mindur\":5"));
    }

    #[test]
    fn test_video_placement_deserialization() {
        let json = r#"{
            "ptype": 1,
            "w": 640,
            "h": 480,
            "mindur": 5,
            "maxdur": 30,
            "skip": 1
        }"#;

        let video: VideoPlacement = serde_json::from_str(json).unwrap();
        assert_eq!(video.ptype, Some(1));
        assert_eq!(video.skip, Some(1));
    }

    #[test]
    fn test_video_placement_linear() {
        let video = VideoPlacement::builder().linear(Some(1)).build().unwrap();

        assert_eq!(video.linear, Some(1));
    }
}
