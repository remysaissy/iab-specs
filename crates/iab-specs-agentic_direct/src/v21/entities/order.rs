use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::OrderStatus;

/// Order entity.
///
/// Represents an advertising order with lifecycle status tracking and optional extension information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Order<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Name of the order (required).
    #[builder(setter(into))]
    pub name: String,

    /// Account identifier (required).
    #[builder(setter(into))]
    pub account_id: String,

    /// Publisher identifier (required).
    #[builder(setter(into))]
    pub publisher_id: String,

    /// Current status of the order (required).
    pub status: OrderStatus,

    /// Currency code (required).
    #[builder(setter(into))]
    pub currency: String,

    /// Total budget for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub budget: Option<f64>,

    /// Start date of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub start_date: Option<String>,

    /// End date of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub end_date: Option<String>,

    /// Advertiser identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub advertiser_id: Option<String>,

    /// Brand identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub brand_id: Option<String>,

    /// List of contacts for the order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub contacts: Vec<super::organization::Contact>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Order {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> OrderBuilder {
        OrderBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order::builder()
            .name("Summer Campaign")
            .account_id("acc-001")
            .publisher_id("pub-001")
            .status(OrderStatus::Draft)
            .currency("USD")
            .build()
            .unwrap();

        assert_eq!(order.name, "Summer Campaign");
        assert_eq!(order.account_id, "acc-001");
        assert_eq!(order.publisher_id, "pub-001");
        assert_eq!(order.status, OrderStatus::Draft);
        assert_eq!(order.currency, "USD");
        assert!(order.id.is_none());
        assert!(order.budget.is_none());
    }

    #[test]
    fn test_order_serialization() {
        let order = Order::builder()
            .name("Test Order")
            .account_id("acc-002")
            .publisher_id("pub-002")
            .status(OrderStatus::Draft)
            .currency("EUR")
            .build()
            .unwrap();

        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("\"name\":\"Test Order\""));
        assert!(json.contains("\"status\":\"draft\""));
        assert!(json.contains("\"currency\":\"EUR\""));
    }

    #[test]
    fn test_order_deserialization() {
        let json = r#"{"name":"Deserialized Order","account_id":"acc-003","publisher_id":"pub-003","status":"pending_review","currency":"GBP"}"#;
        let order: Order = serde_json::from_str(json).unwrap();

        assert_eq!(order.name, "Deserialized Order");
        assert_eq!(order.account_id, "acc-003");
        assert_eq!(order.status, OrderStatus::PendingReview);
        assert_eq!(order.currency, "GBP");
    }

    #[test]
    fn test_order_roundtrip() {
        let order = Order::builder()
            .name("Roundtrip Order")
            .account_id("acc-004")
            .publisher_id("pub-004")
            .status(OrderStatus::Approved)
            .currency("USD")
            .budget(Some(10000.0))
            .start_date("2025-01-01")
            .end_date("2025-12-31")
            .build()
            .unwrap();

        let json = serde_json::to_string(&order).unwrap();
        let parsed: Order = serde_json::from_str(&json).unwrap();
        assert_eq!(order, parsed);
    }

    #[test]
    fn test_order_default() {
        let order = Order::builder()
            .name("Minimal Order")
            .account_id("acc-005")
            .publisher_id("pub-005")
            .status(OrderStatus::Draft)
            .currency("USD")
            .build()
            .unwrap();

        assert_eq!(order.name, "Minimal Order");
        assert!(order.id.is_none());
        assert!(order.budget.is_none());
        assert!(order.start_date.is_none());
        assert!(order.end_date.is_none());
        assert!(order.advertiser_id.is_none());
        assert!(order.brand_id.is_none());
        assert!(order.contacts.is_empty());
        assert!(order.ext.is_none());
    }
}
