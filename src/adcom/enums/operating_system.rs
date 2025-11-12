use serde_repr::{Deserialize_repr, Serialize_repr};

/// Operating systems.
///
/// Operating system of the device.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OperatingSystem {
    /// Apple iOS
    IOS = 1,

    /// Google Android
    Android = 2,

    /// Microsoft Windows
    Windows = 3,

    /// Apple macOS
    MacOS = 4,

    /// Linux
    Linux = 5,

    /// Other/Unknown
    Other = 6,
}
