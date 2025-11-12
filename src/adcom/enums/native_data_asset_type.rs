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
