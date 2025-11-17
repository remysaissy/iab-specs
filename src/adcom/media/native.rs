use crate::Extension;
use crate::adcom::media::{Asset, LinkAsset};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Native Object (Section 3.4)
///
/// Root container for native ad format containing a default link and array
/// of native ad assets.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Native<Ext: Extension = serde_json::Value> {
    /// Default destination link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<LinkAsset>>,

    /// Array of native ad assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Vec<Asset>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Native {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NativeBuilder {
        NativeBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_builder() {
        let native = Native::builder().build().unwrap();

        assert!(native.link.is_none());
        assert!(native.asset.is_none());
    }

    #[test]
    fn test_native_default() {
        let native = Native::builder().build().unwrap();

        assert!(native.link.is_none());
        assert!(native.asset.is_none());
        assert!(native.ext.is_none());
    }

    #[test]
    fn test_native_with_link() {
        let link = LinkAsset::builder()
            .url(Some("https://example.com".to_string()))
            .build()
            .unwrap();

        let native = Native::builder()
            .link(Some(Box::new(link)))
            .build()
            .unwrap();

        assert!(native.link.is_some());
        assert_eq!(
            native.link.as_ref().unwrap().url,
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_native_with_assets() {
        let asset1 = Asset::builder().id(Some(1)).build().unwrap();

        let asset2 = Asset::builder().id(Some(2)).build().unwrap();

        let native = Native::builder()
            .asset(Some(vec![asset1, asset2]))
            .build()
            .unwrap();

        assert!(native.asset.is_some());
        assert_eq!(native.asset.as_ref().unwrap().len(), 2);
        assert_eq!(native.asset.as_ref().unwrap()[0].id, Some(1));
        assert_eq!(native.asset.as_ref().unwrap()[1].id, Some(2));
    }

    #[test]
    fn test_native_serialization() {
        let link = LinkAsset::builder()
            .url(Some("https://example.com".to_string()))
            .build()
            .unwrap();

        let native = Native::builder()
            .link(Some(Box::new(link)))
            .build()
            .unwrap();

        let json = serde_json::to_string(&native).unwrap();
        assert!(json.contains("\"url\":\"https://example.com\""));
    }

    #[test]
    fn test_native_deserialization() {
        let json = r#"{"link":{"url":"https://example.com"}}"#;
        let native: Native = serde_json::from_str(json).unwrap();

        assert!(native.link.is_some());
        assert_eq!(
            native.link.as_ref().unwrap().url,
            Some("https://example.com".to_string())
        );
    }
}
