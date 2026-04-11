use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::AssignmentStatus;

/// Assignment entity.
///
/// Represents the assignment of a creative to a line item, with optional weight
/// and date range overrides for scheduling flexibility.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct Assignment<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<String>,

    /// Creative identifier (required).
    #[builder(setter(into))]
    pub creative_id: String,

    /// Line identifier (required).
    #[builder(setter(into))]
    pub line_id: String,

    /// Current status of the assignment (required).
    pub status: AssignmentStatus,

    /// Weight for creative rotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub weight: Option<f64>,

    /// Start date override for the assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub start_date: Option<String>,

    /// End date override for the assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub end_date: Option<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl Assignment {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> AssignmentBuilder {
        AssignmentBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_creation() {
        let assignment = Assignment::builder()
            .creative_id("creative-001")
            .line_id("line-001")
            .status(AssignmentStatus::Draft)
            .build()
            .unwrap();

        assert_eq!(assignment.creative_id, "creative-001");
        assert_eq!(assignment.line_id, "line-001");
        assert_eq!(assignment.status, AssignmentStatus::Draft);
        assert!(assignment.id.is_none());
        assert!(assignment.weight.is_none());
        assert!(assignment.start_date.is_none());
        assert!(assignment.end_date.is_none());
    }

    #[test]
    fn test_assignment_serialization() {
        let assignment = Assignment::builder()
            .creative_id("creative-002")
            .line_id("line-002")
            .status(AssignmentStatus::Active)
            .build()
            .unwrap();

        let json = serde_json::to_string(&assignment).unwrap();
        assert!(json.contains("\"creative_id\":\"creative-002\""));
        assert!(json.contains("\"status\":\"active\""));
    }

    #[test]
    fn test_assignment_deserialization() {
        let json = r#"{"creative_id":"creative-003","line_id":"line-003","status":"paused"}"#;
        let assignment: Assignment = serde_json::from_str(json).unwrap();

        assert_eq!(assignment.creative_id, "creative-003");
        assert_eq!(assignment.line_id, "line-003");
        assert_eq!(assignment.status, AssignmentStatus::Paused);
    }

    #[test]
    fn test_assignment_roundtrip() {
        let assignment = Assignment::builder()
            .creative_id("creative-004")
            .line_id("line-004")
            .status(AssignmentStatus::Active)
            .weight(Some(0.75))
            .start_date("2025-01-01")
            .end_date("2025-06-30")
            .build()
            .unwrap();

        let json = serde_json::to_string(&assignment).unwrap();
        let parsed: Assignment = serde_json::from_str(&json).unwrap();
        assert_eq!(assignment, parsed);
    }

    #[test]
    fn test_assignment_default() {
        let assignment = Assignment::builder()
            .creative_id("creative-005")
            .line_id("line-005")
            .status(AssignmentStatus::Draft)
            .build()
            .unwrap();

        assert_eq!(assignment.creative_id, "creative-005");
        assert!(assignment.id.is_none());
        assert!(assignment.weight.is_none());
        assert!(assignment.start_date.is_none());
        assert!(assignment.end_date.is_none());
        assert!(assignment.ext.is_none());
    }
}
