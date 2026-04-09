use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A SellerCreative represents an advertising creative (ad asset) managed by the seller.
///
/// Creatives are the actual ad materials (banners, videos, etc.) that can be assigned
/// to line items for delivery. Each creative has format and dimension information.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::SellerCreative;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let creative = SellerCreative::builder()
///     .id("creative-001".to_string())
///     .name("Summer Sale Banner".to_string())
///     .format("display".to_string())
///     .dimensions(Some("300x250".to_string()))
///     .status("active".to_string())
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SellerCreative<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the creative (REQUIRED).
    #[builder(default)]
    pub id: String,

    /// Human-readable name of the creative (REQUIRED).
    #[builder(default)]
    pub name: String,

    /// Format of the creative, e.g., "display", "video", "native" (REQUIRED).
    #[builder(default)]
    pub format: String,

    /// Dimensions of the creative, e.g., "300x250" or "1920x1080".
    /// Optional for some formats like video or native.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dimensions: Option<String>,

    /// Status of the creative, e.g., "active", "inactive", "pending_review" (REQUIRED).
    #[builder(default)]
    pub status: String,

    /// Extension object for creative-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SellerCreative {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SellerCreativeBuilder {
        SellerCreativeBuilder::create_empty()
    }
}

/// A SellerAssignment associates a creative with a line item.
///
/// Assignments determine which creatives are used in which line items, with optional
/// rotation weights for frequency distribution. This enables managing creative rotation
/// and allocation across multiple ads in a campaign.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs::seller_agent::v10::models::SellerAssignment;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let assignment = SellerAssignment::builder()
///     .creative_id("creative-001".to_string())
///     .line_id("line-001".to_string())
///     .rotation_weight(Some(1.5))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct SellerAssignment<Ext: Extension = crate::DefaultExt> {
    /// Identifier of the creative being assigned (REQUIRED).
    #[builder(default)]
    pub creative_id: String,

    /// Identifier of the line item receiving the creative (REQUIRED).
    #[builder(default)]
    pub line_id: String,

    /// Rotation weight for this creative in the line item.
    /// Higher weights increase the frequency of this creative being shown.
    /// If not specified, defaults to equal rotation with other assigned creatives.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rotation_weight: Option<f64>,

    /// Extension object for assignment-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl SellerAssignment {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> SellerAssignmentBuilder {
        SellerAssignmentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== SellerCreative Tests ==========

    #[test]
    fn test_seller_creative_minimal() {
        let creative = SellerCreative::builder()
            .id("creative-001".to_string())
            .name("Summer Sale Banner".to_string())
            .format("display".to_string())
            .status("active".to_string())
            .build()
            .unwrap();

        assert_eq!(creative.id, "creative-001");
        assert_eq!(creative.name, "Summer Sale Banner");
        assert_eq!(creative.format, "display");
        assert_eq!(creative.status, "active");
        assert!(creative.dimensions.is_none());
        assert!(creative.ext.is_none());
    }

    #[test]
    fn test_seller_creative_with_dimensions() {
        let creative = SellerCreative::builder()
            .id("creative-002".to_string())
            .name("Leaderboard Ad".to_string())
            .format("display".to_string())
            .dimensions(Some("728x90".to_string()))
            .status("active".to_string())
            .build()
            .unwrap();

        assert_eq!(creative.id, "creative-002");
        assert_eq!(creative.dimensions, Some("728x90".to_string()));
    }

    #[test]
    fn test_seller_creative_roundtrip() {
        let original = SellerCreative::builder()
            .id("creative-003".to_string())
            .name("Video Bumper".to_string())
            .format("video".to_string())
            .dimensions(Some("1920x1080".to_string()))
            .status("pending_review".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: SellerCreative = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.format, original.format);
        assert_eq!(parsed.dimensions, original.dimensions);
        assert_eq!(parsed.status, original.status);
    }

    #[test]
    fn test_seller_creative_status_values() {
        let statuses = vec!["active", "inactive", "pending_review", "archived"];
        for status in statuses {
            let creative = SellerCreative::builder()
                .id("creative-test".to_string())
                .name("Test".to_string())
                .format("display".to_string())
                .status(status.to_string())
                .build()
                .unwrap();

            assert_eq!(creative.status, status);
        }
    }

    #[test]
    fn test_seller_creative_format_types() {
        let formats = vec!["display", "video", "native", "audio"];
        for format in formats {
            let creative = SellerCreative::builder()
                .id("creative-test".to_string())
                .name("Test".to_string())
                .format(format.to_string())
                .status("active".to_string())
                .build()
                .unwrap();

            assert_eq!(creative.format, format);
        }
    }

    #[test]
    fn test_seller_creative_serialization() {
        let creative = SellerCreative::builder()
            .id("creative-004".to_string())
            .name("Test Banner".to_string())
            .format("display".to_string())
            .dimensions(Some("300x250".to_string()))
            .status("active".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&creative).unwrap();
        assert!(json.contains("\"id\":\"creative-004\""));
        assert!(json.contains("\"name\":\"Test Banner\""));
        assert!(json.contains("\"format\":\"display\""));
        assert!(json.contains("\"dimensions\":\"300x250\""));
        assert!(json.contains("\"status\":\"active\""));
    }

    #[test]
    fn test_seller_creative_deserialization_without_dimensions() {
        let json = r#"{
            "id": "creative-005",
            "name": "Minimal Creative",
            "format": "native",
            "status": "inactive"
        }"#;

        let creative: SellerCreative = serde_json::from_str(json).unwrap();
        assert_eq!(creative.id, "creative-005");
        assert_eq!(creative.name, "Minimal Creative");
        assert_eq!(creative.format, "native");
        assert_eq!(creative.status, "inactive");
        assert!(creative.dimensions.is_none());
    }

    // ========== SellerAssignment Tests ==========

    #[test]
    fn test_seller_assignment_minimal() {
        let assignment = SellerAssignment::builder()
            .creative_id("creative-001".to_string())
            .line_id("line-001".to_string())
            .build()
            .unwrap();

        assert_eq!(assignment.creative_id, "creative-001");
        assert_eq!(assignment.line_id, "line-001");
        assert!(assignment.rotation_weight.is_none());
        assert!(assignment.ext.is_none());
    }

    #[test]
    fn test_seller_assignment_with_rotation_weight() {
        let assignment = SellerAssignment::builder()
            .creative_id("creative-002".to_string())
            .line_id("line-001".to_string())
            .rotation_weight(Some(1.5))
            .build()
            .unwrap();

        assert_eq!(assignment.creative_id, "creative-002");
        assert_eq!(assignment.line_id, "line-001");
        assert_eq!(assignment.rotation_weight, Some(1.5));
    }

    #[test]
    fn test_seller_assignment_rotation_weight_zero() {
        let assignment = SellerAssignment::builder()
            .creative_id("creative-003".to_string())
            .line_id("line-002".to_string())
            .rotation_weight(Some(0.0))
            .build()
            .unwrap();

        assert_eq!(assignment.rotation_weight, Some(0.0));
    }

    #[test]
    fn test_seller_assignment_rotation_weight_high_precision() {
        let assignment = SellerAssignment::builder()
            .creative_id("creative-004".to_string())
            .line_id("line-003".to_string())
            .rotation_weight(Some(2.75555))
            .build()
            .unwrap();

        assert_eq!(assignment.rotation_weight, Some(2.75555));
    }

    #[test]
    fn test_seller_assignment_roundtrip() {
        let original = SellerAssignment::builder()
            .creative_id("creative-005".to_string())
            .line_id("line-004".to_string())
            .rotation_weight(Some(3.0))
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: SellerAssignment = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.creative_id, original.creative_id);
        assert_eq!(parsed.line_id, original.line_id);
        assert_eq!(parsed.rotation_weight, original.rotation_weight);
    }

    #[test]
    fn test_seller_assignment_multiple_creatives_same_line() {
        let assignment1 = SellerAssignment::builder()
            .creative_id("creative-006".to_string())
            .line_id("line-005".to_string())
            .rotation_weight(Some(2.0))
            .build()
            .unwrap();

        let assignment2 = SellerAssignment::builder()
            .creative_id("creative-007".to_string())
            .line_id("line-005".to_string())
            .rotation_weight(Some(1.0))
            .build()
            .unwrap();

        assert_eq!(assignment1.line_id, assignment2.line_id);
        assert_ne!(assignment1.creative_id, assignment2.creative_id);
        assert_eq!(assignment1.rotation_weight, Some(2.0));
        assert_eq!(assignment2.rotation_weight, Some(1.0));
    }

    #[test]
    fn test_seller_assignment_serialization() {
        let assignment = SellerAssignment::builder()
            .creative_id("creative-008".to_string())
            .line_id("line-006".to_string())
            .rotation_weight(Some(1.25))
            .build()
            .unwrap();

        let json = serde_json::to_string(&assignment).unwrap();
        assert!(json.contains("\"creative_id\":\"creative-008\""));
        assert!(json.contains("\"line_id\":\"line-006\""));
        assert!(json.contains("\"rotation_weight\":1.25"));
    }

    #[test]
    fn test_seller_assignment_deserialization_without_rotation_weight() {
        let json = r#"{
            "creative_id": "creative-009",
            "line_id": "line-007"
        }"#;

        let assignment: SellerAssignment = serde_json::from_str(json).unwrap();
        assert_eq!(assignment.creative_id, "creative-009");
        assert_eq!(assignment.line_id, "line-007");
        assert!(assignment.rotation_weight.is_none());
    }

    #[test]
    fn test_seller_assignment_deserialization_with_rotation_weight() {
        let json = r#"{
            "creative_id": "creative-010",
            "line_id": "line-008",
            "rotation_weight": 2.5
        }"#;

        let assignment: SellerAssignment = serde_json::from_str(json).unwrap();
        assert_eq!(assignment.creative_id, "creative-010");
        assert_eq!(assignment.line_id, "line-008");
        assert_eq!(assignment.rotation_weight, Some(2.5));
    }

    #[test]
    fn test_seller_assignment_roundtrip_without_rotation_weight() {
        let original = SellerAssignment::builder()
            .creative_id("creative-011".to_string())
            .line_id("line-009".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: SellerAssignment = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.creative_id, original.creative_id);
        assert_eq!(parsed.line_id, original.line_id);
        assert!(parsed.rotation_weight.is_none());
    }

    /// Seller Agent 1.0 § SellerCreative — default builder yields empty creative
    #[test]
    fn test_seller_creative_default() {
        let creative = SellerCreative::builder().build().unwrap();
        assert_eq!(creative.id, "");
        assert_eq!(creative.name, "");
        assert_eq!(creative.format, "");
        assert_eq!(creative.status, "");
        assert!(creative.dimensions.is_none());
        assert!(creative.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerCreative — optional fields omitted from JSON when None
    #[test]
    fn test_seller_creative_optional_fields_skipped() {
        let creative = SellerCreative::builder()
            .id("c1".to_string())
            .name("n".to_string())
            .format("display".to_string())
            .status("active".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&creative).unwrap();
        assert!(!json.contains("\"dimensions\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § SellerCreative — clone produces identical value
    #[test]
    fn test_seller_creative_clone() {
        let creative = SellerCreative::builder()
            .id("c-clone".to_string())
            .name("Clone Banner".to_string())
            .format("display".to_string())
            .dimensions(Some("300x250".to_string()))
            .status("active".to_string())
            .build()
            .unwrap();

        let cloned = creative.clone();
        assert_eq!(creative, cloned);
    }

    /// Seller Agent 1.0 § SellerCreative — deserialization from minimal JSON
    #[test]
    fn test_seller_creative_deserialization_minimal() {
        let json = r#"{"id":"c1","name":"n","format":"display","status":"active"}"#;
        let creative: SellerCreative = serde_json::from_str(json).unwrap();
        assert_eq!(creative.id, "c1");
        assert_eq!(creative.name, "n");
        assert_eq!(creative.format, "display");
        assert_eq!(creative.status, "active");
        assert!(creative.dimensions.is_none());
    }

    /// Seller Agent 1.0 § SellerAssignment — default builder yields empty assignment
    #[test]
    fn test_seller_assignment_default() {
        let assignment = SellerAssignment::builder().build().unwrap();
        assert_eq!(assignment.creative_id, "");
        assert_eq!(assignment.line_id, "");
        assert!(assignment.rotation_weight.is_none());
        assert!(assignment.ext.is_none());
    }

    /// Seller Agent 1.0 § SellerAssignment — optional fields omitted from JSON when None
    #[test]
    fn test_seller_assignment_optional_fields_skipped() {
        let assignment = SellerAssignment::builder()
            .creative_id("c1".to_string())
            .line_id("l1".to_string())
            .build()
            .unwrap();

        let json = serde_json::to_string(&assignment).unwrap();
        assert!(!json.contains("\"rotation_weight\""));
        assert!(!json.contains("\"ext\""));
    }

    /// Seller Agent 1.0 § SellerAssignment — clone produces identical value
    #[test]
    fn test_seller_assignment_clone() {
        let assignment = SellerAssignment::builder()
            .creative_id("c-clone".to_string())
            .line_id("l-clone".to_string())
            .rotation_weight(Some(2.5))
            .build()
            .unwrap();

        let cloned = assignment.clone();
        assert_eq!(assignment, cloned);
    }

    /// Seller Agent 1.0 § SellerAssignment — deserialization from minimal JSON
    #[test]
    fn test_seller_assignment_deserialization_minimal() {
        let json = r#"{"creative_id":"c1","line_id":"l1"}"#;
        let assignment: SellerAssignment = serde_json::from_str(json).unwrap();
        assert_eq!(assignment.creative_id, "c1");
        assert_eq!(assignment.line_id, "l1");
        assert!(assignment.rotation_weight.is_none());
    }
}
