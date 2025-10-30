use crate::ads_txt::{AdsTxtSystem, ManagerDomain};
use crate::slice_up_to;
use derive_builder::Builder;
use serde::de::Error;
use serde_with::{DeserializeFromStr, SerializeDisplay, serde_as};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// An implementation of the ads.txt from the official specification
/// https://iabtechlab.com/wp-content/uploads/2022/04/Ads.txt-1.1.pdf
#[serde_as]
#[derive(Builder, DeserializeFromStr, SerializeDisplay, Clone, Debug)]
#[builder(build_fn(error = "crate::Error"))]
pub struct AdsTxt {
    /// Contact information
    ///
    /// Some human readable contact information for the owner of the file. This may be
    /// the contact of the advertising operations team for the website. This may be an email
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
    /// When a site or an app contains ad inventory that is owned by another partner - the app
    /// or site may list all domains for those partners via this directive rather than the original
    /// method of maintaining a separate line for every relationship under that partner.
    /// It is expected that the INVENTORYPARTNERDOMAIN reference is followed to an ads.txt
    /// file only (not app-ads.txt). See the implementers notes for more details as this feature
    /// requires coordination with your advertising system.
    #[builder(default)]
    pub inventory_partner_domain: Option<String>,

    /// The business domain of the business entity that owns the domain/site/app
    ///
    /// This should be the same value as the sellers.domain in this Publisher’s entries in
    /// referenced sellers.json files. Like sellers.domain, this should be Public Suffix List+1,
    /// not a full hostname or a URL. For OpenRTB SupplyChain objects that are complete, the node
    /// representing the originating publisher (the node listed first in the schain object) should
    /// have sellers domain that matches the OWNERDOMAIN.
    ///
    /// If more than one instance of this variable is included only the first should be used.
    /// It is recommended that this field is included even if the OWNERDOMAIN is the same as
    /// the domain on which the ads.txt file is found. It is also recommended that buyers mandate
    /// for sellers that are listed as BOTH in sellers.json to correctly list OWNERDOMAIN in all
    /// ads.txt files that they own OR represent.
    #[builder(default)]
    pub owner_domain: Option<String>,

    /// The business domain of a primary or exclusive monetization partner of the publishers inventory
    ///
    /// When the owner of the site does not manage monetization either globally or in a specific
    /// country, the domain of the exclusive management company is included in this variable.
    /// Syntax of the domain is [PSL+1 domain, required], [ISO 3166-1 alpha-2 country code,
    /// optional, blank=global]
    ///
    /// This variable should only be used for a seller who is not the publisher but is the primary
    /// or exclusive programmatic seller for this site. This will typically only apply if the
    /// publisher is not selling their own inventory in the given market.
    ///
    /// It is expected that ad opportunities monetized by the manager, the domain listed in
    /// MANAGERDOMAIN is also that of the node representing the originating publisher in a complete
    /// supply chain object. There can be more than one MANAGERDOMAIN value but only one per country.
    ///
    /// A global/default MANAGERDOMAIN should not have a country “extension” on the variable line.
    /// The default can be overridden by other entries with country extensions included.
    /// See example for country declaration format.
    /// Consult the implementation guide for details on use cases and potential SPO implications.
    #[builder(default)]
    pub manager_domains: Vec<ManagerDomain>,

    /// List of systems declared.
    #[builder(default)]
    pub systems: Vec<AdsTxtSystem>,
}

impl AdsTxt {
    pub fn builder() -> AdsTxtBuilder {
        AdsTxtBuilder::default()
    }
}

impl Display for AdsTxt {
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
        if let Some(v) = &self.owner_domain {
            seq.push(format!("ownerdomain={}", v))
        }
        for v in &self.manager_domains {
            seq.push(format!("managerdomain={}", v))
        }
        for v in &self.systems {
            seq.push(v.to_string())
        }
        let data = seq.join("\n");
        write!(f, "{}", data)
    }
}

impl FromStr for AdsTxt {
    type Err = crate::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        const FIELDS: &[&str] = &[
            "contact",
            "subodmain",
            "inventorypartnerdomain",
            "ownerdomain",
            "managerdomain",
        ];
        let mut contact = None;
        let mut subdomain = None;
        let mut inventory_partner_domain = None;
        let mut owner_domain = None;
        let mut manager_domains = vec![];
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
                        .ok_or_else(|| serde_plain::Error::unknown_field(&line[..100], FIELDS))?;
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
                    match key.trim() {
                        "contact" => contact = Some(value),
                        "subdomain" => subdomain = Some(value),
                        "inventorypartnerdomain" => inventory_partner_domain = Some(value),
                        "ownerdomain" => {
                            if owner_domain.is_none() {
                                owner_domain = Some(value)
                            }
                        }
                        "managerdomain" => {
                            let manager_domain = ManagerDomain::from_str(&value)?;
                            manager_domains.push(manager_domain);
                        }
                        &_ => {
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
        AdsTxt::builder()
            .contact(contact)
            .subdomain(subdomain)
            .inventory_partner_domain(inventory_partner_domain)
            .owner_domain(owner_domain)
            .manager_domains(manager_domains)
            .systems(systems)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ads_txt::SellerRelationType;
    use std::str::FromStr;

    #[test]
    fn deserialize_with_empty_ads_txt() {
        let res = AdsTxt::from_str("");
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_with_multiple_ownerdomain() {
        let ads_txt = r#"
        # Variables
        ownerdomain=myownerdomain.com
        ownerdomain=myownerdomain2.com
        "#;

        let res = AdsTxt::from_str(ads_txt);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.owner_domain.is_some());
        assert_eq!(
            res.owner_domain.as_ref().unwrap().as_str(),
            "myownerdomain.com"
        );
    }

    #[test]
    fn deserialize_with_variables_and_comments() {
        let ads_txt = r#"
        # Variables
        contact=user@mydomain.com
        subdomain=sub.mydomain.com # Subdomain of this ads.txt.
        inventorypartnerdomain=mypartnerdomain.com
        ownerdomain=myownerdomain.com
        managerdomain=mymanagerdomain.com
        "#;

        let res = AdsTxt::from_str(ads_txt);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.contact.is_some());
        assert_eq!(res.contact.as_ref().unwrap().as_str(), "user@mydomain.com");
        assert!(res.subdomain.is_some());
        assert_eq!(res.subdomain.as_ref().unwrap().as_str(), "sub.mydomain.com");
        assert!(res.inventory_partner_domain.is_some());
        assert_eq!(
            res.inventory_partner_domain.as_ref().unwrap().as_str(),
            "mypartnerdomain.com"
        );
        assert!(res.owner_domain.is_some());
        assert_eq!(
            res.owner_domain.as_ref().unwrap().as_str(),
            "myownerdomain.com"
        );
        assert!(!res.manager_domains.is_empty());
        let manager_domain = res.manager_domains.first().unwrap();
        assert_eq!(manager_domain.domain, "mymanagerdomain.com");
    }

    #[test]
    fn deserialize_with_variables_and_systems() {
        let ads_txt = r#"
        # Variables
        subdomain=sub.mydomain.com # Subdomain of this ads.txt.
        greenadexchange.com, XF7342, DIRECT, 5jyxf8k54
        "#;

        let res = AdsTxt::from_str(ads_txt);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.subdomain.is_some());
        assert_eq!(res.subdomain.as_ref().unwrap().as_str(), "sub.mydomain.com");
        assert!(!res.systems.is_empty());
        let system = res.systems.first().unwrap();
        assert_eq!(&system.domain, "greenadexchange.com");
        assert_eq!(&system.publisher_id, "xf7342");
        assert_eq!(system.relation, SellerRelationType::Direct);
        assert_eq!(system.cert_id, Some("5jyxf8k54".to_string()));
        assert_eq!(system.comment, None);
    }

    #[test]
    fn deserialize_skip_top_of_block_comments() {
        let ads_txt = r#"
        # First block
        greenadexchange.com, XF7342, DIRECT, 5jyxf8k54 # comment 1
        greenadexchange.com, XF7343, DIRECT, 5jyxf8k54 # comment 2

        # Second block
        redssp.com, XF7322, DIRECT, 5jyxf8k54 # comment 3
        redssp.com, XF7323, DIRECT, 5jyxf8k54 # comment 4

        # Third block
        # Multi line block
        bluessp.com, XF7312, DIRECT, 5jyxf8k54 # comment 5
        bluessp.com, XF7313, DIRECT, 5jyxf8k54 # comment 6
        # Fourth block
        greenssp.com, XF7352, DIRECT, 5jyxf8k54 # comment 7
        # Fifth block
        # Multi line block
        yellowssp.com, XF7362, DIRECT, 5jyxf8k54 # comment 8
        yellowssp.com, XF7362, DIRECT, 5jyxf8k54 # comment 9


        # Sixth block
        orangessp.com, XF7372, DIRECT, 5jyxf8k54 # comment 10
        "#;

        let res = AdsTxt::from_str(ads_txt);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.systems.len(), 10);
    }

    #[test]
    fn serialize_with_valid_fields() {
        let systems = vec![
            AdsTxtSystem::builder()
                .domain("greenadexchange.com")
                .publisher_id("XF7342")
                .relation(SellerRelationType::Direct)
                .cert_id(Some("5JYXF8K54".to_string()))
                .comment(Some("comment 1".to_string()))
                .build()
                .unwrap(),
            AdsTxtSystem::builder()
                .domain("redssp.com")
                .publisher_id("XF7342")
                .relation(SellerRelationType::Direct)
                .cert_id(Some("5JYXF8K54".to_string()))
                .comment(Some("comment 2".to_string()))
                .build()
                .unwrap(),
            AdsTxtSystem::builder()
                .domain("greenssp.com")
                .publisher_id("XF7342")
                .relation(SellerRelationType::Direct)
                .cert_id(Some("5JYXF8K54".to_string()))
                .comment(Some("comment 3".to_string()))
                .build()
                .unwrap(),
        ];
        let v = AdsTxt::builder()
            .contact(Some("mycontact".to_string()))
            .subdomain(Some("sub.domain.com".to_string()))
            .inventory_partner_domain(Some("inv.domain.com".to_string()))
            .owner_domain(Some("owner.domain.com".to_string()))
            .manager_domains(vec![
                ManagerDomain::builder()
                    .domain("manager.domain.com")
                    .build()
                    .unwrap(),
            ])
            .systems(systems)
            .build()
            .unwrap();
        let res = v.to_string();
        assert_eq!(
            res,
            "contact=mycontact\nsubdomain=sub.domain.com\ninventorypartnerdomain=inv.domain.com\nownerdomain=owner.domain.com\nmanagerdomain=manager.domain.com\ngreenadexchange.com,xf7342,direct,5jyxf8k54 # comment 1\nredssp.com,xf7342,direct,5jyxf8k54 # comment 2\ngreenssp.com,xf7342,direct,5jyxf8k54 # comment 3"
        );
    }

    #[test]
    fn test_builder() {
        let result = AdsTxt::builder().build();
        assert!(result.is_ok());
        let ads_txt = result.unwrap();
        assert!(ads_txt.contact.is_none());
        assert!(ads_txt.subdomain.is_none());
        assert!(ads_txt.systems.is_empty());
    }

    #[test]
    fn test_clone() {
        let original = AdsTxt::builder()
            .contact(Some("test@example.com".to_string()))
            .subdomain(Some("sub.example.com".to_string()))
            .build()
            .unwrap();
        let cloned = original.clone();
        assert_eq!(cloned.contact, original.contact);
        assert_eq!(cloned.subdomain, original.subdomain);
    }

    #[test]
    fn test_debug() {
        let ads_txt = AdsTxt::builder()
            .contact(Some("debug@test.com".to_string()))
            .build()
            .unwrap();
        let debug_str = format!("{:?}", ads_txt);
        assert!(debug_str.contains("AdsTxt"));
        assert!(debug_str.contains("debug@test.com"));
    }

    #[test]
    fn deserialize_with_unknown_variable() {
        let ads_txt = r#"
        unknownvariable=somevalue
        greenadexchange.com, XF7342, DIRECT
        "#;
        let res = AdsTxt::from_str(ads_txt);
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_with_variable_without_value() {
        let ads_txt = r#"
        contact
        greenadexchange.com, XF7342, DIRECT
        "#;
        let res = AdsTxt::from_str(ads_txt);
        assert!(res.is_err());
    }

    #[test]
    fn roundtrip_serialization() {
        let original = r#"
contact=adops@example.com
subdomain=mobile.example.com
ownerdomain=example.com
greenadexchange.com, 12345, DIRECT, f08c47fec0942fa0
silverssp.com, 9876, RESELLER
"#;
        let ads_txt = AdsTxt::from_str(original).unwrap();
        let serialized = ads_txt.to_string();
        let reparsed = AdsTxt::from_str(&serialized).unwrap();

        assert_eq!(ads_txt.contact, reparsed.contact);
        assert_eq!(ads_txt.subdomain, reparsed.subdomain);
        assert_eq!(ads_txt.owner_domain, reparsed.owner_domain);
        assert_eq!(ads_txt.systems.len(), reparsed.systems.len());
    }
}
