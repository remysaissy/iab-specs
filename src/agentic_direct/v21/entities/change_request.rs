use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Change request entity.
///
/// Represents a requested modification to an existing order, including the requested
/// changes payload and the current review status.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ChangeRequest<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the change request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Order identifier this change request applies to (required).
    #[builder(setter(into))]
    pub order_id: String,

    /// Type of requested change (required).
    #[builder(setter(into))]
    pub change_type: String,

    /// Human-readable description of the requested change.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// Arbitrary JSON payload describing the requested changes (required).
    pub requested_changes: serde_json::Value,

    /// Current processing status for the change request (required).
    #[builder(setter(into))]
    pub status: String,

    /// Extension object for custom fields.
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
    fn test_change_request_creation() {
        let change_request = ChangeRequest::builder()
            .order_id("order-001")
            .change_type("date_shift")
            .requested_changes(serde_json::json!({"start_date": "2025-02-01"}))
            .status("pending")
            .build()
            .unwrap();

        assert_eq!(change_request.order_id, "order-001");
        assert_eq!(change_request.change_type, "date_shift");
        assert_eq!(change_request.status, "pending");
        assert!(change_request.id.is_none());
        assert!(change_request.description.is_none());
        assert!(change_request.ext.is_none());
    }

    #[test]
    fn test_change_request_serialization() {
        let change_request = ChangeRequest::builder()
            .id("cr-123")
            .order_id("order-002")
            .change_type("cancellation")
            .description("Cancel due to revised media plan")
            .requested_changes(serde_json::json!({"cancel": true, "effective_date": "2025-03-15"}))
            .status("approved")
            .build()
            .unwrap();

        let json = serde_json::to_string(&change_request).unwrap();
        assert!(json.contains("\"id\":\"cr-123\""));
        assert!(json.contains("\"order_id\":\"order-002\""));
        assert!(json.contains("\"change_type\":\"cancellation\""));
        assert!(json.contains("\"description\":\"Cancel due to revised media plan\""));
        assert!(json.contains("\"status\":\"approved\""));
        assert!(
            json.contains(
                "\"requested_changes\":{\"cancel\":true,\"effective_date\":\"2025-03-15\"}"
            )
        );
    }

    #[test]
    fn test_change_request_deserialization() {
        let json = r#"{"order_id":"order-003","change_type":"date_shift","description":"Move campaign by one week","requested_changes":{"start_date":"2025-04-08","end_date":"2025-04-30"},"status":"pending"}"#;
        let change_request: ChangeRequest = serde_json::from_str(json).unwrap();

        assert_eq!(change_request.order_id, "order-003");
        assert_eq!(change_request.change_type, "date_shift");
        assert_eq!(
            change_request.description,
            Some("Move campaign by one week".to_string())
        );
        assert_eq!(change_request.requested_changes["start_date"], "2025-04-08");
        assert_eq!(change_request.requested_changes["end_date"], "2025-04-30");
        assert_eq!(change_request.status, "pending");
    }

    #[test]
    fn test_change_request_roundtrip() {
        let change_request = ChangeRequest::builder()
            .id("cr-999")
            .order_id("order-004")
            .change_type("budget_update")
            .description("Increase budget for expanded audience")
            .requested_changes(serde_json::json!({"budget": 25000.0, "currency": "USD"}))
            .status("approved")
            .build()
            .unwrap();

        let json = serde_json::to_string(&change_request).unwrap();
        let parsed: ChangeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(change_request, parsed);
    }

    #[test]
    fn test_change_request_with_json_changes() {
        let requested_changes = serde_json::json!({
            "line_items": [
                {"id": "line-1", "quantity": 150000},
                {"id": "line-2", "status": "paused"}
            ],
            "notes": {
                "requested_by": "planner@example.com",
                "priority": "high"
            }
        });

        let change_request = ChangeRequest::builder()
            .order_id("order-005")
            .change_type("line_updates")
            .requested_changes(requested_changes.clone())
            .status("pending")
            .build()
            .unwrap();

        assert_eq!(change_request.requested_changes, requested_changes);

        let json = serde_json::to_string(&change_request).unwrap();
        let parsed: ChangeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(change_request.requested_changes, parsed.requested_changes);
    }

    #[test]
    fn test_change_request_malformed_json_rejected() {
        // Missing required field "requested_changes" (Value type, must be present)
        let json = r#"{"order_id":"o-1","change_type":"cancel","status":"pending"}"#;
        let result: Result<ChangeRequest, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Missing requested_changes should fail");

        // Completely invalid JSON
        let json = r#"{"order_id": broken}"#;
        let result: Result<ChangeRequest, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Broken JSON should fail deserialization");
    }
}
