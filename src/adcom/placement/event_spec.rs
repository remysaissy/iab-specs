use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// EventSpec Object (Section 4.9)
///
/// Event tracking specification for placement-level tracking.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct EventSpec<Ext: Extension = serde_json::Value> {
    /// Event type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Array of tracking methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Vec<i32>>,

    /// Array of API frameworks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,

    /// Array of JavaScript tracker URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jstrk: Option<Vec<String>>,

    /// Array of tracking URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl EventSpec {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EventSpecBuilder {
        EventSpecBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_spec_builder() {
        let event = EventSpec::builder()
            .type_(Some(1))
            .method(Some(vec![1, 2]))
            .build()
            .unwrap();

        assert_eq!(event.type_, Some(1));
        assert_eq!(event.method, Some(vec![1, 2]));
    }

    #[test]
    fn test_event_spec_default() {
        let event = EventSpec::builder().build().unwrap();

        assert!(event.type_.is_none());
        assert!(event.method.is_none());
        assert!(event.api.is_none());
        assert!(event.jstrk.is_none());
        assert!(event.url.is_none());
    }

    #[test]
    fn test_event_spec_with_trackers() {
        let event = EventSpec::builder()
            .type_(Some(1))
            .url(Some(vec![
                "https://tracker.example.com/pixel".to_string(),
                "https://analytics.example.com/event".to_string(),
            ]))
            .build()
            .unwrap();

        assert!(event.url.is_some());
        assert_eq!(event.url.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_event_spec_with_javascript() {
        let event = EventSpec::builder()
            .type_(Some(2))
            .jstrk(Some(vec![
                "https://tracker.example.com/tracker.js".to_string(),
            ]))
            .build()
            .unwrap();

        assert_eq!(
            event.jstrk,
            Some(vec!["https://tracker.example.com/tracker.js".to_string()])
        );
    }

    #[test]
    fn test_event_spec_serialization() {
        let event = EventSpec::builder()
            .type_(Some(1))
            .method(Some(vec![1, 2]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type_\":1"));
        assert!(json.contains("\"method\":[1,2]"));
    }

    #[test]
    fn test_event_spec_deserialization() {
        let json = r#"{"type_":1,"method":[1,2],"api":[5,6]}"#;
        let event: EventSpec = serde_json::from_str(json).unwrap();

        assert_eq!(event.type_, Some(1));
        assert_eq!(event.method, Some(vec![1, 2]));
        assert_eq!(event.api, Some(vec![5, 6]));
    }
}
