use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A Proposal represents a deal proposal from a seller to a buyer
/// in the Seller Agent workflow.
///
/// Proposals track the buyer and seller involved, the current status,
/// and link to the latest revision containing itemized details.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::Proposal;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let proposal = Proposal::builder()
///     .buyer_id("buyer-001")
///     .seller_id("seller-001")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Proposal<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the proposal.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Identifier of the buyer this proposal targets (REQUIRED).
    #[builder(default, setter(into))]
    pub buyer_id: String,

    /// Identifier of the seller making the proposal (REQUIRED).
    #[builder(default, setter(into))]
    pub seller_id: String,

    /// Current status of the proposal.
    #[builder(default)]
    pub status: crate::v10::enums::ProposalStatus,

    /// Identifier of the current active revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub current_revision_id: Option<String>,

    /// Timestamp when the proposal was created (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub created_at: Option<String>,

    /// Timestamp when the proposal was last updated (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub updated_at: Option<String>,

    /// Extension object for proposal-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Proposal {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ProposalBuilder {
        ProposalBuilder::create_empty()
    }
}

/// A ProposalRevision represents a versioned snapshot of a proposal's items and terms.
///
/// Each revision captures the line items, total budget, and any notes for that
/// version of the proposal. Multiple revisions allow tracking negotiation history.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::{ProposalRevision, ProposalItem};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let revision = ProposalRevision::builder()
///     .proposal_id("prop-001")
///     .revision_number(1)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ProposalRevision<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Identifier of the parent proposal (REQUIRED).
    #[builder(default, setter(into))]
    pub proposal_id: String,

    /// Sequential revision number (REQUIRED).
    #[builder(default)]
    pub revision_number: i32,

    /// Line items included in this revision.
    #[builder(default)]
    pub items: Vec<ProposalItem>,

    /// Total budget for this revision in currency units.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub total_budget: Option<f64>,

    /// Free-text notes for this revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub notes: Option<String>,

    /// Timestamp when the revision was created (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub created_at: Option<String>,

    /// Extension object for revision-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ProposalRevision {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ProposalRevisionBuilder {
        ProposalRevisionBuilder::create_empty()
    }
}

/// A ProposalItem represents a single line item within a proposal revision,
/// specifying the product, quantity, pricing, and schedule.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_seller_agent::v10::models::ProposalItem;
/// use iab_specs_agentic_direct::v21::enums::RateType;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let item = ProposalItem::builder()
///     .product_id("prod-001")
///     .quantity(1000)
///     .rate(2.50)
///     .rate_type(RateType::Cpm)
///     .start_date("2026-04-01")
///     .end_date("2026-06-30")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct ProposalItem<Ext: Extension = crate::DefaultExt> {
    /// Identifier of the product being offered (REQUIRED).
    #[builder(default, setter(into))]
    pub product_id: String,

    /// Quantity of impressions or units (REQUIRED).
    #[builder(default)]
    pub quantity: i64,

    /// Price rate in currency units (REQUIRED).
    #[builder(default)]
    pub rate: f64,

    /// Pricing model type (REQUIRED).
    #[builder(default)]
    pub rate_type: iab_specs_agentic_direct::v21::enums::RateType,

    /// Campaign start date in ISO 8601 format (REQUIRED).
    #[builder(default, setter(into))]
    pub start_date: String,

    /// Campaign end date in ISO 8601 format (REQUIRED).
    #[builder(default, setter(into))]
    pub end_date: String,

    /// Extension object for item-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl ProposalItem {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> ProposalItemBuilder {
        ProposalItemBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v10::enums::ProposalStatus;
    use iab_specs_agentic_direct::v21::enums::RateType;

    #[test]
    fn test_proposal_creation() {
        let proposal = Proposal::builder()
            .buyer_id("buyer-001")
            .seller_id("seller-001")
            .build()
            .unwrap();

        assert_eq!(proposal.buyer_id, "buyer-001");
        assert_eq!(proposal.seller_id, "seller-001");
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert!(proposal.id.is_none());
        assert!(proposal.current_revision_id.is_none());
        assert!(proposal.created_at.is_none());
        assert!(proposal.updated_at.is_none());
        assert!(proposal.ext.is_none());
    }

    #[test]
    fn test_proposal_serialization_roundtrip() {
        let original = Proposal::builder()
            .id("prop-001")
            .buyer_id("buyer-001")
            .seller_id("seller-001")
            .status(ProposalStatus::Submitted)
            .current_revision_id("rev-001")
            .created_at("2026-04-01T00:00:00Z")
            .updated_at("2026-04-02T00:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Proposal = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_proposal_revision_with_items() {
        let items = vec![
            ProposalItem::builder()
                .product_id("prod-001")
                .quantity(100_000)
                .rate(2.50)
                .rate_type(RateType::Cpm)
                .start_date("2026-04-01")
                .end_date("2026-04-30")
                .build()
                .unwrap(),
            ProposalItem::builder()
                .product_id("prod-002")
                .quantity(50_000)
                .rate(1.75)
                .rate_type(RateType::Cpc)
                .start_date("2026-05-01")
                .end_date("2026-05-31")
                .build()
                .unwrap(),
            ProposalItem::builder()
                .product_id("prod-003")
                .quantity(25_000)
                .rate(5.00)
                .rate_type(RateType::Cpa)
                .start_date("2026-06-01")
                .end_date("2026-06-30")
                .build()
                .unwrap(),
        ];

        let revision = ProposalRevision::builder()
            .id("rev-001")
            .proposal_id("prop-001")
            .revision_number(1)
            .items(items)
            .total_budget(Some(10000.0))
            .notes("Initial proposal")
            .created_at("2026-04-01T00:00:00Z")
            .build()
            .unwrap();

        assert_eq!(revision.items.len(), 3);
        assert_eq!(revision.revision_number, 1);
        assert_eq!(revision.total_budget, Some(10000.0));

        let json = serde_json::to_string(&revision).unwrap();
        let parsed: ProposalRevision = serde_json::from_str(&json).unwrap();
        assert_eq!(revision, parsed);
    }

    #[test]
    fn test_proposal_item_with_rate_type() {
        let item = ProposalItem::builder()
            .product_id("prod-001")
            .quantity(100_000)
            .rate(3.50)
            .rate_type(RateType::Cpm)
            .start_date("2026-04-01")
            .end_date("2026-06-30")
            .build()
            .unwrap();

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"rate_type\":\"cpm\""));
        assert!(json.contains("\"rate\":3.5"));

        let parsed: ProposalItem = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.rate_type, RateType::Cpm);
    }

    #[test]
    fn test_proposal_default() {
        let proposal = Proposal::builder().build().unwrap();

        assert_eq!(proposal.buyer_id, "");
        assert_eq!(proposal.seller_id, "");
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert!(proposal.id.is_none());
        assert!(proposal.ext.is_none());
    }

    #[test]
    fn test_proposal_deserialization() {
        let json = r#"{
            "id": "prop-100",
            "buyer_id": "buyer-xyz",
            "seller_id": "seller-abc",
            "status": "accepted",
            "current_revision_id": "rev-005",
            "created_at": "2026-01-15T10:30:00Z"
        }"#;

        let proposal: Proposal = serde_json::from_str(json).unwrap();
        assert_eq!(proposal.id, Some("prop-100".to_string()));
        assert_eq!(proposal.buyer_id, "buyer-xyz");
        assert_eq!(proposal.seller_id, "seller-abc");
        assert_eq!(proposal.status, ProposalStatus::Accepted);
        assert_eq!(proposal.current_revision_id, Some("rev-005".to_string()));
        assert_eq!(
            proposal.created_at,
            Some("2026-01-15T10:30:00Z".to_string())
        );
        assert!(proposal.updated_at.is_none());
    }

    /// Seller Agent 1.0 § Proposal — optional fields omitted from JSON when None
    #[test]
    fn test_proposal_optional_fields_skipped() {
        let proposal = Proposal::builder()
            .buyer_id("b")
            .seller_id("s")
            .build()
            .unwrap();

        let json = serde_json::to_string(&proposal).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"current_revision_id\""));
        assert!(!json.contains("\"created_at\""));
        assert!(!json.contains("\"updated_at\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § Proposal — clone produces identical value
    #[test]
    fn test_proposal_clone() {
        let proposal = Proposal::builder()
            .id("prop-c")
            .buyer_id("buyer")
            .seller_id("seller")
            .status(ProposalStatus::Submitted)
            .build()
            .unwrap();

        let cloned = proposal.clone();
        assert_eq!(proposal, cloned);
    }

    /// Seller Agent 1.0 § Proposal — deserialization from minimal JSON
    #[test]
    fn test_proposal_deserialization_minimal() {
        let json = r#"{"buyer_id":"b","seller_id":"s","status":"draft"}"#;
        let proposal: Proposal = serde_json::from_str(json).unwrap();
        assert_eq!(proposal.buyer_id, "b");
        assert_eq!(proposal.seller_id, "s");
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert!(proposal.id.is_none());
    }

    /// Seller Agent 1.0 § ProposalRevision — default builder yields empty revision
    #[test]
    fn test_proposal_revision_default() {
        let rev = ProposalRevision::builder().build().unwrap();
        assert_eq!(rev.proposal_id, "");
        assert_eq!(rev.revision_number, 0);
        assert!(rev.items.is_empty());
        assert!(rev.total_budget.is_none());
        assert!(rev.notes.is_none());
        assert!(rev.created_at.is_none());
        assert!(rev.id.is_none());
        assert!(rev.ext.is_none());
    }

    /// Seller Agent 1.0 § ProposalRevision — optional fields omitted from JSON when None
    #[test]
    fn test_proposal_revision_optional_fields_skipped() {
        let rev = ProposalRevision::builder()
            .proposal_id("p")
            .revision_number(1)
            .build()
            .unwrap();

        let json = serde_json::to_string(&rev).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"total_budget\""));
        assert!(!json.contains("\"notes\""));
        assert!(!json.contains("\"created_at\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § ProposalRevision — clone produces identical value
    #[test]
    fn test_proposal_revision_clone() {
        let rev = ProposalRevision::builder()
            .id("rev-c")
            .proposal_id("p")
            .revision_number(2)
            .total_budget(Some(5000.0))
            .notes("note")
            .build()
            .unwrap();

        let cloned = rev.clone();
        assert_eq!(rev, cloned);
    }

    /// Seller Agent 1.0 § ProposalRevision — deserialization from minimal JSON
    #[test]
    fn test_proposal_revision_deserialization_minimal() {
        let json = r#"{"proposal_id":"p","revision_number":1,"items":[]}"#;
        let rev: ProposalRevision = serde_json::from_str(json).unwrap();
        assert_eq!(rev.proposal_id, "p");
        assert_eq!(rev.revision_number, 1);
        assert!(rev.items.is_empty());
        assert!(rev.total_budget.is_none());
    }

    /// Seller Agent 1.0 § ProposalItem — default builder yields empty item
    #[test]
    fn test_proposal_item_default() {
        let item = ProposalItem::builder().build().unwrap();
        assert_eq!(item.product_id, "");
        assert_eq!(item.quantity, 0);
        assert_eq!(item.rate, 0.0);
        assert_eq!(item.rate_type, RateType::Cpm);
        assert_eq!(item.start_date, "");
        assert_eq!(item.end_date, "");
        assert!(item.ext.is_none());
    }

    /// Seller Agent 1.0 § ProposalItem — optional fields omitted from JSON when None
    #[test]
    fn test_proposal_item_optional_fields_skipped() {
        let item = ProposalItem::builder()
            .product_id("p")
            .quantity(100)
            .rate(1.0)
            .rate_type(RateType::Cpm)
            .start_date("2026-01-01")
            .end_date("2026-02-01")
            .build()
            .unwrap();

        let json = serde_json::to_string(&item).unwrap();
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § ProposalItem — clone produces identical value
    #[test]
    fn test_proposal_item_clone() {
        let item = ProposalItem::builder()
            .product_id("prod-c")
            .quantity(500)
            .rate(3.0)
            .rate_type(RateType::Cpc)
            .start_date("2026-01-01")
            .end_date("2026-03-01")
            .build()
            .unwrap();

        let cloned = item.clone();
        assert_eq!(item, cloned);
    }

    /// Seller Agent 1.0 § ProposalItem — deserialization from minimal JSON
    #[test]
    fn test_proposal_item_deserialization_minimal() {
        let json = r#"{
            "product_id":"prod-1",
            "quantity":100,
            "rate":2.0,
            "rate_type":"cpm",
            "start_date":"2026-01-01",
            "end_date":"2026-02-01"
        }"#;
        let item: ProposalItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.product_id, "prod-1");
        assert_eq!(item.quantity, 100);
        assert_eq!(item.rate, 2.0);
        assert_eq!(item.rate_type, RateType::Cpm);
    }
}
