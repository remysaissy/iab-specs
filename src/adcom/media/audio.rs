use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Audio Object (Section 3.13)
///
/// Details specific to audio ads including format, duration, and delivery.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Audio<Ext: Extension = serde_json::Value> {
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

    /// Ad markup (DAAST document)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,

    /// Markup URL for server-side retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<String>,

    /// Audio protocols
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,

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

impl Audio {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AudioBuilder {
        AudioBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_builder() {
        let audio = Audio::builder()
            .mimes(Some(vec!["audio/mp3".to_string()]))
            .dur(Some(30))
            .bitrate(Some(128))
            .build()
            .unwrap();

        assert_eq!(audio.mimes, Some(vec!["audio/mp3".to_string()]));
        assert_eq!(audio.dur, Some(30));
        assert_eq!(audio.bitrate, Some(128));
    }

    #[test]
    fn test_audio_default() {
        let audio = Audio::builder().build().unwrap();

        assert!(audio.mimes.is_none());
        assert!(audio.dur.is_none());
        assert!(audio.bitrate.is_none());
    }

    #[test]
    fn test_audio_with_protocols() {
        let audio = Audio::builder()
            .mimes(Some(vec!["audio/aac".to_string()]))
            .protocols(Some(vec![2, 3, 5]))
            .build()
            .unwrap();

        assert_eq!(audio.protocols, Some(vec![2, 3, 5]));
    }

    #[test]
    fn test_audio_with_apis() {
        let audio = Audio::builder()
            .mimes(Some(vec!["audio/mp4".to_string()]))
            .apis(Some(vec![3, 5]))
            .build()
            .unwrap();

        assert_eq!(audio.apis, Some(vec![3, 5]));
    }

    #[test]
    fn test_audio_serialization() {
        let audio = Audio::builder()
            .mimes(Some(vec!["audio/mp3".to_string(), "audio/aac".to_string()]))
            .dur(Some(15))
            .bitrate(Some(64))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"mimes\":[\"audio/mp3\",\"audio/aac\"]"));
        assert!(json.contains("\"dur\":15"));
        assert!(json.contains("\"bitrate\":64"));
    }

    #[test]
    fn test_audio_deserialization() {
        let json = r#"{"mimes":["audio/mp3"],"dur":30,"bitrate":128}"#;
        let audio: Audio = serde_json::from_str(json).unwrap();

        assert_eq!(audio.mimes, Some(vec!["audio/mp3".to_string()]));
        assert_eq!(audio.dur, Some(30));
        assert_eq!(audio.bitrate, Some(128));
    }

    #[test]
    fn test_audio_with_daast() {
        let audio = Audio::builder()
            .mimes(Some(vec!["audio/mp3".to_string()]))
            .adm(Some("<DAAST version=\"1.0\">...</DAAST>".to_string()))
            .build()
            .unwrap();

        assert!(audio.adm.is_some());
        assert!(audio.adm.as_ref().unwrap().contains("<DAAST"));
    }
}
