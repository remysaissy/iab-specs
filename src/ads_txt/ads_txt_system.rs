use crate::ads_txt::SellerRelationType;
use crate::slice_up_to;
use derive_builder::Builder;
use serde::de::{Error, Unexpected};
use serde_with::{DeserializeFromStr, SerializeDisplay, serde_as};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The following defines the contents within each field. We refer to the IAB OpenRTB [7]
/// and IAB OpenDirect [9] specs as needed.
#[serde_as]
#[derive(Builder, DeserializeFromStr, SerializeDisplay, Clone, Debug)]
#[builder(build_fn(error = "crate::Error"))]
pub struct AdsTxtSystem {
    /// Domain name of the advertising system
    ///
    /// The canonical domain name of the SSP, Exchange, Header Wrapper, etc system that bidders
    /// connect to. This may be the operational domain of the system, if that is different than
    /// the parent corporate domain, to facilitate WHOIS and reverse IP lookups to establish
    /// clear ownership of the delegate system. Ideally the SSP or Exchange publishes
    /// a document detailing what domain name to use.
    #[builder(setter(into))]
    pub domain: String,

    /// Publisher’s Account ID
    ///
    /// The identifier associated with the seller or reseller account within the advertising
    /// system in field #1. This must contain the same value used in transactions (i.e. OpenRTB bid
    /// requests) in the field specified by the SSP/exchange. Typically, in OpenRTB, this is
    /// publisher.id. For OpenDirect it is typically the publisher’s organization ID.
    #[builder(setter(into))]
    pub publisher_id: String,

    /// Type of Account/ Relationship
    ///
    /// An enumeration of the type of account. A value of ‘DIRECT’ indicates that the
    /// Publisher (content owner) directly controls the account indicated in field #2 on the system
    /// in field #1. This tends to mean a direct business contract between the Publisher and the
    /// advertising system. A value of ‘RESELLER’ indicates that the Publisher has authorized
    /// another entity to control the account indicated in field #2 and resell their ad space via
    /// the system in field #1. Other types may be added in the future.
    ///
    /// Note that this field should be treated as case insensitive when interpreting the data
    pub relation: SellerRelationType,

    /// Certification Authority ID
    ///
    /// An ID that uniquely identifies the advertising system within a certification authority
    /// (this ID maps to the entity listed in field #1). A current certification authority is the
    /// Trustworthy Accountability Group (aka TAG), and the TAGID would be included here [11].
    /// Declaration of certification IDs are superseded by the identifiers object in sellers.json.
    ///
    /// Note: This field may be deprecated in a future version of ads.txt.
    #[builder(default)]
    pub cert_id: Option<String>,

    /// Comment are denoted by the character "#". Any line containing "#" should inform the data
    /// consumer to ignore the data after the "#" character to the end of the line.
    #[builder(default)]
    pub comment: Option<String>,
}

impl AdsTxtSystem {
    pub fn builder() -> AdsTxtSystemBuilder {
        AdsTxtSystemBuilder::default()
    }
}

impl Display for AdsTxtSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let domain = self.domain.to_lowercase();
        let publisher_id = self.publisher_id.to_lowercase();
        let relation = serde_plain::to_string(&self.relation).unwrap();
        let mut data = format!("{},{},{}", &domain, &publisher_id, relation);
        if let Some(cert_id) = self.cert_id.as_ref() {
            let cert_id = cert_id.to_lowercase();
            data = format!("{},{}", data, &cert_id);
        }
        if let Some(comment) = self.comment.as_ref() {
            let comment = comment.to_lowercase();
            data = format!("{} # {}", data, &comment);
        }
        write!(f, "{}", data)
    }
}

impl FromStr for AdsTxtSystem {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut domain = None;
        let mut publisher_id = None;
        let mut relation = None;
        let mut cert_id = None;
        let mut comment = None;
        let mut s = 0;
        let mut field_index = 0;
        for (idx, c) in content.bytes().enumerate() {
            if c == b',' {
                let field_value = content[s..idx].trim().to_lowercase();
                if field_value.is_empty() {
                    return Err(serde_plain::Error::invalid_value(
                        Unexpected::Str(slice_up_to!(content, 100)),
                        &"'domain','publisher_id','relation'[,'cert_id'>][# comments]",
                    )
                    .into());
                }

                match field_index {
                    0 => domain = Some(field_value),
                    1 => publisher_id = Some(field_value),
                    2 => {
                        let rel = SellerRelationType::from_str(&field_value)?;
                        relation = Some(rel)
                    }
                    _ => {
                        return Err(serde_plain::Error::invalid_value(
                            Unexpected::Str(slice_up_to!(field_value, 100)),
                            &"'domain','publisher_id','relation'[,'cert_id'>][# comments]",
                        )
                        .into());
                    }
                }
                field_index += 1;
                s = idx + 1;
            } else if c == b'#' {
                if field_index == 2 {
                    let field_value = content[s..idx].trim().to_lowercase();
                    let rel = SellerRelationType::from_str(&field_value)?;
                    relation = Some(rel);
                    s = idx + 1;
                } else if field_index == 3 {
                    let field_value = content[s..idx].trim().to_lowercase();
                    if !field_value.is_empty() {
                        cert_id = Some(field_value);
                    }
                    s = idx + 1;
                }
                if s < content.len() {
                    let field_value = content[s..].trim().to_lowercase();
                    if !field_value.is_empty() {
                        comment = Some(field_value.to_string());
                    }
                }
                break;
            }
            if idx + 1 == content.len() {
                let field_value = content[s..].trim().to_lowercase();
                if !field_value.is_empty() {
                    if field_index == 2 {
                        let rel = SellerRelationType::from_str(&field_value)?;
                        relation = Some(rel)
                    } else {
                        cert_id = Some(field_value);
                    }
                }
                break;
            }
        }

        let domain = domain.ok_or_else(|| serde_plain::Error::missing_field("domain"))?;
        let publisher_id =
            publisher_id.ok_or_else(|| serde_plain::Error::missing_field("publisher_id"))?;
        let relation = relation.ok_or_else(|| serde_plain::Error::missing_field("relation"))?;

        AdsTxtSystem::builder()
            .domain(domain)
            .publisher_id(publisher_id)
            .relation(relation)
            .cert_id(cert_id)
            .comment(comment)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deserialize_with_empty_ads_txt_system() {
        let res = AdsTxtSystem::from_str("");
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_missing_mandatory_field() {
        let res = AdsTxtSystem::from_str(",XF7342, DIRECT");
        assert!(res.is_err());

        let res = AdsTxtSystem::from_str("greenadexchange.com,, DIRECT");
        assert!(res.is_err());

        let res = AdsTxtSystem::from_str("greenadexchange.com, XF7342");
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_single_system_direct() {
        let res = AdsTxtSystem::from_str("greenadexchange.com, XF7342, DIRECT, 5jyxf8k54");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "greenadexchange.com");
        assert_eq!(&res.publisher_id, "xf7342");
        assert_eq!(res.relation, SellerRelationType::Direct);
        assert_eq!(res.cert_id, Some("5jyxf8k54".to_string()));
        assert_eq!(res.comment, None);

        let res = AdsTxtSystem::from_str("greenadexchange.com, XF7342, DIRECT");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "greenadexchange.com");
        assert_eq!(&res.publisher_id, "xf7342");
        assert_eq!(res.relation, SellerRelationType::Direct);
        assert_eq!(res.cert_id, None);
        assert_eq!(res.comment, None);
    }

    #[test]
    fn with_single_system_direct_with_comment() {
        let res = AdsTxtSystem::from_str(
            "greenadexchange.com, XF7342, DIRECT, 5jyxf8k54 # comment at the end of the #line",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "greenadexchange.com");
        assert_eq!(&res.publisher_id, "xf7342");
        assert_eq!(res.relation, SellerRelationType::Direct);
        assert_eq!(res.cert_id, Some("5jyxf8k54".to_string()));
        assert_eq!(
            res.comment,
            Some("comment at the end of the #line".to_string())
        );

        let res = AdsTxtSystem::from_str(
            "greenadexchange.com, XF7342, DIRECT # comment at the end of the #line",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "greenadexchange.com");
        assert_eq!(&res.publisher_id, "xf7342");
        assert_eq!(res.relation, SellerRelationType::Direct);
        assert_eq!(res.cert_id, None);
        assert_eq!(
            res.comment,
            Some("comment at the end of the #line".to_string())
        );
    }

    #[test]
    fn deserialize_with_single_system_reseller_with_comment() {
        let res =
            AdsTxtSystem::from_str("redssp.com, 57013, RESELLER # comment at the end of the #line");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "redssp.com");
        assert_eq!(&res.publisher_id, "57013");
        assert_eq!(res.relation, SellerRelationType::Reseller);
        assert_eq!(res.cert_id, None);
        assert_eq!(
            res.comment,
            Some("comment at the end of the #line".to_string())
        );
    }

    #[test]
    fn deserialize_with_single_system_direct_without_cert_with_comment() {
        let res = AdsTxtSystem::from_str(
            "greenadexchange.com, XF7342, DIRECT # comment at the end of the #line",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "greenadexchange.com");
        assert_eq!(&res.publisher_id, "xf7342");
        assert_eq!(res.relation, SellerRelationType::Direct);
        assert_eq!(res.cert_id, None);
        assert_eq!(
            res.comment,
            Some("comment at the end of the #line".to_string())
        );
    }

    #[test]
    fn deserialize_with_invalid_characters() {
        let res = AdsTxtSystem::from_str(
            "Ã¯Â»Â¿greenadexchange.com, XF7342, DIRECT # comment at the end of the #line",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(&res.domain, "ã¯â»â¿greenadexchange.com");
        assert_eq!(&res.publisher_id, "xf7342");
        assert_eq!(res.relation, SellerRelationType::Direct);
        assert_eq!(res.cert_id, None);
        assert_eq!(
            res.comment,
            Some("comment at the end of the #line".to_string())
        );
    }

    #[test]
    fn serialize_with_mandatory_fields() {
        let v = AdsTxtSystem::builder()
            .domain("greenadexchange.com")
            .publisher_id("xf7342")
            .relation(SellerRelationType::Direct)
            .comment(Some("comment at the end of the #line".to_string()))
            .build()
            .unwrap();
        let res = v.to_string();
        assert_eq!(
            res,
            "greenadexchange.com,xf7342,direct # comment at the end of the #line"
        );

        let v = AdsTxtSystem::builder()
            .domain("GREENADEXCHANGE.COM")
            .publisher_id("XF7342")
            .relation(SellerRelationType::Direct)
            .comment(Some("comment at the end of the #line".to_string()))
            .build()
            .unwrap();
        let res = v.to_string();
        assert_eq!(
            res,
            "greenadexchange.com,xf7342,direct # comment at the end of the #line"
        );
    }
}
