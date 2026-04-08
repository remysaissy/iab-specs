use super::content::Content;
use super::publisher::Publisher;
use crate::Extension;

/// OpenRTB 2.5 App Object
///
/// This module implements the App object for mobile application context.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Default category taxonomy (1 = IAB Content Category Taxonomy 1.0)
fn default_cattax() -> i32 {
    1
}

/// App object describing non-browser application (OpenRTB 2.5 Section 3.2.14)
///
/// An `App` object should be included if the ad-supported content is a non-browser application
/// (typically in mobile). A bid request must not contain both a Site and an App object.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::App;
///
/// let app = App::builder()
///     .id(Some("app123".to_string()))
///     .name(Some("My Game".to_string()))
///     .bundle(Some("com.example.mygame".to_string()))
///     .storeurl(Some("https://play.google.com/store/apps/details?id=com.example.mygame".to_string()))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct App<Ext: Extension = crate::DefaultExt> {
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
    pub ext: Option<Box<Ext>>,
}

impl App {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AppBuilder {
        AppBuilder::create_empty()
    }
}

impl<Ext: Extension> Default for App<Ext> {
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
        let app = App::builder()
            .id(Some("app123".to_string()))
            .name(Some("My Game".to_string()))
            .bundle(Some("com.example.mygame".to_string()))
            .storeurl(Some("https://play.google.com/store".to_string()))
            .build()
            .unwrap();

        assert_eq!(app.id, Some("app123".to_string()));
        assert_eq!(app.name, Some("My Game".to_string()));
        assert_eq!(app.bundle, Some("com.example.mygame".to_string()));
        assert_eq!(app.cattax, 1); // Default value
    }

    #[test]
    fn test_app_with_publisher() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Publisher Inc".to_string()))
            .build()
            .unwrap();

        let app = App::builder()
            .id(Some("app456".to_string()))
            .publisher(Some(publisher))
            .build()
            .unwrap();

        assert!(app.publisher.is_some());
        assert_eq!(
            app.publisher.as_ref().unwrap().id,
            Some("pub123".to_string())
        );
    }

    #[test]
    fn test_app_serialization() {
        let app = App::builder()
            .id(Some("app123".to_string()))
            .bundle(Some("com.example.mygame".to_string()))
            .build()
            .unwrap();

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

    #[test]
    fn test_app_bundle_field() {
        // Spec: Section 3.2.14
        let app_android = App::builder()
            .bundle(Some("com.example.mygame".to_string()))
            .build()
            .unwrap();
        assert_eq!(app_android.bundle, Some("com.example.mygame".to_string()));

        let app_ios = App::builder()
            .bundle(Some("123456789".to_string()))
            .build()
            .unwrap();
        assert_eq!(app_ios.bundle, Some("123456789".to_string()));
    }

    #[test]
    fn test_app_storeurl_field() {
        // Spec: Section 3.2.14
        let app = App::builder()
            .storeurl(Some(
                "https://play.google.com/store/apps/details?id=com.example.mygame".to_string(),
            ))
            .build()
            .unwrap();
        assert_eq!(
            app.storeurl,
            Some("https://play.google.com/store/apps/details?id=com.example.mygame".to_string())
        );
    }

    #[test]
    fn test_app_category_arrays() {
        // Spec: Section 3.2.14
        let app = App::builder()
            .cat(Some(vec!["IAB9".to_string(), "IAB9-30".to_string()]))
            .sectioncat(Some(vec!["IAB9-5".to_string()]))
            .pagecat(Some(vec!["IAB9-7".to_string()]))
            .build()
            .unwrap();

        assert_eq!(
            app.cat,
            Some(vec!["IAB9".to_string(), "IAB9-30".to_string()])
        );
        assert_eq!(app.sectioncat, Some(vec!["IAB9-5".to_string()]));
        assert_eq!(app.pagecat, Some(vec!["IAB9-7".to_string()]));
    }

    #[test]
    fn test_app_ver_field() {
        // Spec: Section 3.2.14
        let app = App::builder()
            .ver(Some("2.1.0".to_string()))
            .build()
            .unwrap();
        assert_eq!(app.ver, Some("2.1.0".to_string()));
    }

    #[test]
    fn test_app_paid_flag() {
        // Spec: Section 3.2.14
        let free_app = App::builder().paid(Some(0)).build().unwrap();
        assert_eq!(free_app.paid, Some(0));

        let paid_app = App::builder().paid(Some(1)).build().unwrap();
        assert_eq!(paid_app.paid, Some(1));

        let default_app = App::builder().build().unwrap();
        assert_eq!(default_app.paid, None);
    }

    #[test]
    fn test_app_privacypolicy_flag() {
        // Spec: Section 3.2.14
        let app_with = App::builder().privacypolicy(Some(1)).build().unwrap();
        assert_eq!(app_with.privacypolicy, Some(1));

        let app_without = App::builder().privacypolicy(Some(0)).build().unwrap();
        assert_eq!(app_without.privacypolicy, Some(0));
    }

    #[test]
    fn test_app_ext_field() {
        // Spec: Section 3.2.14
        let app = AppBuilder::<serde_json::Value>::default()
            .id(Some("app-ext".to_string()))
            .ext(Some(Box::new(serde_json::json!({
                "app_specific": true
            }))))
            .build()
            .unwrap();

        assert!(app.ext.is_some());
        assert_eq!(app.ext.as_ref().unwrap()["app_specific"], true);
    }

    #[test]
    fn test_app_roundtrip_all_fields() {
        // Spec: Section 3.2.14
        let publisher = Publisher::builder()
            .id(Some("pub-1".to_string()))
            .build()
            .unwrap();

        let content = Content::builder()
            .id(Some("content-1".to_string()))
            .build()
            .unwrap();

        let app = App::builder()
            .id(Some("app-all".to_string()))
            .name(Some("All Fields App".to_string()))
            .bundle(Some("com.all.fields".to_string()))
            .domain(Some("allfields.com".to_string()))
            .storeurl(Some("https://store.example.com".to_string()))
            .cattax(2)
            .cat(Some(vec!["IAB1".to_string()]))
            .sectioncat(Some(vec!["IAB1-1".to_string()]))
            .pagecat(Some(vec!["IAB2-1".to_string()]))
            .ver(Some("3.0.1".to_string()))
            .privacypolicy(Some(1))
            .paid(Some(1))
            .publisher(Some(publisher))
            .content(Some(content))
            .keywords(Some("game,puzzle".to_string()))
            .inventorypartnerdomain(Some("partner.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&app).unwrap();
        let deserialized: App = serde_json::from_str(&json).unwrap();

        assert_eq!(app.id, deserialized.id);
        assert_eq!(app.name, deserialized.name);
        assert_eq!(app.bundle, deserialized.bundle);
        assert_eq!(app.domain, deserialized.domain);
        assert_eq!(app.storeurl, deserialized.storeurl);
        assert_eq!(app.cattax, deserialized.cattax);
        assert_eq!(app.cat, deserialized.cat);
        assert_eq!(app.sectioncat, deserialized.sectioncat);
        assert_eq!(app.pagecat, deserialized.pagecat);
        assert_eq!(app.ver, deserialized.ver);
        assert_eq!(app.privacypolicy, deserialized.privacypolicy);
        assert_eq!(app.paid, deserialized.paid);
        assert_eq!(app.publisher, deserialized.publisher);
        assert_eq!(app.content, deserialized.content);
        assert_eq!(app.keywords, deserialized.keywords);
        assert_eq!(
            app.inventorypartnerdomain,
            deserialized.inventorypartnerdomain
        );
    }
}
