use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// BuyerIdentity encapsulates Seat, Agency, and Advertiser identifiers for tiered pricing lookups.
///
/// BuyerIdentity enables context-aware pricing strategies by identifying the buyer's tier in the
/// organizational hierarchy. This supports dynamic pricing based on the specific seat, agency,
/// or advertiser making the purchase.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::BuyerIdentity;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let identity = BuyerIdentity::builder()
///     .seat_id("seat-123")
///     .agency_id("agency-456")
///     .advertiser_id("advertiser-789")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct BuyerIdentity<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the buyer seat (optional, for tiered pricing lookups).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub seat_id: Option<String>,

    /// Unique identifier for the agency representing the buyer (optional, for tiered pricing lookups).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub agency_id: Option<String>,

    /// Unique identifier for the advertiser (optional, for tiered pricing lookups).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub advertiser_id: Option<String>,

    /// Extension object for buyer identity-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl BuyerIdentity {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BuyerIdentityBuilder {
        BuyerIdentityBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buyer_identity_minimal() {
        let identity = BuyerIdentity::builder().build().unwrap();

        assert!(identity.seat_id.is_none());
        assert!(identity.agency_id.is_none());
        assert!(identity.advertiser_id.is_none());
        assert!(identity.ext.is_none());
    }

    #[test]
    fn test_buyer_identity_with_all_fields() {
        let identity = BuyerIdentity::builder()
            .seat_id("seat-12345")
            .agency_id("agency-67890")
            .advertiser_id("advertiser-abcde")
            .build()
            .unwrap();

        assert_eq!(identity.seat_id, Some("seat-12345".to_string()));
        assert_eq!(identity.agency_id, Some("agency-67890".to_string()));
        assert_eq!(identity.advertiser_id, Some("advertiser-abcde".to_string()));
        assert!(identity.ext.is_none());
    }

    #[test]
    fn test_buyer_identity_with_seat_id_only() {
        let identity = BuyerIdentity::builder()
            .seat_id("seat-xyz")
            .build()
            .unwrap();

        assert_eq!(identity.seat_id, Some("seat-xyz".to_string()));
        assert!(identity.agency_id.is_none());
        assert!(identity.advertiser_id.is_none());
    }

    #[test]
    fn test_buyer_identity_with_partial_fields() {
        let identity = BuyerIdentity::builder()
            .agency_id("agency-test")
            .advertiser_id("advertiser-test")
            .build()
            .unwrap();

        assert!(identity.seat_id.is_none());
        assert_eq!(identity.agency_id, Some("agency-test".to_string()));
        assert_eq!(identity.advertiser_id, Some("advertiser-test".to_string()));
    }

    #[test]
    fn test_buyer_identity_serialization() {
        let identity = BuyerIdentity::builder()
            .seat_id("seat-001")
            .agency_id("agency-002")
            .advertiser_id("advertiser-003")
            .build()
            .unwrap();

        let json = serde_json::to_string(&identity).unwrap();
        assert!(json.contains("\"seat_id\":\"seat-001\""));
        assert!(json.contains("\"agency_id\":\"agency-002\""));
        assert!(json.contains("\"advertiser_id\":\"advertiser-003\""));
    }

    #[test]
    fn test_buyer_identity_deserialization() {
        let json = r#"{
            "seat_id": "seat-111",
            "agency_id": "agency-222",
            "advertiser_id": "advertiser-333"
        }"#;

        let identity: BuyerIdentity = serde_json::from_str(json).unwrap();
        assert_eq!(identity.seat_id, Some("seat-111".to_string()));
        assert_eq!(identity.agency_id, Some("agency-222".to_string()));
        assert_eq!(identity.advertiser_id, Some("advertiser-333".to_string()));
    }

    #[test]
    fn test_buyer_identity_roundtrip() {
        let original = BuyerIdentity::builder()
            .seat_id("seat-rt")
            .agency_id("agency-rt")
            .advertiser_id("advertiser-rt")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: BuyerIdentity = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(parsed.seat_id, Some("seat-rt".to_string()));
        assert_eq!(parsed.agency_id, Some("agency-rt".to_string()));
        assert_eq!(parsed.advertiser_id, Some("advertiser-rt".to_string()));
    }

    #[test]
    fn test_buyer_identity_skip_serializing_none() {
        let identity = BuyerIdentity::builder()
            .seat_id("seat-skip")
            .build()
            .unwrap();

        let json = serde_json::to_string(&identity).unwrap();
        assert!(json.contains("\"seat_id\":\"seat-skip\""));
        assert!(!json.contains("agency_id"));
        assert!(!json.contains("advertiser_id"));
    }

    #[test]
    fn test_buyer_identity_default_derives_all_none() {
        let identity: BuyerIdentity = BuyerIdentity::default();

        assert!(identity.seat_id.is_none());
        assert!(identity.agency_id.is_none());
        assert!(identity.advertiser_id.is_none());
        assert!(identity.ext.is_none());
    }
}
