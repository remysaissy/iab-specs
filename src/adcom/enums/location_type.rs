use serde_repr::{Deserialize_repr, Serialize_repr};

/// Location type for geolocation.
///
/// Describes the source of location data.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocationType {
    /// GPS/Location Services
    GpsLocation = 1,

    /// IP Address
    IpAddress = 2,

    /// User Provided (e.g., registration data)
    UserProvided = 3,
}
