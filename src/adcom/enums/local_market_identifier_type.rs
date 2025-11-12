use serde_repr::{Deserialize_repr, Serialize_repr};

/// Local market identifier types.
///
/// Designates the local market/DMA provider (Nielsen, Kantar, etc.).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocalMarketIdentifierType {
    /// Nielsen DMA
    Nielsen = 1,

    /// Kantar
    Kantar = 2,
}
