use serde_repr::{Deserialize_repr, Serialize_repr};

/// API frameworks supported by the publisher.
///
/// Note that MRAID-1, MRAID-2, and MRAID-3 are numbered 3, 5, and 6 since it was
/// determined that their predecessors, values 1 and 2, were duplicates as the
/// VPAID 1.0 and VPAID 2.0 specifications are inherently HTML5 compliant.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApiFramework {
    /// VPAID 1.0
    Vpaid1 = 1,

    /// VPAID 2.0
    Vpaid2 = 2,

    /// MRAID-1
    Mraid1 = 3,

    /// ORMMA
    Ormma = 4,

    /// MRAID-2
    Mraid2 = 5,

    /// MRAID-3
    Mraid3 = 6,

    /// OMID-1
    Omid1 = 7,

    /// SIMID-1
    Simid1 = 8,

    /// SIMID-1.1
    Simid1_1 = 9,
}
