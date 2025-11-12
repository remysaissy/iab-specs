use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video linearity: "in-stream" or "linear" video refers to pre-roll, mid-roll, and
/// post-roll video ads where the user must watch the ad before viewing the content.
/// Nonlinear refers to video ads that overlay content and may not necessarily interrupt
/// streaming content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoLinearity {
    /// Linear / In-stream
    Linear = 1,

    /// Non-linear / Overlay
    NonLinear = 2,
}
