use serde_repr::{Deserialize_repr, Serialize_repr};

/// Audit status codes.
///
/// Status codes for creative audits and approval.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AuditStatusCode {
    /// Approved
    Approved = 1,

    /// Approved with changes
    ApprovedWithChanges = 2,

    /// Rejected
    Rejected = 3,

    /// Rejected for impressions (creative not served)
    RejectedForImpressions = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid AuditStatusCode values (1-4)
        for value in 1..=4 {
            let json = format!("{}", value);
            let result: Result<AuditStatusCode, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }
    #[test]
    fn test_invalid_value_zero() {
        let json = "0";
        let result: Result<AuditStatusCode, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid AuditStatusCode and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<AuditStatusCode, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<AuditStatusCode, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            AuditStatusCode::Approved,
            AuditStatusCode::ApprovedWithChanges,
            AuditStatusCode::Rejected,
            AuditStatusCode::RejectedForImpressions,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: AuditStatusCode = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
