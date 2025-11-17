use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DataAsset Object (Section 3.10)
///
/// Data/text content for native ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DataAsset<Ext: Extension = serde_json::Value> {
    /// Formatted string of data (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Maximum character length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// Type of data asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DataAsset {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataAssetBuilder {
        DataAssetBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_asset_builder() {
        let data = DataAsset::builder()
            .value(Some("Sponsored by Brand".to_string()))
            .len(Some(100))
            .type_(Some(1))
            .build()
            .unwrap();

        assert_eq!(data.value, Some("Sponsored by Brand".to_string()));
        assert_eq!(data.len, Some(100));
        assert_eq!(data.type_, Some(1));
    }

    #[test]
    fn test_data_asset_default() {
        let data = DataAsset::builder().build().unwrap();

        assert!(data.value.is_none());
        assert!(data.len.is_none());
        assert!(data.type_.is_none());
    }

    #[test]
    fn test_data_asset_serialization() {
        let data = DataAsset::builder()
            .value(Some("4.5 stars".to_string()))
            .type_(Some(3))
            .build()
            .unwrap();

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"value\":\"4.5 stars\""));
        assert!(json.contains("\"type_\":3"));
    }

    #[test]
    fn test_data_asset_deserialization() {
        let json = r#"{"value":"Download Now","len":50,"type_":12}"#;
        let data: DataAsset = serde_json::from_str(json).unwrap();

        assert_eq!(data.value, Some("Download Now".to_string()));
        assert_eq!(data.len, Some(50));
        assert_eq!(data.type_, Some(12));
    }
}
