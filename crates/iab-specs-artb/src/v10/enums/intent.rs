use serde_repr::{Deserialize_repr, Serialize_repr};

/// The declared purpose of a mutation proposed by an agent.
///
/// Each intent maps to a specific payload type and semantic path pattern.
/// The orchestrator uses intents to filter which mutations an agent is
/// allowed to propose via the `applicable_intents` field on `RTBRequest`.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Intent {
    /// Unspecified intent (invalid for actual mutations).
    #[default]
    Unspecified = 0,

    /// Activate user segments by external segment IDs.
    /// Expected payload: `IDsPayload`. Path: `/user/data/segment`.
    ActivateSegments = 1,

    /// Activate deals by external deal IDs.
    /// Expected payload: `IDsPayload`. Path: `/imp/{impId}`.
    ActivateDeals = 2,

    /// Suppress deals by external deal IDs.
    /// Expected payload: `IDsPayload`. Path: `/imp/{impId}`.
    SuppressDeals = 3,

    /// Adjust the bid floor of a specific deal.
    /// Expected payload: `AdjustDealPayload`. Path: `/imp/{impId}/pmp/deals/{dealId}`.
    AdjustDealFloor = 4,

    /// Adjust the margin of a specific deal.
    /// Expected payload: `AdjustDealPayload`. Path: `/imp/{impId}/pmp/deals/{dealId}`.
    AdjustDealMargin = 5,

    /// Adjust the bid price (bid shading).
    /// Expected payload: `AdjustBidPayload`. Path: `/seatbid/{seat}/bid/{bidId}`.
    BidShade = 6,

    /// Add metrics to an impression.
    /// Expected payload: `MetricsPayload`. Path: `/imp/{impId}/metric`.
    AddMetrics = 7,

    /// Add extended content IDs.
    /// Expected payload: `DataPayload`.
    AddCids = 8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        for value in 0..=8 {
            let json = format!("{}", value);
            let result: Result<Intent, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<Intent, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<Intent, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            Intent::Unspecified,
            Intent::ActivateSegments,
            Intent::ActivateDeals,
            Intent::SuppressDeals,
            Intent::AdjustDealFloor,
            Intent::AdjustDealMargin,
            Intent::BidShade,
            Intent::AddMetrics,
            Intent::AddCids,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Intent = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = Intent::default();
        assert_eq!(
            default,
            Intent::Unspecified,
            "Default should be Unspecified"
        );
    }

    #[test]
    fn test_specific_values() {
        let json = "1";
        let result: Intent = serde_json::from_str(json).unwrap();
        assert_eq!(result, Intent::ActivateSegments);

        let json = "6";
        let result: Intent = serde_json::from_str(json).unwrap();
        assert_eq!(result, Intent::BidShade);

        let json = "8";
        let result: Intent = serde_json::from_str(json).unwrap();
        assert_eq!(result, Intent::AddCids);
    }
}
