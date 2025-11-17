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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid OperatingSystem values (1-6)
        for value in 1..=6 {
            let json = format!("{}", value);
            let result: Result<OperatingSystem, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }
    #[test]
    fn test_invalid_value_zero() {
        let json = "0";
        let result: Result<OperatingSystem, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid OperatingSystem and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<OperatingSystem, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<OperatingSystem, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            OperatingSystem::IOS,
            OperatingSystem::Android,
            OperatingSystem::Windows,
            OperatingSystem::MacOS,
            OperatingSystem::Linux,
            OperatingSystem::Other,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: OperatingSystem = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
