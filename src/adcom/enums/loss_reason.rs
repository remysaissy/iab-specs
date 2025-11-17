use serde_repr::{Deserialize_repr, Serialize_repr};

/// Loss reason codes.
///
/// The following table lists the possible codes for bid response loss reasons.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LossReason {
    /// Bid Won
    BidWon = 0,

    /// Internal Error
    InternalError = 1,

    /// Impression Opportunity Expired
    Expired = 2,

    /// Invalid Bid Response
    InvalidBidResponse = 3,

    /// Invalid Deal ID
    InvalidDealId = 4,

    /// Invalid Auction ID
    InvalidAuctionId = 5,

    /// Invalid Advertiser Domain
    InvalidAdvertiserDomain = 6,

    /// Missing Markup
    MissingMarkup = 7,

    /// Missing Creative ID
    MissingCreativeId = 8,

    /// Missing Price
    MissingPrice = 9,

    /// Missing Minimum Creative Approval Data
    MissingCreativeApproval = 10,

    /// Bid was Below Auction Floor
    BelowFloor = 100,

    /// Bid was Below Deal Floor
    BelowDealFloor = 101,

    /// Lost to Higher Bid
    LostToHigherBid = 102,

    /// Lost to a Bid for a PMP Deal
    LostToPmp = 103,

    /// Buyer Seat Blocked
    SeatBlocked = 104,

    /// Creative Filtered - General
    CreativeFiltered = 200,

    /// Creative Filtered - Pending Processing
    CreativePending = 201,

    /// Creative Filtered - Disapproved
    CreativeDisapproved = 202,

    /// Creative Filtered - Size Not Allowed
    CreativeSizeNotAllowed = 203,

    /// Creative Filtered - Not Secure
    CreativeNotSecure = 204,

    /// Creative Filtered - Language Exclusions
    CreativeLanguageExcluded = 205,

    /// Creative Filtered - Category Exclusions
    CreativeCategoryExcluded = 206,

    /// Creative Filtered - Creative Attribute Exclusions
    CreativeAttributeExcluded = 207,

    /// Creative Filtered - Ad Type Exclusions
    CreativeAdTypeExcluded = 208,

    /// Creative Filtered - Animation Too Long
    CreativeAnimationTooLong = 209,

    /// Creative Filtered - Not Allowed in PMP Deal
    CreativeNotAllowedInPmp = 210,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid LossReason values: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210
        let valid_values = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204,
            205, 206, 207, 208, 209, 210,
        ];
        for value in valid_values {
            let json = format!("{}", value);
            let result: Result<LossReason, _> = serde_json::from_str(&json);
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
        let result: Result<LossReason, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<LossReason, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            LossReason::BidWon,
            LossReason::InternalError,
            LossReason::Expired,
            LossReason::InvalidBidResponse,
            LossReason::InvalidDealId,
            LossReason::InvalidAuctionId,
            LossReason::InvalidAdvertiserDomain,
            LossReason::MissingMarkup,
            LossReason::MissingCreativeId,
            LossReason::MissingPrice,
            LossReason::MissingCreativeApproval,
            LossReason::BelowFloor,
            LossReason::BelowDealFloor,
            LossReason::LostToHigherBid,
            LossReason::LostToPmp,
            LossReason::SeatBlocked,
            LossReason::CreativeFiltered,
            LossReason::CreativePending,
            LossReason::CreativeDisapproved,
            LossReason::CreativeSizeNotAllowed,
            LossReason::CreativeNotSecure,
            LossReason::CreativeLanguageExcluded,
            LossReason::CreativeCategoryExcluded,
            LossReason::CreativeAttributeExcluded,
            LossReason::CreativeAdTypeExcluded,
            LossReason::CreativeAnimationTooLong,
            LossReason::CreativeNotAllowedInPmp,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: LossReason = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
