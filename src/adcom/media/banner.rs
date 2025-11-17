use crate::Extension;
use crate::adcom::media::LinkAsset;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Banner Object (Section 3.3)
///
/// Basic banner creative containing image and link assets.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Banner<Ext: Extension = serde_json::Value> {
    /// Image URL (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,

    /// Destination link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<LinkAsset>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Banner {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BannerBuilder {
        BannerBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_builder() {
        let banner = Banner::builder()
            .img(Some("https://cdn.example.com/banner.jpg".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            banner.img,
            Some("https://cdn.example.com/banner.jpg".to_string())
        );
    }

    #[test]
    fn test_banner_default() {
        let banner = Banner::builder().build().unwrap();

        assert!(banner.img.is_none());
        assert!(banner.link.is_none());
    }

    #[test]
    fn test_banner_with_link() {
        let link = LinkAsset::builder()
            .url(Some("https://advertiser.com/product".to_string()))
            .build()
            .unwrap();

        let banner = Banner::builder()
            .img(Some("https://cdn.example.com/ad.png".to_string()))
            .link(Some(Box::new(link)))
            .build()
            .unwrap();

        assert!(banner.link.is_some());
        assert_eq!(
            banner.link.as_ref().unwrap().url,
            Some("https://advertiser.com/product".to_string())
        );
    }

    #[test]
    fn test_banner_serialization() {
        let banner = Banner::builder()
            .img(Some("https://example.com/image.jpg".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&banner).unwrap();
        assert!(json.contains("\"img\":\"https://example.com/image.jpg\""));
    }

    #[test]
    fn test_banner_deserialization() {
        let json = r#"{"img":"https://example.com/banner.png"}"#;
        let banner: Banner = serde_json::from_str(json).unwrap();

        assert_eq!(
            banner.img,
            Some("https://example.com/banner.png".to_string())
        );
    }
}
