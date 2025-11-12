use serde_repr::{Deserialize_repr, Serialize_repr};

/// DOOH Venue type taxonomy.
///
/// Taxonomy defining venue types for Digital Out-Of-Home advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DoohVenueTaxonomy {
    /// AdCOM 1.0
    AdCom1 = 1,

    /// DPAA 2016
    Dpaa2016 = 2,

    /// DMI 2017
    Dmi2017 = 3,
}
