use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Organization type enumeration.
///
/// Represents the type of business entity.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationType {
    #[default]
    Advertiser,
    Agency,
    Publisher,
    Exchange,
}

/// Physical address of an organization.
///
/// Represents address information with optional fields.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Address {
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub street: Option<String>,

    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub city: Option<String>,

    /// State or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub state: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub postal_code: Option<String>,

    /// Country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub country: Option<String>,
}

impl Address {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AddressBuilder {
        AddressBuilder::create_empty()
    }
}

/// Contact information for an organization.
///
/// Represents contact details with optional fields.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct Contact {
    /// Contact person's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub email: Option<String>,

    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub phone: Option<String>,

    /// Contact role or title.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub role: Option<String>,
}

impl Contact {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ContactBuilder {
        ContactBuilder::create_empty()
    }
}

/// Organization entity.
///
/// Represents a business organization with optional address, contact, and extension information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Organization<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Name of the organization (required).
    #[builder(setter(into))]
    pub name: String,

    /// Type of organization.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub type_: Option<OrganizationType>,

    /// Physical address of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub address: Option<Address>,

    /// List of contacts for the organization.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub contacts: Vec<Contact>,

    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub phone: Option<String>,

    /// Website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub url: Option<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Organization {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> OrganizationBuilder {
        OrganizationBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_creation() {
        let org = Organization::builder()
            .name("Tech Corp")
            .type_(Some(OrganizationType::Advertiser))
            .build()
            .unwrap();

        assert_eq!(org.name, "Tech Corp");
        assert_eq!(org.type_, Some(OrganizationType::Advertiser));
        assert!(org.id.is_none());
        assert!(org.address.is_none());
    }

    #[test]
    fn test_organization_serialization() {
        let org = Organization::builder()
            .name("Ad Agency")
            .type_(Some(OrganizationType::Agency))
            .phone("555-1234")
            .build()
            .unwrap();

        let json = serde_json::to_string(&org).unwrap();
        assert!(json.contains("\"name\":\"Ad Agency\""));
        assert!(json.contains("\"type\":\"agency\""));
        assert!(json.contains("\"phone\":\"555-1234\""));
    }

    #[test]
    fn test_organization_deserialization() {
        let json = r#"{"name":"Publisher Inc","type":"publisher","phone":"555-5678"}"#;
        let org: Organization = serde_json::from_str(json).unwrap();

        assert_eq!(org.name, "Publisher Inc");
        assert_eq!(org.type_, Some(OrganizationType::Publisher));
        assert_eq!(org.phone, Some("555-5678".to_string()));
    }

    #[test]
    fn test_organization_roundtrip() {
        let org = Organization::builder()
            .name("Exchange Co")
            .type_(Some(OrganizationType::Exchange))
            .url("https://exchange.example.com")
            .build()
            .unwrap();

        let json = serde_json::to_string(&org).unwrap();
        let parsed: Organization = serde_json::from_str(&json).unwrap();
        assert_eq!(org, parsed);
    }

    #[test]
    fn test_organization_default() {
        let org = Organization::builder().name("Minimal Org").build().unwrap();

        assert_eq!(org.name, "Minimal Org");
        assert!(org.id.is_none());
        assert!(org.type_.is_none());
        assert!(org.address.is_none());
        assert!(org.contacts.is_empty());
        assert!(org.phone.is_none());
        assert!(org.url.is_none());
    }

    #[test]
    fn test_organization_with_address() {
        let address = Address::builder()
            .street("123 Main St")
            .city("San Francisco")
            .state("CA")
            .postal_code("94107")
            .country("USA")
            .build()
            .unwrap();

        let org = Organization::builder()
            .name("SF Corp")
            .address(Some(address))
            .build()
            .unwrap();

        assert_eq!(org.name, "SF Corp");
        assert!(org.address.is_some());
        let addr = org.address.unwrap();
        assert_eq!(addr.city, Some("San Francisco".to_string()));
        assert_eq!(addr.state, Some("CA".to_string()));
    }

    #[test]
    fn test_organization_type_enum() {
        // Test all variants
        let advertiser = OrganizationType::Advertiser;
        let agency = OrganizationType::Agency;
        let publisher = OrganizationType::Publisher;
        let exchange = OrganizationType::Exchange;

        assert_eq!(advertiser, OrganizationType::Advertiser);
        assert_eq!(agency, OrganizationType::Agency);
        assert_eq!(publisher, OrganizationType::Publisher);
        assert_eq!(exchange, OrganizationType::Exchange);

        // Test default
        let default_type: OrganizationType = Default::default();
        assert_eq!(default_type, OrganizationType::Advertiser);

        // Test serialization
        assert_eq!(
            serde_json::to_string(&advertiser).unwrap(),
            "\"advertiser\""
        );
        assert_eq!(serde_json::to_string(&agency).unwrap(), "\"agency\"");
        assert_eq!(serde_json::to_string(&publisher).unwrap(), "\"publisher\"");
        assert_eq!(serde_json::to_string(&exchange).unwrap(), "\"exchange\"");

        // Test roundtrip
        let serialized = serde_json::to_string(&agency).unwrap();
        let deserialized: OrganizationType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(agency, deserialized);
    }

    #[test]
    fn test_address_roundtrip() {
        let address = Address::builder()
            .street("456 Oak Ave")
            .city("New York")
            .build()
            .unwrap();

        let json = serde_json::to_string(&address).unwrap();
        let parsed: Address = serde_json::from_str(&json).unwrap();
        assert_eq!(address, parsed);
    }

    #[test]
    fn test_contact_roundtrip() {
        let contact = Contact::builder()
            .name("John Doe")
            .email("john@example.com")
            .phone("555-9999")
            .role("Manager")
            .build()
            .unwrap();

        let json = serde_json::to_string(&contact).unwrap();
        let parsed: Contact = serde_json::from_str(&json).unwrap();
        assert_eq!(contact, parsed);
    }
}
