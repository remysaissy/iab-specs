use serde_repr::{Deserialize_repr, Serialize_repr};

/// Size unit.
///
/// Units of measurement for sizes.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SizeUnit {
    /// Device Independent Pixels (DIPS)
    Dips = 1,

    /// Physical pixels
    Pixels = 2,
}
