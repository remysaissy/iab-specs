use serde_repr::{Deserialize_repr, Serialize_repr};

/// ID matching methods for user identification.
///
/// Indicates the method used to match a user ID across different contexts.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IDMatchMethod {
    /// No matching - ID came directly from 3rd-party cookie or device IFA
    NoMatching = 0,

    /// First-party observation without user authentication
    FirstParty = 1,

    /// Probabilistic matching based on non-authenticated features
    Probabilistic = 2,

    /// Deterministic matching with user authentication
    Deterministic = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid IDMatchMethod values (0-3)
        for value in 0..=3 {
            let json = format!("{}", value);
            let result: Result<IDMatchMethod, _> = serde_json::from_str(&json);
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
        let result: Result<IDMatchMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<IDMatchMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            IDMatchMethod::NoMatching,
            IDMatchMethod::FirstParty,
            IDMatchMethod::Probabilistic,
            IDMatchMethod::Deterministic,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: IDMatchMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
