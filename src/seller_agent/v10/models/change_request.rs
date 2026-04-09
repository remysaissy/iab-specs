use crate::Extension;
use crate::seller_agent::v10::enums::{ChangeRequestStatus, ChangeSeverity, ChangeType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A ChangeRequest represents a proposed modification to an advertising order.
///
/// Change requests track modifications like date shifts, impression adjustments,
/// price changes, cancellations, and creative swaps. Each request moves through
/// a workflow: Pending → (Approved|Rejected) → Applied.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::ChangeRequest;
/// use iab_specs::seller_agent::v10::enums::{ChangeType, ChangeSeverity, ChangeRequestStatus};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let request = ChangeRequest::builder()
///     .order_id("order-123".to_string())
///     .change_type(ChangeType::PriceChange)
///     .severity(ChangeSeverity::Material)
///     .description("Adjust CPM from $5.00 to $6.50".to_string())
///     .requested_changes(serde_json::json!({
///         "old_cpm": 5.00,
///         "new_cpm": 6.50
///     }))
///     .status(ChangeRequestStatus::Pending)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ChangeRequest<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the change request (OPTIONAL).
    /// Generated server-side if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// The order ID that this change request applies to (REQUIRED).
    /// Must reference an existing order.
    #[builder(default, setter(into))]
    pub order_id: String,

    /// The type of change being requested (REQUIRED).
    /// Categorizes the nature of the modification.
    #[builder(default)]
    pub change_type: ChangeType,

    /// The severity level of this change (REQUIRED).
    /// Indicates how impactful the change is.
    #[builder(default)]
    pub severity: ChangeSeverity,

    /// Human-readable description of the change (REQUIRED).
    /// Explains what is being changed and why.
    #[builder(default, setter(into))]
    pub description: String,

    /// Structured data describing the requested changes (REQUIRED).
    /// Contains the specific parameters being modified, format depends on change_type.
    #[builder(default)]
    pub requested_changes: serde_json::Value,

    /// Current status of the change request (REQUIRED).
    /// Tracks progression through the approval workflow.
    #[builder(default)]
    pub status: ChangeRequestStatus,

    /// Account/person who reviewed the change request (OPTIONAL).
    /// Set when status transitions to Approved or Rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reviewer: Option<String>,

    /// Timestamp when the change request was reviewed (OPTIONAL).
    /// ISO 8601 format when status changed from Pending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reviewed_at: Option<String>,

    /// Extension object for change request-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ChangeRequest {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ChangeRequestBuilder {
        ChangeRequestBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_request_minimal() {
        let request = ChangeRequest::builder()
            .order_id("order-123".to_string())
            .change_type(ChangeType::DateShift)
            .severity(ChangeSeverity::Minor)
            .description("Shift dates by one week".to_string())
            .requested_changes(serde_json::json!({"shift_days": 7}))
            .build()
            .unwrap();

        assert_eq!(request.order_id, "order-123");
        assert_eq!(request.change_type, ChangeType::DateShift);
        assert_eq!(request.severity, ChangeSeverity::Minor);
        assert_eq!(request.description, "Shift dates by one week");
        assert_eq!(request.status, ChangeRequestStatus::Pending);
        assert!(request.id.is_none());
        assert!(request.reviewer.is_none());
        assert!(request.reviewed_at.is_none());
        assert!(request.ext.is_none());
    }

    #[test]
    fn test_change_request_with_id() {
        let request = ChangeRequest::builder()
            .id("change-456".to_string())
            .order_id("order-123".to_string())
            .change_type(ChangeType::PriceChange)
            .severity(ChangeSeverity::Critical)
            .description("Adjust pricing".to_string())
            .requested_changes(serde_json::json!({"new_cpm": 10.0}))
            .build()
            .unwrap();

        assert_eq!(request.id, Some("change-456".to_string()));
        assert_eq!(request.change_type, ChangeType::PriceChange);
        assert_eq!(request.severity, ChangeSeverity::Critical);
    }

    #[test]
    fn test_change_request_with_critical_severity() {
        let request = ChangeRequest::builder()
            .order_id("order-789".to_string())
            .change_type(ChangeType::Cancellation)
            .severity(ChangeSeverity::Critical)
            .description("Emergency cancellation due to compliance issue".to_string())
            .requested_changes(serde_json::json!({"reason": "compliance"}))
            .build()
            .unwrap();

        assert_eq!(request.severity, ChangeSeverity::Critical);
        assert_eq!(request.change_type, ChangeType::Cancellation);
    }

    #[test]
    fn test_change_request_full_with_review() {
        let request = ChangeRequest::builder()
            .id("change-101".to_string())
            .order_id("order-555".to_string())
            .change_type(ChangeType::CreativeSwap)
            .severity(ChangeSeverity::Material)
            .description("Replace creative due to performance issues".to_string())
            .requested_changes(serde_json::json!({
                "old_creative_id": "creative-001",
                "new_creative_id": "creative-002"
            }))
            .status(ChangeRequestStatus::Approved)
            .reviewer("reviewer@example.com".to_string())
            .reviewed_at("2026-04-02T10:30:00Z".to_string())
            .build()
            .unwrap();

        assert_eq!(request.id, Some("change-101".to_string()));
        assert_eq!(request.status, ChangeRequestStatus::Approved);
        assert_eq!(request.reviewer, Some("reviewer@example.com".to_string()));
        assert_eq!(
            request.reviewed_at,
            Some("2026-04-02T10:30:00Z".to_string())
        );
    }

    #[test]
    fn test_change_request_serialization_roundtrip() {
        let original = ChangeRequest::builder()
            .id("change-202".to_string())
            .order_id("order-999".to_string())
            .change_type(ChangeType::ImpressionAdjustment)
            .severity(ChangeSeverity::Material)
            .description("Adjust impressions from 1M to 1.5M".to_string())
            .requested_changes(serde_json::json!({
                "old_impressions": 1000000,
                "new_impressions": 1500000
            }))
            .status(ChangeRequestStatus::Pending)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: ChangeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.order_id, original.order_id);
        assert_eq!(parsed.change_type, original.change_type);
        assert_eq!(parsed.severity, original.severity);
        assert_eq!(parsed.description, original.description);
        assert_eq!(parsed.requested_changes, original.requested_changes);
        assert_eq!(parsed.status, original.status);
    }

    #[test]
    fn test_change_request_deserialization_with_json_payload() {
        let json = r#"{
            "id": "change-333",
            "order_id": "order-111",
            "change_type": "price_change",
            "severity": "critical",
            "description": "Update CPM pricing",
            "requested_changes": {
                "old_cpm": 5.0,
                "new_cpm": 8.0,
                "effective_date": "2026-04-15"
            },
            "status": "approved",
            "reviewer": "admin@seller.com",
            "reviewed_at": "2026-04-02T14:22:00Z"
        }"#;

        let request: ChangeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.id, Some("change-333".to_string()));
        assert_eq!(request.order_id, "order-111");
        assert_eq!(request.change_type, ChangeType::PriceChange);
        assert_eq!(request.severity, ChangeSeverity::Critical);
        assert_eq!(request.status, ChangeRequestStatus::Approved);
        assert_eq!(request.reviewer, Some("admin@seller.com".to_string()));
    }

    #[test]
    fn test_change_request_optional_fields_skipped_in_serialization() {
        let request = ChangeRequest::builder()
            .order_id("order-444".to_string())
            .change_type(ChangeType::DateShift)
            .severity(ChangeSeverity::Minor)
            .description("Minor date adjustment".to_string())
            .requested_changes(serde_json::json!({"days": 1}))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        // Verify optional fields are not in JSON when None
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"reviewer\""));
        assert!(!json.contains("\"reviewed_at\""));
        assert!(!json.contains("\"ext\""));
    }

    #[test]
    fn test_change_request_default_values() {
        let request: ChangeRequest = ChangeRequest::default();
        assert_eq!(request.order_id, "");
        assert_eq!(request.change_type, ChangeType::DateShift);
        assert_eq!(request.severity, ChangeSeverity::Minor);
        assert_eq!(request.description, "");
        assert_eq!(request.status, ChangeRequestStatus::Pending);
        assert_eq!(request.requested_changes, serde_json::Value::Null);
    }

    /// Seller Agent 1.0 § ChangeRequest — clone produces identical value
    #[test]
    fn test_change_request_clone() {
        let request = ChangeRequest::builder()
            .id("cr-clone")
            .order_id("order-clone")
            .change_type(ChangeType::PriceChange)
            .severity(ChangeSeverity::Material)
            .description("Clone test")
            .requested_changes(serde_json::json!({"cpm": 5.0}))
            .status(ChangeRequestStatus::Approved)
            .reviewer("admin@test.com")
            .reviewed_at("2026-04-01T00:00:00Z")
            .build()
            .unwrap();

        let cloned = request.clone();
        assert_eq!(request, cloned);
    }

    /// Seller Agent 1.0 § ChangeRequest — deserialization from minimal JSON
    #[test]
    fn test_change_request_deserialization_minimal() {
        let json = r#"{
            "order_id":"o1",
            "change_type":"date_shift",
            "severity":"minor",
            "description":"shift",
            "requested_changes":{"days":1},
            "status":"pending"
        }"#;
        let request: ChangeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.order_id, "o1");
        assert_eq!(request.change_type, ChangeType::DateShift);
        assert_eq!(request.severity, ChangeSeverity::Minor);
        assert_eq!(request.status, ChangeRequestStatus::Pending);
        assert!(request.id.is_none());
        assert!(request.reviewer.is_none());
        assert!(request.reviewed_at.is_none());
    }
}
