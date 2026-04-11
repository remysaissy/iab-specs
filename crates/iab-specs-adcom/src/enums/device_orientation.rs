use serde_repr::{Deserialize_repr, Serialize_repr};

/// Device interface orientation.
///
/// The orientation of the device when the ad is shown.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum DeviceOrientation {
    /// Portrait orientation
    #[default]
    Portrait = 0,

    /// Landscape orientation
    Landscape = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid DeviceOrientation values (0-1)
        for value in 0..=1 {
            let json = format!("{}", value);
            let result: Result<DeviceOrientation, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<DeviceOrientation, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<DeviceOrientation, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [DeviceOrientation::Portrait, DeviceOrientation::Landscape];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DeviceOrientation = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
