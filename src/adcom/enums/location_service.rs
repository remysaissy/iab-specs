use serde_repr::{Deserialize_repr, Serialize_repr};

/// Location service provider.
///
/// Source of the location service being used.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LocationService {
    /// IP2Location
    Ip2Location = 1,

    /// Neustar (Quova)
    Neustar = 2,

    /// MaxMind
    MaxMind = 3,

    /// NetAcuity (Digital Element)
    NetAcuity = 4,

    /// 51Degrees (High Confidence)
    FiftyOneDegreesHigh = 511,

    /// 51Degrees (Medium Confidence)
    FiftyOneDegreesMed = 512,
}
