use serde_repr::{Deserialize_repr, Serialize_repr};

/// Click type.
///
/// Types of ad click behavior.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClickType {
    /// Non-clickable
    NonClickable = 0,

    /// Clickable
    Clickable = 1,

    /// Clickable with embedded browser
    EmbeddedBrowser = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid ClickType values (0-2)
        for value in 0..=2 {
            let json = format!("{}", value);
            let result: Result<ClickType, _> = serde_json::from_str(&json);
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
        let result: Result<ClickType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<ClickType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            ClickType::NonClickable,
            ClickType::Clickable,
            ClickType::EmbeddedBrowser,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ClickType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
