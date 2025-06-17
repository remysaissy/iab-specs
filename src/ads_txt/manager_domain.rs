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
                        Unexpected::Str(content),
                        &"domain[,iso 3166-1 alpha-2 country_code]",
                    )
                })?;
                let country_code = CountryCode::from_str(country_code.trim()).map_err(|_| {
                    serde_plain::Error::invalid_value(
                        Unexpected::Str(country_code),
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
}
