use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Regs Object (Section 7.10)
///
/// Regulatory conditions in effect.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Regs<Ext: Extension = serde_json::Value> {
    /// COPPA compliance flag (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coppa: Option<i32>,

    /// GDPR applicability (1=yes, 0=no)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn test_regs_builder() {
        let regs = Regs::builder()
            .coppa(Some(1))
            .gdpr(Some(1))
            .build()
            .unwrap();

        assert_eq!(regs.coppa, Some(1));
        assert_eq!(regs.gdpr, Some(1));
    }

    #[test]
    fn test_regs_default() {
        let regs = Regs::builder().build().unwrap();

        assert!(regs.coppa.is_none());
        assert!(regs.gdpr.is_none());
    }

    #[test]
    fn test_regs_coppa_only() {
        let regs = Regs::builder().coppa(Some(1)).build().unwrap();

        assert_eq!(regs.coppa, Some(1));
        assert!(regs.gdpr.is_none());
    }

    #[test]
    fn test_regs_gdpr_only() {
        let regs = Regs::builder().gdpr(Some(1)).build().unwrap();

        assert_eq!(regs.gdpr, Some(1));
        assert!(regs.coppa.is_none());
    }

    #[test]
    fn test_regs_serialization() {
        let regs = Regs::builder()
            .coppa(Some(0))
            .gdpr(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&regs).unwrap();
        assert!(json.contains("\"coppa\":0"));
        assert!(json.contains("\"gdpr\":1"));
    }

    #[test]
    fn test_regs_deserialization() {
        let json = r#"{"coppa":1,"gdpr":0}"#;
        let regs: Regs = serde_json::from_str(json).unwrap();

        assert_eq!(regs.coppa, Some(1));
        assert_eq!(regs.gdpr, Some(0));
    }
}
