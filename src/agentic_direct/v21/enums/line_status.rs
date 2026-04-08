use serde::{Deserialize, Serialize};

/// Status of a line (line item) in an order.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum LineStatus {
    /// Line is in draft state.
    #[default]
    Draft,
    /// Line is pending review.
    PendingReview,
    /// Line inventory has been reserved.
    Reserved,
    /// Line has been booked.
    Booked,
    /// Line is currently in progress.
    InProgress,
    /// Line has been paused.
    Paused,
    /// Line has been completed.
    Completed,
    /// Line was cancelled.
    Cancelled,
    /// Line was rejected.
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("draft", LineStatus::Draft),
            ("pending_review", LineStatus::PendingReview),
            ("reserved", LineStatus::Reserved),
            ("booked", LineStatus::Booked),
            ("in_progress", LineStatus::InProgress),
            ("paused", LineStatus::Paused),
            ("completed", LineStatus::Completed),
            ("cancelled", LineStatus::Cancelled),
            ("rejected", LineStatus::Rejected),
        ];

        for (json_str, expected) in values {
            let result: LineStatus = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<LineStatus, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            LineStatus::Draft,
            LineStatus::PendingReview,
            LineStatus::Reserved,
            LineStatus::Booked,
            LineStatus::InProgress,
            LineStatus::Paused,
            LineStatus::Completed,
            LineStatus::Cancelled,
            LineStatus::Rejected,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: LineStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = LineStatus::default();
        assert_eq!(default, LineStatus::Draft, "Default should be Draft");
    }

    #[test]
    fn test_case_sensitive_deserialization() {
        // Spec: Agentic Direct 2.1 — snake_case serialization is mandatory
        let invalid = ["\"Draft\"", "\"IN_PROGRESS\""];
        for json in &invalid {
            assert!(
                serde_json::from_str::<LineStatus>(json).is_err(),
                "{} should be rejected",
                json
            );
        }
    }
}
