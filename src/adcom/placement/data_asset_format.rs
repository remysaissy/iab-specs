use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DataAssetFormat Object (Section 4.8)
///
/// Data asset constraints for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DataAssetFormat<Ext: Extension = serde_json::Value> {
    /// Data asset type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DataAssetFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataAssetFormatBuilder {
        DataAssetFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_asset_format_builder() {
        let data = DataAssetFormat::builder()
            .type_(Some(1))
            .len(Some(25))
            .build()
            .unwrap();

        assert_eq!(data.type_, Some(1));
        assert_eq!(data.len, Some(25));
    }

    #[test]
    fn test_data_asset_format_default() {
        let data = DataAssetFormat::builder().build().unwrap();

        assert!(data.type_.is_none());
        assert!(data.len.is_none());
        assert!(data.ext.is_none());
    }

    #[test]
    fn test_data_asset_format_serialization() {
        let data = DataAssetFormat::builder()
            .type_(Some(2))
            .len(Some(140))
            .build()
            .unwrap();

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"type_\":2"));
        assert!(json.contains("\"len\":140"));
    }

    #[test]
    fn test_data_asset_format_deserialization() {
        let json = r#"{"type_":1,"len":25}"#;
        let data: DataAssetFormat = serde_json::from_str(json).unwrap();

        assert_eq!(data.type_, Some(1));
        assert_eq!(data.len, Some(25));
    }
}
