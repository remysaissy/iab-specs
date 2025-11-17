use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// BrandVersion helper struct for UserAgent
///
/// Brand and version information for user agent client hints.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct BrandVersion {
    /// Brand name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Version numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,
}

impl BrandVersion {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BrandVersionBuilder {
        BrandVersionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brand_version_builder() {
        let bv = BrandVersion::builder()
            .brand(Some("Chrome".to_string()))
            .version(Some(vec![
                "120".to_string(),
                "0".to_string(),
                "6099".to_string(),
                "109".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(bv.brand, Some("Chrome".to_string()));
        assert_eq!(
            bv.version,
            Some(vec![
                "120".to_string(),
                "0".to_string(),
                "6099".to_string(),
                "109".to_string()
            ])
        );
    }

    #[test]
    fn test_brand_version_default() {
        let bv = BrandVersion::builder().build().unwrap();

        assert!(bv.brand.is_none());
        assert!(bv.version.is_none());
    }

    #[test]
    fn test_brand_version_brand_only() {
        let bv = BrandVersion::builder()
            .brand(Some("Safari".to_string()))
            .build()
            .unwrap();

        assert_eq!(bv.brand, Some("Safari".to_string()));
        assert!(bv.version.is_none());
    }

    #[test]
    fn test_brand_version_serialization() {
        let bv = BrandVersion::builder()
            .brand(Some("Firefox".to_string()))
            .version(Some(vec!["121".to_string(), "0".to_string()]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&bv).unwrap();
        assert!(json.contains("\"brand\":\"Firefox\""));
        assert!(json.contains("\"version\":[\"121\",\"0\"]"));
    }

    #[test]
    fn test_brand_version_deserialization() {
        let json = r#"{"brand":"Edge","version":["120","0","2210","91"]}"#;
        let bv: BrandVersion = serde_json::from_str(json).unwrap();

        assert_eq!(bv.brand, Some("Edge".to_string()));
        assert_eq!(
            bv.version,
            Some(vec![
                "120".to_string(),
                "0".to_string(),
                "2210".to_string(),
                "91".to_string()
            ])
        );
    }

    #[test]
    fn test_brand_version_os_platform() {
        let bv = BrandVersion::builder()
            .brand(Some("Windows".to_string()))
            .version(Some(vec!["10".to_string(), "0".to_string()]))
            .build()
            .unwrap();

        assert_eq!(bv.brand, Some("Windows".to_string()));
        assert_eq!(bv.version.as_ref().unwrap().len(), 2);
    }
}
