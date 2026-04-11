use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Payload containing a list of string identifiers.
///
/// Used with `Intent::ActivateSegments`, `Intent::ActivateDeals`,
/// and `Intent::SuppressDeals` to specify the IDs to activate or suppress.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::IDsPayload;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let payload = IDsPayload::builder()
///     .id(vec!["seg-001".to_string(), "seg-002".to_string()])
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct IDsPayload<Ext: Extension = crate::DefaultExt> {
    /// List of string identifiers (segment IDs, deal IDs, etc.).
    #[builder(default, setter(into))]
    pub id: Vec<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl IDsPayload {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> IDsPayloadBuilder {
        IDsPayloadBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ids_payload_creation() {
        let payload = IDsPayload::builder()
            .id(vec!["seg-001".to_string(), "seg-002".to_string()])
            .build()
            .unwrap();

        assert_eq!(payload.id.len(), 2);
        assert_eq!(payload.id[0], "seg-001");
        assert_eq!(payload.id[1], "seg-002");
    }

    #[test]
    fn test_ids_payload_empty() {
        let payload = IDsPayload::builder().build().unwrap();
        assert!(payload.id.is_empty());
        assert!(payload.ext.is_none());
    }

    #[test]
    fn test_ids_payload_serialization() {
        let payload = IDsPayload::builder()
            .id(vec!["id-1".to_string(), "id-2".to_string()])
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"id\":[\"id-1\",\"id-2\"]"));
    }

    #[test]
    fn test_ids_payload_deserialization() {
        let json = r#"{"id":["seg-1","seg-2","seg-3"]}"#;
        let payload: IDsPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.id.len(), 3);
        assert_eq!(payload.id[0], "seg-1");
    }

    #[test]
    fn test_ids_payload_roundtrip() {
        let payload = IDsPayload::builder()
            .id(vec!["a".to_string(), "b".to_string()])
            .build()
            .unwrap();

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: IDsPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(payload, parsed);
    }

    #[test]
    fn test_ids_payload_single_id() {
        // Spec: IDsPayload with single ID (common for deal activation)
        let payload = IDsPayload::builder()
            .id(vec!["single-id".to_string()])
            .build()
            .unwrap();
        assert_eq!(payload.id.len(), 1);
        assert_eq!(payload.id[0], "single-id");
    }

    #[test]
    fn test_ids_payload_deserialization_extra_fields() {
        // Spec: extra JSON fields silently ignored
        let json = r#"{"id": ["a", "b"], "extra": "ignored"}"#;
        let payload: IDsPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.id.len(), 2);
    }
}
