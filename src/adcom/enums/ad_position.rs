use serde_repr::{Deserialize_repr, Serialize_repr};

/// The position of the ad as a relative measure of visibility or prominence.
///
/// This OpenRTB list has values derived from the Inventory Quality Guidelines (IQG).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum AdPosition {
    /// Unknown position
    #[default]
    Unknown = 0,

    /// Above the fold
    AboveTheFold = 1,

    /// May or may not be initially visible (deprecated by OpenRTB)
    #[deprecated(note = "Use Unknown or other appropriate value")]
    MayNotBeVisible = 2,

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
        // Test all valid AdPosition values (0-7)
        for value in 0..=7 {
            let json = format!("{}", value);
            let result: Result<AdPosition, _> = serde_json::from_str(&json);
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
        let result: Result<AdPosition, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<AdPosition, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        #[allow(deprecated)]
        let values = [
            AdPosition::Unknown,
            AdPosition::AboveTheFold,
            AdPosition::MayNotBeVisible,
            AdPosition::BelowTheFold,
            AdPosition::Header,
            AdPosition::Footer,
            AdPosition::Sidebar,
            AdPosition::FullScreen,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: AdPosition = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = AdPosition::default();
        assert_eq!(default, AdPosition::Unknown, "Default should be Unknown");
    }

    #[test]
    fn test_specific_values() {
        let json = "0";
        let result: AdPosition = serde_json::from_str(json).unwrap();
        assert_eq!(result, AdPosition::Unknown);

        let json = "1";
        let result: AdPosition = serde_json::from_str(json).unwrap();
        assert_eq!(result, AdPosition::AboveTheFold);

        let json = "7";
        let result: AdPosition = serde_json::from_str(json).unwrap();
        assert_eq!(result, AdPosition::FullScreen);
    }
}
