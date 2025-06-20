use crate::slice_up_to;
use serde::de::Error;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// An enumeration of the type of account, either PUBLISHER, INTERMEDIARY, or BOTH.
#[derive(Clone, DeserializeFromStr, SerializeDisplay, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SellerType {
    /// The inventory sold through this account is on a site, app, or other
    /// medium owned by the named entity and the advertising system pays them directly.
    Publisher,

    /// The inventory sold through this account is not owned by the named entity or the
    /// advertising system does not pay them directly.
    Intermediary,

    /// Both types of inventory are transacted by this seller.
    Both,
}

impl Display for SellerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SellerType::Publisher => write!(f, "publisher"),
            SellerType::Intermediary => write!(f, "intermediary"),
            SellerType::Both => write!(f, "both"),
        }
    }
}

impl FromStr for SellerType {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if content.eq_ignore_ascii_case("publisher") {
            Ok(SellerType::Publisher)
        } else if content.eq_ignore_ascii_case("intermediary") {
            Ok(SellerType::Intermediary)
        } else if content.eq_ignore_ascii_case("both") {
            Ok(SellerType::Both)
        } else {
            Err(serde_plain::Error::unknown_field(
                slice_up_to!(content, 100),
                &["publisher", "intermediary", "both"],
            )
            .into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deserialize_with_invalid_seller_type_serde() {
        let res = SellerType::from_str("publisherr");
        assert!(res.is_err());

        let res = SellerType::from_str("intermdiary");
        assert!(res.is_err());

        let res = SellerType::from_str("booth");
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_valid_seller_type_serde() {
        let res = SellerType::from_str("publisher");
        assert!(res.is_ok_and(|v| v == SellerType::Publisher));

        let res = SellerType::from_str("intermediary");
        assert!(res.is_ok_and(|v| v == SellerType::Intermediary));

        let res = SellerType::from_str("both");
        assert!(res.is_ok_and(|v| v == SellerType::Both));

        let res = SellerType::from_str("PUBLISHER");
        assert!(res.is_ok_and(|v| v == SellerType::Publisher));

        let res = SellerType::from_str("INTERMEDIARY");
        assert!(res.is_ok_and(|v| v == SellerType::Intermediary));

        let res = SellerType::from_str("BOTH");
        assert!(res.is_ok_and(|v| v == SellerType::Both));
    }

    #[test]
    fn serialize_from_valid_seller_type_serde() {
        let res = SellerType::Publisher.to_string();
        assert_eq!(res, "publisher");

        let res = SellerType::Intermediary.to_string();
        assert_eq!(res, "intermediary");

        let res = SellerType::Both.to_string();
        assert_eq!(res, "both");
    }
}
