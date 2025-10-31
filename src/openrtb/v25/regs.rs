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
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Regs;
///
/// let regs = Regs {
///     coppa: Some(1), // COPPA applies
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Regs {
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
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regs_creation() {
        let regs = Regs {
            coppa: Some(1),
            ..Default::default()
        };

        assert_eq!(regs.coppa, Some(1));
    }

    #[test]
    fn test_regs_no_coppa() {
        let regs = Regs {
            coppa: Some(0),
            ..Default::default()
        };

        assert_eq!(regs.coppa, Some(0));
    }

    #[test]
    fn test_regs_with_gdpr_extension() {
        let gdpr_ext = serde_json::json!({
            "gdpr": 1,
            "consent": "consent_string_here"
        });

        let regs = Regs {
            coppa: Some(0),
            ext: Some(gdpr_ext),
        };

        assert_eq!(regs.coppa, Some(0));
        assert!(regs.ext.is_some());
        assert_eq!(regs.ext.as_ref().unwrap()["gdpr"], 1);
    }

    #[test]
    fn test_regs_serialization() {
        let regs = Regs {
            coppa: Some(1),
            ..Default::default()
        };

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
        let regs = Regs::default();

        assert_eq!(regs.coppa, None);
        assert_eq!(regs.ext, None);
    }

    #[test]
    fn test_regs_with_us_privacy() {
        let privacy_ext = serde_json::json!({
            "us_privacy": "1YNN"
        });

        let regs = Regs {
            ext: Some(privacy_ext),
            ..Default::default()
        };

        assert!(regs.ext.is_some());
        assert_eq!(regs.ext.as_ref().unwrap()["us_privacy"], "1YNN");
    }
}
