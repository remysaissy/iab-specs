use crate::Extension;
use crate::adcom::context::{Data, Geo};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// User Object (Section 7.7)
///
/// Information about the human user.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct User<Ext: Extension = serde_json::Value> {
    /// Vendor-specific user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Buyer-specific user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<String>,

    /// Year of birth (4-digit integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<i32>,

    /// Gender (M=male, F=female, O=other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// Comma-separated list of keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// Consent string per GDPR/USP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<String>,

    /// Geographic location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Box<Geo>>,

    /// Additional user data segments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl User {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> UserBuilder {
        UserBuilder::create_empty()
    }
}
