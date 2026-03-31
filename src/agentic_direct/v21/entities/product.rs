use super::super::enums::{DeliveryType, ProductAvailability, RateType};
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Product entity.
///
/// Represents a media product available for purchase with pricing, delivery, and targeting information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Product<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Publisher identifier (required).
    #[builder(setter(into))]
    pub publisher_id: String,

    /// Product name (required).
    #[builder(setter(into))]
    pub name: String,

    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// Availability status of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub availability: Option<ProductAvailability>,

    /// Base price for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub base_price: Option<f64>,

    /// Rate type for pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rate_type: Option<RateType>,

    /// Delivery type for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub delivery_type: Option<DeliveryType>,

    /// Minimum spend requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_spend: Option<f64>,

    /// Maximum spend allowance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_spend: Option<f64>,

    /// Currency code (e.g., "USD").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub currency: Option<String>,

    /// Targeting criteria as arbitrary JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub targeting: Option<serde_json::Value>,

    /// List of supported ad units.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub ad_units: Vec<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Product {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ProductBuilder {
        ProductBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_creation() {
        let product = Product::builder()
            .publisher_id("pub-123")
            .name("Premium Display")
            .build()
            .unwrap();

        assert_eq!(product.publisher_id, "pub-123");
        assert_eq!(product.name, "Premium Display");
        assert!(product.id.is_none());
        assert!(product.description.is_none());
        assert!(product.base_price.is_none());
    }

    #[test]
    fn test_product_serialization() {
        let product = Product::builder()
            .publisher_id("pub-456")
            .name("Video Product")
            .description("High-quality video inventory")
            .base_price(Some(10.50))
            .currency("USD")
            .build()
            .unwrap();

        let json = serde_json::to_string(&product).unwrap();
        assert!(json.contains("\"publisher_id\":\"pub-456\""));
        assert!(json.contains("\"name\":\"Video Product\""));
        assert!(json.contains("\"description\":\"High-quality video inventory\""));
        assert!(json.contains("\"base_price\":10.5"));
        assert!(json.contains("\"currency\":\"USD\""));
    }

    #[test]
    fn test_product_deserialization() {
        let json = r#"{"publisher_id":"pub-789","name":"Native Product","base_price":5.0,"currency":"EUR"}"#;
        let product: Product = serde_json::from_str(json).unwrap();

        assert_eq!(product.publisher_id, "pub-789");
        assert_eq!(product.name, "Native Product");
        assert_eq!(product.base_price, Some(5.0));
        assert_eq!(product.currency, Some("EUR".to_string()));
    }

    #[test]
    fn test_product_roundtrip() {
        let product = Product::builder()
            .id("prod-999")
            .publisher_id("pub-111")
            .name("Standard Product")
            .base_price(Some(7.75))
            .currency("GBP")
            .ad_units(vec!["unit-1".to_string(), "unit-2".to_string()])
            .build()
            .unwrap();

        let json = serde_json::to_string(&product).unwrap();
        let parsed: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(product, parsed);
    }

    #[test]
    fn test_product_with_json_targeting() {
        let targeting = serde_json::json!({
            "geos": ["US", "CA"],
            "min_age": 18,
            "interests": ["tech", "finance"]
        });

        let product = Product::builder()
            .publisher_id("pub-222")
            .name("Targeted Product")
            .targeting(Some(targeting.clone()))
            .build()
            .unwrap();

        assert_eq!(product.targeting, Some(targeting));

        let json = serde_json::to_string(&product).unwrap();
        let parsed: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(product.targeting, parsed.targeting);
    }
}
