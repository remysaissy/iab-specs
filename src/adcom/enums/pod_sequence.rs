use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pod sequence.
///
/// Position of ad within a pod.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum PodSequence {
    /// Unknown
    Unknown = 0,

    /// First ad in pod
    First = 1,

    /// Last ad in pod
    Last = 2,

    /// Middle ad in pod
    Middle = 3,

    /// Only ad in pod
    Only = 4,
}
