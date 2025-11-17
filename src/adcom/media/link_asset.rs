use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// LinkAsset Object (Section 3.6)
///
/// Destination link specification with click tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct LinkAsset<Ext: Extension = serde_json::Value> {
    /// Landing URL of the clickable link (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Fallback URL for deeplink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlfb: Option<String>,

    /// Descriptive text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trkr: Option<Vec<String>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl LinkAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> LinkAssetBuilder {
        LinkAssetBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_asset_builder() {
        let link = LinkAsset::builder()
            .url(Some("https://example.com/product".to_string()))
            .build()
            .unwrap();

        assert_eq!(link.url, Some("https://example.com/product".to_string()));
    }

    #[test]
    fn test_link_asset_default() {
        let link = LinkAsset::builder().build().unwrap();

        assert!(link.url.is_none());
        assert!(link.urlfb.is_none());
        assert!(link.trkr.is_none());
    }

    #[test]
    fn test_link_asset_with_fallback() {
        let link = LinkAsset::builder()
            .url(Some("myapp://product/123".to_string()))
            .urlfb(Some("https://example.com/product/123".to_string()))
            .build()
            .unwrap();

        assert_eq!(
            link.urlfb,
            Some("https://example.com/product/123".to_string())
        );
    }

    #[test]
    fn test_link_asset_with_trackers() {
        let link = LinkAsset::builder()
            .url(Some("https://example.com/".to_string()))
            .trkr(Some(vec![
                "https://track1.com".to_string(),
                "https://track2.com".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(
            link.trkr,
            Some(vec![
                "https://track1.com".to_string(),
                "https://track2.com".to_string()
            ])
        );
    }

    #[test]
    fn test_link_asset_serialization() {
        let link = LinkAsset::builder()
            .url(Some("https://example.com/".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains("\"url\":\"https://example.com/\""));
    }

    #[test]
    fn test_link_asset_deserialization() {
        let json = r#"{"url":"https://example.com/page","urlfb":"https://example.com/fallback"}"#;
        let link: LinkAsset = serde_json::from_str(json).unwrap();

        assert_eq!(link.url, Some("https://example.com/page".to_string()));
        assert_eq!(link.urlfb, Some("https://example.com/fallback".to_string()));
    }
}
