use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pod deduplication.
///
/// Deduplication method for ad pods.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PodDeduplication {
    /// Unknown/not specified
    Unknown = 0,

    /// No deduplication
    None = 1,

    /// Deduplicate by creative ID
    ByCreativeId = 2,

    /// Deduplicate by advertiser domain
    ByAdvertiserDomain = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid PodDeduplication values (0-3)
        for value in 0..=3 {
            let json = format!("{}", value);
            let result: Result<PodDeduplication, _> = serde_json::from_str(&json);
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
        let result: Result<PodDeduplication, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<PodDeduplication, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            PodDeduplication::Unknown,
            PodDeduplication::None,
            PodDeduplication::ByCreativeId,
            PodDeduplication::ByAdvertiserDomain,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: PodDeduplication = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
