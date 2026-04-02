use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A DspIntegration represents the distribution of a deal to a specific DSP.
///
/// This tracks how a deal is sent to external demand-side platforms (DSPs),
/// including the DSP name, seat ID, and current distribution status.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::DspIntegration;
/// use iab_specs::seller_agent::v10::enums::DistributionStatus;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let dsp = DspIntegration::builder()
///     .dsp_name("PubMatic")
///     .seat_id("seat-123")
///     .status(DistributionStatus::Sent)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DspIntegration<Ext: Extension = crate::DefaultExt> {
    /// The DSP platform name (REQUIRED).
    /// Examples: "PubMatic", "IndexExchange", "Rubicon"
    #[builder(default, setter(into))]
    pub dsp_name: String,

    /// The seat ID at the DSP (REQUIRED).
    /// The DSP's identifier for this seller's account.
    #[builder(default, setter(into))]
    pub seat_id: String,

    /// The deal ID as known at the DSP platform (OPTIONAL).
    /// May differ from the original deal_id due to DSP-specific mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub deal_id_at_dsp: Option<String>,

    /// The current distribution status (REQUIRED).
    /// Tracks whether the deal has been sent, confirmed, or rejected.
    #[builder(default)]
    pub status: crate::seller_agent::v10::enums::DistributionStatus,

    /// Extension object for DSP-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl DspIntegration {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DspIntegrationBuilder {
        DspIntegrationBuilder::create_empty()
    }
}

/// A DealDistribution represents the distribution of a deal across multiple DSPs.
///
/// This tracks how a specific deal is sent to various demand-side platforms (DSPs)
/// and includes the list of buyer seats and DSP integrations.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::{DealDistribution, DspIntegration};
/// use iab_specs::seller_agent::v10::enums::DistributionStatus;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let distribution = DealDistribution::builder()
///     .deal_id("deal-456")
///     .buyer_seats(vec!["seat-001".to_string(), "seat-002".to_string()])
///     .dsp_integrations(vec![
///         DspIntegration::builder()
///             .dsp_name("PubMatic")
///             .seat_id("pub-seat-1")
///             .status(DistributionStatus::Sent)
///             .build()?,
///     ])
///     .distributed_at(Some("2026-04-01T10:30:00Z".to_string()))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DealDistribution<Ext: Extension = crate::DefaultExt> {
    /// The deal ID being distributed (REQUIRED).
    /// Unique identifier for the deal across the platform.
    #[builder(default, setter(into))]
    pub deal_id: String,

    /// List of buyer seat IDs eligible for this deal.
    /// Empty by default; seller can specify which buyers receive this deal.
    #[builder(default)]
    pub buyer_seats: Vec<String>,

    /// List of DSP integrations for this deal.
    /// Tracks distribution to each external DSP platform.
    #[builder(default)]
    pub dsp_integrations: Vec<DspIntegration<Ext>>,

    /// Timestamp when the deal was distributed (OPTIONAL).
    /// ISO 8601 format: "2026-04-01T10:30:00Z"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub distributed_at: Option<String>,

    /// Extension object for distribution-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl DealDistribution {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DealDistributionBuilder {
        DealDistributionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seller_agent::v10::enums::DistributionStatus;

    // ========== DspIntegration Tests ==========

    #[test]
    fn test_dsp_integration_minimal() {
        let dsp = DspIntegration::builder()
            .dsp_name("PubMatic")
            .seat_id("seat-123")
            .build()
            .unwrap();

        assert_eq!(dsp.dsp_name, "PubMatic");
        assert_eq!(dsp.seat_id, "seat-123");
        assert_eq!(dsp.status, DistributionStatus::Pending);
        assert!(dsp.deal_id_at_dsp.is_none());
        assert!(dsp.ext.is_none());
    }

    #[test]
    fn test_dsp_integration_full() {
        let dsp = DspIntegration::builder()
            .dsp_name("IndexExchange")
            .seat_id("seat-456")
            .deal_id_at_dsp(Some("dsp-deal-789".to_string()))
            .status(DistributionStatus::Sent)
            .build()
            .unwrap();

        assert_eq!(dsp.dsp_name, "IndexExchange");
        assert_eq!(dsp.seat_id, "seat-456");
        assert_eq!(dsp.deal_id_at_dsp, Some("dsp-deal-789".to_string()));
        assert_eq!(dsp.status, DistributionStatus::Sent);
    }

    #[test]
    fn test_dsp_integration_status_transitions() {
        let statuses = vec![
            DistributionStatus::Pending,
            DistributionStatus::Sent,
            DistributionStatus::Confirmed,
            DistributionStatus::Rejected,
            DistributionStatus::Expired,
        ];

        for status in statuses {
            let dsp = DspIntegration::builder()
                .dsp_name("TestDSP")
                .seat_id("test-seat")
                .status(status)
                .build()
                .unwrap();

            assert_eq!(dsp.status, status);
        }
    }

    #[test]
    fn test_dsp_integration_roundtrip() {
        let original = DspIntegration::builder()
            .dsp_name("PubMatic")
            .seat_id("pub-seat-100")
            .deal_id_at_dsp(Some("dsp-id-xyz".to_string()))
            .status(DistributionStatus::Confirmed)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: DspIntegration = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.dsp_name, "PubMatic");
        assert_eq!(parsed.seat_id, "pub-seat-100");
        assert_eq!(parsed.deal_id_at_dsp, Some("dsp-id-xyz".to_string()));
        assert_eq!(parsed.status, DistributionStatus::Confirmed);
    }

    #[test]
    fn test_dsp_integration_serialization_excludes_none() {
        let dsp = DspIntegration::builder()
            .dsp_name("TestDSP")
            .seat_id("test-seat")
            .build()
            .unwrap();

        let json = serde_json::to_string(&dsp).unwrap();
        assert!(!json.contains("deal_id_at_dsp"));
        assert!(!json.contains("null"));
    }

    // ========== DealDistribution Tests ==========

    #[test]
    fn test_deal_distribution_minimal() {
        let dist = DealDistribution::builder()
            .deal_id("deal-001")
            .build()
            .unwrap();

        assert_eq!(dist.deal_id, "deal-001");
        assert!(dist.buyer_seats.is_empty());
        assert!(dist.dsp_integrations.is_empty());
        assert!(dist.distributed_at.is_none());
        assert!(dist.ext.is_none());
    }

    #[test]
    fn test_deal_distribution_with_seats() {
        let dist = DealDistribution::builder()
            .deal_id("deal-002")
            .buyer_seats(vec!["seat-a".to_string(), "seat-b".to_string()])
            .build()
            .unwrap();

        assert_eq!(dist.deal_id, "deal-002");
        assert_eq!(dist.buyer_seats.len(), 2);
        assert!(dist.buyer_seats.contains(&"seat-a".to_string()));
        assert!(dist.buyer_seats.contains(&"seat-b".to_string()));
    }

    #[test]
    fn test_deal_distribution_with_two_dsps() {
        let pubmatic = DspIntegration::builder()
            .dsp_name("PubMatic")
            .seat_id("pm-seat")
            .status(DistributionStatus::Sent)
            .build()
            .unwrap();

        let index = DspIntegration::builder()
            .dsp_name("IndexExchange")
            .seat_id("ix-seat")
            .status(DistributionStatus::Confirmed)
            .build()
            .unwrap();

        let dist = DealDistribution::builder()
            .deal_id("deal-003")
            .dsp_integrations(vec![pubmatic, index])
            .build()
            .unwrap();

        assert_eq!(dist.deal_id, "deal-003");
        assert_eq!(dist.dsp_integrations.len(), 2);
        assert_eq!(dist.dsp_integrations[0].dsp_name, "PubMatic");
        assert_eq!(dist.dsp_integrations[1].dsp_name, "IndexExchange");
    }

    #[test]
    fn test_deal_distribution_full() {
        let dist = DealDistribution::builder()
            .deal_id("deal-004")
            .buyer_seats(vec!["buyer-1".to_string()])
            .dsp_integrations(vec![
                DspIntegration::builder()
                    .dsp_name("PubMatic")
                    .seat_id("pm-seat")
                    .deal_id_at_dsp(Some("pm-deal-100".to_string()))
                    .status(DistributionStatus::Sent)
                    .build()
                    .unwrap(),
            ])
            .distributed_at(Some("2026-04-01T10:30:00Z".to_string()))
            .build()
            .unwrap();

        assert_eq!(dist.deal_id, "deal-004");
        assert_eq!(dist.buyer_seats.len(), 1);
        assert_eq!(dist.dsp_integrations.len(), 1);
        assert_eq!(
            dist.distributed_at,
            Some("2026-04-01T10:30:00Z".to_string())
        );
    }

    #[test]
    fn test_deal_distribution_nested_roundtrip() {
        let original = DealDistribution::builder()
            .deal_id("deal-005")
            .buyer_seats(vec!["seat-x".to_string(), "seat-y".to_string()])
            .dsp_integrations(vec![
                DspIntegration::builder()
                    .dsp_name("PubMatic")
                    .seat_id("pm-seat-1")
                    .deal_id_at_dsp(Some("pm-123".to_string()))
                    .status(DistributionStatus::Confirmed)
                    .build()
                    .unwrap(),
                DspIntegration::builder()
                    .dsp_name("IndexExchange")
                    .seat_id("ix-seat-1")
                    .status(DistributionStatus::Sent)
                    .build()
                    .unwrap(),
            ])
            .distributed_at(Some("2026-03-15T14:22:00Z".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: DealDistribution = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.deal_id, "deal-005");
        assert_eq!(parsed.buyer_seats.len(), 2);
        assert_eq!(parsed.dsp_integrations.len(), 2);
        assert_eq!(parsed.dsp_integrations[0].dsp_name, "PubMatic");
        assert_eq!(parsed.dsp_integrations[1].dsp_name, "IndexExchange");
        assert_eq!(
            parsed.distributed_at,
            Some("2026-03-15T14:22:00Z".to_string())
        );
    }

    #[test]
    fn test_deal_distribution_default() {
        let dist: DealDistribution = DealDistribution::default();

        assert_eq!(dist.deal_id, "");
        assert!(dist.buyer_seats.is_empty());
        assert!(dist.dsp_integrations.is_empty());
        assert!(dist.distributed_at.is_none());
        assert!(dist.ext.is_none());
    }

    #[test]
    fn test_deal_distribution_serialization() {
        let dist = DealDistribution::builder()
            .deal_id("deal-serialize")
            .distributed_at(Some("2026-02-01T00:00:00Z".to_string()))
            .build()
            .unwrap();

        let json = serde_json::to_string(&dist).unwrap();
        assert!(json.contains("\"deal_id\":\"deal-serialize\""));
        assert!(json.contains("\"distributed_at\":\"2026-02-01T00:00:00Z\""));
    }
}
