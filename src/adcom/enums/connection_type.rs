use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of connection.
///
/// The various options for the type of device connectivity.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConnectionType {
    /// Unknown
    Unknown = 0,

    /// Ethernet
    Ethernet = 1,

    /// WIFI
    Wifi = 2,

    /// Cellular Network - Unknown Generation
    CellularUnknown = 3,

    /// Cellular Network - 2G
    Cellular2G = 4,

    /// Cellular Network - 3G
    Cellular3G = 5,

    /// Cellular Network - 4G
    Cellular4G = 6,

    /// Cellular Network - 5G
    Cellular5G = 7,
}
