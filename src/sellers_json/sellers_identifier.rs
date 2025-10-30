use crate::sellers_json::SellersIdentifierName;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// An identifier is an arbitrary name/value pair that is used to communicate common values such
/// as business identifiers, certification identifiers, or any other identifier that a consuming
/// system might need to better interoperate with the seller.
#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
#[builder(build_fn(error = "crate::Error"))]
pub struct SellersIdentifier {
    /// The description of the identifier.
    pub name: SellersIdentifierName,

    /// The value of the identifier.
    #[builder(setter(into))]
    pub value: String,
}

impl SellersIdentifier {
    pub fn builder() -> SellersIdentifierBuilder {
        SellersIdentifierBuilder::default()
    }
}

impl Display for SellersIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "<Serialize error: {e}>"),
        }
    }
}

impl FromStr for SellersIdentifier {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<SellersIdentifier>(content).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_with_invalid_sellers_identifier_serde() {
        let res = serde_json::from_str::<SellersIdentifier>(r#"{"name":"tag-id"}"#);
        assert!(res.is_err());

        let res =
            serde_json::from_str::<SellersIdentifier>(r#"{"name":"tagid","value":"432432432"}"#);
        assert!(res.is_err());

        let res = serde_json::from_str::<SellersIdentifier>(r#"{"name":"tag-id","value":42}"#);
        assert!(res.is_err());

        let res = serde_json::from_str::<SellersIdentifier>(r#"{"Name":"tag-id","value":"42"}"#);
        assert!(res.is_err());

        let res = serde_json::from_str::<SellersIdentifier>(r#"{"name":"tag-id","Value":"42"}"#);
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_valid_seller_identifier_serde() {
        let res =
            serde_json::from_str::<SellersIdentifier>(r#"{"name":"tag-id","value":"432432432"}"#);
        assert!(res.is_ok_and(|v| v.value == "432432432"));

        let res =
            serde_json::from_str::<SellersIdentifier>(r#"{"name":"duns","value":"432432432"}"#);
        assert!(res.is_ok_and(|v| v.value == "432432432"));
    }

    #[test]
    fn serialize_from_valid_seller_identifier_serde() {
        let res = serde_json::to_string(
            &SellersIdentifier::builder()
                .name(SellersIdentifierName::TagId)
                .value("424242")
                .build()
                .unwrap(),
        );
        assert!(res.is_ok_and(|v| v == r#"{"name":"tag-id","value":"424242"}"#));

        let res = serde_json::to_string(
            &SellersIdentifier::builder()
                .name(SellersIdentifierName::Duns)
                .value("424242")
                .build()
                .unwrap(),
        );
        assert!(res.is_ok_and(|v| v == r#"{"name":"duns","value":"424242"}"#));
    }

    #[test]
    fn test_from_str() {
        let json = r#"{"name":"tag-id","value":"12345"}"#;
        let result = SellersIdentifier::from_str(json);
        assert!(result.is_ok());
        let identifier = result.unwrap();
        assert_eq!(identifier.name, SellersIdentifierName::TagId);
        assert_eq!(identifier.value, "12345");
    }

    #[test]
    fn test_from_str_invalid() {
        let json = r#"{"invalid":"json"}"#;
        let result = SellersIdentifier::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let identifier = SellersIdentifier::builder()
            .name(SellersIdentifierName::TagId)
            .value("12345")
            .build()
            .unwrap();
        let display_str = identifier.to_string();
        assert!(display_str.contains("tag-id"));
        assert!(display_str.contains("12345"));
    }

    #[test]
    fn test_clone() {
        let original = SellersIdentifier::builder()
            .name(SellersIdentifierName::Duns)
            .value("98765")
            .build()
            .unwrap();
        let cloned = original.clone();
        assert_eq!(cloned.name, original.name);
        assert_eq!(cloned.value, original.value);
    }

    #[test]
    fn test_debug() {
        let identifier = SellersIdentifier::builder()
            .name(SellersIdentifierName::TagId)
            .value("debug-test")
            .build()
            .unwrap();
        let debug_str = format!("{:?}", identifier);
        assert!(debug_str.contains("SellersIdentifier"));
        assert!(debug_str.contains("debug-test"));
    }

    #[test]
    fn test_builder_with_string() {
        let identifier = SellersIdentifier::builder()
            .name(SellersIdentifierName::TagId)
            .value(String::from("owned-string"))
            .build()
            .unwrap();
        assert_eq!(identifier.value, "owned-string");
    }
}
