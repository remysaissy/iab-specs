use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// BookedLine represents an individual line item from a successful OpenDirect booking.
///
/// A booked line captures the line/order/product association and financial terms
/// for a specific piece of inventory that has been successfully booked.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::BookedLine;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let line = BookedLine::builder()
///     .line_id("line-001")
///     .order_id("order-001")
///     .product_id("product-001")
///     .status("booked")
///     .rate(2.50)
///     .quantity(1000)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct BookedLine<Ext: Extension = crate::DefaultExt> {
    /// The line identifier from the OpenDirect booking (REQUIRED).
    #[builder(setter(into))]
    pub line_id: String,

    /// The order identifier associated with this line (REQUIRED).
    #[builder(setter(into))]
    pub order_id: String,

    /// The product identifier for this line item (REQUIRED).
    #[builder(setter(into))]
    pub product_id: String,

    /// The status of this booked line (e.g., "booked", "in_progress", "completed") (REQUIRED).
    #[builder(setter(into))]
    pub status: String,

    /// The rate/price per unit in currency units (REQUIRED).
    #[builder(default)]
    pub rate: f64,

    /// The quantity of units booked (REQUIRED).
    #[builder(default)]
    pub quantity: i64,

    /// Extension object for line-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl BookedLine {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BookedLineBuilder {
        BookedLineBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_booked_line_minimal() {
        let line = BookedLine::builder()
            .line_id("line-001")
            .order_id("order-001")
            .product_id("product-001")
            .status("booked")
            .rate(2.50)
            .quantity(1000)
            .build()
            .unwrap();

        assert_eq!(line.line_id, "line-001");
        assert_eq!(line.order_id, "order-001");
        assert_eq!(line.product_id, "product-001");
        assert_eq!(line.status, "booked");
        assert_eq!(line.rate, 2.50);
        assert_eq!(line.quantity, 1000);
        assert!(line.ext.is_none());
    }

    #[test]
    fn test_booked_line_with_all_fields() {
        let line = BookedLine::builder()
            .line_id("line-002")
            .order_id("order-002")
            .product_id("product-002")
            .status("in_progress")
            .rate(3.75)
            .quantity(5000)
            .build()
            .unwrap();

        assert_eq!(line.line_id, "line-002");
        assert_eq!(line.order_id, "order-002");
        assert_eq!(line.product_id, "product-002");
        assert_eq!(line.status, "in_progress");
        assert_eq!(line.rate, 3.75);
        assert_eq!(line.quantity, 5000);
    }

    #[test]
    fn test_booked_line_serialization() {
        let line = BookedLine::builder()
            .line_id("line-003")
            .order_id("order-003")
            .product_id("product-003")
            .status("completed")
            .rate(1.50)
            .quantity(2000)
            .build()
            .unwrap();

        let json = serde_json::to_string(&line).unwrap();
        assert!(json.contains("\"line_id\":\"line-003\""));
        assert!(json.contains("\"order_id\":\"order-003\""));
        assert!(json.contains("\"product_id\":\"product-003\""));
        assert!(json.contains("\"status\":\"completed\""));
        assert!(json.contains("\"rate\":1.5"));
        assert!(json.contains("\"quantity\":2000"));
    }

    #[test]
    fn test_booked_line_deserialization() {
        let json = r#"{
            "line_id": "line-004",
            "order_id": "order-004",
            "product_id": "product-004",
            "status": "booked",
            "rate": 4.25,
            "quantity": 750
        }"#;

        let line: BookedLine = serde_json::from_str(json).unwrap();
        assert_eq!(line.line_id, "line-004");
        assert_eq!(line.order_id, "order-004");
        assert_eq!(line.product_id, "product-004");
        assert_eq!(line.status, "booked");
        assert_eq!(line.rate, 4.25);
        assert_eq!(line.quantity, 750);
    }

    #[test]
    fn test_booked_line_roundtrip() {
        let original = BookedLine::builder()
            .line_id("line-005")
            .order_id("order-005")
            .product_id("product-005")
            .status("in_progress")
            .rate(2.99)
            .quantity(3500)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: BookedLine = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.line_id, original.line_id);
        assert_eq!(parsed.order_id, original.order_id);
        assert_eq!(parsed.product_id, original.product_id);
        assert_eq!(parsed.status, original.status);
        assert_eq!(parsed.rate, original.rate);
        assert_eq!(parsed.quantity, original.quantity);
    }

    #[test]
    fn test_booked_line_large_quantity() {
        let line = BookedLine::builder()
            .line_id("line-006")
            .order_id("order-006")
            .product_id("product-006")
            .status("booked")
            .rate(0.50)
            .quantity(1_000_000)
            .build()
            .unwrap();

        assert_eq!(line.quantity, 1_000_000);
        assert_eq!(line.rate, 0.50);
    }

    #[test]
    fn test_booked_line_zero_rate() {
        let line = BookedLine::builder()
            .line_id("line-007")
            .order_id("order-007")
            .product_id("product-007")
            .status("booked")
            .rate(0.0)
            .quantity(100)
            .build()
            .unwrap();

        assert_eq!(line.rate, 0.0);
        assert_eq!(line.quantity, 100);
    }
}
