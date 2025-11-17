use serde_repr::{Deserialize_repr, Serialize_repr};

/// Display placement type.
///
/// General type or context of the display placement.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayPlacementType {
    /// In-feed placement (e.g., newsfeed, content stream)
    InFeed = 1,

    /// Sidebar placement
    Sidebar = 2,

    /// Interstitial/Overlay placement
    Interstitial = 3,

    /// Floating placement
    Floating = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid DisplayPlacementType values (1-4)
        for value in 1..=4 {
            let json = format!("{}", value);
            let result: Result<DisplayPlacementType, _> = serde_json::from_str(&json);
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
        let result: Result<DisplayPlacementType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid DisplayPlacementType and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<DisplayPlacementType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<DisplayPlacementType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            DisplayPlacementType::InFeed,
            DisplayPlacementType::Sidebar,
            DisplayPlacementType::Interstitial,
            DisplayPlacementType::Floating,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DisplayPlacementType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
