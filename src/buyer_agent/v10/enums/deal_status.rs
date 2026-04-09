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

    /// Buyer Agent 1.0 § DealStatus — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = DealStatus::Negotiating;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, DealStatus::Negotiating);
    }

    /// Buyer Agent 1.0 § DealStatus — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DealStatus::Quoted);
        set.insert(DealStatus::Negotiating);
        set.insert(DealStatus::Accepted);
        set.insert(DealStatus::Booking);
        set.insert(DealStatus::Booked);
        set.insert(DealStatus::Delivering);
        set.insert(DealStatus::Completed);
        set.insert(DealStatus::Cancelled);
        set.insert(DealStatus::Rejected);
        set.insert(DealStatus::Expired);
        set.insert(DealStatus::Failed);
        set.insert(DealStatus::MakegoodPending);
        set.insert(DealStatus::PartiallyCanceled);

        assert_eq!(set.len(), 13);
        assert!(set.contains(&DealStatus::Quoted));
        assert!(set.contains(&DealStatus::PartiallyCanceled));
    }

    /// Buyer Agent 1.0 § DealStatus — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(DealStatus::Quoted, DealStatus::Negotiating);
        assert_ne!(DealStatus::Negotiating, DealStatus::Accepted);
        assert_ne!(DealStatus::Accepted, DealStatus::Booking);
        assert_ne!(DealStatus::Booking, DealStatus::Booked);
        assert_ne!(DealStatus::Booked, DealStatus::Delivering);
        assert_ne!(DealStatus::Delivering, DealStatus::Completed);
        assert_ne!(DealStatus::Completed, DealStatus::Cancelled);
        assert_ne!(DealStatus::Cancelled, DealStatus::Rejected);
        assert_ne!(DealStatus::Rejected, DealStatus::Expired);
        assert_ne!(DealStatus::Expired, DealStatus::Failed);
        assert_ne!(DealStatus::Failed, DealStatus::MakegoodPending);
        assert_ne!(DealStatus::MakegoodPending, DealStatus::PartiallyCanceled);
    }

    /// Buyer Agent 1.0 § DealStatus — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Negotiating\"", "\"BriefReceived\""];

        for example in &pascal_case_examples {
            let result: Result<DealStatus, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Buyer Agent 1.0 § DealStatus — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (DealStatus::Quoted, "\"quoted\""),
            (DealStatus::Negotiating, "\"negotiating\""),
            (DealStatus::Accepted, "\"accepted\""),
            (DealStatus::Booking, "\"booking\""),
            (DealStatus::Booked, "\"booked\""),
            (DealStatus::Delivering, "\"delivering\""),
            (DealStatus::Completed, "\"completed\""),
            (DealStatus::Cancelled, "\"cancelled\""),
            (DealStatus::Rejected, "\"rejected\""),
            (DealStatus::Expired, "\"expired\""),
            (DealStatus::Failed, "\"failed\""),
            (DealStatus::MakegoodPending, "\"makegood_pending\""),
            (DealStatus::PartiallyCanceled, "\"partially_canceled\""),
        ];

        for (variant, expected_json) in &expected {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(
                &json, expected_json,
                "Mismatch for {:?}: got {}, expected {}",
                variant, json, expected_json
            );
        }
    }
}
