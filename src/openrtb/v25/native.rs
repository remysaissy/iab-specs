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
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Native;
///
/// let native = Native {
///     request: r#"{"ver":"1.2","assets":[...]}"#.to_string(),
///     ver: Some("1.2".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Native {
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
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_creation() {
        let native = Native {
            request: r#"{"ver":"1.2"}"#.to_string(),
            ver: Some("1.2".to_string()),
            ..Default::default()
        };

        assert_eq!(native.request, r#"{"ver":"1.2"}"#);
        assert_eq!(native.ver, Some("1.2".to_string()));
    }

    #[test]
    fn test_native_serialization() {
        let native = Native {
            request: r#"{"ver":"1.2"}"#.to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&native).unwrap();
        assert!(json.contains(r#""request":"{\"ver\":\"1.2\"}""#));
    }

    #[test]
    fn test_native_deserialization() {
        let json = r#"{"request":"{\"ver\":\"1.2\"}"}"#;
        let native: Native = serde_json::from_str(json).unwrap();

        assert_eq!(native.request, r#"{"ver":"1.2"}"#);
    }
}
