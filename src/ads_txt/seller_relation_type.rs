use crate::slice_up_to;
use serde::de::{Error, Unexpected};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, DeserializeFromStr, SerializeDisplay, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SellerRelationType {
    // A value of ‘DIRECT’ indicates that the
    // Publisher (content owner) directly controls the
    // account indicated in `publisher_id` on the system in
    // `domain`. This tends to mean a direct business
    // contract between the Publisher and the
    // advertising system.
    Direct,

    // A value of ‘RESELLER’
    // indicates that the Publisher has authorized
    // another entity to control the account indicated in
    // `publisher_id` and resell their ad space via the system
    // in `domain`.
    Reseller,
}

impl Display for SellerRelationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SellerRelationType::Direct => write!(f, "direct"),
            SellerRelationType::Reseller => write!(f, "reseller"),
        }
    }
}

impl FromStr for SellerRelationType {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if content.eq_ignore_ascii_case("direct") {
            Ok(SellerRelationType::Direct)
        } else if content.eq_ignore_ascii_case("reseller") {
            Ok(SellerRelationType::Reseller)
        } else {
            Err(serde_plain::Error::invalid_value(
                Unexpected::Str(slice_up_to!(content, 100)),
                &"'direct' or 'indirect'",
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
    fn deserialize_with_valid_seller_relation_type() {
        let res = SellerRelationType::from_str("direct");
        assert!(res.is_ok());

        let res = SellerRelationType::from_str("reseller");
        assert!(res.is_ok());

        let res = SellerRelationType::from_str("DIRECT");
        assert!(res.is_ok());

        let res = SellerRelationType::from_str("RESELLER");
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_with_invalid_seller_relation_type() {
        let res = SellerRelationType::from_str("directe");
        assert!(res.is_err());

        let res = SellerRelationType::from_str("reseler");
        assert!(res.is_err());
    }

    #[test]
    fn serialize_seller_relation_type() {
        let res = SellerRelationType::Direct.to_string();
        assert_eq!(res, "direct");

        let res = SellerRelationType::Reseller.to_string();
        assert_eq!(res, "reseller");
    }

    #[test]
    fn test_equality() {
        assert_eq!(SellerRelationType::Direct, SellerRelationType::Direct);
        assert_eq!(SellerRelationType::Reseller, SellerRelationType::Reseller);
        assert_ne!(SellerRelationType::Direct, SellerRelationType::Reseller);
    }

    #[test]
    // Spec: Section 3.1.1 Field 3 — Case-insensitive relationship type parsing
    fn parse_mixed_case_relation_type() {
        // Test mixed case: "Direct" should parse to Direct
        let res = SellerRelationType::from_str("Direct");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Direct);

        // Test mixed case: "Reseller" should parse to Reseller
        let res = SellerRelationType::from_str("Reseller");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Reseller);

        // Test all caps
        let res = SellerRelationType::from_str("DIRECT");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Direct);

        // Test all caps
        let res = SellerRelationType::from_str("RESELLER");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Reseller);

        // Test fully lower case
        let res = SellerRelationType::from_str("direct");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Direct);

        // Test random mixed case
        let res = SellerRelationType::from_str("dIrEcT");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Direct);

        let res = SellerRelationType::from_str("rEsElLeR");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), SellerRelationType::Reseller);
    }

    #[test]
    // Spec: Section 3.1.1 Field 3 — Empty string produces error
    fn parse_empty_relation_type() {
        let res = SellerRelationType::from_str("");
        assert!(res.is_err());
    }
}
