use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Account status enumeration.
///
/// Represents the current operational status of an account.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum AccountStatus {
    #[default]
    Active,
    Paused,
    Closed,
}

/// Account entity.
///
/// Represents a buyer or advertiser account with required advertiser and buyer identifiers.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Account<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Advertiser identifier (required).
    #[builder(setter(into))]
    pub advertiser_id: String,

    /// Buyer identifier (required).
    #[builder(setter(into))]
    pub buyer_id: String,

    /// Account name (required).
    #[builder(setter(into))]
    pub name: String,

    /// Current status of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub status: Option<AccountStatus>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Account {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AccountBuilder {
        AccountBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_creation() {
        let account = Account::builder()
            .advertiser_id("adv-123")
            .buyer_id("buyer-456")
            .name("Test Account")
            .build()
            .unwrap();

        assert_eq!(account.advertiser_id, "adv-123");
        assert_eq!(account.buyer_id, "buyer-456");
        assert_eq!(account.name, "Test Account");
        assert!(account.id.is_none());
        assert!(account.status.is_none());
    }

    #[test]
    fn test_account_serialization() {
        let account = Account::builder()
            .advertiser_id("adv-789")
            .buyer_id("buyer-101")
            .name("Premium Account")
            .status(Some(AccountStatus::Active))
            .build()
            .unwrap();

        let json = serde_json::to_string(&account).unwrap();
        assert!(json.contains("\"advertiser_id\":\"adv-789\""));
        assert!(json.contains("\"buyer_id\":\"buyer-101\""));
        assert!(json.contains("\"name\":\"Premium Account\""));
        assert!(json.contains("\"status\":\"active\""));
    }

    #[test]
    fn test_account_deserialization() {
        let json = r#"{"advertiser_id":"adv-202","buyer_id":"buyer-303","name":"Restored Account","status":"paused"}"#;
        let account: Account = serde_json::from_str(json).unwrap();

        assert_eq!(account.advertiser_id, "adv-202");
        assert_eq!(account.buyer_id, "buyer-303");
        assert_eq!(account.name, "Restored Account");
        assert_eq!(account.status, Some(AccountStatus::Paused));
    }

    #[test]
    fn test_account_roundtrip() {
        let account = Account::builder()
            .id("acc-999")
            .advertiser_id("adv-555")
            .buyer_id("buyer-666")
            .name("Roundtrip Account")
            .status(Some(AccountStatus::Closed))
            .build()
            .unwrap();

        let json = serde_json::to_string(&account).unwrap();
        let parsed: Account = serde_json::from_str(&json).unwrap();
        assert_eq!(account, parsed);
    }

    #[test]
    fn test_account_status_enum() {
        // Test all variants
        let active = AccountStatus::Active;
        let paused = AccountStatus::Paused;
        let closed = AccountStatus::Closed;

        assert_eq!(active, AccountStatus::Active);
        assert_eq!(paused, AccountStatus::Paused);
        assert_eq!(closed, AccountStatus::Closed);

        // Test default
        let default_status: AccountStatus = Default::default();
        assert_eq!(default_status, AccountStatus::Active);

        // Test serialization
        assert_eq!(serde_json::to_string(&active).unwrap(), "\"active\"");
        assert_eq!(serde_json::to_string(&paused).unwrap(), "\"paused\"");
        assert_eq!(serde_json::to_string(&closed).unwrap(), "\"closed\"");

        // Test roundtrip
        let serialized = serde_json::to_string(&paused).unwrap();
        let deserialized: AccountStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(paused, deserialized);
    }
}
