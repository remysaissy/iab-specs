use crate::Extension;
/// OpenRTB 2.5 Native Ad Object
///
/// This module implements the Native object for OpenRTB 2.5.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Native ad impression (OpenRTB 2.5 Section 3.2.9)
///
/// A `Native` object represents a native ad impression conforming to the
/// Dynamic Native Ads API specification. The actual native ad request is
/// JSON-encoded in the `request` field.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Native;
///
/// let native = Native::builder()
///     .request(r#"{"ver":"1.2","assets":[...]}"#.to_string())
///     .ver(Some("1.2".to_string()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Native<Ext: Extension = crate::DefaultExt> {
    /// JSON-encoded native ad request payload conforming to the Dynamic Native Ads API.
    /// **Required field**.
    #[builder(setter(into))]
    pub request: String,

    /// Version of the Dynamic Native Ads API to which the request complies.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ver: Option<String>,

    /// List of supported API frameworks for this impression.
    /// Refer to AdCOM `ApiFramework` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub api: Option<Vec<i32>>,

    /// Blocked creative attributes.
    /// Refer to AdCOM `CreativeAttribute` enumeration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub battr: Option<Vec<i32>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
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
    fn test_native_creation() {
        let native = Native::builder()
            .request(r#"{"ver":"1.2"}"#.to_string())
            .ver(Some("1.2".to_string()))
            .build()
            .unwrap();

        assert_eq!(native.request, r#"{"ver":"1.2"}"#);
        assert_eq!(native.ver, Some("1.2".to_string()));
    }

    #[test]
    fn test_native_serialization() {
        let native = Native::builder()
            .request(r#"{"ver":"1.2"}"#.to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&native).unwrap();
        assert!(json.contains(r#""request":"{\"ver\":\"1.2\"}""#));
    }

    #[test]
    fn test_native_deserialization() {
        let json = r#"{"request":"{\"ver\":\"1.2\"}"}"#;
        let native: Native = serde_json::from_str(json).unwrap();

        assert_eq!(native.request, r#"{"ver":"1.2"}"#);
    }

    #[test]
    fn test_native_with_api() {
        let native = Native::builder()
            .request(r#"{"ver":"1.2"}"#.to_string())
            .ver(Some("1.2".to_string()))
            .api(Some(vec![3, 5]))
            .build()
            .unwrap();

        assert_eq!(native.api.as_ref().unwrap().len(), 2);
        assert_eq!(native.api.as_ref().unwrap()[0], 3);
    }

    #[test]
    fn test_native_with_battr() {
        let native = Native::builder()
            .request(r#"{"ver":"1.2"}"#.to_string())
            .battr(Some(vec![1, 2, 3]))
            .build()
            .unwrap();

        assert_eq!(native.battr.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_native_with_ext() {
        let ext_value = Box::new(serde_json::json!({"custom": "native_data"}));

        let native = Native {
            request: r#"{"ver":"1.2"}"#.to_string(),
            ext: Some(ext_value.clone()),
            ..Default::default()
        };

        assert_eq!(native.ext, Some(ext_value));
    }

    // === Spec-Driven Hardening Tests ===

    #[test]
    fn test_native_request_required() {
        // Spec: Section 3.2.5
        let json_missing = r#"{"ver":"1.2"}"#;
        let result: Result<Native, _> = serde_json::from_str(json_missing);
        assert!(
            result.is_err(),
            "Native without required 'request' field should fail"
        );

        let json_empty = r#"{"request":""}"#;
        let result_empty: Result<Native, _> = serde_json::from_str(json_empty);
        assert!(result_empty.is_ok());
        assert_eq!(result_empty.unwrap().request, "");
    }

    #[test]
    fn test_native_ver_field() {
        // Spec: Section 3.2.5
        let native_with_ver = Native::builder()
            .request(r#"{"ver":"1.2","assets":[]}"#.to_string())
            .ver(Some("1.2".to_string()))
            .build()
            .unwrap();
        assert_eq!(native_with_ver.ver, Some("1.2".to_string()));

        let native_without_ver = Native::builder()
            .request(r#"{"assets":[]}"#.to_string())
            .build()
            .unwrap();
        assert!(native_without_ver.ver.is_none());

        let json = serde_json::to_string(&native_with_ver).unwrap();
        assert!(json.contains("\"ver\":\"1.2\""));

        let json_no_ver = serde_json::to_string(&native_without_ver).unwrap();
        assert!(!json_no_ver.contains("ver"));
    }

    #[test]
    fn test_native_roundtrip_all_fields() {
        // Spec: Section 3.2.5
        let native = Native::builder()
            .request(
                r#"{"ver":"1.2","assets":[{"id":1,"required":1,"title":{"len":90}}]}"#.to_string(),
            )
            .ver(Some("1.2".to_string()))
            .api(Some(vec![3, 5, 6]))
            .battr(Some(vec![1, 2, 13]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&native).unwrap();
        let deserialized: Native = serde_json::from_str(&json).unwrap();

        assert_eq!(native.request, deserialized.request);
        assert_eq!(native.ver, deserialized.ver);
        assert_eq!(native.api, deserialized.api);
        assert_eq!(native.battr, deserialized.battr);
        assert_eq!(native, deserialized);
    }
}
