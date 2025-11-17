use serde_repr::{Deserialize_repr, Serialize_repr};

/// Auction type, where 1 = First Price, 2 = Second Price Plus, 3 = the value passed in
/// bidfloor is the agreed upon deal price.
///
/// Additional auction types can be defined by the exchange.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum AuctionType {
    /// First price auction
    FirstPrice = 1,

    /// Second price plus auction (default)
    #[default]
    SecondPricePlus = 2,

    /// Fixed price specified in bidfloor attribute
    FixedPrice = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid AuctionType values (1-3)
        for value in 1..=3 {
            let json = format!("{}", value);
            let result: Result<AuctionType, _> = serde_json::from_str(&json);
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
        let result: Result<AuctionType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid AuctionType and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<AuctionType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<AuctionType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            AuctionType::FirstPrice,
            AuctionType::SecondPricePlus,
            AuctionType::FixedPrice,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: AuctionType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = AuctionType::default();
        assert_eq!(
            default,
            AuctionType::SecondPricePlus,
            "Default should be SecondPricePlus"
        );
    }

    #[test]
    fn test_specific_values() {
        let json = "1";
        let result: AuctionType = serde_json::from_str(json).unwrap();
        assert_eq!(result, AuctionType::FirstPrice);

        let json = "2";
        let result: AuctionType = serde_json::from_str(json).unwrap();
        assert_eq!(result, AuctionType::SecondPricePlus);

        let json = "3";
        let result: AuctionType = serde_json::from_str(json).unwrap();
        assert_eq!(result, AuctionType::FixedPrice);
    }
}
