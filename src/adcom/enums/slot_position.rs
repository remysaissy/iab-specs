use serde_repr::{Deserialize_repr, Serialize_repr};

/// Slot position within an ad pod.
///
/// Indicates the position of the individual ad slot within an ad pod for video/audio.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum SlotPosition {
    /// Last ad in the pod
    Last = -1,

    /// Any other position (middle of pod)
    Any = 0,

    /// First ad in the pod
    First = 1,

    /// First or last position in the pod
    FirstOrLast = 2,
}
