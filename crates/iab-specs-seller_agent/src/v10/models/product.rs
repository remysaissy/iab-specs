use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A SellerProduct represents an advertising product or inventory offering
/// that a seller makes available to buyers.
///
/// Products define the core properties of an advertising offering, including
/// the product name, identifier, base pricing, and the inventory segments
/// that make up the product.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::{SellerProduct, InventorySegment};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let segment = InventorySegment::builder()
///     .id("seg-001".to_string())
///     .name("Premium Display".to_string())
///     .ad_format("970x250".to_string())
///     .estimated_impressions(100000)
///     .build()?;
///
/// let product = SellerProduct::builder()
///     .id("prod-001".to_string())
///     .name("Premium Display Bundle".to_string())
///     .base_cpm(5.50)
///     .segments(vec![segment])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SellerProduct<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the product (REQUIRED).
    #[builder(default)]
    pub id: String,

    /// Display name of the product (REQUIRED).
    #[builder(default)]
    pub name: String,

    /// Base CPM price for this product in currency units (REQUIRED).
    /// Represents the baseline pricing for the inventory.
    #[builder(default)]
    pub base_cpm: f64,

    /// Collection of inventory segments that comprise this product.
    /// Segments define distinct advertising opportunities within the product.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub segments: Vec<InventorySegment<Ext>>,

    /// Extension object for product-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SellerProduct {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SellerProductBuilder {
        SellerProductBuilder::create_empty()
    }
}

/// An InventorySegment represents a distinct advertising opportunity
/// within a product offering.
///
/// Segments define specific combinations of ad formats, estimated impressions,
/// and other properties that collectively make up a product's inventory.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::InventorySegment;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let segment = InventorySegment::builder()
///     .id("seg-001".to_string())
///     .name("Above-the-fold".to_string())
///     .ad_format("300x250".to_string())
///     .estimated_impressions(50000)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct InventorySegment<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the segment (REQUIRED).
    #[builder(default)]
    pub id: String,

    /// Display name of the segment (REQUIRED).
    #[builder(default)]
    pub name: String,

    /// Ad format specification for this segment (REQUIRED).
    /// Typically expressed as "WIDTHxHEIGHT" or format name.
    #[builder(default)]
    pub ad_format: String,

    /// Estimated number of impressions available (REQUIRED).
    /// Provides capacity information for the segment.
    #[builder(default)]
    pub estimated_impressions: i64,

    /// Extension object for segment-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl InventorySegment {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> InventorySegmentBuilder {
        InventorySegmentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_segment_creation() -> Result<(), Box<dyn std::error::Error>> {
        let segment = InventorySegment::builder()
            .id("seg-001".to_string())
            .name("Premium Display".to_string())
            .ad_format("970x250".to_string())
            .estimated_impressions(100000)
            .build()?;

        assert_eq!(segment.id, "seg-001");
        assert_eq!(segment.name, "Premium Display");
        assert_eq!(segment.ad_format, "970x250");
        assert_eq!(segment.estimated_impressions, 100000);
        assert_eq!(segment.ext, None);

        Ok(())
    }

    #[test]
    fn test_seller_product_creation() -> Result<(), Box<dyn std::error::Error>> {
        let product = SellerProduct::builder()
            .id("prod-001".to_string())
            .name("Premium Display Bundle".to_string())
            .base_cpm(5.50)
            .build()?;

        assert_eq!(product.id, "prod-001");
        assert_eq!(product.name, "Premium Display Bundle");
        assert_eq!(product.base_cpm, 5.50);
        assert!(product.segments.is_empty());
        assert_eq!(product.ext, None);

        Ok(())
    }

    #[test]
    fn test_seller_product_with_segments() -> Result<(), Box<dyn std::error::Error>> {
        let segment1 = InventorySegment::builder()
            .id("seg-001".to_string())
            .name("Above-the-fold".to_string())
            .ad_format("300x250".to_string())
            .estimated_impressions(50000)
            .build()?;

        let segment2 = InventorySegment::builder()
            .id("seg-002".to_string())
            .name("Below-the-fold".to_string())
            .ad_format("300x250".to_string())
            .estimated_impressions(30000)
            .build()?;

        let product = SellerProduct::builder()
            .id("prod-001".to_string())
            .name("Display Inventory".to_string())
            .base_cpm(3.75)
            .segments(vec![segment1, segment2])
            .build()?;

        assert_eq!(product.id, "prod-001");
        assert_eq!(product.segments.len(), 2);
        assert_eq!(product.segments[0].id, "seg-001");
        assert_eq!(product.segments[1].estimated_impressions, 30000);

        Ok(())
    }

    #[test]
    fn test_segment_serialization_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let segment = InventorySegment::builder()
            .id("seg-123".to_string())
            .name("Test Segment".to_string())
            .ad_format("728x90".to_string())
            .estimated_impressions(250000)
            .build()?;

        let json = serde_json::to_string(&segment)?;
        let deserialized: InventorySegment = serde_json::from_str(&json)?;

        assert_eq!(segment, deserialized);
        Ok(())
    }

    #[test]
    fn test_product_serialization_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let segment = InventorySegment::builder()
            .id("seg-001".to_string())
            .name("Premium".to_string())
            .ad_format("970x250".to_string())
            .estimated_impressions(100000)
            .build()?;

        let product = SellerProduct::builder()
            .id("prod-001".to_string())
            .name("Premium Display Bundle".to_string())
            .base_cpm(5.50)
            .segments(vec![segment])
            .build()?;

        let json = serde_json::to_string(&product)?;
        let deserialized: SellerProduct = serde_json::from_str(&json)?;

        assert_eq!(product, deserialized);
        assert_eq!(deserialized.segments.len(), 1);
        assert_eq!(deserialized.segments[0].name, "Premium");

        Ok(())
    }

    #[test]
    fn test_empty_segments_not_serialized() -> Result<(), Box<dyn std::error::Error>> {
        let product = SellerProduct::builder()
            .id("prod-001".to_string())
            .name("Product".to_string())
            .base_cpm(2.50)
            .build()?;

        let json = serde_json::to_string(&product)?;
        assert!(!json.contains("\"segments\""));

        Ok(())
    }

    #[test]
    fn test_extension_support_in_segment_with_default() -> Result<(), Box<dyn std::error::Error>> {
        let segment = InventorySegment::builder()
            .id("seg-001".to_string())
            .name("Extended Segment".to_string())
            .ad_format("300x250".to_string())
            .estimated_impressions(50000)
            .build()?;

        assert!(segment.ext.is_none());

        let json = serde_json::to_string(&segment)?;
        let deserialized: InventorySegment = serde_json::from_str(&json)?;

        assert!(deserialized.ext.is_none());
        Ok(())
    }

    #[test]
    fn test_extension_support_in_product_with_default() -> Result<(), Box<dyn std::error::Error>> {
        let product = SellerProduct::builder()
            .id("prod-001".to_string())
            .name("Extended Product".to_string())
            .base_cpm(4.50)
            .build()?;

        assert!(product.ext.is_none());

        let json = serde_json::to_string(&product)?;
        let deserialized: SellerProduct = serde_json::from_str(&json)?;

        assert!(deserialized.ext.is_none());
        Ok(())
    }

    /// Seller Agent 1.0 § SellerProduct — default builder yields empty product
    #[test]
    fn test_seller_product_default() {
        let product = SellerProduct::builder().build().unwrap();
        assert_eq!(product.id, "");
        assert_eq!(product.name, "");
        assert_eq!(product.base_cpm, 0.0);
        assert!(product.segments.is_empty());
        assert!(product.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerProduct — optional fields omitted from JSON when None
    #[test]
    fn test_seller_product_optional_fields_skipped() {
        let product = SellerProduct::builder()
            .id("p".to_string())
            .name("n".to_string())
            .base_cpm(1.0)
            .build()
            .unwrap();

        let json = serde_json::to_string(&product).unwrap();
        assert!(!json.contains("\"segments\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § SellerProduct — clone produces identical value
    #[test]
    fn test_seller_product_clone() {
        let product = SellerProduct::builder()
            .id("prod-c".to_string())
            .name("Clone Test".to_string())
            .base_cpm(4.0)
            .segments(vec![
                InventorySegment::builder()
                    .id("s1".to_string())
                    .name("seg".to_string())
                    .ad_format("300x250".to_string())
                    .estimated_impressions(1000)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let cloned = product.clone();
        assert_eq!(product, cloned);
    }

    /// Seller Agent 1.0 § SellerProduct — deserialization from minimal JSON
    #[test]
    fn test_seller_product_deserialization_minimal() {
        let json = r#"{"id":"p","name":"n","base_cpm":2.0}"#;
        let product: SellerProduct = serde_json::from_str(json).unwrap();
        assert_eq!(product.id, "p");
        assert_eq!(product.name, "n");
        assert_eq!(product.base_cpm, 2.0);
        assert!(product.segments.is_empty());
        assert!(product.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerProduct — segments default when missing from JSON
    #[test]
    fn test_seller_product_deserialization_missing_segments() {
        let json = r#"{"id":"p","name":"n","base_cpm":1.0}"#;
        let product: SellerProduct = serde_json::from_str(json).unwrap();
        assert!(product.segments.is_empty());
    }

    /// Seller Agent 1.0 § SellerProduct — deserialization with populated segments
    #[test]
    fn test_seller_product_deserialization_with_segments() {
        let json = r#"{
            "id": "prod-1",
            "name": "Bundle",
            "base_cpm": 3.5,
            "segments": [
                {"id":"s1","name":"Seg1","ad_format":"728x90","estimated_impressions":5000}
            ]
        }"#;
        let product: SellerProduct = serde_json::from_str(json).unwrap();
        assert_eq!(product.segments.len(), 1);
        assert_eq!(product.segments[0].id, "s1");
        assert_eq!(product.segments[0].ad_format, "728x90");
    }

    /// Seller Agent 1.0 § InventorySegment — default builder yields empty segment
    #[test]
    fn test_inventory_segment_default() {
        let segment = InventorySegment::builder().build().unwrap();
        assert_eq!(segment.id, "");
        assert_eq!(segment.name, "");
        assert_eq!(segment.ad_format, "");
        assert_eq!(segment.estimated_impressions, 0);
        assert!(segment.ext.is_none());
    }

    /// Seller Agent 1.0 § InventorySegment — optional fields omitted from JSON when None
    #[test]
    fn test_inventory_segment_optional_fields_skipped() {
        let segment = InventorySegment::builder()
            .id("s".to_string())
            .name("n".to_string())
            .ad_format("300x250".to_string())
            .estimated_impressions(100)
            .build()
            .unwrap();

        let json = serde_json::to_string(&segment).unwrap();
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § InventorySegment — clone produces identical value
    #[test]
    fn test_inventory_segment_clone() {
        let segment = InventorySegment::builder()
            .id("seg-c".to_string())
            .name("Clone".to_string())
            .ad_format("160x600".to_string())
            .estimated_impressions(25000)
            .build()
            .unwrap();

        let cloned = segment.clone();
        assert_eq!(segment, cloned);
    }

    /// Seller Agent 1.0 § InventorySegment — deserialization from minimal JSON
    #[test]
    fn test_inventory_segment_deserialization_minimal() {
        let json = r#"{"id":"s","name":"n","ad_format":"300x250","estimated_impressions":500}"#;
        let segment: InventorySegment = serde_json::from_str(json).unwrap();
        assert_eq!(segment.id, "s");
        assert_eq!(segment.name, "n");
        assert_eq!(segment.ad_format, "300x250");
        assert_eq!(segment.estimated_impressions, 500);
        assert!(segment.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerProduct — Default trait yields same as default builder
    #[test]
    fn test_seller_product_default_trait() {
        let product = SellerProduct::<crate::DefaultExt>::default();
        assert_eq!(product.id, "");
        assert_eq!(product.name, "");
        assert_eq!(product.base_cpm, 0.0);
        assert!(product.segments.is_empty());
        assert!(product.ext.is_none());
    }

    /// Seller Agent 1.0 § InventorySegment — Default trait yields same as default builder
    #[test]
    fn test_inventory_segment_default_trait() {
        let segment = InventorySegment::<crate::DefaultExt>::default();
        assert_eq!(segment.id, "");
        assert_eq!(segment.name, "");
        assert_eq!(segment.ad_format, "");
        assert_eq!(segment.estimated_impressions, 0);
        assert!(segment.ext.is_none());
    }
}
