use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// TitleAsset Object (Section 3.7)
///
/// Text heading for native ad.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct TitleAsset<Ext: Extension = serde_json::Value> {
    /// Title text (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl TitleAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TitleAssetBuilder {
        TitleAssetBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_asset_builder() {
        let title = TitleAsset::builder()
            .text(Some("Best Product Ever".to_string()))
            .len(Some(25))
            .build()
            .unwrap();

        assert_eq!(title.text, Some("Best Product Ever".to_string()));
        assert_eq!(title.len, Some(25));
    }

    #[test]
    fn test_title_asset_default() {
        let title = TitleAsset::builder().build().unwrap();

        assert!(title.text.is_none());
        assert!(title.len.is_none());
    }

    #[test]
    fn test_title_asset_serialization() {
        let title = TitleAsset::builder()
            .text(Some("Sponsored Content".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&title).unwrap();
        assert!(json.contains("\"text\":\"Sponsored Content\""));
    }

    #[test]
    fn test_title_asset_deserialization() {
        let json = r#"{"text":"Amazing Deal","len":20}"#;
        let title: TitleAsset = serde_json::from_str(json).unwrap();

        assert_eq!(title.text, Some("Amazing Deal".to_string()));
        assert_eq!(title.len, Some(20));
    }
}
