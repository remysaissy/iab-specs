use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of device from which the impression originates.
///
/// OpenRTB version 2.2 of the specification added distinct values for Mobile and Tablet.
/// It is recommended that any bidder with differentiation in their campaign-creative
/// management systems between these 2 device types properly determine and use these types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeviceType {
    /// Mobile/Tablet - General (deprecated, use specific types)
    #[deprecated(note = "Use Mobile or Tablet")]
    MobileTablet = 1,

    /// Personal Computer
    PersonalComputer = 2,

    /// Connected TV
    ConnectedTv = 3,

    /// Phone
    Phone = 4,

    /// Tablet
    Tablet = 5,

    /// Connected Device
    ConnectedDevice = 6,

    /// Set Top Box
    SetTopBox = 7,

    /// Out of Home (OOH) Device
    OutOfHome = 8,
}
