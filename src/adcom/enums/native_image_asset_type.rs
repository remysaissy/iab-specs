use serde_repr::{Deserialize_repr, Serialize_repr};

/// Native image asset types.
///
/// Types of image assets in native ads.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NativeImageAssetType {
    /// Icon image (typically small, square)
    Icon = 1,

    /// Logo image
    Logo = 2,

    /// Large image (main creative image)
    Main = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid NativeImageAssetType values (1-3)
        for value in 1..=3 {
            let json = format!("{}", value);
            let result: Result<NativeImageAssetType, _> = serde_json::from_str(&json);
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
        let result: Result<NativeImageAssetType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid NativeImageAssetType and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<NativeImageAssetType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<NativeImageAssetType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            NativeImageAssetType::Icon,
            NativeImageAssetType::Logo,
            NativeImageAssetType::Main,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: NativeImageAssetType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
