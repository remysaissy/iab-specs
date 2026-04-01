use serde::{Deserialize, Serialize};

/// The current state of a Deal in the Buyer Agent workflow.
///
/// Deals progress through various states from initial quotes through booking to completion or cancellation.
/// All serialization uses snake_case format (e.g., `"makegood_pending"` for `MakegoodPending`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum DealStatus {
    /// Initial quote provided to the publisher.
    #[default]
    Quoted,

    /// Deal is under active negotiation.
    Negotiating,

    /// Deal terms have been accepted by both parties.
    Accepted,

    /// Deal is in the booking/reservation process.
    Booking,

    /// Deal has been successfully booked.
    Booked,

    /// Deal inventory is actively being delivered.
    Delivering,

    /// Deal has successfully completed all delivery.
    Completed,

    /// Deal was cancelled before completion.
    Cancelled,

    /// Deal was rejected during review or negotiation.
    Rejected,

    /// Deal has expired past its agreed end date.
    Expired,

    /// Deal failed to deliver as agreed.
    Failed,

    /// Deal is awaiting makegood inventory delivery.
    MakegoodPending,

    /// Deal was partially cancelled.
    PartiallyCanceled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            DealStatus::Quoted,
            DealStatus::Negotiating,
            DealStatus::Accepted,
            DealStatus::Booking,
            DealStatus::Booked,
            DealStatus::Delivering,
            DealStatus::Completed,
            DealStatus::Cancelled,
            DealStatus::Rejected,
            DealStatus::Expired,
            DealStatus::Failed,
            DealStatus::MakegoodPending,
            DealStatus::PartiallyCanceled,
        ];

        for variant in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            assert!(
                serialized.starts_with('"') && serialized.ends_with('"'),
                "Serialized value {} should be a JSON string",
                serialized
            );
            let unquoted = &serialized[1..serialized.len() - 1];
            assert!(
                unquoted.chars().all(|c| c.is_lowercase() || c == '_'),
                "Serialized value {} should be snake_case",
                unquoted
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_status\"";
        let result: Result<DealStatus, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Invalid status should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            DealStatus::Quoted,
            DealStatus::Negotiating,
            DealStatus::Accepted,
            DealStatus::Booking,
            DealStatus::Booked,
            DealStatus::Delivering,
            DealStatus::Completed,
            DealStatus::Cancelled,
            DealStatus::Rejected,
            DealStatus::Expired,
            DealStatus::Failed,
            DealStatus::MakegoodPending,
            DealStatus::PartiallyCanceled,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: DealStatus =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(
                original, &deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = DealStatus::default();
        assert_eq!(default, DealStatus::Quoted, "Default should be Quoted");
    }
}
