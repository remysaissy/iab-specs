use crate::Extension;
use crate::v10::enums::PackageType;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A MediaKit represents a collection of advertising packages offered by a publisher.
///
/// The MediaKit contains packages that aggregate ad inventory by topic, format, or audience.
/// Each package can have multiple products and pricing information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::MediaKit;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let media_kit = MediaKit::builder()
///     .publisher_id("pub-123".to_string())
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct MediaKit<Ext: Extension = crate::DefaultExt> {
    /// Publisher ID (REQUIRED).
    /// Unique identifier for the publisher offering this media kit.
    #[builder(default, setter(into))]
    pub publisher_id: String,

    /// Packages available in this media kit.
    #[builder(default)]
    pub packages: Vec<Package>,

    /// Last update time for this media kit.
    /// ISO 8601 formatted timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub updated_at: Option<String>,

    /// Extension object for media kit-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl MediaKit {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> MediaKitBuilder {
        MediaKitBuilder::create_empty()
    }
}

/// A Package represents a collection of ad inventory with bundled pricing.
///
/// Packages allow publishers to offer inventory grouped by topic, format, or audience,
/// with optional pricing for the entire package.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::Package;
/// use iab_specs_seller_agent::v10::enums::PackageType;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let package = Package::builder()
///     .name("Premium Display".to_string())
///     .package_type(PackageType::Curated)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Package<Ext: Extension = crate::DefaultExt> {
    /// Package ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Package name (REQUIRED).
    /// Human-readable name for the package.
    #[builder(default, setter(into))]
    pub name: String,

    /// Package description.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// Product IDs contained in this package.
    #[builder(default)]
    pub product_ids: Vec<String>,

    /// Bundle price for the entire package.
    /// If set, represents a discounted or fixed price for all products in the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bundle_price: Option<f64>,

    /// Type of package (e.g., Curated or Dynamic).
    #[builder(default)]
    pub package_type: PackageType,

    /// Extension object for package-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Package {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PackageBuilder {
        PackageBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_kit_creation() {
        let package1 = Package::builder()
            .name("Premium Display".to_string())
            .package_type(PackageType::Curated)
            .build()
            .unwrap();

        let package2 = Package::builder()
            .name("Video Inventory".to_string())
            .package_type(PackageType::Dynamic)
            .build()
            .unwrap();

        let media_kit = MediaKit::builder()
            .publisher_id("pub-123".to_string())
            .packages(vec![package1, package2])
            .build()
            .unwrap();

        assert_eq!(media_kit.publisher_id, "pub-123");
        assert_eq!(media_kit.packages.len(), 2);
        assert_eq!(media_kit.packages[0].name, "Premium Display");
        assert_eq!(media_kit.packages[1].name, "Video Inventory");
        assert!(media_kit.updated_at.is_none());
        assert!(media_kit.ext.is_none());
    }

    #[test]
    fn test_media_kit_roundtrip() {
        let media_kit = MediaKit::builder()
            .publisher_id("pub-456".to_string())
            .packages(vec![
                Package::builder()
                    .name("Test Package".to_string())
                    .build()
                    .unwrap(),
            ])
            .updated_at("2026-04-02T10:30:00Z".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&media_kit).unwrap();
        let parsed: MediaKit = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.publisher_id, "pub-456");
        assert_eq!(parsed.packages.len(), 1);
        assert_eq!(parsed.updated_at, Some("2026-04-02T10:30:00Z".to_string()));
    }

    #[test]
    fn test_package_curated() {
        let package = Package::builder()
            .id("pkg-001".to_string())
            .name("Curated News".to_string())
            .description("Hand-picked news inventory".to_string())
            .package_type(PackageType::Curated)
            .product_ids(vec!["prod-1".to_string(), "prod-2".to_string()])
            .build()
            .unwrap();

        assert_eq!(package.id, Some("pkg-001".to_string()));
        assert_eq!(package.name, "Curated News");
        assert_eq!(package.package_type, PackageType::Curated);
        assert_eq!(package.product_ids.len(), 2);
    }

    #[test]
    fn test_package_dynamic() {
        let package = Package::builder()
            .name("Dynamic Programmatic".to_string())
            .package_type(PackageType::Dynamic)
            .bundle_price(Some(5000.0))
            .build()
            .unwrap();

        assert_eq!(package.name, "Dynamic Programmatic");
        assert_eq!(package.package_type, PackageType::Dynamic);
        assert_eq!(package.bundle_price, Some(5000.0));
    }

    #[test]
    fn test_package_default() {
        let package = Package::builder()
            .name("Default Package".to_string())
            .build()
            .unwrap();

        assert_eq!(package.name, "Default Package");
        assert_eq!(package.package_type, PackageType::Curated); // default
        assert!(package.id.is_none());
        assert!(package.description.is_none());
        assert!(package.bundle_price.is_none());
        assert!(package.product_ids.is_empty());
        assert!(package.ext.is_none());
    }

    /// Seller Agent 1.0 § MediaKit — default builder yields empty media kit
    #[test]
    fn test_media_kit_default() {
        let kit = MediaKit::builder().build().unwrap();
        assert_eq!(kit.publisher_id, "");
        assert!(kit.packages.is_empty());
        assert!(kit.updated_at.is_none());
        assert!(kit.ext.is_none());
    }

    /// Seller Agent 1.0 § MediaKit — optional fields omitted from JSON when None
    #[test]
    fn test_media_kit_optional_fields_skipped() {
        let kit = MediaKit::builder().publisher_id("pub-1").build().unwrap();

        let json = serde_json::to_string(&kit).unwrap();
        assert!(!json.contains("\"updated_at\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § MediaKit — clone produces identical value
    #[test]
    fn test_media_kit_clone() {
        let kit = MediaKit::builder()
            .publisher_id("pub-clone")
            .packages(vec![
                Package::builder()
                    .name("Pkg".to_string())
                    .package_type(PackageType::Curated)
                    .build()
                    .unwrap(),
            ])
            .updated_at("2026-01-01T00:00:00Z")
            .build()
            .unwrap();

        let cloned = kit.clone();
        assert_eq!(kit, cloned);
    }

    /// Seller Agent 1.0 § MediaKit — deserialization from minimal JSON
    #[test]
    fn test_media_kit_deserialization_minimal() {
        let json = r#"{"publisher_id":"pub-1","packages":[]}"#;
        let kit: MediaKit = serde_json::from_str(json).unwrap();
        assert_eq!(kit.publisher_id, "pub-1");
        assert!(kit.packages.is_empty());
        assert!(kit.updated_at.is_none());
    }

    /// Seller Agent 1.0 § Package — default builder yields empty package
    #[test]
    fn test_package_default_builder() {
        let pkg = Package::builder().build().unwrap();
        assert_eq!(pkg.name, "");
        assert_eq!(pkg.package_type, PackageType::Curated);
        assert!(pkg.id.is_none());
        assert!(pkg.description.is_none());
        assert!(pkg.bundle_price.is_none());
        assert!(pkg.product_ids.is_empty());
        assert!(pkg.ext.is_none());
    }

    /// Seller Agent 1.0 § Package — optional fields omitted from JSON when None
    #[test]
    fn test_package_optional_fields_skipped() {
        let pkg = Package::builder().name("Pkg".to_string()).build().unwrap();

        let json = serde_json::to_string(&pkg).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"description\""));
        assert!(!json.contains("\"bundle_price\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § Package — clone produces identical value
    #[test]
    fn test_package_clone() {
        let pkg = Package::builder()
            .id("pkg-c".to_string())
            .name("Clone Pkg".to_string())
            .description("desc".to_string())
            .product_ids(vec!["p1".to_string()])
            .bundle_price(Some(1000.0))
            .package_type(PackageType::Dynamic)
            .build()
            .unwrap();

        let cloned = pkg.clone();
        assert_eq!(pkg, cloned);
    }

    /// Seller Agent 1.0 § Package — deserialization from minimal JSON
    #[test]
    fn test_package_deserialization_minimal() {
        let json = r#"{"name":"Pkg","product_ids":[],"package_type":"curated"}"#;
        let pkg: Package = serde_json::from_str(json).unwrap();
        assert_eq!(pkg.name, "Pkg");
        assert_eq!(pkg.package_type, PackageType::Curated);
        assert!(pkg.id.is_none());
        assert!(pkg.description.is_none());
        assert!(pkg.bundle_price.is_none());
    }
}
