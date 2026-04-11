use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::LineStatus;
use super::super::enums::RateType;

/// Frequency cap configuration for a line.
///
/// Defines the maximum number of times an ad can be shown within a given time period.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct FrequencyCap {
    /// Maximum number of impressions within the period.
    pub count: i32,
    /// Duration of the frequency cap period in seconds.
    pub period_seconds: i64,
}

impl FrequencyCap {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> FrequencyCapBuilder {
        FrequencyCapBuilder::create_empty()
    }
}

/// Line entity.
///
/// Represents a line item within an advertising order, with targeting, budget,
/// rate configuration, and lifecycle status tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Line<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the line.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Name of the line (required).
    #[builder(setter(into))]
    pub name: String,

    /// Order identifier this line belongs to (required).
    #[builder(setter(into))]
    pub order_id: String,

    /// Product identifier (required).
    #[builder(setter(into))]
    pub product_id: String,

    /// Current status of the line (required).
    pub status: LineStatus,

    /// Start date of the line (required).
    #[builder(setter(into))]
    pub start_date: String,

    /// End date of the line (required).
    #[builder(setter(into))]
    pub end_date: String,

    /// Rate type / pricing model (required).
    pub rate_type: RateType,

    /// Rate amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rate: Option<f64>,

    /// Quantity of impressions or actions (required).
    pub quantity: i64,

    /// Budget for the line.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub budget: Option<f64>,

    /// Targeting criteria as a JSON value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub targeting: Option<serde_json::Value>,

    /// Frequency cap configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frequency_cap: Option<FrequencyCap>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Line {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> LineBuilder {
        LineBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let line = Line::builder()
            .name("Premium Banner")
            .order_id("order-001")
            .product_id("prod-001")
            .status(LineStatus::Draft)
            .start_date("2025-01-01")
            .end_date("2025-12-31")
            .rate_type(RateType::Cpm)
            .quantity(100000)
            .build()
            .unwrap();

        assert_eq!(line.name, "Premium Banner");
        assert_eq!(line.order_id, "order-001");
        assert_eq!(line.product_id, "prod-001");
        assert_eq!(line.status, LineStatus::Draft);
        assert_eq!(line.start_date, "2025-01-01");
        assert_eq!(line.end_date, "2025-12-31");
        assert_eq!(line.rate_type, RateType::Cpm);
        assert_eq!(line.quantity, 100000);
        assert!(line.id.is_none());
        assert!(line.rate.is_none());
        assert!(line.budget.is_none());
        assert!(line.targeting.is_none());
        assert!(line.frequency_cap.is_none());
    }

    #[test]
    fn test_line_serialization() {
        let line = Line::builder()
            .name("Test Line")
            .order_id("order-002")
            .product_id("prod-002")
            .status(LineStatus::Draft)
            .start_date("2025-06-01")
            .end_date("2025-06-30")
            .rate_type(RateType::Cpc)
            .quantity(50000)
            .build()
            .unwrap();

        let json = serde_json::to_string(&line).unwrap();
        assert!(json.contains("\"name\":\"Test Line\""));
        assert!(json.contains("\"status\":\"draft\""));
        assert!(json.contains("\"rate_type\":\"cpc\""));
    }

    #[test]
    fn test_line_deserialization() {
        let json = r#"{"name":"Deserialized Line","order_id":"order-003","product_id":"prod-003","status":"pending_review","start_date":"2025-01-01","end_date":"2025-12-31","rate_type":"cpm","quantity":200000}"#;
        let line: Line = serde_json::from_str(json).unwrap();

        assert_eq!(line.name, "Deserialized Line");
        assert_eq!(line.order_id, "order-003");
        assert_eq!(line.status, LineStatus::PendingReview);
        assert_eq!(line.rate_type, RateType::Cpm);
        assert_eq!(line.quantity, 200000);
    }

    #[test]
    fn test_line_roundtrip() {
        let line = Line::builder()
            .name("Roundtrip Line")
            .order_id("order-004")
            .product_id("prod-004")
            .status(LineStatus::Booked)
            .start_date("2025-01-01")
            .end_date("2025-12-31")
            .rate_type(RateType::Cpm)
            .rate(Some(5.50))
            .quantity(100000)
            .budget(Some(550.0))
            .build()
            .unwrap();

        let json = serde_json::to_string(&line).unwrap();
        let parsed: Line = serde_json::from_str(&json).unwrap();
        assert_eq!(line, parsed);
    }

    #[test]
    fn test_line_default() {
        let line = Line::builder()
            .name("Minimal Line")
            .order_id("order-005")
            .product_id("prod-005")
            .status(LineStatus::Draft)
            .start_date("2025-01-01")
            .end_date("2025-12-31")
            .rate_type(RateType::Cpm)
            .quantity(10000)
            .build()
            .unwrap();

        assert_eq!(line.name, "Minimal Line");
        assert!(line.id.is_none());
        assert!(line.rate.is_none());
        assert!(line.budget.is_none());
        assert!(line.targeting.is_none());
        assert!(line.frequency_cap.is_none());
        assert!(line.ext.is_none());
    }

    #[test]
    fn test_line_with_frequency_cap() {
        let freq_cap = FrequencyCap::builder()
            .count(3)
            .period_seconds(3600)
            .build()
            .unwrap();

        let line = Line::builder()
            .name("Capped Line")
            .order_id("order-006")
            .product_id("prod-006")
            .status(LineStatus::Draft)
            .start_date("2025-01-01")
            .end_date("2025-12-31")
            .rate_type(RateType::Cpm)
            .quantity(50000)
            .frequency_cap(Some(freq_cap))
            .build()
            .unwrap();

        let cap = line.frequency_cap.as_ref().unwrap();
        assert_eq!(cap.count, 3);
        assert_eq!(cap.period_seconds, 3600);

        let json = serde_json::to_string(&line).unwrap();
        let parsed: Line = serde_json::from_str(&json).unwrap();
        assert_eq!(line, parsed);
    }
}
