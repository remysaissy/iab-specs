use serde_repr::{Deserialize_repr, Serialize_repr};

/// DOOH multiplier measurement source types.
///
/// Identifies the entity providing quantity measurement for impression multipliers
/// in Digital Out-of-Home advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DOOHMultiplierMeasurementSource {
    /// Unknown source
    Unknown = 0,

    /// Measurement vendor provided
    MeasurementVendor = 1,

    /// Publisher provided
    Publisher = 2,

    /// Exchange provided
    Exchange = 3,
}
