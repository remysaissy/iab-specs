use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Audit Object (Section 3.14)
///
/// Quality and safety review status for creative content.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Audit<Ext: Extension = serde_json::Value> {
    /// Audit status code (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

    /// Correction flag (1=corrected, 0=original)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corr: Option<Vec<String>>,

    /// Organization conducting the audit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<i64>,

    /// Timestamp of last audit modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<i64>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Audit {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AuditBuilder {
        AuditBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_builder() {
        let audit = Audit::builder()
            .status(Some(1))
            .init(Some(1234567890))
            .lastmod(Some(1234567900))
            .build()
            .unwrap();

        assert_eq!(audit.status, Some(1));
        assert_eq!(audit.init, Some(1234567890));
        assert_eq!(audit.lastmod, Some(1234567900));
    }

    #[test]
    fn test_audit_default() {
        let audit = Audit::builder().build().unwrap();

        assert!(audit.status.is_none());
        assert!(audit.corr.is_none());
        assert!(audit.init.is_none());
    }

    #[test]
    fn test_audit_with_corrections() {
        let audit = Audit::builder()
            .status(Some(2))
            .corr(Some(vec![
                "correction1".to_string(),
                "correction2".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(
            audit.corr,
            Some(vec!["correction1".to_string(), "correction2".to_string()])
        );
    }

    #[test]
    fn test_audit_serialization() {
        let audit = Audit::builder()
            .status(Some(1))
            .init(Some(1234567890))
            .build()
            .unwrap();

        let json = serde_json::to_string(&audit).unwrap();
        assert!(json.contains("\"status\":1"));
        assert!(json.contains("\"init\":1234567890"));
    }

    #[test]
    fn test_audit_deserialization() {
        let json = r#"{"status":2,"init":1234567890,"lastmod":1234567900}"#;
        let audit: Audit = serde_json::from_str(json).unwrap();

        assert_eq!(audit.status, Some(2));
        assert_eq!(audit.init, Some(1234567890));
        assert_eq!(audit.lastmod, Some(1234567900));
    }
}
