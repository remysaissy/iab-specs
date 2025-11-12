use serde_repr::{Deserialize_repr, Serialize_repr};

/// Content delivery methods.
///
/// The various options for content delivery.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentDeliveryMethod {
    /// Streaming
    Streaming = 1,

    /// Progressive
    Progressive = 2,

    /// Download
    Download = 3,
}
