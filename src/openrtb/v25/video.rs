/// OpenRTB 2.5 Video Ad Object
///
/// This module implements the Video object for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::banner::Banner;

/// Default value for boxingallowed field (1 = allowed)
fn default_boxingallowed() -> i32 {
    1
}

/// Video ad impression (OpenRTB 2.5 Section 3.2.7)
///
/// A `Video` object represents a video ad impression with VAST compliance.
/// It describes the video player capabilities, supported formats, and playback requirements.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Video;
///
/// let video = Video {
///     mimes: vec!["video/mp4".to_string(), "video/webm".to_string()],
///     minduration: 5,
///     maxduration: Some(30),
///     protocols: Some(vec![2, 3, 5, 6]),
///     w: Some(640),
///     h: Some(480),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Video {
    /// Content MIME types supported (e.g., "video/mp4").
    /// **Required field** - at least one MIME type must be specified.
    #[builder(setter(into))]
    pub mimes: Vec<String>,

    /// Minimum video ad duration in seconds.
    /// Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub minduration: i32,

    /// Maximum video ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxduration: Option<i32>,

    /// Start delay in seconds for pre-roll, mid-roll, or post-roll ad placement.
    /// Refer to AdCOM `StartDelay` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub startdelay: Option<i32>,

    /// Maximum number of ads that can be played in a dynamic video ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxseq: Option<i32>,

    /// Total duration of the video ad pod in seconds.
    /// For dynamic pods, this represents the target duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub poddur: Option<i32>,

    /// Array of supported video protocols.
    /// Refer to AdCOM `Protocol` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub protocols: Option<Vec<i32>>,

    /// Width of the video player in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub w: Option<i32>,

    /// Height of the video player in device-independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub h: Option<i32>,

    /// Unique identifier for the ad pod.
    /// Used to group multiple impressions for pod-based bidding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub podid: Option<String>,

    /// Sequence number of the impression within an ad pod (0-indexed).
    /// Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub podseq: i32,

    /// Array of exact video durations (in seconds) that are required.
    /// Mutually exclusive with minduration and maxduration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rqddurs: Option<Vec<i32>>,

    /// Placement type for the video.
    /// Refer to AdCOM `PlacementType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub plcmt: Option<i32>,

    /// Indicates if the impression must be linear, non-linear, etc.
    /// Refer to AdCOM `Linearity` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub linearity: Option<i32>,

    /// Indicates if the player will allow the video to be skipped:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub skip: Option<i32>,

    /// Minimum video ad duration before skip button appears (in seconds).
    /// Only applicable if skip=1. Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub skipmin: i32,

    /// Number of seconds after which skip button appears.
    /// Only applicable if skip=1. Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub skipafter: i32,

    /// For ad pods, indicates the impression's position guarantee:
    /// - 0 = no guarantee
    /// - >0 = guaranteed position
    /// Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub slotinpod: i32,

    /// Minimum CPM per second.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mincpmpersec: Option<f64>,

    /// Blocked creative attributes.
    /// Refer to AdCOM `CreativeAttribute` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub battr: Option<Vec<i32>>,

    /// Maximum extended video ad duration beyond maxduration:
    /// - -1 = unlimited
    /// - 0 = no extension allowed
    /// - >0 = maximum extension in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxextended: Option<i32>,

    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minbitrate: Option<i32>,

    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxbitrate: Option<i32>,

    /// Indicates if letter-boxing of 4:3 content into a 16:9 window is allowed:
    /// - 0 = no
    /// - 1 = yes (default)
    #[serde(default = "default_boxingallowed")]
    #[builder(default = "default_boxingallowed()")]
    pub boxingallowed: i32,

    /// Playback methods that may be in use.
    /// Refer to AdCOM `PlaybackMethod` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub playbackmethod: Option<Vec<i32>>,

    /// The event that causes playback to end.
    /// Refer to AdCOM `PlaybackCessationMode` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub playbackend: Option<i32>,

    /// Supported delivery methods (e.g., streaming, progressive).
    /// Refer to AdCOM `DeliveryMethod` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub delivery: Option<Vec<i32>>,

    /// Ad position on screen.
    /// Refer to AdCOM `AdPosition` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pos: Option<i32>,

    /// Array of Banner objects representing companion ads available for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub companionad: Option<Vec<Banner>>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM `ApiFramework` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub api: Option<Vec<i32>>,

    /// Supported VAST companion ad types.
    /// Refer to AdCOM `CompanionType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub companiontype: Option<Vec<i32>>,

    /// Pod deduplication settings.
    /// Refer to AdCOM `PodDeduplication` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub poddedupe: Option<Vec<i32>>,

    /// Array of DurFloors objects defining duration-based floor prices.
    /// Uses placeholder until DurFloors is implemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub durfloors: Option<Vec<serde_json::Value>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            mimes: Vec::new(),
            minduration: 0,
            maxduration: None,
            startdelay: None,
            maxseq: None,
            poddur: None,
            protocols: None,
            w: None,
            h: None,
            podid: None,
            podseq: 0,
            rqddurs: None,
            plcmt: None,
            linearity: None,
            skip: None,
            skipmin: 0,
            skipafter: 0,
            slotinpod: 0,
            mincpmpersec: None,
            battr: None,
            maxextended: None,
            minbitrate: None,
            maxbitrate: None,
            boxingallowed: default_boxingallowed(),
            playbackmethod: None,
            playbackend: None,
            delivery: None,
            pos: None,
            companionad: None,
            api: None,
            companiontype: None,
            poddedupe: None,
            durfloors: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_creation() {
        let video = Video {
            mimes: vec!["video/mp4".to_string()],
            minduration: 5,
            maxduration: Some(30),
            w: Some(640),
            h: Some(480),
            ..Default::default()
        };

        assert_eq!(video.mimes.len(), 1);
        assert_eq!(video.minduration, 5);
        assert_eq!(video.maxduration, Some(30));
        assert_eq!(video.boxingallowed, 1); // Default value
    }

    #[test]
    fn test_video_defaults() {
        let video = Video {
            mimes: vec!["video/mp4".to_string()],
            ..Default::default()
        };

        assert_eq!(video.minduration, 0);
        assert_eq!(video.podseq, 0);
        assert_eq!(video.skipmin, 0);
        assert_eq!(video.skipafter, 0);
        assert_eq!(video.slotinpod, 0);
        assert_eq!(video.boxingallowed, 1);
    }

    #[test]
    fn test_video_serialization() {
        let video = Video {
            mimes: vec!["video/mp4".to_string()],
            w: Some(640),
            h: Some(480),
            ..Default::default()
        };

        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("\"mimes\":[\"video/mp4\"]"));
        assert!(json.contains("\"w\":640"));
    }

    #[test]
    fn test_video_deserialization() {
        let json = r#"{"mimes":["video/mp4"],"w":640,"h":480}"#;
        let video: Video = serde_json::from_str(json).unwrap();

        assert_eq!(video.mimes, vec!["video/mp4"]);
        assert_eq!(video.w, Some(640));
        assert_eq!(video.h, Some(480));
    }
}
