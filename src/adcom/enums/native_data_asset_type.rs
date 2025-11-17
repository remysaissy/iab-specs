use serde_repr::{Deserialize_repr, Serialize_repr};

/// Native data asset types.
///
/// Types of data assets in native ads.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum NativeDataAssetType {
    /// Sponsored by message
    Sponsored = 1,

    /// Descriptive text
    Description = 2,

    /// Rating (e.g., 5 stars)
    Rating = 3,

    /// Number of likes
    Likes = 4,

    /// Number of downloads
    Downloads = 5,

    /// Product price
    Price = 6,

    /// Sale price (discounted)
    SalePrice = 7,

    /// Phone number
    Phone = 8,

    /// Address
    Address = 9,

    /// Additional descriptive text
    Description2 = 10,

    /// Display URL
    DisplayUrl = 11,

    /// Call to action text
    CallToAction = 12,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid NativeDataAssetType values (1-12)
        for value in 1..=12 {
            let json = format!("{}", value);
            let result: Result<NativeDataAssetType, _> = serde_json::from_str(&json);
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
        let result: Result<NativeDataAssetType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid NativeDataAssetType and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<NativeDataAssetType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<NativeDataAssetType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            NativeDataAssetType::Sponsored,
            NativeDataAssetType::Description,
            NativeDataAssetType::Rating,
            NativeDataAssetType::Likes,
            NativeDataAssetType::Downloads,
            NativeDataAssetType::Price,
            NativeDataAssetType::SalePrice,
            NativeDataAssetType::Phone,
            NativeDataAssetType::Address,
            NativeDataAssetType::Description2,
            NativeDataAssetType::DisplayUrl,
            NativeDataAssetType::CallToAction,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: NativeDataAssetType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
