/// OpenRTB 2.5 User Object
///
/// This module implements the User object for user information.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::data::Data;
use super::geo::Geo;

/// User object representing human user (OpenRTB 2.5 Section 3.2.20)
///
/// A `User` object describes the user of the device. The user is typically the
/// human being whose impressions are being made available for auction. The user is
/// not the device or the browser.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{User, Geo};
///
/// let user = User {
///     id: Some("user123".to_string()),
///     yob: Some(1990),
///     gender: Some("M".to_string()),
///     geo: Some(Geo {
///         country: Some("USA".to_string()),
///         region: Some("CA".to_string()),
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct User {
    /// Exchange-specific ID for the user. At least one of id or buyeruid is recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Buyer-specific ID for the user as mapped by the exchange for the buyer.
    /// At least one of buyeruid or id is recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub buyeruid: Option<String>,

    /// Year of birth as a 4-digit integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub yob: Option<i32>,

    /// Gender, where:
    /// - "M" = male
    /// - "F" = female
    /// - "O" = known to be other (i.e., omitted is unknown)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gender: Option<String>,

    /// Comma separated list of keywords, interests, or intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub keywords: Option<String>,

    /// Array of keywords about the user.
    /// Mutually exclusive with `keywords` field.
    /// OpenRTB 2.6+ field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub kwarray: Option<Vec<String>>,

    /// Optional feature to pass bidder data that was set in the exchange's cookie.
    /// The string must be in base85 cookie safe characters and be in any format.
    /// Proper JSON encoding must be used to include "escaped" quotation marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub customdata: Option<String>,

    /// Location of the user's home base defined by a Geo object.
    /// This is not necessarily their current location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub geo: Option<Geo>,

    /// Additional user data. Each Data object represents a different data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<Vec<Data>>,

    /// Consent string as defined by the Transparency & Consent Framework.
    /// OpenRTB 2.6+ field for GDPR compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub consent: Option<String>,

    /// GDPR applicability indicator where:
    /// - 0 = GDPR does not apply
    /// - 1 = GDPR applies
    ///
    /// OpenRTB 2.6+ field (added retroactively to 2.5 via ext in practice).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gdpr: Option<i32>,

    /// Extension object for exchange-specific extensions.
    /// Commonly used for GDPR consent data in OpenRTB 2.5.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: Some("user123".to_string()),
            yob: Some(1990),
            gender: Some("M".to_string()),
            ..Default::default()
        };

        assert_eq!(user.id, Some("user123".to_string()));
        assert_eq!(user.yob, Some(1990));
        assert_eq!(user.gender, Some("M".to_string()));
    }

    #[test]
    fn test_user_with_geo() {
        let geo = Geo {
            country: Some("USA".to_string()),
            region: Some("CA".to_string()),
            ..Default::default()
        };

        let user = User {
            id: Some("user456".to_string()),
            geo: Some(geo),
            ..Default::default()
        };

        assert!(user.geo.is_some());
        assert_eq!(user.geo.as_ref().unwrap().country, Some("USA".to_string()));
    }

    #[test]
    fn test_user_with_data() {
        let data = Data {
            id: Some("data123".to_string()),
            name: Some("BlueKai".to_string()),
            ..Default::default()
        };

        let user = User {
            id: Some("user789".to_string()),
            data: Some(vec![data]),
            ..Default::default()
        };

        assert!(user.data.is_some());
        assert_eq!(user.data.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: Some("user123".to_string()),
            buyeruid: Some("buyer456".to_string()),
            yob: Some(1990),
            gender: Some("M".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"id\":\"user123\""));
        assert!(json.contains("\"buyeruid\":\"buyer456\""));
        assert!(json.contains("\"yob\":1990"));
        assert!(json.contains("\"gender\":\"M\""));
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{"id":"user123","yob":1990,"gender":"F"}"#;
        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user.id, Some("user123".to_string()));
        assert_eq!(user.yob, Some(1990));
        assert_eq!(user.gender, Some("F".to_string()));
    }

    #[test]
    fn test_user_with_keywords() {
        let user = User {
            id: Some("user123".to_string()),
            keywords: Some("sports,technology,travel".to_string()),
            ..Default::default()
        };

        assert_eq!(user.keywords, Some("sports,technology,travel".to_string()));
    }

    #[test]
    fn test_user_with_gdpr() {
        let user = User {
            id: Some("user123".to_string()),
            gdpr: Some(1),
            consent: Some("consent_string_here".to_string()),
            ..Default::default()
        };

        assert_eq!(user.gdpr, Some(1));
        assert_eq!(user.consent, Some("consent_string_here".to_string()));
    }
}
