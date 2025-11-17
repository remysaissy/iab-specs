use serde_repr::{Deserialize_repr, Serialize_repr};

/// Display context type.
///
/// Context in which a display ad appears.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayContextType {
    /// Content-centric context (e.g., newsfeed, article)
    ContentCentric = 1,

    /// Social-centric context (e.g., social network feed)
    SocialCentric = 2,

    /// Product context (e.g., product details, reviews)
    ProductContext = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid DisplayContextType values (1-3)
        for value in 1..=3 {
            let json = format!("{}", value);
            let result: Result<DisplayContextType, _> = serde_json::from_str(&json);
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
        let result: Result<DisplayContextType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid DisplayContextType and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<DisplayContextType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<DisplayContextType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            DisplayContextType::ContentCentric,
            DisplayContextType::SocialCentric,
            DisplayContextType::ProductContext,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DisplayContextType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
