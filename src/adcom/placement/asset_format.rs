use crate::Extension;
use crate::adcom::placement::{DataAssetFormat, ImageAssetFormat, TitleAssetFormat};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// AssetFormat Object (Section 4.5)
///
/// Native asset format specifications defining requirements for title, image, or data assets.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AssetFormat<Ext: Extension = serde_json::Value> {
    /// Asset format identifier (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Required flag (1=required, 0=optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<i32>,

    /// Title asset format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<TitleAssetFormat>>,

    /// Image asset format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<Box<ImageAssetFormat>>,

    /// Data asset format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<DataAssetFormat>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl AssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssetFormatBuilder {
        AssetFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_format_builder() {
        let asset = AssetFormat::builder()
            .id(Some(1))
            .req(Some(1))
            .build()
            .unwrap();

        assert_eq!(asset.id, Some(1));
        assert_eq!(asset.req, Some(1));
    }

    #[test]
    fn test_asset_format_default() {
        let asset = AssetFormat::builder().build().unwrap();

        assert!(asset.id.is_none());
        assert!(asset.req.is_none());
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
        assert!(asset.data.is_none());
    }

    #[test]
    fn test_asset_format_with_title() {
        let title = TitleAssetFormat::builder().len(Some(25)).build().unwrap();

        let asset = AssetFormat::builder()
            .id(Some(1))
            .title(Some(Box::new(title)))
            .build()
            .unwrap();

        assert!(asset.title.is_some());
        assert_eq!(asset.title.as_ref().unwrap().len, Some(25));
    }

    #[test]
    fn test_asset_format_with_image() {
        let img = ImageAssetFormat::builder()
            .type_(Some(3))
            .w(Some(1200))
            .h(Some(627))
            .build()
            .unwrap();

        let asset = AssetFormat::builder()
            .id(Some(2))
            .img(Some(Box::new(img)))
            .build()
            .unwrap();

        assert!(asset.img.is_some());
        assert_eq!(asset.img.as_ref().unwrap().w, Some(1200));
    }

    #[test]
    fn test_asset_format_with_data() {
        let data = DataAssetFormat::builder()
            .type_(Some(1))
            .len(Some(25))
            .build()
            .unwrap();

        let asset = AssetFormat::builder()
            .id(Some(3))
            .data(Some(Box::new(data)))
            .build()
            .unwrap();

        assert!(asset.data.is_some());
        assert_eq!(asset.data.as_ref().unwrap().type_, Some(1));
    }

    #[test]
    fn test_asset_format_serialization() {
        let asset = AssetFormat::builder()
            .id(Some(1))
            .req(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&asset).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"req\":1"));
    }

    #[test]
    fn test_asset_format_deserialization() {
        let json = r#"{"id":1,"req":1}"#;
        let asset: AssetFormat = serde_json::from_str(json).unwrap();

        assert_eq!(asset.id, Some(1));
        assert_eq!(asset.req, Some(1));
    }
}
