use crate::Extension;
use crate::adcom::context::{Content, Publisher};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Dooh Object (Section 7.3)
///
/// Distribution channel for Digital Out-of-Home (DOOH) advertising.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Dooh<Ext: Extension = serde_json::Value> {
    /// Vendor-specific unique DOOH identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// DOOH venue name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Publisher of the DOOH venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_: Option<Box<Publisher>>,

    /// Content currently being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<Content>>,

    /// Publisher domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Venue type categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Venue type taxonomy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i32>,

    /// Array of venue type IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetype: Option<Vec<String>>,

    /// Venue type taxonomy used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetypetax: Option<i32>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl Dooh {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DoohBuilder {
        DoohBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dooh_builder() {
        let dooh = Dooh::builder()
            .id(Some("dooh123".to_string()))
            .name(Some("Times Square Billboard".to_string()))
            .domain(Some("billboard.example.com".to_string()))
            .build()
            .unwrap();

        assert_eq!(dooh.id, Some("dooh123".to_string()));
        assert_eq!(dooh.name, Some("Times Square Billboard".to_string()));
        assert_eq!(dooh.domain, Some("billboard.example.com".to_string()));
    }

    #[test]
    fn test_dooh_default() {
        let dooh = Dooh::builder().build().unwrap();

        assert!(dooh.id.is_none());
        assert!(dooh.name.is_none());
        assert!(dooh.pub_.is_none());
        assert!(dooh.content.is_none());
    }

    #[test]
    fn test_dooh_with_publisher() {
        let publisher = Publisher::builder()
            .id(Some("pub123".to_string()))
            .name(Some("Digital Signage Co".to_string()))
            .build()
            .unwrap();

        let dooh = Dooh::builder()
            .id(Some("dooh456".to_string()))
            .pub_(Some(Box::new(publisher)))
            .build()
            .unwrap();

        assert!(dooh.pub_.is_some());
        assert_eq!(dooh.pub_.as_ref().unwrap().id, Some("pub123".to_string()));
    }

    #[test]
    fn test_dooh_serialization() {
        let dooh = Dooh::builder()
            .id(Some("dooh789".to_string()))
            .name(Some("Airport Terminal".to_string()))
            .domain(Some("airport-ads.com".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&dooh).unwrap();
        assert!(json.contains("\"id\":\"dooh789\""));
        assert!(json.contains("\"name\":\"Airport Terminal\""));
        assert!(json.contains("\"domain\":\"airport-ads.com\""));
    }

    #[test]
    fn test_dooh_deserialization() {
        let json = r#"{"id":"dooh999","name":"Shopping Mall Display","domain":"mall-media.com"}"#;
        let dooh: Dooh = serde_json::from_str(json).unwrap();

        assert_eq!(dooh.id, Some("dooh999".to_string()));
        assert_eq!(dooh.name, Some("Shopping Mall Display".to_string()));
        assert_eq!(dooh.domain, Some("mall-media.com".to_string()));
    }

    #[test]
    fn test_dooh_with_venue_types() {
        let dooh = Dooh::builder()
            .id(Some("dooh111".to_string()))
            .name(Some("Transit Station".to_string()))
            .venuetype(Some(vec!["transit".to_string(), "subway".to_string()]))
            .venuetypetax(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            dooh.venuetype,
            Some(vec!["transit".to_string(), "subway".to_string()])
        );
        assert_eq!(dooh.venuetypetax, Some(1));
    }

    #[test]
    fn test_dooh_with_categories() {
        let dooh = Dooh::builder()
            .id(Some("dooh222".to_string()))
            .name(Some("Sports Stadium".to_string()))
            .cat(Some(vec!["IAB17".to_string(), "IAB17-1".to_string()]))
            .cattax(Some(1))
            .build()
            .unwrap();

        assert_eq!(
            dooh.cat,
            Some(vec!["IAB17".to_string(), "IAB17-1".to_string()])
        );
        assert_eq!(dooh.cattax, Some(1));
    }
}
