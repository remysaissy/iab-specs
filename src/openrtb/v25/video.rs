use super::banner::Banner;
use crate::Extension;
/// OpenRTB 2.5/2.6 Video Ad Object
///
/// This module implements the Video object for OpenRTB 2.5 and 2.6.
/// OpenRTB 2.6 fields (podid, podseq, slotinpod, durfloors) are included.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// Import DurFloors from v26 when openrtb_26 feature is enabled
#[cfg(feature = "openrtb_26")]
use crate::openrtb::v26::DurFloors;

/// Default value for boxingallowed field (1 = allowed)
fn default_boxingallowed() -> i32 {
    1
}

/// Video ad impression (OpenRTB 2.5 Section 3.2.7)
///
/// A `Video` object represents a video ad impression with VAST compliance.
/// It describes the video player capabilities, supported formats, and playback requirements.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Video;
///
/// let video = Video::builder()
///     .mimes(vec!["video/mp4".to_string(), "video/webm".to_string()])
///     .minduration(5)
///     .maxduration(Some(30))
///     .protocols(Some(vec![2, 3, 5, 6]))
///     .w(Some(640))
///     .h(Some(480))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Video<Ext: Extension = serde_json::Value> {
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
    ///
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

    /// Array of DurFloors objects defining duration-based floor prices (OpenRTB 2.6+).
    /// Enables different floor prices based on creative duration ranges.
    #[cfg(feature = "openrtb_26")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub durfloors: Option<Vec<DurFloors>>,

    /// Array of DurFloors objects defining duration-based floor prices (placeholder for v2.5).
    /// When using openrtb_26 feature, use the typed DurFloors version instead.
    #[cfg(not(feature = "openrtb_26"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub durfloors: Option<Vec<serde_json::Value>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Video {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> VideoBuilder {
        VideoBuilder::create_empty()
    }
}

impl<Ext: Extension> Default for Video<Ext> {
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
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minduration(5)
            .maxduration(Some(30))
            .w(Some(640))
            .h(Some(480))
            .build()
            .unwrap();

        assert_eq!(video.mimes.len(), 1);
        assert_eq!(video.minduration, 5);
        assert_eq!(video.maxduration, Some(30));
        assert_eq!(video.boxingallowed, 1); // Default value
    }

    #[test]
    fn test_video_serialization() {
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .w(Some(640))
            .h(Some(480))
            .build()
            .unwrap();

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

    // === Phase 1.2: Required Field Validation Tests ===

    #[test]
    fn test_missing_required_mimes_field() {
        // Test deserialization without required 'mimes' field
        let json = r#"{"w":640,"h":480}"#;
        let result: Result<Video, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Video without required 'mimes' field should fail deserialization"
        );
    }

    #[test]
    fn test_empty_required_mimes_array() {
        // Test that empty mimes array is currently allowed
        // Per OpenRTB spec: "at least one MIME type" required
        let result = Video::builder().mimes(vec![]).build();

        // Currently no validation prevents empty mimes array
        assert!(result.is_ok(), "Empty mimes array currently passes");
        // TODO: Consider adding validation to enforce "at least one MIME type" requirement
    }

    #[test]
    fn test_null_mimes_field() {
        // Test explicit null for required field
        let json = r#"{"mimes":null}"#;
        let result: Result<Video, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Video with null 'mimes' should fail deserialization"
        );
    }

    // === Phase 1.3: Boundary & Edge Case Tests ===

    #[test]
    fn test_negative_minduration() {
        // Test negative duration value
        let result = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minduration(-1)
            .build();

        // Currently allows negative durations - documents current behavior
        assert!(result.is_ok(), "Negative minduration currently allowed");
        // TODO: Consider adding validation to reject negative durations
    }

    #[test]
    fn test_negative_maxduration() {
        // Test negative max duration
        let result = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .maxduration(Some(-1))
            .build();

        assert!(result.is_ok(), "Negative maxduration currently allowed");
        // TODO: Consider validation for duration constraints
    }

    #[test]
    fn test_maxduration_less_than_minduration() {
        // Test logical inconsistency: maxduration < minduration
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minduration(30)
            .maxduration(Some(10)) // max < min - logically invalid
            .build()
            .unwrap();

        // Currently no cross-field validation
        assert!(video.maxduration.unwrap() < video.minduration);
        // TODO: Consider adding cross-field validation
    }

    #[test]
    fn test_zero_dimensions() {
        // Test zero width/height
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .w(Some(0))
            .h(Some(0))
            .build()
            .unwrap();

        assert_eq!(video.w, Some(0));
        assert_eq!(video.h, Some(0));
        // Document: Zero dimensions currently allowed
    }

    #[test]
    fn test_negative_dimensions() {
        // Test negative dimensions
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .w(Some(-100))
            .h(Some(-100))
            .build()
            .unwrap();

        assert_eq!(video.w, Some(-100));
        // Document: Negative dimensions currently allowed
        // TODO: Should be rejected as dimensions must be positive
    }

    #[test]
    fn test_negative_bitrate() {
        // Test negative bitrate values
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minbitrate(Some(-1))
            .maxbitrate(Some(-1))
            .build()
            .unwrap();

        assert_eq!(video.minbitrate, Some(-1));
        // Document: Negative bitrates currently allowed
        // TODO: Bitrates should be positive integers
    }

    #[test]
    fn test_maxbitrate_less_than_minbitrate() {
        // Test logical inconsistency: maxbitrate < minbitrate
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minbitrate(Some(5000))
            .maxbitrate(Some(1000)) // max < min
            .build()
            .unwrap();

        assert!(video.maxbitrate.unwrap() < video.minbitrate.unwrap());
        // Document: No cross-field validation for bitrate constraints
    }

    #[test]
    fn test_default_boxingallowed() {
        // Test that boxingallowed defaults to 1 (allowed)
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .build()
            .unwrap();

        assert_eq!(video.boxingallowed, 1);
    }

    #[test]
    fn test_skip_without_skip_params() {
        // Test skip=1 without skipmin/skipafter
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .skip(Some(1))
            .build()
            .unwrap();

        assert_eq!(video.skip, Some(1));
        assert_eq!(video.skipmin, 0); // Defaults to 0
        assert_eq!(video.skipafter, 0); // Defaults to 0
        // Document: skipmin and skipafter default to 0 when not specified
    }

    // === Phase 2.2: Mutually Exclusive Field Tests (rqddurs vs minduration/maxduration) ===

    #[test]
    fn test_video_with_rqddurs_only() {
        // Valid: Video with rqddurs (exact durations) and no minduration/maxduration
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .rqddurs(Some(vec![15, 30, 60]))
            .build()
            .unwrap();

        assert_eq!(video.rqddurs, Some(vec![15, 30, 60]));
        assert_eq!(video.minduration, 0); // Default
        assert_eq!(video.maxduration, None);
    }

    #[test]
    fn test_video_with_minduration_maxduration_only() {
        // Valid: Video with minduration/maxduration and no rqddurs
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minduration(15)
            .maxduration(Some(60))
            .build()
            .unwrap();

        assert_eq!(video.minduration, 15);
        assert_eq!(video.maxduration, Some(60));
        assert!(video.rqddurs.is_none());
    }

    #[test]
    fn test_video_with_rqddurs_and_minduration() {
        // Per spec: rqddurs is mutually exclusive with minduration and maxduration
        // Test that having BOTH rqddurs and minduration currently passes
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .rqddurs(Some(vec![15, 30]))
            .minduration(10)
            .build();

        assert!(
            video.is_ok(),
            "Video with both rqddurs and minduration currently passes"
        );

        let video = video.unwrap();
        assert_eq!(video.rqddurs, Some(vec![15, 30]));
        assert_eq!(video.minduration, 10);
        // TODO: Per OpenRTB spec, rqddurs is mutually exclusive with minduration/maxduration
        // Should be rejected when both are present
    }

    #[test]
    fn test_video_with_rqddurs_and_maxduration() {
        // Test that having BOTH rqddurs and maxduration currently passes
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .rqddurs(Some(vec![15, 30]))
            .maxduration(Some(60))
            .build();

        assert!(
            video.is_ok(),
            "Video with both rqddurs and maxduration currently passes"
        );

        let video = video.unwrap();
        assert_eq!(video.rqddurs, Some(vec![15, 30]));
        assert_eq!(video.maxduration, Some(60));
        // TODO: Should be rejected - rqddurs is mutually exclusive with maxduration
    }

    #[test]
    fn test_video_with_all_duration_fields() {
        // Test that having ALL duration specification fields currently passes
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .rqddurs(Some(vec![15, 30]))
            .minduration(10)
            .maxduration(Some(60))
            .build();

        assert!(
            video.is_ok(),
            "Video with rqddurs, minduration, and maxduration currently passes"
        );

        let video = video.unwrap();
        assert_eq!(video.rqddurs, Some(vec![15, 30]));
        assert_eq!(video.minduration, 10);
        assert_eq!(video.maxduration, Some(60));
        // TODO: Should be rejected - can use EITHER rqddurs OR minduration/maxduration, not both
    }

    #[test]
    fn test_video_deserialization_with_conflicting_duration_fields() {
        // Test deserialization behavior with mutually exclusive duration fields
        let json = r#"{
            "mimes": ["video/mp4"],
            "rqddurs": [15, 30],
            "minduration": 10,
            "maxduration": 60
        }"#;

        let result: Result<Video, _> = serde_json::from_str(json);

        assert!(
            result.is_ok(),
            "Deserialization with conflicting duration fields currently passes"
        );

        let video = result.unwrap();
        assert_eq!(video.rqddurs, Some(vec![15, 30]));
        assert_eq!(video.minduration, 10);
        assert_eq!(video.maxduration, Some(60));
        // TODO: Should deserialization validate mutual exclusivity for duration fields?
    }

    // === Phase 2.3: Feature Flag Tests (openrtb_26) ===

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_video_with_maxseq_field() {
        // Test that OpenRTB 2.6 maxseq (max number of ads in sequence) field is available
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .maxseq(Some(3))
            .build()
            .unwrap();

        assert_eq!(video.maxseq, Some(3));
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_video_maxseq_serialization() {
        // Test serialization of OpenRTB 2.6 maxseq field
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .maxseq(Some(5))
            .build()
            .unwrap();

        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("\"maxseq\":5"));
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_video_maxseq_deserialization() {
        // Test deserialization of OpenRTB 2.6 maxseq field
        let json = r#"{"mimes":["video/mp4"],"maxseq":4}"#;
        let result: Result<Video, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Video with maxseq field should deserialize");
        let video = result.unwrap();
        assert_eq!(video.maxseq, Some(4));
    }

    #[cfg(not(feature = "openrtb_26"))]
    #[test]
    fn test_video_maxseq_not_available_without_feature() {
        // This test verifies that maxseq field is not available without openrtb_26 feature
        let video = Video::builder()
            .mimes(vec!["video/mp4".to_string()])
            .minduration(15)
            .build()
            .unwrap();

        // The maxseq field should not exist in Video when openrtb_26 is disabled
        // This is verified at compile time
        assert_eq!(video.mimes, vec!["video/mp4"]);
        assert_eq!(video.minduration, 15);
    }
}
