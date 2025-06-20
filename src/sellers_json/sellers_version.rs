use crate::slice_up_to;
use serde::de::Error;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The version of the sellers.json spec, currently the only valid value is 1.0.
#[derive(Clone, DeserializeFromStr, SerializeDisplay, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SellersVersion {
    OneZero,
}

impl Display for SellersVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "1.0")
    }
}

impl FromStr for SellersVersion {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if content.eq_ignore_ascii_case("1.0") {
            Ok(SellersVersion::OneZero)
        } else {
            Err(serde_plain::Error::unknown_field(slice_up_to!(content, 100), &["1.0"]).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn serialize_with_invalid_sellers_version_serde() {
        let res = SellersVersion::from_str("1.1");
        assert!(res.is_err());

        let res = SellersVersion::from_str("1");
        assert!(res.is_err());
    }

    #[test]
    fn serialize_with_valid_seller_version_serde() {
        let res = SellersVersion::from_str("1.0");
        assert!(res.is_ok_and(|v| v == SellersVersion::OneZero));
    }

    #[test]
    fn deserialize_from_valid_seller_version_serde() {
        let res = SellersVersion::OneZero.to_string();
        assert_eq!(res, "1.0");
    }
}
