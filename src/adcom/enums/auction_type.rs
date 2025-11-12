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
