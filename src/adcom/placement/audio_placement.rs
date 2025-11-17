use crate::Extension;
use crate::adcom::placement::Companion;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// AudioPlacement Object (Section 4.11)
///
/// Placement details for audio ad formats.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AudioPlacement<Ext: Extension = serde_json::Value> {
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

    /// Feed type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<i32>,

    /// Volume normalization mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvol: Option<i32>,

    /// MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// API frameworks supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Creative subtypes permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<Vec<i32>>,

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

impl AudioPlacement {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AudioPlacementBuilder {
        AudioPlacementBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_placement_builder() {
        let audio = AudioPlacement::builder()
            .delay(Some(0))
            .feed(Some(1))
            .mindur(Some(15))
            .maxdur(Some(30))
            .build()
            .unwrap();

        assert_eq!(audio.delay, Some(0));
        assert_eq!(audio.feed, Some(1));
        assert_eq!(audio.mindur, Some(15));
        assert_eq!(audio.maxdur, Some(30));
    }

    #[test]
    fn test_audio_placement_default() {
        let audio = AudioPlacement::builder().build().unwrap();

        assert!(audio.delay.is_none());
        assert!(audio.skip.is_none());
        assert!(audio.mindur.is_none());
        assert!(audio.maxdur.is_none());
        assert!(audio.mime.is_none());
    }

    #[test]
    fn test_audio_placement_skippable() {
        let audio = AudioPlacement::builder()
            .skip(Some(1))
            .skipmin(Some(5))
            .skipafter(Some(5))
            .build()
            .unwrap();

        assert_eq!(audio.skip, Some(1));
        assert_eq!(audio.skipmin, Some(5));
        assert_eq!(audio.skipafter, Some(5));
    }

    #[test]
    fn test_audio_placement_duration() {
        let audio = AudioPlacement::builder()
            .mindur(Some(15))
            .maxdur(Some(60))
            .maxext(Some(120))
            .build()
            .unwrap();

        assert_eq!(audio.mindur, Some(15));
        assert_eq!(audio.maxdur, Some(60));
        assert_eq!(audio.maxext, Some(120));
    }

    #[test]
    fn test_audio_placement_bitrate() {
        let audio = AudioPlacement::builder()
            .minbitrate(Some(64))
            .maxbitrate(Some(320))
            .build()
            .unwrap();

        assert_eq!(audio.minbitrate, Some(64));
        assert_eq!(audio.maxbitrate, Some(320));
    }

    #[test]
    fn test_audio_placement_with_mime_and_api() {
        let audio = AudioPlacement::builder()
            .mime(Some(vec!["audio/mp3".to_string(), "audio/aac".to_string()]))
            .api(Some(vec![1, 2]))
            .build()
            .unwrap();

        assert_eq!(
            audio.mime,
            Some(vec!["audio/mp3".to_string(), "audio/aac".to_string()])
        );
        assert_eq!(audio.api, Some(vec![1, 2]));
    }

    #[test]
    fn test_audio_placement_serialization() {
        let audio = AudioPlacement::builder()
            .delay(Some(0))
            .mindur(Some(15))
            .maxdur(Some(30))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"delay\":0"));
        assert!(json.contains("\"mindur\":15"));
        assert!(json.contains("\"maxdur\":30"));
    }

    #[test]
    fn test_audio_placement_deserialization() {
        let json = r#"{"delay":0,"mindur":15,"maxdur":30,"feed":1}"#;
        let audio: AudioPlacement = serde_json::from_str(json).unwrap();

        assert_eq!(audio.delay, Some(0));
        assert_eq!(audio.mindur, Some(15));
        assert_eq!(audio.maxdur, Some(30));
        assert_eq!(audio.feed, Some(1));
    }
}
