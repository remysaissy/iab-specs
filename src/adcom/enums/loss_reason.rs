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
