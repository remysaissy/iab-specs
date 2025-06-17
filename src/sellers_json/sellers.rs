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
    pub fn builder() -> SellersBuilder {
        SellersBuilder::default()
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
}
