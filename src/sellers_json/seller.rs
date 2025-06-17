use crate::sellers_json::SellerType;
use derive_builder::Builder;
use serde::de::{Error, MapAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The identification of the selling legal entity that is paid for inventory sold on behalf
/// of seller_id. It is invalid for a seller_id to represent multiple entities.
/// Every seller_id must map to only a single entity that is paid for inventory transacted with
/// that seller_id. It is valid for a selling entity to have multiple seller_ids
/// within an advertising system.
#[serde_as]
#[derive(Builder, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Seller {
    /// This is the same ID that appears in an ads.txt file and in the SupplyChain Nodes array sid
    /// property. In most cases will also appear in the Publisher Id property of an OpenRTB request.
    #[builder(setter(into))]
    pub seller_id: String,

    /// Indicates whether the identity of the seller is confidential, where 0 = is
    // not confidential and 1 = is confidential.
    #[serde_as(as = "serde_with::BoolFromInt")]
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_confidential: bool,

    /// An enumeration of the type of account, either PUBLISHER, INTERMEDIARY, or BOTH.
    pub seller_type: SellerType,

    /// A passthrough seller is a facilitator of inventory from the upstream supplier to
    /// the consumer of the inventory. The upstream supplier and consumer must establish a
    /// business relationship with each other such that the upstream supplier has control of
    /// their account within the consumer’s platform.
    ///
    // A value of 1 indicates the following:
    ///  - This seller has an account control relationship with the downstream/consuming
    ///    advertising system.
    ///  - If this seller is the last link in a SupplyChain, the buying system has to have
    ///    established an account control relationship with this seller to transact
    ///    the seller’s inventory.
    ///  - If this is not the last link in a SupplyChain than this seller should exist between
    ///    two entities that have an account control relationship.
    ///
    /// A value of 0 indicates:
    ///  - The downstream/consuming advertising system has no account control relationship
    ///    with this seller
    #[serde_as(as = "serde_with::BoolFromInt")]
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_passthrough: bool,

    /// The name of the company (the legal entity) that is paid for inventory that is transacted
    /// under the given seller_id. Can be omitted only when is_confidential is set to 1.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The business domain name of the company (the legal entity) that is paid for inventory
    /// that is transacted under the given seller_id. When the seller_type property
    /// is set to INTERMEDIARY or BOTH, this should be the root domain name of
    /// the seller’s Sellers.json file. Can be omitted when is_confidential is set to 1
    /// or when the seller doesn’t have a web presence.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Any helpful description for this inventory. It is useful for sellers that
    // have multiple seller ids to describe what this seller_id represents.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Placeholder for advertising-system specific extensions to this object.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
}

impl Seller {
    pub fn builder() -> SellerBuilder {
        SellerBuilder::default()
    }
}

impl Display for Seller {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "<Serialize error: {e}>"),
        }
    }
}

impl FromStr for Seller {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Seller>(content).map_err(|e| e.into())
    }
}

impl<'de> Deserialize<'de> for Seller {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Eq, PartialEq)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            SellerId,
            IsConfidential,
            SellerType,
            IsPassthrough,
            Name,
            Domain,
            Comment,
            Ext,
        }
        const FIELDS: &[&str] = &[
            "seller_id",
            "is_confidential",
            "seller_type",
            "is_passthrough",
            "name",
            "domain",
            "comment",
            "ext",
        ];
        struct SellerVisitor;

        impl<'de> Visitor<'de> for SellerVisitor {
            type Value = Seller;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Required fields: `seller_id`, `is_confidential`, `seller_type`, `is_passthrough`, `name`, `domain`, `comment` or `ext`.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut seller_id = None;
                let mut is_confidential = None;
                let mut seller_type = None;
                let mut is_passthrough = None;
                let mut name = None;
                let mut domain = None;
                let mut comment = None;
                let mut ext = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SellerId => {
                            if seller_id.is_some() {
                                return Err(Error::duplicate_field("seller_id"));
                            }
                            seller_id = Some(map.next_value()?);
                        }
                        Field::SellerType => {
                            if seller_type.is_some() {
                                return Err(Error::duplicate_field("seller_type"));
                            }
                            seller_type = Some(map.next_value()?);
                        }
                        Field::IsConfidential | Field::IsPassthrough => {
                            if key == Field::IsConfidential && is_confidential.is_some() {
                                return Err(Error::duplicate_field("is_confidential"));
                            }
                            if key == Field::IsPassthrough && is_passthrough.is_some() {
                                return Err(Error::duplicate_field("is_passthrough"));
                            }
                            let v = match map.next_value::<u64>()? {
                                0 => Ok(false),
                                1 => Ok(true),
                                unexp => Err(Error::invalid_value(
                                    Unexpected::Unsigned(unexp),
                                    &"0 or 1",
                                )),
                            }?;
                            if key == Field::IsConfidential {
                                is_confidential = Some(v);
                            } else {
                                is_passthrough = Some(v);
                            }
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Domain => {
                            if domain.is_some() {
                                return Err(Error::duplicate_field("domain"));
                            }
                            domain = Some(map.next_value()?);
                        }
                        Field::Comment => {
                            if comment.is_some() {
                                return Err(Error::duplicate_field("comment"));
                            }
                            comment = Some(map.next_value()?);
                        }
                        Field::Ext => {
                            if ext.is_some() {
                                return Err(Error::duplicate_field("ext"));
                            }
                            ext = Some(map.next_value()?);
                        }
                    }
                }

                let seller_id: String =
                    seller_id.ok_or_else(|| Error::missing_field("seller_id"))?;
                let seller_type = seller_type.ok_or_else(|| Error::missing_field("seller_type"))?;
                let is_confidential = is_confidential.unwrap_or(false);
                let is_passthrough = is_passthrough.unwrap_or(false);
                if !is_confidential && name.is_none() {
                    return Err(Error::missing_field("name"));
                }

                Seller::builder()
                    .seller_id(seller_id)
                    .is_confidential(is_confidential)
                    .seller_type(seller_type)
                    .is_passthrough(is_passthrough)
                    .name(name)
                    .domain(domain)
                    .comment(comment)
                    .ext(ext)
                    .build()
                    .map_err(|e| Error::custom(e.to_string()))
            }
        }
        deserializer.deserialize_struct("Seller", FIELDS, SellerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deserialize_with_empty_seller() {
        let res = Seller::from_str("{}");
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_missing_mandatory_field() {
        // a strictly required field is missing.
        let res = Seller::from_str(
            r#"{
 "seller_id": "1942009976"
 }"#,
        );
        assert!(res.is_err());

        // is_confidential = 0 and name is missing.
        let res = Seller::from_str(
            r#"{
 "seller_id": "1942009976",
 "seller_type": "PUBLISHER"
 }"#,
        );
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_mandatory_field() {
        // all strictly required field are present.
        let res = Seller::from_str(
            r#"{
 "seller_id": "1942009976",
 "seller_type": "PUBLISHER",
 "name": "company"
 }"#,
        );
        assert!(res.is_ok());

        let res = Seller::from_str(
            r#"{
 "seller_id": "1942009976",
 "seller_type": "PUBLISHER",
 "name": "company",
 "is_confidential":1
 }"#,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn serialize_with_mandatory_fields() {
        let v = Seller::builder()
            .seller_id("1234")
            .seller_type(SellerType::Publisher)
            .build()
            .unwrap();
        let res = serde_json::to_string(&v);
        assert!(res.is_ok_and(|v| v == r#"{"seller_id":"1234","seller_type":"publisher"}"#));
    }

    #[test]
    fn serialize_with_optional_fields() {
        let v = Seller::builder()
            .seller_id("1234")
            .seller_type(SellerType::Publisher)
            .is_confidential(false)
            .name(Some("ssp".to_string()))
            .build()
            .unwrap();
        let res = v.to_string();
        assert_eq!(
            res,
            r#"{"seller_id":"1234","seller_type":"publisher","name":"ssp"}"#
        );

        let v = Seller::builder()
            .seller_id("1234")
            .seller_type(SellerType::Publisher)
            .is_confidential(true)
            .name(Some("ssp".to_string()))
            .build()
            .unwrap();
        let res = v.to_string();
        assert_eq!(
            res,
            r#"{"seller_id":"1234","is_confidential":1,"seller_type":"publisher","name":"ssp"}"#
        );
    }
}
