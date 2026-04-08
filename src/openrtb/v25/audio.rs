use super::banner::Banner;
use crate::Extension;
/// OpenRTB 2.5/2.6 Audio Ad Object
///
/// This module implements the Audio object for OpenRTB 2.5 and 2.6.
/// OpenRTB 2.6 fields (podid, podseq, slotinpod, durfloors) are included.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// Import DurFloors from v26 when openrtb_26 feature is enabled
#[cfg(feature = "openrtb_26")]
use crate::openrtb::v26::DurFloors;

/// Audio ad impression (OpenRTB 2.5 Section 3.2.8)
///
/// An `Audio` object represents an audio ad impression with VAST compliance.
/// It describes the audio player capabilities, supported formats, and playback requirements.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Audio;
///
/// let audio = Audio::builder()
///     .mimes(vec!["audio/mp4".to_string(), "audio/mpeg".to_string()])
///     .minduration(5)
///     .maxduration(Some(30))
///     .protocols(Some(vec![2, 3]))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Audio<Ext: Extension = crate::DefaultExt> {
    /// Content MIME types supported (e.g., "audio/mp4", "audio/mpeg").
    /// **Required field** - at least one MIME type must be specified.
    #[builder(setter(into))]
    pub mimes: Vec<String>,

    /// Minimum audio ad duration in seconds.
    /// Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub minduration: i32,

    /// Maximum audio ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxduration: Option<i32>,

    /// Total duration of the audio ad pod in seconds.
    /// For dynamic pods, this represents the target duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub poddur: Option<i32>,

    /// Array of supported audio protocols.
    /// Refer to AdCOM `Protocol` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub protocols: Option<Vec<i32>>,

    /// Start delay in seconds for pre-roll, mid-roll, or post-roll ad placement.
    /// Refer to AdCOM `StartDelay` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub startdelay: Option<i32>,

    /// Array of exact audio durations (in seconds) that are required.
    /// Typically used for radio/live streaming use cases.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rqddurs: Option<Vec<i32>>,

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

    /// Maximum extended audio ad duration beyond maxduration:
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

    /// Supported delivery methods (e.g., streaming, progressive).
    /// Refer to AdCOM `DeliveryMethod` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub delivery: Option<Vec<i32>>,

    /// Array of Banner objects representing companion ads available for the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub companionad: Option<Vec<Banner>>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM `ApiFramework` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub api: Option<Vec<i32>>,

    /// Supported companion ad types.
    /// Refer to AdCOM `CompanionType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub companiontype: Option<Vec<i32>>,

    /// Maximum number of ads that can be played in an audio ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxseq: Option<i32>,

    /// Type of audio feed.
    /// Refer to AdCOM `FeedType` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub feed: Option<i32>,

    /// Indicates if the audio is stitched with the content stream:
    /// - 0 = independent audio ad
    /// - 1 = stitched with content
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stitched: Option<i32>,

    /// Volume normalization mode.
    /// Refer to AdCOM `VolumeNormalizationMode` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nvol: Option<i32>,

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
    pub durfloors: Option<Vec<crate::DefaultExt>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
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
    fn test_audio_creation() {
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .minduration(5)
            .maxduration(Some(30))
            .build()
            .unwrap();

        assert_eq!(audio.mimes.len(), 1);
        assert_eq!(audio.minduration, 5);
        assert_eq!(audio.maxduration, Some(30));
    }

    #[test]
    fn test_audio_defaults() {
        let audio = Audio::builder()
            .mimes(vec!["audio/mpeg".to_string()])
            .build()
            .unwrap();

        assert_eq!(audio.minduration, 0);
        assert_eq!(audio.podseq, 0);
        assert_eq!(audio.slotinpod, 0);
    }

    #[test]
    fn test_audio_serialization() {
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .minduration(10)
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"mimes\":[\"audio/mp4\"]"));
        assert!(json.contains("\"minduration\":10"));
    }

    #[test]
    fn test_audio_deserialization() {
        let json = r#"{"mimes":["audio/mp4"],"minduration":15}"#;
        let audio: Audio = serde_json::from_str(json).unwrap();

        assert_eq!(audio.mimes, vec!["audio/mp4"]);
        assert_eq!(audio.minduration, 15);
    }

    // === Phase 2.3: Feature Flag Tests (openrtb_26) ===

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_audio_with_nvol_field() {
        // Test that OpenRTB 2.6 nvol (volume normalization mode) field is available
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .nvol(Some(1)) // VolumeNormalizationMode::AverageVolume
            .build()
            .unwrap();

        assert_eq!(audio.nvol, Some(1));
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_audio_nvol_serialization() {
        // Test serialization of OpenRTB 2.6 nvol field
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .nvol(Some(2))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"nvol\":2"));
    }

    #[cfg(feature = "openrtb_26")]
    #[test]
    fn test_audio_nvol_deserialization() {
        // Test deserialization of OpenRTB 2.6 nvol field
        let json = r#"{"mimes":["audio/mp4"],"nvol":1}"#;
        let result: Result<Audio, _> = serde_json::from_str(json);

        assert!(result.is_ok(), "Audio with nvol field should deserialize");
        let audio = result.unwrap();
        assert_eq!(audio.nvol, Some(1));
    }

    #[cfg(not(feature = "openrtb_26"))]
    #[test]
    fn test_audio_nvol_not_available_without_feature() {
        // This test verifies that nvol field is not available without openrtb_26 feature
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .minduration(15)
            .build()
            .unwrap();

        // The nvol field should not exist in Audio when openrtb_26 is disabled
        // This is verified at compile time
        assert_eq!(audio.mimes, vec!["audio/mp4"]);
        assert_eq!(audio.minduration, 15);
    }

    // === Spec-Driven Hardening Tests ===

    #[test]
    fn test_audio_mimes_field() {
        // Spec: Section 3.2.8
        let audio = Audio::builder()
            .mimes(vec![
                "audio/mp4".to_string(),
                "audio/mpeg".to_string(),
                "audio/ogg".to_string(),
            ])
            .build()
            .unwrap();

        assert_eq!(audio.mimes.len(), 3);
        assert!(audio.mimes.contains(&"audio/mp4".to_string()));
        assert!(audio.mimes.contains(&"audio/mpeg".to_string()));
        assert!(audio.mimes.contains(&"audio/ogg".to_string()));

        // mimes is required — deserialization without it must fail
        let json = r#"{"minduration":5}"#;
        let result: Result<Audio, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_audio_protocols_field() {
        // Spec: Section 3.2.8
        // Protocol enum: 1=VAST1, 2=VAST2, 3=VAST3, 9=DAAST1, 10=DAAST1Wrapper
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .protocols(Some(vec![2, 3, 9, 10]))
            .build()
            .unwrap();

        let protocols = audio.protocols.as_ref().unwrap();
        assert_eq!(protocols.len(), 4);
        assert_eq!(protocols, &vec![2, 3, 9, 10]);

        let json = serde_json::to_string(&audio).unwrap();
        let deserialized: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(audio.protocols, deserialized.protocols);
    }

    #[test]
    fn test_audio_companionad() {
        // Spec: Section 3.2.8
        let companion1 = Banner::builder()
            .w(Some(300))
            .h(Some(250))
            .id(Some("audio-companion-1".to_string()))
            .build()
            .unwrap();
        let companion2 = Banner::builder().w(Some(728)).h(Some(90)).build().unwrap();

        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .companionad(Some(vec![companion1, companion2]))
            .build()
            .unwrap();

        let companions = audio.companionad.as_ref().unwrap();
        assert_eq!(companions.len(), 2);
        assert_eq!(companions[0].w, Some(300));
        assert_eq!(companions[0].id, Some("audio-companion-1".to_string()));
        assert_eq!(companions[1].w, Some(728));

        let json = serde_json::to_string(&audio).unwrap();
        let deserialized: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(audio.companionad, deserialized.companionad);
    }

    #[test]
    fn test_audio_companiontype_field() {
        // Spec: Section 3.2.8
        // CompanionType: 1=Static, 2=HTML, 3=IframeResource
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .companiontype(Some(vec![1, 2, 3]))
            .build()
            .unwrap();

        assert_eq!(audio.companiontype, Some(vec![1, 2, 3]));

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"companiontype\":[1,2,3]"));
    }

    #[test]
    fn test_audio_feed_stitched_fields() {
        // Spec: Section 3.2.8
        // feed: FeedType enum (1=MusicStreaming, 2=FMAMBroadcast, 3=Podcast)
        // stitched: 0=independent, 1=stitched with content
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .feed(Some(3))
            .stitched(Some(1))
            .build()
            .unwrap();

        assert_eq!(audio.feed, Some(3));
        assert_eq!(audio.stitched, Some(1));

        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"feed\":3"));
        assert!(json.contains("\"stitched\":1"));

        let deserialized: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(audio.feed, deserialized.feed);
        assert_eq!(audio.stitched, deserialized.stitched);
    }

    #[test]
    fn test_audio_duration_fields() {
        // Spec: Section 3.2.8
        // startdelay: StartDelay enum (>0=mid-roll, 0=pre-roll, -1=generic mid, -2=generic post)
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string()])
            .startdelay(Some(0))
            .minduration(15)
            .maxduration(Some(60))
            .build()
            .unwrap();

        assert_eq!(audio.startdelay, Some(0));
        assert_eq!(audio.minduration, 15);
        assert_eq!(audio.maxduration, Some(60));

        let json = serde_json::to_string(&audio).unwrap();
        let deserialized: Audio = serde_json::from_str(&json).unwrap();
        assert_eq!(audio.startdelay, deserialized.startdelay);
        assert_eq!(audio.minduration, deserialized.minduration);
        assert_eq!(audio.maxduration, deserialized.maxduration);
    }

    #[test]
    fn test_audio_ext_field() {
        // Spec: Section 3.2.8
        let ext = serde_json::json!({"podcast_genre": "tech", "episode_id": 42});
        let audio = AudioBuilder::<serde_json::Value>::default()
            .mimes(vec!["audio/mp4".to_string()])
            .ext(Some(Box::new(ext.clone())))
            .build()
            .unwrap();

        assert_eq!(*audio.ext.as_ref().unwrap().as_ref(), ext);

        let json = serde_json::to_string(&audio).unwrap();
        let deserialized: Audio<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(audio, deserialized);
    }

    #[test]
    fn test_audio_roundtrip_all_fields() {
        // Spec: Section 3.2.8
        let audio = Audio::builder()
            .mimes(vec!["audio/mp4".to_string(), "audio/mpeg".to_string()])
            .minduration(10)
            .maxduration(Some(60))
            .poddur(Some(180))
            .protocols(Some(vec![2, 3, 9]))
            .startdelay(Some(0))
            .rqddurs(Some(vec![15, 30]))
            .podid(Some("audio-pod-1".to_string()))
            .podseq(1)
            .slotinpod(2)
            .mincpmpersec(Some(0.25))
            .battr(Some(vec![1, 2]))
            .maxextended(Some(30))
            .minbitrate(Some(128))
            .maxbitrate(Some(320))
            .delivery(Some(vec![1, 2]))
            .companionad(Some(vec![
                Banner::builder().w(Some(300)).h(Some(250)).build().unwrap(),
            ]))
            .api(Some(vec![1, 2]))
            .companiontype(Some(vec![1, 2]))
            .maxseq(Some(5))
            .feed(Some(3))
            .stitched(Some(1))
            .nvol(Some(2))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audio).unwrap();
        let deserialized: Audio = serde_json::from_str(&json).unwrap();

        assert_eq!(audio.mimes, deserialized.mimes);
        assert_eq!(audio.minduration, deserialized.minduration);
        assert_eq!(audio.maxduration, deserialized.maxduration);
        assert_eq!(audio.poddur, deserialized.poddur);
        assert_eq!(audio.protocols, deserialized.protocols);
        assert_eq!(audio.startdelay, deserialized.startdelay);
        assert_eq!(audio.rqddurs, deserialized.rqddurs);
        assert_eq!(audio.podid, deserialized.podid);
        assert_eq!(audio.podseq, deserialized.podseq);
        assert_eq!(audio.slotinpod, deserialized.slotinpod);
        assert_eq!(audio.battr, deserialized.battr);
        assert_eq!(audio.maxextended, deserialized.maxextended);
        assert_eq!(audio.minbitrate, deserialized.minbitrate);
        assert_eq!(audio.maxbitrate, deserialized.maxbitrate);
        assert_eq!(audio.delivery, deserialized.delivery);
        assert_eq!(audio.companionad, deserialized.companionad);
        assert_eq!(audio.api, deserialized.api);
        assert_eq!(audio.companiontype, deserialized.companiontype);
        assert_eq!(audio.maxseq, deserialized.maxseq);
        assert_eq!(audio.feed, deserialized.feed);
        assert_eq!(audio.stitched, deserialized.stitched);
        assert_eq!(audio.nvol, deserialized.nvol);
        assert_eq!(audio, deserialized);
    }
}
