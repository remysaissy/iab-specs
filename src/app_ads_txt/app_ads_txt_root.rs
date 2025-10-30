use crate::ads_txt::AdsTxtSystem;
use crate::slice_up_to;
use derive_builder::Builder;
use serde::de::Error;
use serde_with::{DeserializeFromStr, SerializeDisplay, serde_as};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// An implementation of the app-ads.txt v1.0 from the official specification
///
/// https://iabtechlab.com/wp-content/uploads/2019/03/app-ads.txt-v1.0-final-.pdf
///
/// The app-ads.txt file format is based on the ads.txt specification and provides a mechanism
/// for content owners of mobile and OTT (Over-The-Top) apps to declare who is authorized to
/// sell their inventory. The specification follows the established ads.txt standard but is
/// specifically designed for applications distributed through app stores.
///
/// # Discovery Method
///
/// Unlike ads.txt which is hosted at the root domain of a website, app-ads.txt files are:
/// 1. Hosted on the app developer's website (declared in app store metadata)
/// 2. Accessed via the developer URL found in the app's store listing
/// 3. Located at: `https://developer-website.com/app-ads.txt`
///
/// # Differences from ads.txt 1.1
///
/// This implementation is based on app-ads.txt v1.0 (March 2019) and does NOT include
/// the ads.txt 1.1 features (August 2022) such as:
/// - `OWNERDOMAIN` directive (not in app-ads.txt v1.0)
/// - `MANAGERDOMAIN` directive (not in app-ads.txt v1.0)
///
/// Attempting to parse an app-ads.txt file containing these directives will result in an error.
#[serde_as]
#[derive(Builder, DeserializeFromStr, SerializeDisplay, Clone, Debug)]
#[builder(build_fn(error = "crate::Error"))]
pub struct AppAdsTxt {
    /// Contact information
    ///
    /// Some human readable contact information for the owner of the file. This may be
    /// the contact of the advertising operations team for the app. This may be an email
    /// address, phone number, link to a contact form, or other suitable means of communication.
    #[builder(default)]
    pub contact: Option<String>,

    /// Pointer to a subdomain file
    ///
    /// A machine readable subdomain pointer to a subdomain within the root domain, on which an
    /// ads.txt can be found. The crawler should fetch and consume associate the data to the
    /// subdomain, not the current domain. This referral should be exempt from the public suffix
    /// truncation process. Only root domains should refer crawlers to subdomains. Subdomains
    /// should not refer to other subdomains.
    #[builder(default)]
    pub subdomain: Option<String>,

    /// A pointer to the domain of an entity that is explicitly authorized to monetize ads within
    /// the application's content.
    ///
    /// When an app contains ad inventory that is owned by another partner - the app
    /// may list all domains for those partners via this directive rather than the original
    /// method of maintaining a separate line for every relationship under that partner.
    /// It is expected that the INVENTORYPARTNERDOMAIN reference is followed to an ads.txt
    /// file only (not app-ads.txt). See the implementers notes for more details as this feature
    /// requires coordination with your advertising system.
    #[builder(default)]
    pub inventory_partner_domain: Option<String>,

    /// List of authorized advertising systems
    ///
    /// Each system entry declares an authorized seller for the app's ad inventory.
    /// Format: `<domain>, <publisher_account_id>, <account_type>, [<certification_authority_id>]`
    #[builder(default)]
    pub systems: Vec<AdsTxtSystem>,
}

impl AppAdsTxt {
    pub fn builder() -> AppAdsTxtBuilder {
        AppAdsTxtBuilder::default()
    }
}

impl Display for AppAdsTxt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut seq = vec![];
        if let Some(v) = &self.contact {
            seq.push(format!("contact={}", v))
        }
        if let Some(v) = &self.subdomain {
            seq.push(format!("subdomain={}", v))
        }
        if let Some(v) = &self.inventory_partner_domain {
            seq.push(format!("inventorypartnerdomain={}", v))
        }
        for v in &self.systems {
            seq.push(v.to_string())
        }
        let data = seq.join("\n");
        write!(f, "{}", data)
    }
}

impl FromStr for AppAdsTxt {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        const FIELDS: &[&str] = &[
            "contact",
            "subdomain",
            "inventorypartnerdomain",
        ];
        let mut contact = None;
        let mut subdomain = None;
        let mut inventory_partner_domain = None;
        let mut systems = vec![];

        for line in content.lines() {
            let line = line.trim();
            // Both empty lines and top of block comments are skipped.
            if line.is_empty() || line.starts_with('#') {
                continue;
            } else {
                // Variable
                if line.contains('=') {
                    let (key, value) = line
                        .split_once("=")
                        .ok_or_else(|| serde_plain::Error::unknown_field(&line[..line.len().min(100)], FIELDS))?;
                    let value = match value.contains("#") {
                        true => {
                            let (v, _) = value
                                .split_once("#")
                                .ok_or_else(|| serde_plain::Error::missing_field("comment"))?;
                            v
                        }
                        false => value,
                    };
                    let value = value.trim().to_lowercase();
                    let key_lower = key.trim().to_lowercase();
                    match key_lower.as_str() {
                        "contact" => contact = Some(value),
                        "subdomain" => subdomain = Some(value),
                        "inventorypartnerdomain" => inventory_partner_domain = Some(value),
                        "ownerdomain" => {
                            return Err(serde_plain::Error::custom(
                                "OWNERDOMAIN is not supported in app-ads.txt v1.0 (added in ads.txt 1.1)"
                            ).into());
                        }
                        "managerdomain" => {
                            return Err(serde_plain::Error::custom(
                                "MANAGERDOMAIN is not supported in app-ads.txt v1.0 (added in ads.txt 1.1)"
                            ).into());
                        }
                        _ => {
                            return Err(serde_plain::Error::unknown_field(
                                slice_up_to!(line, 100),
                                FIELDS,
                            )
                            .into());
                        }
                    }
                } else {
                    // System
                    let system = AdsTxtSystem::from_str(line)?;
                    systems.push(system);
                }
            }
        }
        AppAdsTxt::builder()
            .contact(contact)
            .subdomain(subdomain)
            .inventory_partner_domain(inventory_partner_domain)
            .systems(systems)
            .build()
    }
}

// Conversion from AdsTxt (drops 1.1 fields with validation)
impl TryFrom<crate::ads_txt::AdsTxt> for AppAdsTxt {
    type Error = crate::Error;

    fn try_from(ads_txt: crate::ads_txt::AdsTxt) -> Result<Self, Self::Error> {
        // Validate that no 1.1-only fields are present
        if ads_txt.owner_domain.is_some() {
            return Err(serde_plain::Error::custom(
                "Cannot convert ads.txt with OWNERDOMAIN to app-ads.txt v1.0"
            ).into());
        }
        if !ads_txt.manager_domains.is_empty() {
            return Err(serde_plain::Error::custom(
                "Cannot convert ads.txt with MANAGERDOMAIN to app-ads.txt v1.0"
            ).into());
        }

        Ok(AppAdsTxt {
            contact: ads_txt.contact,
            subdomain: ads_txt.subdomain,
            inventory_partner_domain: ads_txt.inventory_partner_domain,
            systems: ads_txt.systems,
        })
    }
}

// Conversion to AdsTxt (1.1 fields are None/empty)
impl From<AppAdsTxt> for crate::ads_txt::AdsTxt {
    fn from(app_ads: AppAdsTxt) -> Self {
        crate::ads_txt::AdsTxt::builder()
            .contact(app_ads.contact)
            .subdomain(app_ads.subdomain)
            .inventory_partner_domain(app_ads.inventory_partner_domain)
            .systems(app_ads.systems)
            .owner_domain(None)
            .manager_domains(vec![])
            .build()
            .expect("Valid conversion from AppAdsTxt to AdsTxt")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ads_txt::SellerRelationType;

    // Test cases from app-ads.txt v1.0 specification examples

    #[test]
    fn deserialize_empty_app_ads_txt() {
        let res = AppAdsTxt::from_str("");
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert!(app_ads.contact.is_none());
        assert!(app_ads.subdomain.is_none());
        assert!(app_ads.inventory_partner_domain.is_none());
        assert!(app_ads.systems.is_empty());
    }

    #[test]
    fn deserialize_basic_system_direct() {
        // Example from specification: basic DIRECT relationship
        let content = "greenadexchange.com, 12345, DIRECT, d75815a79";
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert_eq!(app_ads.systems.len(), 1);
        let system = &app_ads.systems[0];
        assert_eq!(system.domain, "greenadexchange.com");
        assert_eq!(system.publisher_id, "12345");
        assert_eq!(system.relation, SellerRelationType::Direct);
        assert_eq!(system.cert_id, Some("d75815a79".to_string()));
    }

    #[test]
    fn deserialize_basic_system_reseller() {
        // Example from specification: basic RESELLER relationship
        let content = "silverssp.com, 9876, RESELLER, f6578439";
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert_eq!(app_ads.systems.len(), 1);
        let system = &app_ads.systems[0];
        assert_eq!(system.domain, "silverssp.com");
        assert_eq!(system.publisher_id, "9876");
        assert_eq!(system.relation, SellerRelationType::Reseller);
        assert_eq!(system.cert_id, Some("f6578439".to_string()));
    }

    #[test]
    fn deserialize_with_contact() {
        let content = r#"
contact=adops@example.com
greenadexchange.com, 12345, DIRECT, d75815a79
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert_eq!(app_ads.contact, Some("adops@example.com".to_string()));
        assert_eq!(app_ads.systems.len(), 1);
    }

    #[test]
    fn deserialize_with_subdomain() {
        let content = r#"
subdomain=mobile.example.com
greenadexchange.com, 12345, DIRECT
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert_eq!(app_ads.subdomain, Some("mobile.example.com".to_string()));
    }

    #[test]
    fn deserialize_with_inventory_partner_domain() {
        let content = r#"
inventorypartnerdomain=partner.example.com
greenadexchange.com, 12345, DIRECT
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert_eq!(app_ads.inventory_partner_domain, Some("partner.example.com".to_string()));
    }

    #[test]
    fn deserialize_multiple_systems_with_comments() {
        // Comprehensive example with comments
        let content = r#"
# App-ads.txt for MyAwesome Game
contact=monetization@mygame.com
subdomain=games.mygame.com

# Primary ad networks
greenadexchange.com, 12345, DIRECT, d75815a79 # Primary network
silverssp.com, 9876, RESELLER, f6578439 # Reseller partner

# Additional networks
bluessp.com, 54321, DIRECT # No TAG ID
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        assert_eq!(app_ads.contact, Some("monetization@mygame.com".to_string()));
        assert_eq!(app_ads.subdomain, Some("games.mygame.com".to_string()));
        assert_eq!(app_ads.systems.len(), 3);
    }

    #[test]
    fn deserialize_case_insensitive_fields() {
        let content = r#"
CONTACT=ADOPS@EXAMPLE.COM
SUBDOMAIN=MOBILE.EXAMPLE.COM
INVENTORYPARTNERDOMAIN=PARTNER.EXAMPLE.COM
GreenAdExchange.COM, ABC123, DIRECT, TAG12345
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_ok());
        let app_ads = res.unwrap();
        // Values should be lowercased
        assert_eq!(app_ads.contact, Some("adops@example.com".to_string()));
        assert_eq!(app_ads.subdomain, Some("mobile.example.com".to_string()));
        assert_eq!(app_ads.inventory_partner_domain, Some("partner.example.com".to_string()));
    }

    // NEGATIVE TEST CASES - These should FAIL

    #[test]
    fn reject_owner_domain() {
        // app-ads.txt v1.0 does not support OWNERDOMAIN (ads.txt 1.1 feature)
        let content = r#"
ownerdomain=example.com
greenadexchange.com, 12345, DIRECT
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("OWNERDOMAIN"));
        assert!(err_msg.contains("not supported"));
    }

    #[test]
    fn reject_manager_domain() {
        // app-ads.txt v1.0 does not support MANAGERDOMAIN (ads.txt 1.1 feature)
        let content = r#"
managerdomain=manager.example.com
greenadexchange.com, 12345, DIRECT
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("MANAGERDOMAIN"));
        assert!(err_msg.contains("not supported"));
    }

    #[test]
    fn reject_unknown_variable() {
        let content = r#"
unknownfield=somevalue
greenadexchange.com, 12345, DIRECT
"#;
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_err());
    }

    #[test]
    fn reject_invalid_system_format() {
        // Missing required relation field
        let content = "greenadexchange.com, 12345";
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_err());
    }

    #[test]
    fn reject_empty_system_field() {
        // Empty domain field
        let content = ", 12345, DIRECT";
        let res = AppAdsTxt::from_str(content);
        assert!(res.is_err());
    }

    // SERIALIZATION TESTS

    #[test]
    fn serialize_basic() {
        let app_ads = AppAdsTxt::builder()
            .contact(Some("adops@example.com".to_string()))
            .systems(vec![
                AdsTxtSystem::builder()
                    .domain("greenadexchange.com")
                    .publisher_id("12345")
                    .relation(SellerRelationType::Direct)
                    .cert_id(Some("d75815a79".to_string()))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let output = app_ads.to_string();
        assert!(output.contains("contact=adops@example.com"));
        assert!(output.contains("greenadexchange.com,12345,direct,d75815a79"));
    }

    #[test]
    fn serialize_with_all_fields() {
        let app_ads = AppAdsTxt::builder()
            .contact(Some("adops@example.com".to_string()))
            .subdomain(Some("mobile.example.com".to_string()))
            .inventory_partner_domain(Some("partner.example.com".to_string()))
            .systems(vec![
                AdsTxtSystem::builder()
                    .domain("greenadexchange.com")
                    .publisher_id("12345")
                    .relation(SellerRelationType::Direct)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let output = app_ads.to_string();
        assert!(output.contains("contact=adops@example.com"));
        assert!(output.contains("subdomain=mobile.example.com"));
        assert!(output.contains("inventorypartnerdomain=partner.example.com"));
        assert!(output.contains("greenadexchange.com,12345,direct"));
    }

    #[test]
    fn roundtrip_serialization() {
        let original = r#"
contact=adops@example.com
subdomain=mobile.example.com
greenadexchange.com, 12345, DIRECT, d75815a79
silverssp.com, 9876, RESELLER
"#;
        let app_ads = AppAdsTxt::from_str(original).unwrap();
        let serialized = app_ads.to_string();
        let reparsed = AppAdsTxt::from_str(&serialized).unwrap();

        assert_eq!(app_ads.contact, reparsed.contact);
        assert_eq!(app_ads.subdomain, reparsed.subdomain);
        assert_eq!(app_ads.systems.len(), reparsed.systems.len());
    }

    // CONVERSION TESTS

    #[test]
    fn convert_to_ads_txt() {
        let app_ads = AppAdsTxt::builder()
            .contact(Some("adops@example.com".to_string()))
            .subdomain(Some("mobile.example.com".to_string()))
            .build()
            .unwrap();

        let ads_txt: crate::ads_txt::AdsTxt = app_ads.into();
        assert_eq!(ads_txt.contact, Some("adops@example.com".to_string()));
        assert_eq!(ads_txt.subdomain, Some("mobile.example.com".to_string()));
        assert_eq!(ads_txt.owner_domain, None);
        assert!(ads_txt.manager_domains.is_empty());
    }

    #[test]
    fn try_convert_from_ads_txt_compatible() {
        let ads_txt = crate::ads_txt::AdsTxt::builder()
            .contact(Some("adops@example.com".to_string()))
            .subdomain(Some("mobile.example.com".to_string()))
            .build()
            .unwrap();

        let result = AppAdsTxt::try_from(ads_txt);
        assert!(result.is_ok());
        let app_ads = result.unwrap();
        assert_eq!(app_ads.contact, Some("adops@example.com".to_string()));
        assert_eq!(app_ads.subdomain, Some("mobile.example.com".to_string()));
    }

    #[test]
    fn try_convert_from_ads_txt_with_owner_domain_fails() {
        let ads_txt = crate::ads_txt::AdsTxt::builder()
            .owner_domain(Some("example.com".to_string()))
            .build()
            .unwrap();

        let result = AppAdsTxt::try_from(ads_txt);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("OWNERDOMAIN"));
    }

    #[test]
    fn try_convert_from_ads_txt_with_manager_domain_fails() {
        use crate::ads_txt::ManagerDomain;

        let ads_txt = crate::ads_txt::AdsTxt::builder()
            .manager_domains(vec![
                ManagerDomain::builder()
                    .domain("manager.example.com")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let result = AppAdsTxt::try_from(ads_txt);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("MANAGERDOMAIN"));
    }
}
