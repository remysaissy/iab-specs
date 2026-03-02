use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Payload containing OpenRTB Data objects.
///
/// Used with `Intent::AddCids` to add extended content data.
/// The `data` field contains OpenRTB `Data` objects represented as JSON values,
/// allowing compatibility with any OpenRTB version.
///
/// # Generic Parameters
///
/// * `P` - Payload type for data objects (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// #[cfg(all(feature = "json", not(feature = "proto")))]
/// {
/// use iab_specs::artb::v10::DataPayload;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let payload = DataPayload::builder()
///     .data(vec![
///         serde_json::json!({
///             "id": "data-provider-1",
///             "name": "Content Taxonomy",
///             "segment": [{"id": "cat-123", "value": "Sports"}]
///         }),
///     ])
///     .build()?;
/// # Ok(())
/// # }
/// }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "P: Extension, Ext: Extension",
    deserialize = "P: Extension, Ext: Extension"
))]
pub struct DataPayload<P: Extension = crate::DefaultExt, Ext: Extension = crate::DefaultExt> {
    /// Array of OpenRTB Data objects containing content classification data.
    #[builder(default, setter(into))]
    pub data: Vec<P>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl DataPayload {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataPayloadBuilder<crate::DefaultExt, crate::DefaultExt> {
        DataPayloadBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(feature = "json", not(feature = "proto")))]
    #[test]
    fn test_data_payload_creation() {
        let payload = DataPayload::builder()
            .data(vec![serde_json::json!({
                "id": "provider-1",
                "name": "Taxonomy",
                "segment": [{"id": "seg-1"}]
            })])
            .build()
            .unwrap();

        assert_eq!(payload.data.len(), 1);
    }

    #[test]
    fn test_data_payload_empty() {
        let payload = DataPayload::builder().build().unwrap();
        assert!(payload.data.is_empty());
    }

    #[cfg(all(feature = "json", not(feature = "proto")))]
    #[test]
    fn test_data_payload_serialization() {
        let payload = DataPayload::builder()
            .data(vec![serde_json::json!({"id": "dp-1", "name": "Provider"})])
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"id\":\"dp-1\""));
    }

    #[cfg(all(feature = "json", not(feature = "proto")))]
    #[test]
    fn test_data_payload_deserialization() {
        let json =
            r#"{"data":[{"id":"dp-1","name":"Provider","segment":[{"id":"s1","value":"v1"}]}]}"#;
        let payload: DataPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.data.len(), 1);
        assert_eq!(payload.data[0]["id"], "dp-1");
    }

    #[cfg(all(feature = "json", not(feature = "proto")))]
    #[test]
    fn test_data_payload_roundtrip() {
        let payload = DataPayload::builder()
            .data(vec![
                serde_json::json!({"id": "dp-1", "name": "Provider 1"}),
                serde_json::json!({"id": "dp-2", "name": "Provider 2"}),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: DataPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(payload, parsed);
    }
}
