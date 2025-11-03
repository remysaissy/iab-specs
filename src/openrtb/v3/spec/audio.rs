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
/// # Example
///
/// ```rust
/// use iab_specs::openrtb::v3::spec::AudioPlacement;
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let audio = AudioPlacement {
///     mindur: Some(5),
///     maxdur: Some(30),
///     maxext: Some(10),
///     feed: Some(1), // Music service
///     nvol: Some(100), // Normal volume
///     mime: Some(vec![
///         "audio/mp4".to_string(),
///         "audio/mpeg".to_string(),
///     ]),
///     ..Default::default()
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct AudioPlacement {
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
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_placement_creation() {
        let audio = AudioPlacement {
            mindur: Some(5),
            maxdur: Some(30),
            feed: Some(1),
            ..Default::default()
        };

        assert_eq!(audio.mindur, Some(5));
        assert_eq!(audio.maxdur, Some(30));
        assert_eq!(audio.feed, Some(1));
    }

    #[test]
    fn test_audio_placement_with_protocols() {
        let audio = AudioPlacement {
            protocol: Some(vec![2, 3, 5, 6]),
            ..Default::default()
        };

        assert_eq!(audio.protocol.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_audio_placement_with_mime_types() {
        let audio = AudioPlacement {
            mime: Some(vec![
                "audio/mp4".to_string(),
                "audio/mpeg".to_string(),
                "audio/ogg".to_string(),
            ]),
            ..Default::default()
        };

        assert_eq!(audio.mime.as_ref().unwrap().len(), 3);
        assert!(
            audio
                .mime
                .as_ref()
                .unwrap()
                .contains(&"audio/mp4".to_string())
        );
    }

    #[test]
    fn test_audio_placement_podcast() {
        let audio = AudioPlacement {
            feed: Some(3), // Podcast
            mindur: Some(15),
            maxdur: Some(60),
            ..Default::default()
        };

        assert_eq!(audio.feed, Some(3));
    }

    #[test]
    fn test_audio_placement_bitrate() {
        let audio = AudioPlacement {
            minbitr: Some(64),
            maxbitr: Some(320),
            ..Default::default()
        };

        assert_eq!(audio.minbitr, Some(64));
        assert_eq!(audio.maxbitr, Some(320));
    }

    #[test]
    fn test_audio_placement_stitched() {
        let audio = AudioPlacement {
            stitched: Some(1),
            ..Default::default()
        };

        assert_eq!(audio.stitched, Some(1));
    }

    #[test]
    fn test_audio_placement_serialization() {
        let audio = AudioPlacement {
            mindur: Some(5),
            maxdur: Some(30),
            feed: Some(1),
            ..Default::default()
        };

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"mindur\":5"));
        assert!(json.contains("\"maxdur\":30"));
        assert!(json.contains("\"feed\":1"));
    }

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

    #[test]
    fn test_audio_placement_builder() {
        let audio = AudioPlacementBuilder::default()
            .mindur(Some(5))
            .maxdur(Some(30))
            .feed(Some(1))
            .build()
            .unwrap();

        assert_eq!(audio.mindur, Some(5));
        assert_eq!(audio.feed, Some(1));
    }

    #[test]
    fn test_audio_placement_volume_normalization() {
        let audio = AudioPlacement {
            nvol: Some(3), // Loudness normalization
            ..Default::default()
        };

        assert_eq!(audio.nvol, Some(3));
    }
}
