use crate::Extension;
use crate::adcom::placement::DisplayPlacement;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Companion Object (Section 4.12)
///
/// Companion ad specification for video/audio ads.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Companion<Ext: Extension = serde_json::Value> {
    /// Companion ad identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Companion type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,

    /// Display placement for companion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<DisplayPlacement>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Companion {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CompanionBuilder {
        CompanionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_companion_builder() {
        let companion = Companion::builder()
            .id(Some("comp1".to_string()))
            .w(Some(300))
            .h(Some(250))
            .type_(Some(1))
            .build()
            .unwrap();

        assert_eq!(companion.id, Some("comp1".to_string()));
        assert_eq!(companion.w, Some(300));
        assert_eq!(companion.h, Some(250));
        assert_eq!(companion.type_, Some(1));
    }

    #[test]
    fn test_companion_default() {
        let companion = Companion::builder().build().unwrap();

        assert!(companion.id.is_none());
        assert!(companion.w.is_none());
        assert!(companion.h.is_none());
        assert!(companion.type_.is_none());
        assert!(companion.display.is_none());
    }

    #[test]
    fn test_companion_with_display() {
        let display = DisplayPlacement::builder()
            .pos(Some(1))
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        let companion = Companion::builder()
            .id(Some("comp2".to_string()))
            .display(Some(Box::new(display)))
            .build()
            .unwrap();

        assert!(companion.display.is_some());
        assert_eq!(companion.display.as_ref().unwrap().pos, Some(1));
    }

    #[test]
    fn test_companion_serialization() {
        let companion = Companion::builder()
            .id(Some("comp3".to_string()))
            .w(Some(728))
            .h(Some(90))
            .build()
            .unwrap();

        let json = serde_json::to_string(&companion).unwrap();
        assert!(json.contains("\"id\":\"comp3\""));
        assert!(json.contains("\"w\":728"));
        assert!(json.contains("\"h\":90"));
    }

    #[test]
    fn test_companion_deserialization() {
        let json = r#"{"id":"comp4","w":300,"h":250,"type_":1}"#;
        let companion: Companion = serde_json::from_str(json).unwrap();

        assert_eq!(companion.id, Some("comp4".to_string()));
        assert_eq!(companion.w, Some(300));
        assert_eq!(companion.h, Some(250));
        assert_eq!(companion.type_, Some(1));
    }
}
