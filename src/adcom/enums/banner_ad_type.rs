use serde_repr::{Deserialize_repr, Serialize_repr};

/// Banner ad types.
///
/// The type of banner creative to be served using an AdUnit.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BannerAdType {
    /// XHTML Text Ad (usually mobile)
    XhtmlTextAd = 1,

    /// XHTML Banner Ad (usually mobile)
    XhtmlBannerAd = 2,

    /// JavaScript Ad; must be valid XHTML (i.e., script tags included)
    JavaScriptAd = 3,

    /// iFrame
    IFrame = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid BannerAdType values (1-4)
        for value in 1..=4 {
            let json = format!("{}", value);
            let result: Result<BannerAdType, _> = serde_json::from_str(&json);
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
        let result: Result<BannerAdType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid BannerAdType and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<BannerAdType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<BannerAdType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            BannerAdType::XhtmlTextAd,
            BannerAdType::XhtmlBannerAd,
            BannerAdType::JavaScriptAd,
            BannerAdType::IFrame,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: BannerAdType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
