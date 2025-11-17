use crate::Extension;
use crate::adcom::media::{DataAsset, ImageAsset, LinkAsset, TitleAsset, VideoAsset};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Asset Object (Section 3.5)
///
/// Container for a native ad component. Must include exactly one asset subtype
/// (title, img, video, or data).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Asset<Ext: Extension = serde_json::Value> {
    /// Asset format ID reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Required flag (1=required, 0=optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<i32>,

    /// Title asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<TitleAsset>>,

    /// Image asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<Box<ImageAsset>>,

    /// Video asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<VideoAsset>>,

    /// Data asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<DataAsset>>,

    /// Link asset for this specific asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<LinkAsset>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Asset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssetBuilder {
        AssetBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_builder() {
        let asset = Asset::builder().id(Some(1)).req(Some(1)).build().unwrap();

        assert_eq!(asset.id, Some(1));
        assert_eq!(asset.req, Some(1));
    }

    #[test]
    fn test_asset_default() {
        let asset = Asset::builder().build().unwrap();

        assert!(asset.id.is_none());
        assert!(asset.req.is_none());
        assert!(asset.title.is_none());
        assert!(asset.img.is_none());
    }

    #[test]
    fn test_asset_with_title() {
        let title = TitleAsset::builder()
            .text(Some("Sponsored Content".to_string()))
            .build()
            .unwrap();

        let asset = Asset::builder()
            .id(Some(1))
            .title(Some(Box::new(title)))
            .build()
            .unwrap();

        assert!(asset.title.is_some());
        assert_eq!(
            asset.title.as_ref().unwrap().text,
            Some("Sponsored Content".to_string())
        );
    }

    #[test]
    fn test_asset_with_image() {
        let img = ImageAsset::builder()
            .url(Some("https://example.com/image.jpg".to_string()))
            .build()
            .unwrap();

        let asset = Asset::builder()
            .id(Some(2))
            .img(Some(Box::new(img)))
            .build()
            .unwrap();

        assert!(asset.img.is_some());
    }

    #[test]
    fn test_asset_serialization() {
        let asset = Asset::builder().id(Some(1)).req(Some(1)).build().unwrap();

        let json = serde_json::to_string(&asset).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"req\":1"));
    }

    #[test]
    fn test_asset_deserialization() {
        let json = r#"{"id":1,"req":1}"#;
        let asset: Asset = serde_json::from_str(json).unwrap();

        assert_eq!(asset.id, Some(1));
        assert_eq!(asset.req, Some(1));
    }
}
