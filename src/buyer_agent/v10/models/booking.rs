use crate::Extension;
use crate::buyer_agent::v10::enums::CampaignStatus;
use crate::buyer_agent::v10::models::campaign::CampaignAllocation;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A BookingJob represents a request to book advertising inventory for a campaign.
///
/// Booking jobs track the status of inventory bookings, including allocations across channels,
/// recommendations from the research phase, and approval status. Each booking job is associated
/// with a campaign brief and drives execution of the campaign's advertising plan.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::BookingJob;
/// use iab_specs::buyer_agent::v10::enums::CampaignStatus;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let booking = BookingJob::builder()
///     .campaign_brief_id("brief-123")
///     .status(CampaignStatus::Researching)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct BookingJob<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the booking job.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// ID of the associated campaign brief (REQUIRED).
    #[builder(setter(into))]
    pub campaign_brief_id: String,

    /// Current status of the booking job (defaults to Initialized).
    #[serde(default)]
    #[builder(default)]
    pub status: CampaignStatus,

    /// List of budget allocations across channels for this booking job.
    #[serde(default)]
    #[builder(default)]
    pub allocations: Vec<CampaignAllocation<Ext>>,

    /// List of recommendations from research phase (arbitrary JSON blobs).
    #[serde(default)]
    #[builder(default)]
    pub recommendations: Vec<serde_json::Value>,

    /// Whether the booking job has been approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub approved: Option<bool>,

    /// User or system that approved the booking job.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub approved_by: Option<String>,

    /// Timestamp of approval in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub approved_at: Option<String>,

    /// Extension object for booking job-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl BookingJob {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BookingJobBuilder {
        BookingJobBuilder::create_empty()
    }
}

/// A BookingRecommendation represents a specific recommendation to book inventory from a seller.
///
/// Recommendations include the seller, product/inventory details, pricing, and expected impressions.
/// These recommendations are typically generated during the research phase and included in booking jobs.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::buyer_agent::v10::models::BookingRecommendation;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let recommendation = BookingRecommendation::builder()
///     .seller_name("Premium Exchange")
///     .product_id("prod-456")
///     .price(2.50)
///     .impressions(100000)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct BookingRecommendation<Ext: Extension = crate::DefaultExt> {
    /// Name of the seller providing the inventory (REQUIRED).
    #[builder(setter(into))]
    pub seller_name: String,

    /// Product or inventory ID being recommended (REQUIRED).
    #[builder(setter(into))]
    pub product_id: String,

    /// Recommended price in currency units (REQUIRED).
    #[builder(default)]
    pub price: f64,

    /// Expected number of impressions (REQUIRED).
    #[builder(default)]
    pub impressions: i64,

    /// Explanation for this recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub rationale: Option<String>,

    /// Channel or inventory type (e.g., "display", "video", "native").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub channel: Option<String>,

    /// Extension object for recommendation-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl BookingRecommendation {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> BookingRecommendationBuilder {
        BookingRecommendationBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== BookingJob Tests ==========

    #[test]
    fn test_booking_job_minimal() {
        let job = BookingJob::builder()
            .campaign_brief_id("brief-123")
            .build()
            .unwrap();

        assert_eq!(job.campaign_brief_id, "brief-123");
        assert_eq!(job.status, CampaignStatus::Initialized);
        assert_eq!(job.allocations.len(), 0);
        assert_eq!(job.recommendations.len(), 0);
        assert!(job.approved.is_none());
        assert!(job.approved_by.is_none());
        assert!(job.approved_at.is_none());
        assert!(job.ext.is_none());
    }

    #[test]
    fn test_booking_job_with_id() {
        let job = BookingJob::builder()
            .id("job-456")
            .campaign_brief_id("brief-123")
            .build()
            .unwrap();

        assert_eq!(job.id, Some("job-456".to_string()));
        assert_eq!(job.campaign_brief_id, "brief-123");
    }

    #[test]
    fn test_booking_job_with_status_researching() {
        let job = BookingJob::builder()
            .campaign_brief_id("brief-789")
            .status(CampaignStatus::Researching)
            .allocations(vec![
                CampaignAllocation::builder()
                    .channel("display")
                    .budget_share(0.6)
                    .build()
                    .unwrap(),
                CampaignAllocation::builder()
                    .channel("video")
                    .budget_share(0.4)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(job.status, CampaignStatus::Researching);
        assert_eq!(job.allocations.len(), 2);
        assert_eq!(job.allocations[0].channel, "display");
        assert_eq!(job.allocations[1].channel, "video");
    }

    #[test]
    fn test_booking_job_with_recommendations() {
        let job = BookingJob::builder()
            .campaign_brief_id("brief-999")
            .recommendations(vec![
                serde_json::json!({
                    "seller": "exchange1",
                    "inventory_type": "standard_display"
                }),
                serde_json::json!({
                    "seller": "exchange2",
                    "inventory_type": "premium_video"
                }),
            ])
            .build()
            .unwrap();

        assert_eq!(job.recommendations.len(), 2);
        assert!(job.recommendations[0].is_object());
        assert!(job.recommendations[1].is_object());
    }

    #[test]
    fn test_booking_job_with_approval() {
        let job = BookingJob::builder()
            .campaign_brief_id("brief-111")
            .approved(Some(true))
            .approved_by("user@example.com")
            .approved_at("2024-04-01T10:30:00Z")
            .build()
            .unwrap();

        assert_eq!(job.approved, Some(true));
        assert_eq!(job.approved_by, Some("user@example.com".to_string()));
        assert_eq!(job.approved_at, Some("2024-04-01T10:30:00Z".to_string()));
    }

    #[test]
    fn test_booking_job_full() {
        let job = BookingJob::builder()
            .id("job-full-001")
            .campaign_brief_id("brief-full")
            .status(CampaignStatus::ExecutingBookings)
            .allocations(vec![
                CampaignAllocation::builder()
                    .channel("display")
                    .budget_share(0.5)
                    .rationale("High volume channel")
                    .build()
                    .unwrap(),
            ])
            .recommendations(vec![
                serde_json::json!({"seller": "exchange1", "price": 2.50}),
            ])
            .approved(Some(true))
            .approved_by("manager")
            .approved_at("2024-04-01T09:00:00Z")
            .build()
            .unwrap();

        assert_eq!(job.id, Some("job-full-001".to_string()));
        assert_eq!(job.campaign_brief_id, "brief-full");
        assert_eq!(job.status, CampaignStatus::ExecutingBookings);
        assert_eq!(job.allocations.len(), 1);
        assert_eq!(job.recommendations.len(), 1);
        assert_eq!(job.approved, Some(true));
        assert_eq!(job.approved_by, Some("manager".to_string()));
    }

    #[test]
    fn test_booking_job_serialization() {
        let job = BookingJob::builder()
            .id("job-ser-001")
            .campaign_brief_id("brief-ser")
            .status(CampaignStatus::Researching)
            .build()
            .unwrap();

        let json = serde_json::to_string(&job).unwrap();
        assert!(json.contains("\"campaign_brief_id\":\"brief-ser\""));
        assert!(json.contains("\"researching\""));
    }

    #[test]
    fn test_booking_job_status_serialization_snake_case() {
        let job = BookingJob::builder()
            .campaign_brief_id("brief-snake")
            .status(CampaignStatus::Researching)
            .build()
            .unwrap();

        let json = serde_json::to_string(&job).unwrap();
        // Verify status is serialized as snake_case
        assert!(json.contains("\"status\":\"researching\""));
        assert!(!json.contains("Researching"));
    }

    #[test]
    fn test_booking_job_roundtrip() {
        let original = BookingJob::builder()
            .id("job-rt-001")
            .campaign_brief_id("brief-rt")
            .status(CampaignStatus::AwaitingApproval)
            .allocations(vec![
                CampaignAllocation::builder()
                    .channel("email")
                    .budget_share(0.7)
                    .build()
                    .unwrap(),
            ])
            .recommendations(vec![serde_json::json!({"cost_per_impression": 0.005})])
            .approved(Some(false))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: BookingJob = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.campaign_brief_id, original.campaign_brief_id);
        assert_eq!(parsed.status, original.status);
        assert_eq!(parsed.allocations.len(), original.allocations.len());
        assert_eq!(parsed.recommendations.len(), original.recommendations.len());
        assert_eq!(parsed.approved, original.approved);
    }

    // ========== BookingRecommendation Tests ==========

    #[test]
    fn test_booking_recommendation_minimal() {
        let rec = BookingRecommendation::builder()
            .seller_name("Exchange A")
            .product_id("prod-123")
            .price(2.50)
            .impressions(100000)
            .build()
            .unwrap();

        assert_eq!(rec.seller_name, "Exchange A");
        assert_eq!(rec.product_id, "prod-123");
        assert_eq!(rec.price, 2.50);
        assert_eq!(rec.impressions, 100000);
        assert!(rec.rationale.is_none());
        assert!(rec.channel.is_none());
        assert!(rec.ext.is_none());
    }

    #[test]
    fn test_booking_recommendation_with_channel() {
        let rec = BookingRecommendation::builder()
            .seller_name("Video Network")
            .product_id("video-pkg-456")
            .price(5.00)
            .impressions(50000)
            .channel("video")
            .build()
            .unwrap();

        assert_eq!(rec.seller_name, "Video Network");
        assert_eq!(rec.channel, Some("video".to_string()));
    }

    #[test]
    fn test_booking_recommendation_with_rationale() {
        let rec = BookingRecommendation::builder()
            .seller_name("Native Network")
            .product_id("native-123")
            .price(3.75)
            .impressions(75000)
            .rationale("High engagement rate, good audience match")
            .channel("native")
            .build()
            .unwrap();

        assert_eq!(
            rec.rationale,
            Some("High engagement rate, good audience match".to_string())
        );
        assert_eq!(rec.channel, Some("native".to_string()));
    }

    #[test]
    fn test_booking_recommendation_full() {
        let rec = BookingRecommendation::builder()
            .seller_name("Premium Exchange")
            .product_id("premium-display-789")
            .price(4.25)
            .impressions(200000)
            .rationale("Premium inventory with viewability guarantees")
            .channel("display")
            .build()
            .unwrap();

        assert_eq!(rec.seller_name, "Premium Exchange");
        assert_eq!(rec.product_id, "premium-display-789");
        assert_eq!(rec.price, 4.25);
        assert_eq!(rec.impressions, 200000);
        assert_eq!(
            rec.rationale,
            Some("Premium inventory with viewability guarantees".to_string())
        );
        assert_eq!(rec.channel, Some("display".to_string()));
    }

    #[test]
    fn test_booking_recommendation_large_impressions() {
        let rec = BookingRecommendation::builder()
            .seller_name("Mega Network")
            .product_id("mega-scale")
            .price(0.50)
            .impressions(10000000)
            .build()
            .unwrap();

        assert_eq!(rec.impressions, 10000000);
    }

    #[test]
    fn test_booking_recommendation_zero_price() {
        let rec = BookingRecommendation::builder()
            .seller_name("Trial Exchange")
            .product_id("trial-001")
            .price(0.0)
            .impressions(1000)
            .build()
            .unwrap();

        assert_eq!(rec.price, 0.0);
    }

    #[test]
    fn test_booking_recommendation_serialization() {
        let rec = BookingRecommendation::builder()
            .seller_name("Serializable Exchange")
            .product_id("ser-prod-001")
            .price(3.50)
            .impressions(150000)
            .channel("display")
            .build()
            .unwrap();

        let json = serde_json::to_string(&rec).unwrap();
        assert!(json.contains("\"seller_name\":\"Serializable Exchange\""));
        assert!(json.contains("\"product_id\":\"ser-prod-001\""));
        assert!(json.contains("\"price\":3.5"));
        assert!(json.contains("\"impressions\":150000"));
        assert!(json.contains("\"channel\":\"display\""));
    }

    #[test]
    fn test_booking_recommendation_deserialization() {
        let json = r#"{
            "seller_name": "Deserialized Exchange",
            "product_id": "deser-001",
            "price": 2.75,
            "impressions": 100000,
            "channel": "video",
            "rationale": "Good performance history"
        }"#;

        let rec: BookingRecommendation = serde_json::from_str(json).unwrap();
        assert_eq!(rec.seller_name, "Deserialized Exchange");
        assert_eq!(rec.product_id, "deser-001");
        assert_eq!(rec.price, 2.75);
        assert_eq!(rec.impressions, 100000);
        assert_eq!(rec.channel, Some("video".to_string()));
        assert_eq!(rec.rationale, Some("Good performance history".to_string()));
    }

    #[test]
    fn test_booking_recommendation_roundtrip() {
        let original = BookingRecommendation::builder()
            .seller_name("Roundtrip Exchange")
            .product_id("rt-prod-002")
            .price(3.25)
            .impressions(250000)
            .rationale("Roundtrip test")
            .channel("native")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: BookingRecommendation = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.seller_name, original.seller_name);
        assert_eq!(parsed.product_id, original.product_id);
        assert_eq!(parsed.price, original.price);
        assert_eq!(parsed.impressions, original.impressions);
        assert_eq!(parsed.rationale, original.rationale);
        assert_eq!(parsed.channel, original.channel);
    }
}
