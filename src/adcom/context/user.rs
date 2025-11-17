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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_builder() {
        let user = User::builder()
            .id(Some("user123".to_string()))
            .buyeruid(Some("buyer456".to_string()))
            .yob(Some(1985))
            .gender(Some("M".to_string()))
            .build()
            .unwrap();

        assert_eq!(user.id, Some("user123".to_string()));
        assert_eq!(user.buyeruid, Some("buyer456".to_string()));
        assert_eq!(user.yob, Some(1985));
        assert_eq!(user.gender, Some("M".to_string()));
    }

    #[test]
    fn test_user_default() {
        let user = User::builder().build().unwrap();

        assert!(user.id.is_none());
        assert!(user.buyeruid.is_none());
        assert!(user.yob.is_none());
        assert!(user.geo.is_none());
    }

    #[test]
    fn test_user_with_geo() {
        let geo = Geo::builder()
            .country(Some("USA".to_string()))
            .city(Some("New York".to_string()))
            .build()
            .unwrap();

        let user = User::builder()
            .id(Some("user789".to_string()))
            .geo(Some(Box::new(geo)))
            .build()
            .unwrap();

        assert!(user.geo.is_some());
        assert_eq!(user.geo.as_ref().unwrap().country, Some("USA".to_string()));
    }

    #[test]
    fn test_user_serialization() {
        let user = User::builder()
            .id(Some("user999".to_string()))
            .yob(Some(1990))
            .gender(Some("F".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"id\":\"user999\""));
        assert!(json.contains("\"yob\":1990"));
        assert!(json.contains("\"gender\":\"F\""));
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{"id":"user111","buyeruid":"buyer222","yob":1992,"gender":"M"}"#;
        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user.id, Some("user111".to_string()));
        assert_eq!(user.buyeruid, Some("buyer222".to_string()));
        assert_eq!(user.yob, Some(1992));
        assert_eq!(user.gender, Some("M".to_string()));
    }

    #[test]
    fn test_user_with_consent() {
        let user = User::builder()
            .id(Some("user555".to_string()))
            .consent(Some("CPtRHYQPtRHYQAGABCENBCCsAP_AAH_AACiQHItf_X_fb3_j-_59_9t0eY1f9_7_v-0zjhfdt-8N2f_X_L8X42M7vF36tq4KuR4ku3bBIQNtHMnUDUmxaolVrzHsak2cpyNKJ7LEmnMbe2dYGH9Pn9lD-YKZ7_5_9_f52T_9_9_-39z3_9f___dv_-__-vjf_599n_9_3_3-8BAA".to_string()))
            .build()
            .unwrap();

        assert!(user.consent.is_some());
        assert!(user.consent.as_ref().unwrap().starts_with("CPtRHYQ"));
    }
}
