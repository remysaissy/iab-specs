use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ImageAsset Object (Section 3.8)
///
/// Image specification for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ImageAsset<Ext: Extension = serde_json::Value> {
    /// Image URL (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Native image asset type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl ImageAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImageAssetBuilder {
        ImageAssetBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_asset_builder() {
        let img = ImageAsset::builder()
            .url(Some("https://cdn.example.com/logo.png".to_string()))
            .w(Some(300))
            .h(Some(250))
            .type_(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            img.url,
            Some("https://cdn.example.com/logo.png".to_string())
        );
        assert_eq!(img.w, Some(300));
        assert_eq!(img.h, Some(250));
        assert_eq!(img.type_, Some(1));
    }

    #[test]
    fn test_image_asset_default() {
        let img = ImageAsset::builder().build().unwrap();

        assert!(img.url.is_none());
        assert!(img.w.is_none());
        assert!(img.h.is_none());
    }

    #[test]
    fn test_image_asset_serialization() {
        let img = ImageAsset::builder()
            .url(Some("https://example.com/image.jpg".to_string()))
            .w(Some(1200))
            .h(Some(628))
            .build()
            .unwrap();

        let json = serde_json::to_string(&img).unwrap();
        assert!(json.contains("\"url\":\"https://example.com/image.jpg\""));
        assert!(json.contains("\"w\":1200"));
        assert!(json.contains("\"h\":628"));
    }

    #[test]
    fn test_image_asset_deserialization() {
        let json = r#"{"url":"https://example.com/icon.png","w":50,"h":50,"type_":3}"#;
        let img: ImageAsset = serde_json::from_str(json).unwrap();

        assert_eq!(img.url, Some("https://example.com/icon.png".to_string()));
        assert_eq!(img.w, Some(50));
        assert_eq!(img.h, Some(50));
        assert_eq!(img.type_, Some(3));
    }
}
