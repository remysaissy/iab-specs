use super::{Request, Response};
use crate::Extension;
/// OpenRTB 3.0 Root Container Object
///
/// This module implements the root Openrtb object which serves as the top-level
/// container for all OpenRTB 3.0 transactions.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Openrtb root container (OpenRTB 3.0 Section 3.1)
///
/// The `Openrtb` object is the top-level object for all OpenRTB 3.0 transactions.
/// It contains either a [`Request`] or [`Response`] object, along with version
/// information for both the protocol and domain specifications.
///
/// This layered approach separates protocol versioning from domain object versioning,
/// allowing independent evolution of the transaction layer and domain layer.
///
/// # Version Information
///
/// - `ver`: OpenRTB protocol version (e.g., "3.0")
/// - `domainspec`: Domain specification identifier (typically "adcom")
/// - `domainver`: Domain specification version (e.g., "1.0")
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example: Request Container
///
/// ```rust
/// use iab_specs::openrtb::v30::{Openrtb, Request};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let openrtb = Openrtb::builder()
///     .ver("3.0".to_string())
///     .domainspec("adcom".to_string())
///     .domainver("1.0".to_string())
///     .request(Some(Request::builder()
///         .id("req-123".to_string())
///         .build()
///         .unwrap()
///     ))
///     .build()
///     .unwrap();
///
/// let json = serde_json::to_string(&openrtb)?;
/// # Ok(())
/// # }
/// ```
///
/// # Example: Response Container
///
/// ```rust
/// use iab_specs::openrtb::v30::{Openrtb, Response};
///
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let openrtb = Openrtb::builder()
///     .ver("3.0".to_string())
///     .domainspec("adcom".to_string())
///     .domainver("1.0".to_string())
///     .response(Some(Response::builder()
///         .id("resp-123".to_string())
///         .build()
///         .unwrap()
///     ))
///     .build()
///     .unwrap();
///
/// let json = serde_json::to_string(&openrtb)?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Openrtb<Ext: Extension = serde_json::Value> {
    /// Version of the OpenRTB protocol (e.g., "3.0").
    /// REQUIRED by the specification.
    pub ver: String,

    /// Domain specification used for objects in the payload.
    /// REQUIRED. Typically "adcom" for AdCOM 1.0.
    pub domainspec: String,

    /// Version of the domain specification (e.g., "1.0" for AdCOM).
    /// REQUIRED by the specification.
    pub domainver: String,

    /// Bid request object.
    /// Exactly one of `request` or `response` must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub request: Option<Request<Ext>>,

    /// Bid response object.
    /// Exactly one of `request` or `response` must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub response: Option<Response<Ext>>,
}

impl Openrtb {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> OpenrtbBuilder {
        OpenrtbBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openrtb_with_request() {
        let request = Request::builder()
            .id("req-123".to_string())
            .build()
            .unwrap();

        let openrtb = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .request(Some(request))
            .build()
            .unwrap();

        assert_eq!(openrtb.ver, "3.0");
        assert_eq!(openrtb.domainspec, "adcom");
        assert_eq!(openrtb.domainver, "1.0");
        assert!(openrtb.request.is_some());
        assert!(openrtb.response.is_none());
    }

    #[test]
    fn test_openrtb_with_response() {
        let response = Response::builder()
            .id("resp-123".to_string())
            .build()
            .unwrap();

        let openrtb = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .response(Some(response))
            .build()
            .unwrap();

        assert_eq!(openrtb.ver, "3.0");
        assert!(openrtb.request.is_none());
        assert!(openrtb.response.is_some());
    }

    #[test]
    fn test_openrtb_serialization() {
        let request = Request::builder()
            .id("req-123".to_string())
            .build()
            .unwrap();

        let openrtb = Openrtb::builder()
            .ver("3.0".to_string())
            .domainspec("adcom".to_string())
            .domainver("1.0".to_string())
            .request(Some(request))
            .build()
            .unwrap();

        let json = serde_json::to_string(&openrtb).unwrap();
        assert!(json.contains("\"ver\":\"3.0\""));
        assert!(json.contains("\"domainspec\":\"adcom\""));
        assert!(json.contains("\"domainver\":\"1.0\""));
        assert!(json.contains("\"request\""));
    }

    #[test]
    fn test_openrtb_deserialization() {
        let json = r#"{
            "ver": "3.0",
            "domainspec": "adcom",
            "domainver": "1.0",
            "request": {
                "id": "req-123"
            }
        }"#;

        let openrtb: Openrtb = serde_json::from_str(json).unwrap();
        assert_eq!(openrtb.ver, "3.0");
        assert_eq!(openrtb.domainspec, "adcom");
        assert!(openrtb.request.is_some());
        assert_eq!(openrtb.request.unwrap().id, "req-123");
    }
}
