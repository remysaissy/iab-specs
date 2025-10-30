use crate::slice_up_to;
use derive_builder::Builder;
use isosphere::CountryCode;
use serde::de::{Error, Unexpected};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The business domain of a primary or exclusive monetization
/// partner of the publishers inventory.
#[derive(Builder, DeserializeFromStr, SerializeDisplay, Clone, Debug)]
#[builder(build_fn(error = "crate::Error"))]
pub struct ManagerDomain {
    #[builder(setter(into))]
    pub domain: String,
    #[builder(default)]
    pub country_code: Option<CountryCode>,
}

impl ManagerDomain {
    pub fn builder() -> ManagerDomainBuilder {
        ManagerDomainBuilder::default()
    }
}

impl Display for ManagerDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let domain = self.domain.to_lowercase();
        match self.country_code {
            None => write!(f, "{domain}"),
            Some(country_code) => write!(f, "{},{}", &domain, country_code),
        }
    }
}

impl FromStr for ManagerDomain {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let (domain, country_code) = match content.contains(",") {
            true => {
                let (domain, country_code) = content.split_once(",").ok_or_else(|| {
                    serde_plain::Error::invalid_value(
                        Unexpected::Str(slice_up_to!(content, 100)),
                        &"domain[,iso 3166-1 alpha-2 country_code]",
                    )
                })?;
                let country_code = CountryCode::from_str(country_code.trim()).map_err(|_| {
                    serde_plain::Error::invalid_value(
                        Unexpected::Str(slice_up_to!(content, 100)),
                        &"iso 3166-1 alpha-2 country_code",
                    )
                })?;
                (domain.trim().to_lowercase(), Some(country_code))
            }
            false => (content.trim().to_lowercase(), None),
        };
        let manager_domain = ManagerDomain::builder()
            .domain(domain)
            .country_code(country_code)
            .build()?;
        Ok(manager_domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_with_valid_entry() {
        let res = ManagerDomain::from_str("managerdomain.com");
        assert!(res.is_ok_and(|v| v.domain == "managerdomain.com" && v.country_code.is_none()));

        let res = ManagerDomain::from_str("MANAGERDOMAIN.COM");
        assert!(res.is_ok_and(|v| v.domain == "managerdomain.com" && v.country_code.is_none()));

        let res = ManagerDomain::from_str("managerdomain.com, FR");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.domain, "managerdomain.com");
        assert!(res.country_code.is_some_and(|v| v.is_alpha2()));
    }

    #[test]
    fn deserialize_with_invalid_entry() {
        let res = ManagerDomain::from_str("managerdomain.com,");
        assert!(res.is_err());

        let res = ManagerDomain::from_str("managerdomain.com, ZZ");
        assert!(res.is_err());
    }

    #[test]
    fn serialize_with_valid_entry() {
        let manager_domain = ManagerDomain::builder()
            .domain("managerdomain.com")
            .build()
            .unwrap();
        let res = manager_domain.to_string();
        assert_eq!(res, "managerdomain.com");

        let manager_domain = ManagerDomain::builder()
            .domain("MANAGERDOMAIN.COM")
            .build()
            .unwrap();
        let res = manager_domain.to_string();
        assert_eq!(res, "managerdomain.com");

        let manager_domain = ManagerDomain::builder()
            .domain("managerdomain.com, FR")
            .build()
            .unwrap();
        let res = manager_domain.to_string();
        assert_eq!(res, "managerdomain.com, fr");
    }

    #[test]
    fn test_clone() {
        let original = ManagerDomain::builder()
            .domain("test.com")
            .country_code(Some(CountryCode::from_str("US").unwrap()))
            .build()
            .unwrap();
        let cloned = original.clone();
        assert_eq!(cloned.domain, original.domain);
        assert_eq!(cloned.country_code, original.country_code);
    }

    #[test]
    fn test_debug() {
        let manager_domain = ManagerDomain::builder()
            .domain("debug.com")
            .build()
            .unwrap();
        let debug_str = format!("{:?}", manager_domain);
        assert!(debug_str.contains("ManagerDomain"));
        assert!(debug_str.contains("debug.com"));
    }

    #[test]
    fn serialize_with_country_code() {
        let manager_domain = ManagerDomain::builder()
            .domain("example.com")
            .country_code(Some(CountryCode::from_str("US").unwrap()))
            .build()
            .unwrap();
        let res = manager_domain.to_string();
        assert_eq!(res, "example.com,US");
    }

    #[test]
    fn test_builder() {
        let result = ManagerDomain::builder()
            .domain("builder-test.com")
            .build();
        assert!(result.is_ok());
        let manager = result.unwrap();
        assert_eq!(manager.domain, "builder-test.com");
        assert!(manager.country_code.is_none());
    }
}
