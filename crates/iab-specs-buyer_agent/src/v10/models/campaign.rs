use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A CampaignBrief represents the high-level goals, budget, and constraints of an advertising campaign.
///
/// Campaign briefs define the overall strategy and parameters that drive allocation decisions
/// for individual channels and inventory items.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::CampaignBrief;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let campaign = CampaignBrief::builder()
///     .name("Q2 Display Campaign")
///     .budget(50000.0)
///     .start_date("2024-04-01")
///     .end_date("2024-06-30")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct CampaignBrief<Ext: Extension = crate::DefaultExt> {
    /// Human-readable name of the campaign (REQUIRED).
    #[builder(setter(into))]
    pub name: String,

    /// Campaign objectives as a list of strings.
    #[serde(default)]
    #[builder(default)]
    pub objectives: Vec<String>,

    /// Total campaign budget in currency units (REQUIRED).
    #[builder(default)]
    pub budget: f64,

    /// Campaign start date in ISO 8601 format (REQUIRED).
    #[builder(setter(into))]
    pub start_date: String,

    /// Campaign end date in ISO 8601 format (REQUIRED).
    #[builder(setter(into))]
    pub end_date: String,

    /// Target audience definition as arbitrary JSON blob.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub target_audience: Option<serde_json::Value>,

    /// Key Performance Indicators as arbitrary JSON blob.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub kpis: Option<serde_json::Value>,

    /// List of channels (e.g., "email", "display", "social").
    #[serde(default)]
    #[builder(default)]
    pub channels: Vec<String>,

    /// Operational constraints as arbitrary JSON blob.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub constraints: Option<serde_json::Value>,

    /// Extension object for campaign-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl CampaignBrief {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CampaignBriefBuilder {
        CampaignBriefBuilder::create_empty()
    }
}

/// A CampaignAllocation represents the budget and strategy for a specific channel within a campaign.
///
/// Allocations determine how the campaign's overall budget is distributed across channels,
/// along with priority and rationale for the allocation decision.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::CampaignAllocation;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let allocation = CampaignAllocation::builder()
///     .channel("display")
///     .budget_share(0.5)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct CampaignAllocation<Ext: Extension = crate::DefaultExt> {
    /// The channel name (e.g., "email", "display", "social") (REQUIRED).
    #[builder(setter(into))]
    pub channel: String,

    /// Budget share for this channel as a fraction of total budget (0.0 to 1.0) (REQUIRED).
    #[builder(default)]
    pub budget_share: f64,

    /// Priority level for this channel allocation (defaults to 0).
    #[serde(default)]
    #[builder(default)]
    pub priority: i32,

    /// Explanation for the allocation decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub rationale: Option<String>,

    /// Extension object for allocation-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl CampaignAllocation {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> CampaignAllocationBuilder {
        CampaignAllocationBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== CampaignBrief Tests ==========

    #[test]
    fn test_campaign_brief_minimal() {
        let campaign = CampaignBrief::builder()
            .name("Q2 Campaign")
            .budget(50000.0)
            .start_date("2024-04-01")
            .end_date("2024-06-30")
            .build()
            .unwrap();

        assert_eq!(campaign.name, "Q2 Campaign");
        assert_eq!(campaign.budget, 50000.0);
        assert_eq!(campaign.start_date, "2024-04-01");
        assert_eq!(campaign.end_date, "2024-06-30");
        assert_eq!(campaign.objectives.len(), 0);
        assert!(campaign.target_audience.is_none());
        assert!(campaign.kpis.is_none());
        assert_eq!(campaign.channels.len(), 0);
        assert!(campaign.constraints.is_none());
        assert!(campaign.ext.is_none());
    }

    #[test]
    fn test_campaign_brief_full() {
        let campaign = CampaignBrief::builder()
            .name("Premium Q2 Campaign")
            .objectives(vec![
                "Increase brand awareness".to_string(),
                "Drive conversions".to_string(),
            ])
            .budget(100000.0)
            .start_date("2024-04-01")
            .end_date("2024-06-30")
            .target_audience(Some(serde_json::json!({
                "age_range": "25-54",
                "interests": ["tech", "sports"],
                "location": "US"
            })))
            .kpis(Some(serde_json::json!({
                "impression_target": 1000000,
                "cpa": 5.0,
                "roas": 3.0
            })))
            .channels(vec!["display".to_string(), "email".to_string()])
            .constraints(Some(serde_json::json!({
                "max_cpc": 2.0,
                "min_viewability": 0.5
            })))
            .build()
            .unwrap();

        assert_eq!(campaign.name, "Premium Q2 Campaign");
        assert_eq!(campaign.objectives.len(), 2);
        assert_eq!(campaign.budget, 100000.0);
        assert_eq!(campaign.start_date, "2024-04-01");
        assert_eq!(campaign.end_date, "2024-06-30");
        assert!(campaign.target_audience.is_some());
        assert!(campaign.kpis.is_some());
        assert_eq!(campaign.channels.len(), 2);
        assert!(campaign.constraints.is_some());
    }

    #[test]
    fn test_campaign_brief_target_audience_json_blob() {
        let target_audience = serde_json::json!({
            "age": "25-54",
            "interests": ["sports", "tech"],
            "devices": {
                "desktop": true,
                "mobile": true
            }
        });

        let campaign = CampaignBrief::builder()
            .name("Targeted Campaign")
            .budget(25000.0)
            .start_date("2024-05-01")
            .end_date("2024-05-31")
            .target_audience(Some(target_audience.clone()))
            .build()
            .unwrap();

        assert_eq!(campaign.target_audience, Some(target_audience.clone()));

        // Verify roundtrip preserves JSON blob
        let json = serde_json::to_string(&campaign).unwrap();
        let parsed: CampaignBrief = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.target_audience, Some(target_audience));
    }

    #[test]
    fn test_campaign_brief_serialization() {
        let campaign = CampaignBrief::builder()
            .name("Serializable Campaign")
            .budget(75000.0)
            .start_date("2024-03-01")
            .end_date("2024-03-31")
            .objectives(vec!["Awareness".to_string()])
            .channels(vec!["display".to_string()])
            .build()
            .unwrap();

        let json = serde_json::to_string(&campaign).unwrap();
        assert!(json.contains("\"name\":\"Serializable Campaign\""));
        assert!(json.contains("\"budget\":75000"));
        assert!(json.contains("\"start_date\":\"2024-03-01\""));
        assert!(json.contains("\"end_date\":\"2024-03-31\""));
        assert!(json.contains("\"objectives\":[\"Awareness\"]"));
        assert!(json.contains("\"channels\":[\"display\"]"));
    }

    #[test]
    fn test_campaign_brief_deserialization() {
        let json = r#"{
            "name": "Deserialized Campaign",
            "budget": 50000.0,
            "start_date": "2024-02-01",
            "end_date": "2024-02-28",
            "objectives": ["Conversions"],
            "channels": ["email", "social"],
            "target_audience": {"geo": "US"},
            "kpis": {"roas": 2.5}
        }"#;

        let campaign: CampaignBrief = serde_json::from_str(json).unwrap();
        assert_eq!(campaign.name, "Deserialized Campaign");
        assert_eq!(campaign.budget, 50000.0);
        assert_eq!(campaign.start_date, "2024-02-01");
        assert_eq!(campaign.end_date, "2024-02-28");
        assert_eq!(campaign.objectives.len(), 1);
        assert_eq!(campaign.channels.len(), 2);
        assert!(campaign.target_audience.is_some());
        assert!(campaign.kpis.is_some());
    }

    #[test]
    fn test_campaign_brief_roundtrip_with_all_fields() {
        let original = CampaignBrief::builder()
            .name("Roundtrip Campaign")
            .objectives(vec!["Brand".to_string(), "Sales".to_string()])
            .budget(125000.0)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .target_audience(Some(serde_json::json!({"age": "18-65", "location": "NA"})))
            .kpis(Some(serde_json::json!({"roi": 4.0})))
            .channels(vec!["display".to_string(), "video".to_string()])
            .constraints(Some(serde_json::json!({"frequency_cap": 3})))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: CampaignBrief = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.objectives, original.objectives);
        assert_eq!(parsed.budget, original.budget);
        assert_eq!(parsed.start_date, original.start_date);
        assert_eq!(parsed.end_date, original.end_date);
        assert_eq!(parsed.target_audience, original.target_audience);
        assert_eq!(parsed.kpis, original.kpis);
        assert_eq!(parsed.channels, original.channels);
        assert_eq!(parsed.constraints, original.constraints);
    }

    // ========== CampaignAllocation Tests ==========

    #[test]
    fn test_campaign_allocation_minimal() {
        let allocation = CampaignAllocation::builder()
            .channel("display")
            .budget_share(0.5)
            .build()
            .unwrap();

        assert_eq!(allocation.channel, "display");
        assert_eq!(allocation.budget_share, 0.5);
        assert_eq!(allocation.priority, 0);
        assert!(allocation.rationale.is_none());
        assert!(allocation.ext.is_none());
    }

    #[test]
    fn test_campaign_allocation_full() {
        let allocation = CampaignAllocation::builder()
            .channel("email")
            .budget_share(0.3)
            .priority(2)
            .rationale("High engagement channel with lower CPM")
            .build()
            .unwrap();

        assert_eq!(allocation.channel, "email");
        assert_eq!(allocation.budget_share, 0.3);
        assert_eq!(allocation.priority, 2);
        assert_eq!(
            allocation.rationale,
            Some("High engagement channel with lower CPM".to_string())
        );
        assert!(allocation.ext.is_none());
    }

    #[test]
    fn test_campaign_allocation_budget_share_range() {
        // Test valid range (0.0 to 1.0)
        let low = CampaignAllocation::builder()
            .channel("social")
            .budget_share(0.0)
            .build()
            .unwrap();
        assert_eq!(low.budget_share, 0.0);

        let mid = CampaignAllocation::builder()
            .channel("social")
            .budget_share(0.5)
            .build()
            .unwrap();
        assert_eq!(mid.budget_share, 0.5);

        let high = CampaignAllocation::builder()
            .channel("social")
            .budget_share(1.0)
            .build()
            .unwrap();
        assert_eq!(high.budget_share, 1.0);
    }

    #[test]
    fn test_campaign_allocation_serialization() {
        let allocation = CampaignAllocation::builder()
            .channel("video")
            .budget_share(0.25)
            .priority(1)
            .rationale("Premium video inventory")
            .build()
            .unwrap();

        let json = serde_json::to_string(&allocation).unwrap();
        assert!(json.contains("\"channel\":\"video\""));
        assert!(json.contains("\"budget_share\":0.25"));
        assert!(json.contains("\"priority\":1"));
        assert!(json.contains("\"rationale\":\"Premium video inventory\""));
    }

    #[test]
    fn test_campaign_allocation_deserialization() {
        let json = r#"{
            "channel": "native",
            "budget_share": 0.2,
            "priority": 3,
            "rationale": "Native ads drive higher engagement"
        }"#;

        let allocation: CampaignAllocation = serde_json::from_str(json).unwrap();
        assert_eq!(allocation.channel, "native");
        assert_eq!(allocation.budget_share, 0.2);
        assert_eq!(allocation.priority, 3);
        assert_eq!(
            allocation.rationale,
            Some("Native ads drive higher engagement".to_string())
        );
    }

    #[test]
    fn test_campaign_allocation_roundtrip() {
        let original = CampaignAllocation::builder()
            .channel("search")
            .budget_share(0.4)
            .priority(1)
            .rationale("High intent channel")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: CampaignAllocation = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.channel, original.channel);
        assert_eq!(parsed.budget_share, original.budget_share);
        assert_eq!(parsed.priority, original.priority);
        assert_eq!(parsed.rationale, original.rationale);
    }

    #[test]
    fn test_campaign_brief_default_trait() {
        let brief: CampaignBrief = CampaignBrief::default();
        assert_eq!(brief.name, "");
        assert_eq!(brief.budget, 0.0);
        assert_eq!(brief.start_date, "");
        assert_eq!(brief.end_date, "");
        assert!(brief.objectives.is_empty());
        assert!(brief.channels.is_empty());
        assert!(brief.target_audience.is_none());
        assert!(brief.kpis.is_none());
        assert!(brief.constraints.is_none());
        assert!(brief.ext.is_none());
    }

    #[test]
    fn test_campaign_allocation_default_trait() {
        let alloc: CampaignAllocation = CampaignAllocation::default();
        assert_eq!(alloc.channel, "");
        assert_eq!(alloc.budget_share, 0.0);
        assert_eq!(alloc.priority, 0);
        assert!(alloc.rationale.is_none());
        assert!(alloc.ext.is_none());
    }

    #[test]
    fn test_campaign_allocation_negative_budget_share() {
        let alloc = CampaignAllocation::builder()
            .channel("test")
            .budget_share(-0.5)
            .build()
            .unwrap();
        assert_eq!(alloc.budget_share, -0.5);
    }

    #[test]
    fn test_campaign_allocation_over_one_budget_share() {
        let alloc = CampaignAllocation::builder()
            .channel("test")
            .budget_share(1.5)
            .build()
            .unwrap();
        assert_eq!(alloc.budget_share, 1.5);
    }

    #[test]
    fn test_campaign_brief_empty_name_accepted() {
        let brief = CampaignBrief::builder()
            .name("")
            .budget(100.0)
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .build()
            .unwrap();
        assert_eq!(brief.name, "");
    }

    #[test]
    fn test_campaign_brief_deserialization_missing_optional_fields() {
        let json =
            r#"{"name":"Test","budget":100.0,"start_date":"2024-01-01","end_date":"2024-12-31"}"#;
        let brief: CampaignBrief = serde_json::from_str(json).unwrap();
        assert_eq!(brief.name, "Test");
        assert!(brief.objectives.is_empty());
        assert!(brief.channels.is_empty());
        assert!(brief.target_audience.is_none());
        assert!(brief.kpis.is_none());
        assert!(brief.constraints.is_none());
    }

    #[test]
    fn test_campaign_brief_with_json_extension() {
        let brief = CampaignBriefBuilder::<serde_json::Value>::default()
            .name("Ext Test".to_string())
            .budget(1000.0)
            .start_date("2024-01-01".to_string())
            .end_date("2024-12-31".to_string())
            .ext(Some(Box::new(serde_json::json!({"custom_field": "value"}))))
            .build()
            .unwrap();

        assert!(brief.ext.is_some());
        assert_eq!(brief.ext.as_ref().unwrap()["custom_field"], "value");

        let json = serde_json::to_string(&brief).unwrap();
        assert!(json.contains("\"custom_field\":\"value\""));

        let parsed: CampaignBrief<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ext.as_ref().unwrap()["custom_field"], "value");
    }
}
