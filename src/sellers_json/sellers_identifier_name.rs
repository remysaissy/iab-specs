use serde::de::Error;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The following list defines standard identifiers that should be used in the identifier list.
#[derive(Clone, DeserializeFromStr, SerializeDisplay, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SellersIdentifierName {
    /// Trustworthy Accountability Group ID
    TagId,

    /// Dun & Bradstreet DUNS Number
    Duns,
}

impl Display for SellersIdentifierName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SellersIdentifierName::TagId => write!(f, "tag-id"),
            SellersIdentifierName::Duns => write!(f, "duns"),
        }
    }
}

impl FromStr for SellersIdentifierName {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if content.eq_ignore_ascii_case("tag-id") {
            Ok(SellersIdentifierName::TagId)
        } else if content.eq_ignore_ascii_case("duns") {
            Ok(SellersIdentifierName::Duns)
        } else {
            Err(serde_plain::Error::unknown_field(content, &["tag-id", "duns"]).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn serialize_with_invalid_sellers_identifier_name_serde() {
        let res = SellersIdentifierName::from_str("tagid");
        assert!(res.is_err());

        let res = SellersIdentifierName::from_str("dun");
        assert!(res.is_err());
    }

    #[test]
    fn serialize_with_valid_seller_identifier_name_serde() {
        let res = SellersIdentifierName::from_str("tag-id");
        assert!(res.is_ok_and(|v| v == SellersIdentifierName::TagId));

        let res = SellersIdentifierName::from_str("duns");
        assert!(res.is_ok_and(|v| v == SellersIdentifierName::Duns));

        let res = SellersIdentifierName::from_str("TAG-ID");
        assert!(res.is_ok_and(|v| v == SellersIdentifierName::TagId));

        let res = SellersIdentifierName::from_str("DUNS");
        assert!(res.is_ok_and(|v| v == SellersIdentifierName::Duns));
    }

    #[test]
    fn deserialize_from_valid_seller_identifier_name_serde() {
        let res = SellersIdentifierName::TagId.to_string();
        assert_eq!(res, "tag-id");

        let res = SellersIdentifierName::Duns.to_string();
        assert_eq!(res, "duns");
    }
}
