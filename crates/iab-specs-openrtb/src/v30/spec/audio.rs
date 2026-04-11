use crate::Extension;
/// OpenRTB 3.0 Audio Placement Specification
///
/// This module implements the AudioPlacement object for audio ad placements.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// AudioPlacement specification (AdCOM 1.0 Section 6.6)
///
/// The `AudioPlacement` object describes an audio ad placement including
/// protocols, durations, and playback capabilities.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AudioPlacement<Ext: Extension = crate::DefaultExt> {
    /// Minimum audio ad duration in seconds.
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mindur: Option<i32>,

    /// Maximum audio ad duration in seconds.
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxdur: Option<i32>,

    /// Maximum extended audio ad duration if extension is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxext: Option<i32>,

    /// Array of supported audio protocols.
    /// Refer to AdCOM Creative Subtypes - Audio/Video enumeration.
    /// RECOMMENDED by the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub protocol: Option<Vec<i32>>,

    /// Type of audio feed:
    /// - 1 = Music service
    /// - 2 = FM/AM broadcast
    /// - 3 = Podcast
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub feed: Option<i32>,

    /// Volume normalization mode:
    /// - 0 = None
    /// - 1 = Average volume
    /// - 2 = Peak volume
    /// - 3 = Loudness
    /// - 4 = Custom
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nvol: Option<i32>,

    /// Array of supported MIME types (e.g., "audio/mp4").
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

    /// Indicator for stitched audio:
    /// - 0 = no
    /// - 1 = yes
    ///
    /// Stitched means the ad is part of a continuous audio stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stitched: Option<i32>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AudioPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AudioPlacementBuilder {
        AudioPlacementBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec: Object: AudioPlacement — verifies builder creates placement with duration and feed fields
    #[test]
    fn test_audio_placement_creation() {
        let audio = AudioPlacement::builder()
            .mindur(Some(5))
            .maxdur(Some(30))
            .feed(Some(1))
            .build()
            .unwrap();

        assert_eq!(audio.mindur, Some(5));
        assert_eq!(audio.maxdur, Some(30));
        assert_eq!(audio.feed, Some(1));
    }

    // Spec: Object: AudioPlacement — verifies protocol field accepts array of audio/video protocol IDs
    #[test]
    fn test_audio_placement_with_protocols() {
        let audio = AudioPlacement::builder()
            .protocol(Some(vec![2, 3, 5, 6]))
            .build()
            .unwrap();

        assert_eq!(audio.protocol.as_ref().unwrap().len(), 4);
    }

    // Spec: Object: AudioPlacement — verifies mime field accepts array of MIME type strings
    #[test]
    fn test_audio_placement_with_mime_types() {
        let audio = AudioPlacement::builder()
            .mime(Some(vec![
                "audio/mp4".to_string(),
                "audio/mpeg".to_string(),
                "audio/ogg".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(audio.mime.as_ref().unwrap().len(), 3);
        assert!(
            audio
                .mime
                .as_ref()
                .unwrap()
                .contains(&"audio/mp4".to_string())
        );
    }

    // Spec: Object: AudioPlacement — verifies feed=3 (podcast) with duration constraints
    #[test]
    fn test_audio_placement_podcast() {
        let audio = AudioPlacement::builder()
            .feed(Some(3)) // Podcast
            .mindur(Some(15))
            .maxdur(Some(60))
            .build()
            .unwrap();

        assert_eq!(audio.feed, Some(3));
    }

    // Spec: Object: AudioPlacement — verifies minbitr and maxbitr fields for bitrate constraints
    #[test]
    fn test_audio_placement_bitrate() {
        let audio = AudioPlacement::builder()
            .minbitr(Some(64))
            .maxbitr(Some(320))
            .build()
            .unwrap();

        assert_eq!(audio.minbitr, Some(64));
        assert_eq!(audio.maxbitr, Some(320));
    }

    // Spec: Object: AudioPlacement — verifies stitched=1 indicates continuous audio stream
    #[test]
    fn test_audio_placement_stitched() {
        let audio = AudioPlacement::builder().stitched(Some(1)).build().unwrap();

        assert_eq!(audio.stitched, Some(1));
    }

    // Spec: Object: AudioPlacement — verifies JSON serialization includes mindur, maxdur, and feed
    #[test]
    fn test_audio_placement_serialization() {
        let audio = AudioPlacement::builder()
            .mindur(Some(5))
            .maxdur(Some(30))
            .feed(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"mindur\":5"));
        assert!(json.contains("\"maxdur\":30"));
        assert!(json.contains("\"feed\":1"));
    }

    // Spec: Object: AudioPlacement — verifies JSON deserialization restores fields from JSON string
    #[test]
    fn test_audio_placement_deserialization() {
        let json = r#"{
            "mindur": 5,
            "maxdur": 30,
            "feed": 1,
            "nvol": 100
        }"#;

        let audio: AudioPlacement = serde_json::from_str(json).unwrap();
        assert_eq!(audio.mindur, Some(5));
        assert_eq!(audio.feed, Some(1));
    }

    // Spec: Object: AudioPlacement — verifies nvol field for volume normalization mode
    #[test]
    fn test_audio_placement_volume_normalization() {
        let audio = AudioPlacement::builder()
            .nvol(Some(3)) // Loudness normalization
            .build()
            .unwrap();

        assert_eq!(audio.nvol, Some(3));
    }

    // Spec: Object: AudioPlacement — verifies default() produces all None fields
    #[test]
    fn test_audio_placement_default() {
        let audio: AudioPlacement = AudioPlacement::default();
        assert_eq!(audio.mindur, None);
        assert_eq!(audio.maxdur, None);
        assert_eq!(audio.maxext, None);
        assert_eq!(audio.protocol, None);
        assert_eq!(audio.feed, None);
        assert_eq!(audio.nvol, None);
        assert_eq!(audio.mime, None);
        assert_eq!(audio.api, None);
        assert_eq!(audio.delivery, None);
        assert_eq!(audio.minbitr, None);
        assert_eq!(audio.maxbitr, None);
        assert_eq!(audio.stitched, None);
        assert!(audio.ext.is_none());
    }

    // Spec: Object: AudioPlacement — verifies serialize then deserialize roundtrip preserves all fields
    #[test]
    fn test_audio_placement_roundtrip() {
        let audio = AudioPlacement::builder()
            .mindur(Some(5))
            .maxdur(Some(30))
            .feed(Some(1))
            .nvol(Some(2))
            .minbitr(Some(64))
            .maxbitr(Some(320))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        let deserialized: AudioPlacement = serde_json::from_str(&json).unwrap();
        assert_eq!(audio, deserialized);
    }

    // Spec: Object: AudioPlacement — verifies api field accepts array of API framework IDs
    #[test]
    fn test_audio_placement_with_api() {
        let audio = AudioPlacement::builder()
            .api(Some(vec![1, 2, 5]))
            .build()
            .unwrap();

        let api = audio.api.unwrap();
        assert_eq!(api.len(), 3);
        assert_eq!(api, vec![1, 2, 5]);
    }

    // Spec: Object: AudioPlacement — verifies delivery field accepts array of delivery method IDs
    #[test]
    fn test_audio_placement_with_delivery() {
        let audio = AudioPlacement::builder()
            .delivery(Some(vec![1, 2, 3]))
            .build()
            .unwrap();

        let delivery = audio.delivery.unwrap();
        assert_eq!(delivery.len(), 3);
        assert_eq!(delivery, vec![1, 2, 3]);
    }

    // Spec: Object: AudioPlacement — verifies maxext field for maximum extended duration
    #[test]
    fn test_audio_placement_with_maxext() {
        let audio = AudioPlacement::builder().maxext(Some(15)).build().unwrap();

        assert_eq!(audio.maxext, Some(15));
    }

    // Spec: Object: AudioPlacement — verifies empty placement serializes to empty JSON object
    #[test]
    fn test_audio_placement_optional_fields_not_in_json() {
        let audio: AudioPlacement = AudioPlacement::default();
        let json = serde_json::to_string(&audio).unwrap();
        assert_eq!(json, "{}");
    }
}
