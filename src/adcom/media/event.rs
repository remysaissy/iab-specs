use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Event Object (Section 3.11)
///
/// Tracks advertiser or buyer events for measurement purposes.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Event<Ext: Extension = serde_json::Value> {
    /// Event type (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Tracking method (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<i32>,

    /// Array of tracking URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,

    /// Array of JavaScript trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jstrk: Option<Vec<String>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Event {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> EventBuilder {
        EventBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_builder() {
        let event = Event::builder()
            .type_(Some(1))
            .method(Some(1))
            .url(Some(vec!["https://tracker.example.com/event".to_string()]))
            .build()
            .unwrap();

        assert_eq!(event.type_, Some(1));
        assert_eq!(event.method, Some(1));
        assert_eq!(
            event.url,
            Some(vec!["https://tracker.example.com/event".to_string()])
        );
    }

    #[test]
    fn test_event_default() {
        let event = Event::builder().build().unwrap();

        assert!(event.type_.is_none());
        assert!(event.method.is_none());
        assert!(event.url.is_none());
    }

    #[test]
    fn test_event_with_jstrk() {
        let event = Event::builder()
            .type_(Some(2))
            .method(Some(2))
            .jstrk(Some(vec!["<script>...</script>".to_string()]))
            .build()
            .unwrap();

        assert_eq!(event.jstrk, Some(vec!["<script>...</script>".to_string()]));
    }

    #[test]
    fn test_event_serialization() {
        let event = Event::builder()
            .type_(Some(1))
            .method(Some(1))
            .url(Some(vec!["https://track.com/pixel".to_string()]))
            .build()
            .unwrap();

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type_\":1"));
        assert!(json.contains("\"method\":1"));
        assert!(json.contains("\"url\":[\"https://track.com/pixel\"]"));
    }

    #[test]
    fn test_event_deserialization() {
        let json = r#"{"type_":1,"method":1,"url":["https://example.com/track"]}"#;
        let event: Event = serde_json::from_str(json).unwrap();

        assert_eq!(event.type_, Some(1));
        assert_eq!(event.method, Some(1));
        assert_eq!(
            event.url,
            Some(vec!["https://example.com/track".to_string()])
        );
    }
}
