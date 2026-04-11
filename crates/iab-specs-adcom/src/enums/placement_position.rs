use serde_repr::{Deserialize_repr, Serialize_repr};

/// Placement position.
///
/// Ad position on screen (may duplicate AdPosition for legacy reasons).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlacementPosition {
    /// Unknown
    Unknown = 0,

    /// Above the fold
    AboveTheFold = 1,

    /// Below the fold
    BelowTheFold = 3,

    /// Header
    Header = 4,

    /// Footer
    Footer = 5,

    /// Sidebar
    Sidebar = 6,

    /// Full screen
    FullScreen = 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid PlacementPosition values: 0, 1, 3, 4, 5, 6, 7
        let valid_values = [0, 1, 3, 4, 5, 6, 7];
        for value in valid_values {
            let json = format!("{}", value);
            let result: Result<PlacementPosition, _> = serde_json::from_str(&json);
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
        let result: Result<PlacementPosition, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<PlacementPosition, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            PlacementPosition::Unknown,
            PlacementPosition::AboveTheFold,
            PlacementPosition::BelowTheFold,
            PlacementPosition::Header,
            PlacementPosition::Footer,
            PlacementPosition::Sidebar,
            PlacementPosition::FullScreen,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: PlacementPosition = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
