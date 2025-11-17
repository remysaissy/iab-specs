use crate::Extension;
use crate::adcom::context::BrandVersion;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// UserAgent Object (Section 7.5)
///
/// Structured user agent information per User-Agent Client Hints.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct UserAgent<Ext: Extension = serde_json::Value> {
    /// Browser marketing name array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<Vec<BrandVersion>>,

    /// Platform/OS name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<BrandVersion>>,

    /// Mobile device flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i32>,

    /// Platform architecture (e.g., "x86", "arm")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Platform bitness (e.g., "64")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,

    /// Device model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Source of user agent data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl UserAgent {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> UserAgentBuilder {
        UserAgentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent_builder() {
        let ua = UserAgent::builder()
            .mobile(Some(1))
            .architecture(Some("arm64".to_string()))
            .bitness(Some("64".to_string()))
            .model(Some("iPhone14,2".to_string()))
            .build()
            .unwrap();

        assert_eq!(ua.mobile, Some(1));
        assert_eq!(ua.architecture, Some("arm64".to_string()));
        assert_eq!(ua.bitness, Some("64".to_string()));
        assert_eq!(ua.model, Some("iPhone14,2".to_string()));
    }

    #[test]
    fn test_user_agent_default() {
        let ua = UserAgent::builder().build().unwrap();

        assert!(ua.mobile.is_none());
        assert!(ua.platform.is_none());
        assert!(ua.browsers.is_none());
        assert!(ua.architecture.is_none());
    }

    #[test]
    fn test_user_agent_with_platform() {
        let platform = BrandVersion::builder()
            .brand(Some("macOS".to_string()))
            .version(Some(vec!["13".to_string(), "0".to_string()]))
            .build()
            .unwrap();

        let ua = UserAgent::builder()
            .platform(Some(Box::new(platform)))
            .architecture(Some("x86_64".to_string()))
            .build()
            .unwrap();

        assert!(ua.platform.is_some());
        assert_eq!(
            ua.platform.as_ref().unwrap().brand,
            Some("macOS".to_string())
        );
        assert_eq!(ua.architecture, Some("x86_64".to_string()));
    }

    #[test]
    fn test_user_agent_with_browsers() {
        let chrome = BrandVersion::builder()
            .brand(Some("Chrome".to_string()))
            .version(Some(vec![
                "120".to_string(),
                "0".to_string(),
                "6099".to_string(),
                "109".to_string(),
            ]))
            .build()
            .unwrap();

        let chromium = BrandVersion::builder()
            .brand(Some("Chromium".to_string()))
            .version(Some(vec![
                "120".to_string(),
                "0".to_string(),
                "6099".to_string(),
                "109".to_string(),
            ]))
            .build()
            .unwrap();

        let ua = UserAgent::builder()
            .browsers(Some(vec![chrome, chromium]))
            .build()
            .unwrap();

        assert!(ua.browsers.is_some());
        assert_eq!(ua.browsers.as_ref().unwrap().len(), 2);
        assert_eq!(
            ua.browsers.as_ref().unwrap()[0].brand,
            Some("Chrome".to_string())
        );
    }

    #[test]
    fn test_user_agent_serialization() {
        let ua = UserAgent::builder()
            .mobile(Some(0))
            .architecture(Some("x86".to_string()))
            .bitness(Some("64".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&ua).unwrap();
        assert!(json.contains("\"mobile\":0"));
        assert!(json.contains("\"architecture\":\"x86\""));
        assert!(json.contains("\"bitness\":\"64\""));
    }

    #[test]
    fn test_user_agent_deserialization() {
        let json = r#"{"mobile":1,"architecture":"arm64","bitness":"64","model":"SM-G998B"}"#;
        let ua: UserAgent = serde_json::from_str(json).unwrap();

        assert_eq!(ua.mobile, Some(1));
        assert_eq!(ua.architecture, Some("arm64".to_string()));
        assert_eq!(ua.bitness, Some("64".to_string()));
        assert_eq!(ua.model, Some("SM-G998B".to_string()));
    }
}
