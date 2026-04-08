use crate::sellers_json::{Seller, SellersIdentifier, SellersVersion};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// An implementation of the sellers.json from the official specification
// https://iabtechlab.com/wp-content/uploads/2019/07/Sellers.json_Final.pdf

/// It is a container for all properties in a sellers.json file
#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Sellers {
    /// The business address of the advertising system.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_address: Option<String>,

    /// An email address to use to contact the Advertising System for
    // questions or inquiries about this file.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,

    /// The version of this spec, currently the only valid value is 1.0.
    pub version: SellersVersion,

    /// Placeholder for advertising-system specific extensions to this object.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,

    /// Array of Identifier objects associated with this advertising system.
    /// Examples could be Tag-Ids, Dun & Bradstreet business identifiers, or
    /// any custom identifier that a consuming advertising system might need.
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifiers: Vec<SellersIdentifier>,

    /// The list of all Seller objects that are represented by the advertising
    /// system. All sellers must be included even if they are confidential.
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sellers: Vec<Seller>,
}

impl Sellers {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SellersBuilder {
        SellersBuilder::create_empty()
    }
}

impl Display for Sellers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "<Serialize error: {e}>"),
        }
    }
}

impl FromStr for Sellers {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Sellers>(content).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sellers_json::{SellerType, SellersIdentifierName};

    #[test]
    fn deserialize_with_valid_serde() {
        let res = Sellers::from_str(
            r#"{"contact_address":"Advertising System Inc., 101 Main Street, New York, NY 10101","contact_email":"adops@advertisingsystem.com","version":"1.0","identifiers":[{"name":"tag-id","value":"28cb65e5bbc0bd5f"}],"sellers":[{"seller_id":"1942009976","seller_type":"publisher","name":"Publisher1","domain":"publisher1.com"}]}"#,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_with_invalid_serde() {
        let res = Sellers::from_str(
            r#"{"contact_address":"Advertising System Inc., 101 Main Street, New York, NY 10101","contact_email":"adops@advertisingsystem.com","identifiers":[{"name":"tag-id","value":"28cb65e5bbc0bd5f"}],"sellers":[{"seller_id":"1942009976","seller_type":"publisher","name":"Publisher1","domain":"publisher1.com"}]}"#,
        );
        assert!(res.is_err());
    }

    #[test]
    fn serialize_with_valid_serde() {
        let input = Sellers::builder()
            .contact_email(Some("adops@advertisingsystem.com".to_string()))
            .contact_address(Some(
                "Advertising System Inc., 101 Main Street, New York, NY 10101".to_string(),
            ))
            .version(SellersVersion::OneZero)
            .identifiers(vec![
                SellersIdentifier::builder()
                    .name(SellersIdentifierName::TagId)
                    .value("28cb65e5bbc0bd5f")
                    .build()
                    .unwrap(),
            ])
            .sellers(vec![
                Seller::builder()
                    .seller_id("1942009976")
                    .name(Some("Publisher1".to_string()))
                    .domain(Some("publisher1.com".to_string()))
                    .seller_type(SellerType::Publisher)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let res = input.to_string();
        assert_eq!(
            res,
            r#"{"contact_address":"Advertising System Inc., 101 Main Street, New York, NY 10101","contact_email":"adops@advertisingsystem.com","version":"1.0","identifiers":[{"name":"tag-id","value":"28cb65e5bbc0bd5f"}],"sellers":[{"seller_id":"1942009976","seller_type":"publisher","name":"Publisher1","domain":"publisher1.com"}]}"#
        );
    }

    #[test]
    fn deserialize_with_required_fields_only() {
        // Spec: Section 2.1
        let res = Sellers::from_str(r#"{"version":"1.0","identifiers":[],"sellers":[]}"#);
        assert!(res.is_ok());
        let sellers = res.unwrap();
        assert!(sellers.contact_email.is_none());
        assert!(sellers.contact_address.is_none());
        assert!(sellers.ext.is_none());
        assert!(sellers.identifiers.is_empty());
        assert!(sellers.sellers.is_empty());
    }

    #[test]
    fn round_trip_json_sellers() {
        // Spec: Section 2.1
        let original = Sellers::builder()
            .contact_email(Some("test@example.com".to_string()))
            .contact_address(Some("123 Main St".to_string()))
            .version(SellersVersion::OneZero)
            .ext(Some("ext-data".to_string()))
            .identifiers(vec![
                SellersIdentifier::builder()
                    .name(SellersIdentifierName::TagId)
                    .value("abc123")
                    .build()
                    .unwrap(),
            ])
            .sellers(vec![
                Seller::builder()
                    .seller_id("s1")
                    .seller_type(SellerType::Publisher)
                    .name(Some("Pub1".to_string()))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();
        let json_str = serde_json::to_string(&original).unwrap();
        let value1: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let deserialized: Sellers = serde_json::from_str(&json_str).unwrap();
        let json_str2 = serde_json::to_string(&deserialized).unwrap();
        let value2: serde_json::Value = serde_json::from_str(&json_str2).unwrap();
        assert_eq!(value1, value2);
    }

    #[test]
    fn deserialize_with_unknown_fields_tolerated() {
        // Spec: Section 2.1
        let res = Sellers::from_str(
            r#"{"version":"1.0","identifiers":[],"sellers":[],"unknown_field":"should be ignored","another":42}"#,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn builder_with_required_only() {
        // Spec: Section 2.1
        let res = Sellers::builder().version(SellersVersion::OneZero).build();
        assert!(res.is_ok());
        let sellers = res.unwrap();
        let json = serde_json::to_string(&sellers).unwrap();
        assert!(json.contains("\"version\":\"1.0\""));
    }

    #[test]
    fn deserialize_with_null_optional_fields() {
        // Spec: Section 2.1
        let res = Sellers::from_str(
            r#"{"version":"1.0","contact_email":null,"contact_address":null,"ext":null,"identifiers":[],"sellers":[]}"#,
        );
        assert!(res.is_ok());
        let sellers = res.unwrap();
        assert!(sellers.contact_email.is_none());
        assert!(sellers.contact_address.is_none());
        assert!(sellers.ext.is_none());
    }

    #[test]
    fn deserialize_with_empty_sellers_array() {
        // Spec: Section 2.1
        let res = Sellers::from_str(r#"{"version":"1.0","identifiers":[],"sellers":[]}"#);
        assert!(res.is_ok());
        let sellers = res.unwrap();
        assert!(sellers.sellers.is_empty());
    }

    #[test]
    fn deserialize_with_multiple_identifiers() {
        // Spec: Section 2.1, 2.2
        let res = Sellers::from_str(
            r#"{"version":"1.0","identifiers":[{"name":"tag-id","value":"abc"},{"name":"duns","value":"123"}],"sellers":[]}"#,
        );
        assert!(res.is_ok());
        let sellers = res.unwrap();
        assert_eq!(sellers.identifiers.len(), 2);
        assert_eq!(sellers.identifiers[0].name, SellersIdentifierName::TagId);
        assert_eq!(sellers.identifiers[1].name, SellersIdentifierName::Duns);
    }
}
