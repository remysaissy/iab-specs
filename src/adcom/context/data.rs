use crate::Extension;
use crate::adcom::context::Segment;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Data Object (Section 7.8)
///
/// First-party data segment with user information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Data<Ext: Extension = serde_json::Value> {
    /// Vendor-specific data provider identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Data provider name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Array of data segments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<Segment>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Data {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DataBuilder {
        DataBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_builder() {
        let data = Data::builder()
            .id(Some("data123".to_string()))
            .name(Some("DMP Provider".to_string()))
            .build()
            .unwrap();

        assert_eq!(data.id, Some("data123".to_string()));
        assert_eq!(data.name, Some("DMP Provider".to_string()));
    }

    #[test]
    fn test_data_default() {
        let data = Data::builder().build().unwrap();

        assert!(data.id.is_none());
        assert!(data.name.is_none());
        assert!(data.segment.is_none());
    }

    #[test]
    fn test_data_with_segments() {
        let segment1 = Segment::builder()
            .id(Some("seg1".to_string()))
            .name(Some("Age".to_string()))
            .build()
            .unwrap();

        let segment2 = Segment::builder()
            .id(Some("seg2".to_string()))
            .name(Some("Interest".to_string()))
            .build()
            .unwrap();

        let data = Data::builder()
            .id(Some("data456".to_string()))
            .name(Some("First Party Data".to_string()))
            .segment(Some(vec![segment1, segment2]))
            .build()
            .unwrap();

        assert_eq!(data.id, Some("data456".to_string()));
        assert_eq!(data.segment.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_data_serialization() {
        let data = Data::builder()
            .id(Some("data789".to_string()))
            .name(Some("Data Exchange".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"id\":\"data789\""));
        assert!(json.contains("\"name\":\"Data Exchange\""));
    }

    #[test]
    fn test_data_deserialization() {
        let json = r#"{"id":"data999","name":"Third Party Data"}"#;
        let data: Data = serde_json::from_str(json).unwrap();

        assert_eq!(data.id, Some("data999".to_string()));
        assert_eq!(data.name, Some("Third Party Data".to_string()));
    }
}
