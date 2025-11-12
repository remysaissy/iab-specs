use crate::Extension;
/// OpenRTB 2.5 Regs Object
///
/// This module implements the Regs object for regulatory compliance.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Regs object for regulatory conditions (OpenRTB 2.5 Section 3.2.3)
///
/// A `Regs` object contains any legal, governmental, or industry regulations that
/// apply to the request. The primary use case is to indicate COPPA compliance.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Regs;
///
/// let regs = Regs::builder()
///     .coppa(Some(1)) // COPPA applies
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Regs<Ext: Extension = serde_json::Value> {
    /// Flag indicating if this request is subject to the COPPA regulations
    /// established by the USA FTC:
    /// - 0 = no
    /// - 1 = yes
    ///
    /// If omitted, the request is not subject to COPPA.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub coppa: Option<i32>,

    /// Extension object for exchange-specific extensions.
    ///
    /// Common extensions include:
    /// - `gdpr`: Flag indicating if GDPR regulations apply (0=no, 1=yes)
    /// - `us_privacy`: US Privacy String per IAB CCPA Compliance Framework
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Regs {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RegsBuilder {
        RegsBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regs_creation() {
        let regs = Regs::builder().coppa(Some(1)).build().unwrap();

        assert_eq!(regs.coppa, Some(1));
    }

    #[test]
    fn test_regs_no_coppa() {
        let regs = Regs::builder().coppa(Some(0)).build().unwrap();

        assert_eq!(regs.coppa, Some(0));
    }

    #[test]
    fn test_regs_with_gdpr_extension() {
        let gdpr_ext = serde_json::json!({
            "gdpr": 1,
            "consent": "consent_string_here"
        });

        let regs = Regs::builder()
            .coppa(Some(0))
            .ext(Some(Box::new(gdpr_ext)))
            .build()
            .unwrap();

        assert_eq!(regs.coppa, Some(0));
        assert!(regs.ext.is_some());
        assert_eq!(regs.ext.as_ref().unwrap()["gdpr"], 1);
    }

    #[test]
    fn test_regs_serialization() {
        let regs = Regs::builder().coppa(Some(1)).build().unwrap();

        let json = serde_json::to_string(&regs).unwrap();
        assert!(json.contains("\"coppa\":1"));
    }

    #[test]
    fn test_regs_deserialization() {
        let json = r#"{"coppa":1}"#;
        let regs: Regs = serde_json::from_str(json).unwrap();

        assert_eq!(regs.coppa, Some(1));
    }

    #[test]
    fn test_regs_empty() {
        let regs = Regs::builder().build().unwrap();

        assert_eq!(regs.coppa, None);
        assert_eq!(regs.ext, None);
    }

    #[test]
    fn test_regs_with_us_privacy() {
        let privacy_ext = serde_json::json!({
            "us_privacy": "1YNN"
        });

        let regs = Regs::builder()
            .ext(Some(Box::new(privacy_ext)))
            .build()
            .unwrap();

        assert!(regs.ext.is_some());
        assert_eq!(regs.ext.as_ref().unwrap()["us_privacy"], "1YNN");
    }
}
