use crate::seller_agent::v10::enums::{OrganizationRole, PricingTierType};
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents a seller organization with its role and configuration.
///
/// SellerOrganization extends the base organization concept with seller-specific
/// role information and extensibility for custom fields.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::SellerOrganization;
/// use iab_specs::seller_agent::v10::enums::OrganizationRole;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let org = SellerOrganization::builder()
///     .name("Example Publisher".to_string())
///     .role(OrganizationRole::Seller)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SellerOrganization<Ext: Extension = crate::DefaultExt> {
    /// Organization name (REQUIRED).
    /// Identifies the organization in the advertising workflow.
    #[builder(default)]
    pub name: String,

    /// Organization role (REQUIRED).
    /// Specifies the organization's function in the advertising ecosystem.
    #[builder(default)]
    pub role: OrganizationRole,

    /// Extension object for organization-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SellerOrganization {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SellerOrganizationBuilder {
        SellerOrganizationBuilder::create_empty()
    }
}

/// Represents a seller account with its tier and credit configuration.
///
/// SellerAccount extends the base account concept with seller-specific tier
/// and credit limit information for managing account-level constraints.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::SellerAccount;
/// use iab_specs::seller_agent::v10::enums::PricingTierType;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let account = SellerAccount::builder()
///     .name("Agency Partner Account".to_string())
///     .tier(PricingTierType::Agency)
///     .credit_limit(Some(50000.0))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SellerAccount<Ext: Extension = crate::DefaultExt> {
    /// Account name (REQUIRED).
    /// Identifies the account in the advertising workflow.
    #[builder(default)]
    pub name: String,

    /// Pricing tier type (REQUIRED).
    /// Determines the applicable pricing level for this account.
    #[builder(default)]
    pub tier: PricingTierType,

    /// Credit limit in currency units.
    /// Optional maximum credit that can be extended to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub credit_limit: Option<f64>,

    /// Extension object for account-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SellerAccount {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SellerAccountBuilder {
        SellerAccountBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== SellerOrganization Tests ==========

    #[test]
    fn test_seller_organization_minimal() {
        let org = SellerOrganization::builder()
            .name("Example Publisher".to_string())
            .role(OrganizationRole::Seller)
            .build()
            .unwrap();

        assert_eq!(org.name, "Example Publisher");
        assert_eq!(org.role, OrganizationRole::Seller);
        assert!(org.ext.is_none());
    }

    #[test]
    fn test_seller_organization_all_roles() {
        let roles = [
            OrganizationRole::Buyer,
            OrganizationRole::Seller,
            OrganizationRole::Agent,
            OrganizationRole::Curator,
        ];

        for role in &roles {
            let org = SellerOrganization::builder()
                .name(format!("Org with {:?} role", role))
                .role(*role)
                .build()
                .unwrap();

            assert_eq!(org.role, *role);
        }
    }

    #[test]
    fn test_seller_organization_serialization_roundtrip() {
        let original = SellerOrganization::builder()
            .name("Publisher Corp".to_string())
            .role(OrganizationRole::Seller)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: SellerOrganization = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.role, original.role);
    }

    #[test]
    fn test_seller_organization_serialization_format() {
        let org = SellerOrganization::builder()
            .name("Test Org".to_string())
            .role(OrganizationRole::Agent)
            .build()
            .unwrap();

        let json = serde_json::to_string(&org).unwrap();
        assert!(json.contains("\"name\":\"Test Org\""));
        assert!(json.contains("\"role\":\"agent\""));
    }

    // ========== SellerAccount Tests ==========

    #[test]
    fn test_seller_account_minimal() {
        let account = SellerAccount::builder()
            .name("Standard Account".to_string())
            .tier(PricingTierType::Public)
            .build()
            .unwrap();

        assert_eq!(account.name, "Standard Account");
        assert_eq!(account.tier, PricingTierType::Public);
        assert!(account.credit_limit.is_none());
        assert!(account.ext.is_none());
    }

    #[test]
    fn test_seller_account_with_credit_limit() {
        let account = SellerAccount::builder()
            .name("Premium Account".to_string())
            .tier(PricingTierType::Agency)
            .credit_limit(Some(50000.0))
            .build()
            .unwrap();

        assert_eq!(account.name, "Premium Account");
        assert_eq!(account.tier, PricingTierType::Agency);
        assert_eq!(account.credit_limit, Some(50000.0));
    }

    #[test]
    fn test_seller_account_all_tiers() {
        let tiers = [
            PricingTierType::Public,
            PricingTierType::Seat,
            PricingTierType::Agency,
            PricingTierType::Advertiser,
        ];

        for tier in &tiers {
            let account = SellerAccount::builder()
                .name(format!("Account with {:?} tier", tier))
                .tier(*tier)
                .build()
                .unwrap();

            assert_eq!(account.tier, *tier);
        }
    }

    #[test]
    fn test_seller_account_serialization_roundtrip() {
        let original = SellerAccount::builder()
            .name("Agency Partner".to_string())
            .tier(PricingTierType::Agency)
            .credit_limit(Some(25000.50))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: SellerAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.tier, original.tier);
        assert_eq!(parsed.credit_limit, original.credit_limit);
    }

    #[test]
    fn test_seller_account_serialization_format() {
        let account = SellerAccount::builder()
            .name("Test Account".to_string())
            .tier(PricingTierType::Advertiser)
            .credit_limit(Some(100000.0))
            .build()
            .unwrap();

        let json = serde_json::to_string(&account).unwrap();
        assert!(json.contains("\"name\":\"Test Account\""));
        assert!(json.contains("\"tier\":\"advertiser\""));
        assert!(json.contains("\"credit_limit\":100000"));
    }

    #[test]
    fn test_seller_account_zero_credit_limit() {
        let account = SellerAccount::builder()
            .name("No Credit".to_string())
            .tier(PricingTierType::Public)
            .credit_limit(Some(0.0))
            .build()
            .unwrap();

        assert_eq!(account.credit_limit, Some(0.0));
    }

    #[test]
    fn test_seller_account_high_precision_credit_limit() {
        let account = SellerAccount::builder()
            .name("Precision Test".to_string())
            .tier(PricingTierType::Agency)
            .credit_limit(Some(12345.6789))
            .build()
            .unwrap();

        assert_eq!(account.credit_limit, Some(12345.6789));

        let json = serde_json::to_string(&account).unwrap();
        let parsed: SellerAccount = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.credit_limit, Some(12345.6789));
    }

    /// Seller Agent 1.0 § SellerOrganization — default builder yields empty org
    #[test]
    fn test_seller_organization_default() {
        let org = SellerOrganization::builder().build().unwrap();
        assert_eq!(org.name, "");
        assert_eq!(org.role, OrganizationRole::Seller);
        assert!(org.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerOrganization — optional fields omitted from JSON when None
    #[test]
    fn test_seller_organization_optional_fields_skipped() {
        let org = SellerOrganization::builder()
            .name("Org".to_string())
            .role(OrganizationRole::Seller)
            .build()
            .unwrap();

        let json = serde_json::to_string(&org).unwrap();
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § SellerOrganization — clone produces identical value
    #[test]
    fn test_seller_organization_clone() {
        let org = SellerOrganization::builder()
            .name("Clone Org".to_string())
            .role(OrganizationRole::Agent)
            .build()
            .unwrap();

        let cloned = org.clone();
        assert_eq!(org, cloned);
    }

    /// Seller Agent 1.0 § SellerOrganization — deserialization from minimal JSON
    #[test]
    fn test_seller_organization_deserialization_minimal() {
        let json = r#"{"name":"Pub","role":"seller"}"#;
        let org: SellerOrganization = serde_json::from_str(json).unwrap();
        assert_eq!(org.name, "Pub");
        assert_eq!(org.role, OrganizationRole::Seller);
        assert!(org.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerAccount — default builder yields empty account
    #[test]
    fn test_seller_account_default() {
        let account = SellerAccount::builder().build().unwrap();
        assert_eq!(account.name, "");
        assert_eq!(account.tier, PricingTierType::Public);
        assert!(account.credit_limit.is_none());
        assert!(account.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerAccount — optional fields omitted from JSON when None
    #[test]
    fn test_seller_account_optional_fields_skipped() {
        let account = SellerAccount::builder()
            .name("Acc".to_string())
            .tier(PricingTierType::Public)
            .build()
            .unwrap();

        let json = serde_json::to_string(&account).unwrap();
        assert!(!json.contains("\"credit_limit\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § SellerAccount — clone produces identical value
    #[test]
    fn test_seller_account_clone() {
        let account = SellerAccount::builder()
            .name("Clone Acc".to_string())
            .tier(PricingTierType::Agency)
            .credit_limit(Some(10000.0))
            .build()
            .unwrap();

        let cloned = account.clone();
        assert_eq!(account, cloned);
    }

    /// Seller Agent 1.0 § SellerAccount — deserialization from minimal JSON
    #[test]
    fn test_seller_account_deserialization_minimal() {
        let json = r#"{"name":"Acc","tier":"agency"}"#;
        let account: SellerAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.name, "Acc");
        assert_eq!(account.tier, PricingTierType::Agency);
        assert!(account.credit_limit.is_none());
    }
}
