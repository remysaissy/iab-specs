use crate::Extension;
use crate::adcom::context::{Content, Publisher};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// App Object (Section 7.2)
///
/// Distribution channel for mobile/tablet application advertising.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct App<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique app identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// App name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// App domain (e.g., "example.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Content categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Category taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Section categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Page categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// App version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// App store bundle/package name (e.g., "com.example.app")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// Privacy policy flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,

    /// Paid app flag (0=free, 1=paid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// App store URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl App {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AppBuilder {
        AppBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder() {
        let app = App::builder()
            .id(Some("app123".to_string()))
            .name(Some("My Game".to_string()))
            .bundle(Some("com.example.mygame".to_string()))
            .ver(Some("1.0.0".to_string()))
            .build()
            .unwrap();

        assert_eq!(app.id, Some("app123".to_string()));
        assert_eq!(app.name, Some("My Game".to_string()));
        assert_eq!(app.bundle, Some("com.example.mygame".to_string()));
        assert_eq!(app.ver, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_app_default() {
        let app = App::builder().build().unwrap();

        assert!(app.id.is_none());
        assert!(app.name.is_none());
        assert!(app.bundle.is_none());
        assert!(app.pub_.is_none());
    }

    #[test]
    fn test_app_with_publisher() {
        let publisher = Publisher::builder()
            .id(Some("pub456".to_string()))
            .name(Some("Game Studio".to_string()))
            .build()
            .unwrap();

        let app = App::builder()
            .id(Some("app789".to_string()))
            .pub_(Some(Box::new(publisher)))
            .build()
            .unwrap();

        assert!(app.pub_.is_some());
        assert_eq!(app.pub_.as_ref().unwrap().id, Some("pub456".to_string()));
    }

    #[test]
    fn test_app_serialization() {
        let app = App::builder()
            .id(Some("app999".to_string()))
            .bundle(Some("com.example.app".to_string()))
            .paid(Some(0))
            .privacypolicy(Some(1))
            .build()
            .unwrap();

        let json = serde_json::to_string(&app).unwrap();
        assert!(json.contains("\"id\":\"app999\""));
        assert!(json.contains("\"bundle\":\"com.example.app\""));
        assert!(json.contains("\"paid\":0"));
    }

    #[test]
    fn test_app_deserialization() {
        let json =
            r#"{"id":"app111","name":"Puzzle Game","bundle":"com.puzzle.game","ver":"2.1.0"}"#;
        let app: App = serde_json::from_str(json).unwrap();

        assert_eq!(app.id, Some("app111".to_string()));
        assert_eq!(app.name, Some("Puzzle Game".to_string()));
        assert_eq!(app.bundle, Some("com.puzzle.game".to_string()));
        assert_eq!(app.ver, Some("2.1.0".to_string()));
    }

    #[test]
    fn test_app_with_store_url() {
        let app = App::builder()
            .id(Some("app222".to_string()))
            .bundle(Some("com.social.app".to_string()))
            .storeurl(Some(
                "https://play.google.com/store/apps/details?id=com.social.app".to_string(),
            ))
            .build()
            .unwrap();

        assert!(app.storeurl.is_some());
        assert!(
            app.storeurl
                .as_ref()
                .unwrap()
                .starts_with("https://play.google.com")
        );
    }
}
