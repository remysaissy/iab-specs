/// OpenRTB 2.5 App Object
///
/// This module implements the App object for mobile application context.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::content::Content;
use super::publisher::Publisher;

/// Default category taxonomy (1 = IAB Content Category Taxonomy 1.0)
fn default_cattax() -> i32 {
    1
}

/// App object describing non-browser application (OpenRTB 2.5 Section 3.2.14)
///
/// An `App` object should be included if the ad-supported content is a non-browser application
/// (typically in mobile). A bid request must not contain both a Site and an App object.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::App;
///
/// let app = App {
///     id: Some("app123".to_string()),
///     name: Some("My Game".to_string()),
///     bundle: Some("com.example.mygame".to_string()),
///     storeurl: Some("https://play.google.com/store/apps/details?id=com.example.mygame".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct App {
    /// Exchange-specific app ID.
    /// Recommended by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// App name (may be aliased at the publisher's request).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// A platform-specific application identifier intended to be unique to the app and
    /// independent of the exchange. On Android, this should be a bundle or package name
    /// (e.g., com.foo.mygame). On iOS, it is a numeric ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bundle: Option<String>,

    /// Domain of the app (e.g., "mygame.foo.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<String>,

    /// App store URL for an installed app; for IQG 2.1 compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub storeurl: Option<String>,

    /// The taxonomy in use for cat attribute.
    /// Default is 1 (IAB Content Category Taxonomy 1.0).
    /// Refer to AdCOM `CategoryTaxonomy` enumeration.
    #[serde(default = "default_cattax")]
    #[builder(default = "default_cattax()")]
    pub cattax: i32,

    /// Array of IAB content categories of the app.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current section of the app.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current page or view of the app.
    /// Refer to enum `ContentCategory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pagecat: Option<Vec<String>>,

    /// Application version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ver: Option<String>,

    /// Indicates if the app has a privacy policy:
    /// - 0 = no
    /// - 1 = yes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub privacypolicy: Option<i32>,

    /// Indicates if the app is free or paid:
    /// - 0 = free
    /// - 1 = paid
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub paid: Option<i32>,

    /// Details about the Publisher of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub publisher: Option<Publisher>,

    /// Details about the Content within the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub content: Option<Content>,

    /// Comma-separated list of keywords about the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub keywords: Option<String>,

    /// Array of keywords about the app.
    /// Mutually exclusive with `keywords` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub kwarray: Option<Vec<String>>,

    /// Used for inventory authorization in chain of custody scenarios.
    /// Domain of the inventory partner authorized to sell this ad space.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub inventorypartnerdomain: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            bundle: None,
            domain: None,
            storeurl: None,
            cattax: default_cattax(),
            cat: None,
            sectioncat: None,
            pagecat: None,
            ver: None,
            privacypolicy: None,
            paid: None,
            publisher: None,
            content: None,
            keywords: None,
            kwarray: None,
            inventorypartnerdomain: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App {
            id: Some("app123".to_string()),
            name: Some("My Game".to_string()),
            bundle: Some("com.example.mygame".to_string()),
            storeurl: Some("https://play.google.com/store".to_string()),
            ..Default::default()
        };

        assert_eq!(app.id, Some("app123".to_string()));
        assert_eq!(app.name, Some("My Game".to_string()));
        assert_eq!(app.bundle, Some("com.example.mygame".to_string()));
        assert_eq!(app.cattax, 1); // Default value
    }

    #[test]
    fn test_app_with_publisher() {
        let publisher = Publisher {
            id: Some("pub123".to_string()),
            name: Some("Publisher Inc".to_string()),
            ..Default::default()
        };

        let app = App {
            id: Some("app456".to_string()),
            publisher: Some(publisher),
            ..Default::default()
        };

        assert!(app.publisher.is_some());
        assert_eq!(
            app.publisher.as_ref().unwrap().id,
            Some("pub123".to_string())
        );
    }

    #[test]
    fn test_app_serialization() {
        let app = App {
            id: Some("app123".to_string()),
            bundle: Some("com.example.mygame".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&app).unwrap();
        assert!(json.contains("\"id\":\"app123\""));
        assert!(json.contains("\"bundle\":\"com.example.mygame\""));
    }

    #[test]
    fn test_app_deserialization() {
        let json = r#"{"id":"app123","bundle":"com.example.mygame"}"#;
        let app: App = serde_json::from_str(json).unwrap();

        assert_eq!(app.id, Some("app123".to_string()));
        assert_eq!(app.bundle, Some("com.example.mygame".to_string()));
    }
}
