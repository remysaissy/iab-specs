use super::audio::Audio;
use super::banner::Banner;
use super::native::Native;
use super::video::Video;
use crate::Extension;
/// OpenRTB 2.5/2.6 Impression Object
///
/// This module implements the Imp (Impression) object for OpenRTB 2.5 and 2.6.
/// OpenRTB 2.6 fields (qty, dt, refresh) are included.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// Import Qty and Refresh from v26 when openrtb_26 feature is enabled
#[cfg(feature = "openrtb_26")]
use crate::openrtb::v26::{Qty, Refresh};

/// Default currency for bid floor (USD per OpenRTB 2.5 spec)
fn default_bidfloorcur() -> String {
    "USD".to_string()
}

/// Impression object (OpenRTB 2.5 Section 3.2.4)
///
/// An `Imp` object describes an ad placement being auctioned within a bid request.
/// At least one of `banner`, `video`, `audio`, or `native` must be present.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `MetricExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
/// * `PmpExt` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "Ext: Extension, MetricExt: Extension, PmpExt: Extension",
    deserialize = "Ext: Extension, MetricExt: Extension, PmpExt: Extension"
))]
pub struct Imp<
    Ext: Extension = serde_json::Value,
    MetricExt: Extension = serde_json::Value,
    PmpExt: Extension = serde_json::Value,
> {
    /// Unique identifier for this impression within the context of the bid request.
    /// **Required field**.
    #[builder(setter(into))]
    pub id: String,

    /// Array of Metric objects for viewability or verification measurement.
    /// Uses placeholder until Metric is implemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metric: Option<Vec<Box<MetricExt>>>,

    /// Banner object indicating a banner impression is offered.
    /// At least one of banner, video, audio, or native must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub banner: Option<Banner>,

    /// Video object indicating a video impression is offered.
    /// At least one of banner, video, audio, or native must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub video: Option<Video>,

    /// Audio object indicating an audio impression is offered.
    /// At least one of banner, video, audio, or native must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub audio: Option<Audio>,

    /// Native object indicating a native ad impression is offered.
    /// At least one of banner, video, audio, or native must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub native: Option<Native>,

    /// Pmp object containing private marketplace deals.
    /// Uses placeholder until Pmp is implemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pmp: Option<Box<PmpExt>>,

    /// Name of ad mediation partner, SDK technology, or player responsible for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub displaymanager: Option<String>,

    /// Version of the display manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub displaymanagerver: Option<String>,

    /// Indicator of interstitial or full-screen placement:
    /// - 0 = not interstitial (default)
    /// - 1 = interstitial or full-screen
    #[serde(default)]
    #[builder(default)]
    pub instl: i32,

    /// Identifier for specific ad placement or ad tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tagid: Option<String>,

    /// Minimum bid for this impression expressed in CPM.
    /// Default is 0.
    #[serde(default)]
    #[builder(default)]
    pub bidfloor: f64,

    /// Currency for bid floor using ISO-4217 alpha codes.
    /// Default is "USD".
    #[serde(default = "default_bidfloorcur")]
    #[builder(default = "default_bidfloorcur()")]
    pub bidfloorcur: String,

    /// Indicates the type of browser opened when clicking the ad:
    /// - 0 = embedded browser
    /// - 1 = native browser
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clickbrowser: Option<i32>,

    /// Flag to indicate if the impression requires secure HTTPS URL creative assets:
    /// - 0 = non-secure
    /// - 1 = secure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub secure: Option<i32>,

    /// Array of exchange-specific names of supported iframe busters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub iframebuster: Option<Vec<String>>,

    /// Indicates if the user receives a reward for viewing the ad:
    /// - 0 = no reward (default)
    /// - 1 = user receives reward
    #[serde(default)]
    #[builder(default)]
    pub rwdd: i32,

    /// Server-side ad insertion indicator:
    /// - 0 = no server-side insertion (default)
    /// - 1 = server-side ad insertion in use
    #[serde(default)]
    #[builder(default)]
    pub ssai: i32,

    /// Advisory as to the number of seconds that may elapse between the auction
    /// and the actual impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub exp: Option<i32>,

    /// Qty object containing impression multiplier information (OpenRTB 2.6+).
    /// Used for DOOH multi-viewer impression counting.
    #[cfg(feature = "openrtb_26")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub qty: Option<Qty>,

    /// Qty object containing impression multiplier information (placeholder for v2.5).
    /// When using openrtb_26 feature, use the typed Qty version instead.
    #[cfg(not(feature = "openrtb_26"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub qty: Option<serde_json::Value>,

    /// Unix timestamp for impression fulfillment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dt: Option<f64>,

    /// Refresh object containing auto-refresh details (OpenRTB 2.6+).
    /// Used for rotating ad slots in continuous display contexts.
    #[cfg(feature = "openrtb_26")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub refresh: Option<Refresh>,

    /// Refresh object containing auto-refresh details (placeholder for v2.5).
    /// When using openrtb_26 feature, use the typed Refresh version instead.
    #[cfg(not(feature = "openrtb_26"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub refresh: Option<serde_json::Value>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Imp {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImpBuilder {
        ImpBuilder::create_empty()
    }
}

impl<Ext: Extension, MetricExt: Extension, PmpExt: Extension> Default
    for Imp<Ext, MetricExt, PmpExt>
{
    fn default() -> Self {
        Self {
            id: String::new(),
            metric: None,
            banner: None,
            video: None,
            audio: None,
            native: None,
            pmp: None,
            displaymanager: None,
            displaymanagerver: None,
            instl: 0,
            tagid: None,
            bidfloor: 0.0,
            bidfloorcur: default_bidfloorcur(),
            clickbrowser: None,
            secure: None,
            iframebuster: None,
            rwdd: 0,
            ssai: 0,
            exp: None,
            qty: None,
            dt: None,
            refresh: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imp_with_banner() {
        let imp = Imp::builder()
            .id("imp1".to_string())
            .banner(Some(
                Banner::builder().w(Some(300)).h(Some(250)).build().unwrap(),
            ))
            .bidfloor(1.5)
            .build()
            .unwrap();

        assert_eq!(imp.id, "imp1");
        assert!(imp.banner.is_some());
        assert_eq!(imp.bidfloor, 1.5);
        assert_eq!(imp.bidfloorcur, "USD"); // Default value
    }

    #[test]
    fn test_imp_with_video() {
        let imp = Imp::builder()
            .id("imp2".to_string())
            .video(Some(
                Video::builder()
                    .mimes(vec!["video/mp4".to_string()])
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(imp.id, "imp2");
        assert!(imp.video.is_some());
    }

    #[test]
    fn test_imp_defaults() {
        let imp = Imp::builder().id("imp3".to_string()).build().unwrap();

        assert_eq!(imp.instl, 0);
        assert_eq!(imp.bidfloor, 0.0);
        assert_eq!(imp.bidfloorcur, "USD");
        assert_eq!(imp.rwdd, 0);
        assert_eq!(imp.ssai, 0);
    }

    #[test]
    fn test_imp_serialization() {
        let imp = Imp::builder()
            .id("imp1".to_string())
            .bidfloor(2.0)
            .build()
            .unwrap();

        let json = serde_json::to_string(&imp).unwrap();
        assert!(json.contains("\"id\":\"imp1\""));
        assert!(json.contains("\"bidfloor\":2"));
    }

    #[test]
    fn test_imp_deserialization() {
        let json = r#"{"id":"imp1","bidfloor":1.5}"#;
        let imp: Imp = serde_json::from_str(json).unwrap();

        assert_eq!(imp.id, "imp1");
        assert_eq!(imp.bidfloor, 1.5);
    }
}
