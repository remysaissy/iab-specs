use serde_repr::{Deserialize_repr, Serialize_repr};

/// Auto-refresh trigger.
///
/// Trigger that causes a placement to auto-refresh.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AutoRefreshTrigger {
    /// User-initiated refresh
    UserInitiated = 1,

    /// Time-based expiration
    TimeExpiration = 2,

    /// Scroll-based refresh
    Scroll = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid AutoRefreshTrigger values (1-3)
        for value in 1..=3 {
            let json = format!("{}", value);
            let result: Result<AutoRefreshTrigger, _> = serde_json::from_str(&json);
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
        let result: Result<AutoRefreshTrigger, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid AutoRefreshTrigger and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<AutoRefreshTrigger, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<AutoRefreshTrigger, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            AutoRefreshTrigger::UserInitiated,
            AutoRefreshTrigger::TimeExpiration,
            AutoRefreshTrigger::Scroll,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: AutoRefreshTrigger = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
