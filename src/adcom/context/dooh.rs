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
