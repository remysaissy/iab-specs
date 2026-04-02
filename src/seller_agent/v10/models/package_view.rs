use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A PublicPackageView represents a public-facing view of a package
/// with limited information for external consumers.
///
/// This view contains only the basic package information that is safe
/// to expose to potential buyers, without pricing or detailed internal data.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::PublicPackageView;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let view = PublicPackageView::builder()
///     .name("Premium Display".to_string())
///     .description("High-impact display inventory".to_string())
///     .category("Display".to_string())
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct PublicPackageView<Ext: Extension = crate::DefaultExt> {
    /// Package name (REQUIRED).
    /// Human-readable name for the package.
    #[builder(default, setter(into))]
    pub name: String,

    /// Package description.
    /// Brief description of what this package offers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub description: Option<String>,

    /// Package category.
    /// Classification or category of the package (e.g., "Display", "Video", "Native").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub category: Option<String>,

    /// Extension object for package view-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl PublicPackageView {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> PublicPackageViewBuilder {
        PublicPackageViewBuilder::create_empty()
    }
}

/// An AuthenticatedPackageView represents an authenticated view of a package
/// with full details including pricing and related inventory information.
///
/// This view is intended for authenticated buyers and contains complete
/// package information, including the underlying Package and TieredPricing.
///
/// # Note on Generics
///
/// Since `Package` and `TieredPricing` are generic over `Ext`, but
/// `AuthenticatedPackageView` needs to embed concrete instances, we use
/// the default concrete type `Package` (which is `Package<DefaultExt>`) and
/// `TieredPricing` (which is `TieredPricing<DefaultExt>`).
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::{AuthenticatedPackageView, Package, TieredPricing};
/// use iab_specs::seller_agent::v10::enums::PackageType;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let package = Package::builder()
///     .name("Premium Display".to_string())
///     .package_type(PackageType::Curated)
///     .build()?;
///
/// let pricing = TieredPricing::builder()
///     .tiers(vec![])
///     .build()?;
///
/// let view = AuthenticatedPackageView::builder()
///     .package(package)
///     .pricing(pricing)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct AuthenticatedPackageView<Ext: Extension = crate::DefaultExt> {
    /// The Package details (REQUIRED).
    /// Contains the full package information.
    #[builder(default)]
    pub package: super::Package,

    /// The TieredPricing details (REQUIRED).
    /// Contains the pricing structure for this package.
    #[builder(default)]
    pub pricing: super::TieredPricing,

    /// Extension object for authenticated package view-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl AuthenticatedPackageView {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AuthenticatedPackageViewBuilder {
        AuthenticatedPackageViewBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seller_agent::v10::enums::{PackageType, PricingTierType};
    use crate::seller_agent::v10::models::{Package, PricingTier, TieredPricing};

    // ========== PublicPackageView Tests ==========

    #[test]
    fn test_public_package_view_minimal() {
        let view = PublicPackageView::builder()
            .name("Premium Display".to_string())
            .build()
            .unwrap();

        assert_eq!(view.name, "Premium Display");
        assert!(view.description.is_none());
        assert!(view.category.is_none());
        assert!(view.ext.is_none());
    }

    #[test]
    fn test_public_package_view_full() {
        let view = PublicPackageView::builder()
            .name("Video Package".to_string())
            .description("High-quality video inventory".to_string())
            .category("Video".to_string())
            .build()
            .unwrap();

        assert_eq!(view.name, "Video Package");
        assert_eq!(
            view.description,
            Some("High-quality video inventory".to_string())
        );
        assert_eq!(view.category, Some("Video".to_string()));
        assert!(view.ext.is_none());
    }

    #[test]
    fn test_public_package_view_roundtrip() {
        let original = PublicPackageView::builder()
            .name("Native Package".to_string())
            .description("Native ad placements".to_string())
            .category("Native".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: PublicPackageView = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.description, original.description);
        assert_eq!(parsed.category, original.category);
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_public_package_view_partial_fields() {
        let view = PublicPackageView::builder()
            .name("Partial Package".to_string())
            .category("Display".to_string())
            .build()
            .unwrap();

        assert_eq!(view.name, "Partial Package");
        assert!(view.description.is_none());
        assert_eq!(view.category, Some("Display".to_string()));
    }

    #[test]
    fn test_public_package_view_serialization() {
        let view = PublicPackageView::builder()
            .name("Test Package".to_string())
            .description("Test description".to_string())
            .category("Test".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&view).unwrap();
        assert!(json.contains("\"name\":\"Test Package\""));
        assert!(json.contains("\"description\":\"Test description\""));
        assert!(json.contains("\"category\":\"Test\""));
    }

    // ========== AuthenticatedPackageView Tests ==========

    #[test]
    fn test_authenticated_package_view_minimal() {
        let package = Package::builder()
            .name("Premium Display".to_string())
            .package_type(PackageType::Curated)
            .build()
            .unwrap();

        let pricing = TieredPricing::builder().tiers(vec![]).build().unwrap();

        let view = AuthenticatedPackageView::builder()
            .package(package)
            .pricing(pricing)
            .build()
            .unwrap();

        assert_eq!(view.package.name, "Premium Display");
        assert_eq!(view.pricing.tiers.len(), 0);
        assert!(view.ext.is_none());
    }

    #[test]
    fn test_authenticated_package_view_with_tiers() {
        let package = Package::builder()
            .id("pkg-001".to_string())
            .name("Tiered Package".to_string())
            .package_type(PackageType::Dynamic)
            .build()
            .unwrap();

        let pricing = TieredPricing::builder()
            .tiers(vec![
                PricingTier::builder()
                    .tier_type(PricingTierType::Public)
                    .discount_percent(0.0)
                    .negotiation_enabled(false)
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Agency)
                    .discount_percent(10.0)
                    .negotiation_enabled(true)
                    .min_spend(Some(5000.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let view = AuthenticatedPackageView::builder()
            .package(package)
            .pricing(pricing)
            .build()
            .unwrap();

        assert_eq!(view.package.id, Some("pkg-001".to_string()));
        assert_eq!(view.package.name, "Tiered Package");
        assert_eq!(view.pricing.tiers.len(), 2);
        assert_eq!(view.pricing.tiers[0].tier_type, PricingTierType::Public);
        assert_eq!(view.pricing.tiers[1].tier_type, PricingTierType::Agency);
        assert_eq!(view.pricing.tiers[1].discount_percent, 10.0);
    }

    #[test]
    fn test_authenticated_package_view_roundtrip() {
        let package = Package::builder()
            .name("Roundtrip Package".to_string())
            .build()
            .unwrap();

        let pricing = TieredPricing::builder()
            .tiers(vec![
                PricingTier::builder()
                    .tier_type(PricingTierType::Seat)
                    .discount_percent(5.0)
                    .negotiation_enabled(true)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let original = AuthenticatedPackageView::builder()
            .package(package)
            .pricing(pricing)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: AuthenticatedPackageView = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.package.name, original.package.name);
        assert_eq!(parsed.pricing.tiers.len(), original.pricing.tiers.len());
        assert_eq!(
            parsed.pricing.tiers[0].tier_type,
            original.pricing.tiers[0].tier_type
        );
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_authenticated_package_view_complex() {
        let package = Package::builder()
            .id("pkg-complex".to_string())
            .name("Complex Package".to_string())
            .description("A complex package for testing".to_string())
            .product_ids(vec![
                "prod-1".to_string(),
                "prod-2".to_string(),
                "prod-3".to_string(),
            ])
            .package_type(PackageType::Dynamic)
            .bundle_price(Some(10000.0))
            .build()
            .unwrap();

        let pricing = TieredPricing::builder()
            .tiers(vec![
                PricingTier::builder()
                    .tier_type(PricingTierType::Public)
                    .discount_percent(0.0)
                    .negotiation_enabled(false)
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Agency)
                    .discount_percent(10.0)
                    .negotiation_enabled(true)
                    .min_spend(Some(5000.0))
                    .build()
                    .unwrap(),
                PricingTier::builder()
                    .tier_type(PricingTierType::Advertiser)
                    .discount_percent(20.0)
                    .negotiation_enabled(true)
                    .min_spend(Some(50000.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let view = AuthenticatedPackageView::builder()
            .package(package)
            .pricing(pricing)
            .build()
            .unwrap();

        assert_eq!(view.package.id, Some("pkg-complex".to_string()));
        assert_eq!(view.package.name, "Complex Package");
        assert_eq!(view.package.product_ids.len(), 3);
        assert_eq!(view.package.bundle_price, Some(10000.0));
        assert_eq!(view.pricing.tiers.len(), 3);
        assert_eq!(view.pricing.tiers[2].discount_percent, 20.0);
    }

    #[test]
    fn test_authenticated_package_view_serialization() {
        let package = Package::builder()
            .name("Serialization Test".to_string())
            .build()
            .unwrap();

        let pricing = TieredPricing::builder().tiers(vec![]).build().unwrap();

        let view = AuthenticatedPackageView::builder()
            .package(package)
            .pricing(pricing)
            .build()
            .unwrap();

        let json = serde_json::to_string(&view).unwrap();
        assert!(json.contains("\"package\""));
        assert!(json.contains("\"pricing\""));
        assert!(json.contains("\"name\":\"Serialization Test\""));
    }
}
