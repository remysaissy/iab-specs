use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ImageAssetFormat Object (Section 4.7)
///
/// Image asset constraints for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ImageAssetFormat<Ext: Extension = serde_json::Value> {
    /// Native image asset type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Array of MIME types supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Minimum width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Minimum height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,

    /// Width as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Height as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl ImageAssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ImageAssetFormatBuilder {
        ImageAssetFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_asset_format_builder() {
        let img = ImageAssetFormat::builder()
            .type_(Some(3))
            .w(Some(1200))
            .h(Some(627))
            .build()
            .unwrap();

        assert_eq!(img.type_, Some(3));
        assert_eq!(img.w, Some(1200));
        assert_eq!(img.h, Some(627));
    }

    #[test]
    fn test_image_asset_format_default() {
        let img = ImageAssetFormat::builder().build().unwrap();

        assert!(img.type_.is_none());
        assert!(img.w.is_none());
        assert!(img.h.is_none());
        assert!(img.mime.is_none());
    }

    #[test]
    fn test_image_asset_format_with_ratio() {
        let img = ImageAssetFormat::builder()
            .wratio(Some(16))
            .hratio(Some(9))
            .build()
            .unwrap();

        assert_eq!(img.wratio, Some(16));
        assert_eq!(img.hratio, Some(9));
    }

    #[test]
    fn test_image_asset_format_with_min_sizes() {
        let img = ImageAssetFormat::builder()
            .wmin(Some(200))
            .hmin(Some(200))
            .build()
            .unwrap();

        assert_eq!(img.wmin, Some(200));
        assert_eq!(img.hmin, Some(200));
    }

    #[test]
    fn test_image_asset_format_with_mime() {
        let img = ImageAssetFormat::builder()
            .mime(Some(vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(
            img.mime,
            Some(vec!["image/jpeg".to_string(), "image/png".to_string()])
        );
    }

    #[test]
    fn test_image_asset_format_serialization() {
        let img = ImageAssetFormat::builder()
            .type_(Some(3))
            .w(Some(1200))
            .h(Some(627))
            .build()
            .unwrap();

        let json = serde_json::to_string(&img).unwrap();
        assert!(json.contains("\"type_\":3"));
        assert!(json.contains("\"w\":1200"));
        assert!(json.contains("\"h\":627"));
    }

    #[test]
    fn test_image_asset_format_deserialization() {
        let json = r#"{"type_":3,"w":1200,"h":627}"#;
        let img: ImageAssetFormat = serde_json::from_str(json).unwrap();

        assert_eq!(img.type_, Some(3));
        assert_eq!(img.w, Some(1200));
        assert_eq!(img.h, Some(627));
    }
}
