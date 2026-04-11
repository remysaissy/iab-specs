use crate::placement::AssetFormat;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// NativeFormat Object (Section 4.4)
///
/// Native ad format requirements including required assets and event tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct NativeFormat<Ext: Extension = crate::DefaultExt> {
    /// Array of asset format specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Vec<AssetFormat>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl NativeFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> NativeFormatBuilder {
        NativeFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_format_builder() {
        let native = NativeFormat::builder().build().unwrap();

        assert!(native.asset.is_none());
    }

    #[test]
    fn test_native_format_default() {
        let native = NativeFormat::builder().build().unwrap();

        assert!(native.asset.is_none());
        assert!(native.ext.is_none());
    }

    #[test]
    fn test_native_format_with_assets() {
        let asset1 = AssetFormat::builder().id(Some(1)).build().unwrap();

        let asset2 = AssetFormat::builder().id(Some(2)).build().unwrap();

        let native = NativeFormat::builder()
            .asset(Some(vec![asset1, asset2]))
            .build()
            .unwrap();

        assert!(native.asset.is_some());
        assert_eq!(native.asset.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_native_format_serialization() {
        let asset = AssetFormat::builder().id(Some(1)).build().unwrap();

        let native = NativeFormat::builder()
            .asset(Some(vec![asset]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&native).unwrap();
        assert!(json.contains("\"asset\""));
    }

    #[test]
    fn test_native_format_deserialization() {
        let json = r#"{"asset":[{"id":1},{"id":2}]}"#;
        let native: NativeFormat = serde_json::from_str(json).unwrap();

        assert!(native.asset.is_some());
        assert_eq!(native.asset.as_ref().unwrap().len(), 2);
    }

    /// AdCOM 1.0 Section 4.4 - NativeFormat serialization roundtrip
    #[test]
    fn test_native_format_serialization_roundtrip() {
        let original = NativeFormat::builder()
            .asset(Some(vec![AssetFormat::builder()
                .id(Some(1))
                .build()
                .unwrap()]))
            .build()
            .unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: NativeFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    /// AdCOM 1.0 Section 4.4 - NativeFormat extension field handling
    #[test]
    fn test_native_format_ext() {
        let obj = NativeFormatBuilder::<serde_json::Value>::default()
            .asset(Some(vec![AssetFormat::builder()
                .id(Some(1))
                .build()
                .unwrap()]))
            .ext(Some(Box::new(
                serde_json::json!({"custom_field": "custom_value"}),
            )))
            .build()
            .unwrap();
        let json = serde_json::to_string(&obj).unwrap();
        assert!(json.contains("custom_field"));
        let deserialized: NativeFormat<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert!(deserialized.ext.is_some());
    }
}
