use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// An ExecutionOrder represents a campaign order synchronized with an external ad server.
///
/// This struct tracks the state of a campaign order as it is synced to ad server platforms
/// like Google Ad Manager, FreeWheel, or other integrations. It maintains order metadata,
/// sync status, and any associated error information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::ExecutionOrder;
/// use iab_specs::seller_agent::v10::enums::{AdServerType, SyncStatus};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let order = ExecutionOrder::builder()
///     .order_id("order-123")
///     .ad_server_type(AdServerType::GoogleAdManager)
///     .sync_status(SyncStatus::Synced)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ExecutionOrder<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for this execution order.
    /// Optional field representing the internal execution order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// The original order ID from the campaign management system (REQUIRED).
    /// This references the campaign order being synchronized.
    #[builder(default, setter(into))]
    pub order_id: String,

    /// The type of ad server platform this order is synced to (REQUIRED).
    /// Specifies which ad server (Google Ad Manager, FreeWheel, etc.) is being used.
    #[builder(default)]
    pub ad_server_type: crate::seller_agent::v10::enums::AdServerType,

    /// The ad server's unique identifier for this order.
    /// Optional field that stores the order ID assigned by the external ad server.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ad_server_order_id: Option<String>,

    /// The current synchronization status of this order (REQUIRED).
    /// Tracks whether the sync is pending, in progress, completed, failed, or stale.
    #[builder(default)]
    pub sync_status: crate::seller_agent::v10::enums::SyncStatus,

    /// The timestamp when this order was last synchronized to the ad server.
    /// Optional field storing the ISO 8601 datetime of the most recent sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub last_synced_at: Option<String>,

    /// Error message if the sync operation failed.
    /// Only populated when sync_status is Failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub error_message: Option<String>,

    /// Extension object for order-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ExecutionOrder {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ExecutionOrderBuilder {
        ExecutionOrderBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seller_agent::v10::enums::{AdServerType, SyncStatus};

    #[test]
    fn test_execution_order_minimal() {
        let order = ExecutionOrder::builder()
            .order_id("order-123")
            .ad_server_type(AdServerType::GoogleAdManager)
            .sync_status(SyncStatus::Pending)
            .build()
            .unwrap();

        assert_eq!(order.order_id, "order-123");
        assert_eq!(order.ad_server_type, AdServerType::GoogleAdManager);
        assert_eq!(order.sync_status, SyncStatus::Pending);
        assert!(order.id.is_none());
        assert!(order.ad_server_order_id.is_none());
        assert!(order.last_synced_at.is_none());
        assert!(order.error_message.is_none());
        assert!(order.ext.is_none());
    }

    #[test]
    fn test_execution_order_with_google_ad_manager() {
        let order = ExecutionOrder::builder()
            .id("exec-001")
            .order_id("order-456")
            .ad_server_type(AdServerType::GoogleAdManager)
            .ad_server_order_id("gam-987654")
            .sync_status(SyncStatus::Synced)
            .last_synced_at("2026-04-01T10:30:00Z")
            .build()
            .unwrap();

        assert_eq!(order.id, Some("exec-001".to_string()));
        assert_eq!(order.order_id, "order-456");
        assert_eq!(order.ad_server_type, AdServerType::GoogleAdManager);
        assert_eq!(order.ad_server_order_id, Some("gam-987654".to_string()));
        assert_eq!(order.sync_status, SyncStatus::Synced);
        assert_eq!(
            order.last_synced_at,
            Some("2026-04-01T10:30:00Z".to_string())
        );
        assert!(order.error_message.is_none());
    }

    #[test]
    fn test_execution_order_with_syncing_status() {
        let order = ExecutionOrder::builder()
            .order_id("order-789")
            .ad_server_type(AdServerType::FreeWheel)
            .sync_status(SyncStatus::Syncing)
            .build()
            .unwrap();

        assert_eq!(order.order_id, "order-789");
        assert_eq!(order.ad_server_type, AdServerType::FreeWheel);
        assert_eq!(order.sync_status, SyncStatus::Syncing);
    }

    #[test]
    fn test_execution_order_serialization() {
        let order = ExecutionOrder::builder()
            .order_id("order-111")
            .ad_server_type(AdServerType::GoogleAdManager)
            .sync_status(SyncStatus::Synced)
            .build()
            .unwrap();

        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("\"order_id\":\"order-111\""));
        assert!(json.contains("\"ad_server_type\":\"google_ad_manager\""));
        assert!(json.contains("\"sync_status\":\"synced\""));
    }

    #[test]
    fn test_execution_order_roundtrip() {
        let original = ExecutionOrder::builder()
            .id("exec-005")
            .order_id("order-222")
            .ad_server_type(AdServerType::Csv)
            .ad_server_order_id("csv-123")
            .sync_status(SyncStatus::Stale)
            .last_synced_at("2026-03-15T14:20:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: ExecutionOrder = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.order_id, original.order_id);
        assert_eq!(parsed.ad_server_type, original.ad_server_type);
        assert_eq!(parsed.ad_server_order_id, original.ad_server_order_id);
        assert_eq!(parsed.sync_status, original.sync_status);
        assert_eq!(parsed.last_synced_at, original.last_synced_at);
    }

    #[test]
    fn test_execution_order_with_error() {
        let order = ExecutionOrder::builder()
            .order_id("order-333")
            .ad_server_type(AdServerType::Custom)
            .sync_status(SyncStatus::Failed)
            .error_message("Connection timeout to ad server")
            .build()
            .unwrap();

        assert_eq!(order.sync_status, SyncStatus::Failed);
        assert_eq!(
            order.error_message,
            Some("Connection timeout to ad server".to_string())
        );
    }

    #[test]
    fn test_execution_order_deserialization() {
        let json = r#"{
            "id": "exec-010",
            "order_id": "order-444",
            "ad_server_type": "google_ad_manager",
            "ad_server_order_id": "gam-555",
            "sync_status": "synced",
            "last_synced_at": "2026-04-01T12:00:00Z"
        }"#;

        let order: ExecutionOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, Some("exec-010".to_string()));
        assert_eq!(order.order_id, "order-444");
        assert_eq!(order.ad_server_type, AdServerType::GoogleAdManager);
        assert_eq!(order.ad_server_order_id, Some("gam-555".to_string()));
        assert_eq!(order.sync_status, SyncStatus::Synced);
        assert_eq!(
            order.last_synced_at,
            Some("2026-04-01T12:00:00Z".to_string())
        );
    }

    /// Seller Agent 1.0 § ExecutionOrder — default builder yields empty order
    #[test]
    fn test_execution_order_default() {
        let order = ExecutionOrder::builder().build().unwrap();
        assert_eq!(order.order_id, "");
        assert_eq!(order.ad_server_type, AdServerType::GoogleAdManager);
        assert_eq!(order.sync_status, SyncStatus::Pending);
        assert!(order.id.is_none());
        assert!(order.ad_server_order_id.is_none());
        assert!(order.last_synced_at.is_none());
        assert!(order.error_message.is_none());
        assert!(order.ext.is_none());
    }

    /// Seller Agent 1.0 § ExecutionOrder — optional fields omitted from JSON when None
    #[test]
    fn test_execution_order_optional_fields_skipped() {
        let order = ExecutionOrder::builder()
            .order_id("order-skip")
            .ad_server_type(AdServerType::FreeWheel)
            .sync_status(SyncStatus::Pending)
            .build()
            .unwrap();

        let json = serde_json::to_string(&order).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"ad_server_order_id\""));
        assert!(!json.contains("\"last_synced_at\""));
        assert!(!json.contains("\"error_message\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § ExecutionOrder — clone produces identical value
    #[test]
    fn test_execution_order_clone() {
        let order = ExecutionOrder::builder()
            .id("exec-c")
            .order_id("order-c")
            .ad_server_type(AdServerType::Csv)
            .sync_status(SyncStatus::Synced)
            .last_synced_at("2026-01-01T00:00:00Z")
            .build()
            .unwrap();

        let cloned = order.clone();
        assert_eq!(order, cloned);
    }

    /// Seller Agent 1.0 § ExecutionOrder — deserialization from minimal JSON
    #[test]
    fn test_execution_order_deserialization_minimal() {
        let json = r#"{"order_id":"o1","ad_server_type":"free_wheel","sync_status":"pending"}"#;
        let order: ExecutionOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_id, "o1");
        assert_eq!(order.ad_server_type, AdServerType::FreeWheel);
        assert_eq!(order.sync_status, SyncStatus::Pending);
        assert!(order.id.is_none());
        assert!(order.ad_server_order_id.is_none());
    }
}
