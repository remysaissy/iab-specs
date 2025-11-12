use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video or audio protocols supported.
///
/// OpenRTB version 2.5 list. VAST versions are numbered in a sub-range to distinguish
/// from other protocol values. DAAST is included for audio ads. OpenRTB 2.6 adds support
/// for VAST 4.2 and 4.3.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Protocol {
    /// VAST 1.0
    Vast1 = 1,

    /// VAST 2.0
    Vast2 = 2,

    /// VAST 3.0
    Vast3 = 3,

    /// VAST 1.0 Wrapper
    Vast1Wrapper = 4,

    /// VAST 2.0 Wrapper
    Vast2Wrapper = 5,

    /// VAST 3.0 Wrapper
    Vast3Wrapper = 6,

    /// VAST 4.0
    Vast4 = 7,

    /// VAST 4.0 Wrapper
    Vast4Wrapper = 8,

    /// DAAST 1.0
    Daast1 = 9,

    /// DAAST 1.0 Wrapper
    Daast1Wrapper = 10,

    /// VAST 4.1
    Vast4_1 = 11,

    /// VAST 4.1 Wrapper
    Vast4_1Wrapper = 12,

    /// VAST 4.2
    Vast4_2 = 13,

    /// VAST 4.2 Wrapper
    Vast4_2Wrapper = 14,

    /// VAST 4.3
    Vast4_3 = 15,

    /// VAST 4.3 Wrapper
    Vast4_3Wrapper = 16,
}
