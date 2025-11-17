use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// TitleAssetFormat Object (Section 4.6)
///
/// Title asset constraints for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct TitleAssetFormat<Ext: Extension = serde_json::Value> {
    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl TitleAssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TitleAssetFormatBuilder {
        TitleAssetFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_asset_format_builder() {
        let title = TitleAssetFormat::builder().len(Some(25)).build().unwrap();

        assert_eq!(title.len, Some(25));
    }

    #[test]
    fn test_title_asset_format_default() {
        let title = TitleAssetFormat::builder().build().unwrap();

        assert!(title.len.is_none());
        assert!(title.ext.is_none());
    }

    #[test]
    fn test_title_asset_format_serialization() {
        let title = TitleAssetFormat::builder().len(Some(90)).build().unwrap();

        let json = serde_json::to_string(&title).unwrap();
        assert!(json.contains("\"len\":90"));
    }

    #[test]
    fn test_title_asset_format_deserialization() {
        let json = r#"{"len":25}"#;
        let title: TitleAssetFormat = serde_json::from_str(json).unwrap();

        assert_eq!(title.len, Some(25));
    }
}
