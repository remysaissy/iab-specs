use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Segment Object (Section 7.9)
///
/// Specific data segment about a user.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Segment<Ext: Extension = serde_json::Value> {
    /// Segment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Segment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Segment value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Segment {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SegmentBuilder {
        SegmentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_builder() {
        let segment = Segment::builder()
            .id(Some("seg123".to_string()))
            .name(Some("Demographics".to_string()))
            .value(Some("25-34".to_string()))
            .build()
            .unwrap();

        assert_eq!(segment.id, Some("seg123".to_string()));
        assert_eq!(segment.name, Some("Demographics".to_string()));
        assert_eq!(segment.value, Some("25-34".to_string()));
    }

    #[test]
    fn test_segment_default() {
        let segment = Segment::builder().build().unwrap();

        assert!(segment.id.is_none());
        assert!(segment.name.is_none());
        assert!(segment.value.is_none());
    }

    #[test]
    fn test_segment_serialization() {
        let segment = Segment::builder()
            .id(Some("seg456".to_string()))
            .name(Some("Interest".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&segment).unwrap();
        assert!(json.contains("\"id\":\"seg456\""));
        assert!(json.contains("\"name\":\"Interest\""));
    }

    #[test]
    fn test_segment_deserialization() {
        let json = r#"{"id":"seg789","name":"Behavioral","value":"high_intent"}"#;
        let segment: Segment = serde_json::from_str(json).unwrap();

        assert_eq!(segment.id, Some("seg789".to_string()));
        assert_eq!(segment.name, Some("Behavioral".to_string()));
        assert_eq!(segment.value, Some("high_intent".to_string()));
    }
}
