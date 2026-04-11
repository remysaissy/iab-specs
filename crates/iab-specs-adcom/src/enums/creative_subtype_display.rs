use serde_repr::{Deserialize_repr, Serialize_repr};

/// Creative subtype for display ads.
///
/// More granular categorization of display creative types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeSubtypeDisplay {
    /// HTML banner
    HtmlBanner = 1,

    /// VAST tag for video
    Vast = 2,

    /// VPAID for interactive video
    Vpaid = 3,

    /// JavaScript tag
    JavaScript = 4,

    /// iFrame
    IFrame = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid CreativeSubtypeDisplay values (1-5)
        for value in 1..=5 {
            let json = format!("{}", value);
            let result: Result<CreativeSubtypeDisplay, _> = serde_json::from_str(&json);
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
        let result: Result<CreativeSubtypeDisplay, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid CreativeSubtypeDisplay and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<CreativeSubtypeDisplay, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<CreativeSubtypeDisplay, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            CreativeSubtypeDisplay::HtmlBanner,
            CreativeSubtypeDisplay::Vast,
            CreativeSubtypeDisplay::Vpaid,
            CreativeSubtypeDisplay::JavaScript,
            CreativeSubtypeDisplay::IFrame,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CreativeSubtypeDisplay = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
