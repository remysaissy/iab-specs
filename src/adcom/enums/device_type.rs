use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of device from which the impression originates.
///
/// OpenRTB version 2.2 of the specification added distinct values for Mobile and Tablet.
/// It is recommended that any bidder with differentiation in their campaign-creative
/// management systems between these 2 device types properly determine and use these types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeviceType {
    /// Mobile/Tablet - General (deprecated, use specific types)
    #[deprecated(note = "Use Mobile or Tablet")]
    MobileTablet = 1,

    /// Personal Computer
    PersonalComputer = 2,

    /// Connected TV
    ConnectedTv = 3,

    /// Phone
    Phone = 4,

    /// Tablet
    Tablet = 5,

    /// Connected Device
    ConnectedDevice = 6,

    /// Set Top Box
    SetTopBox = 7,

    /// Out of Home (OOH) Device
    OutOfHome = 8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid DeviceType values (1-8)
        for value in 1..=8 {
            let json = format!("{}", value);
            let result: Result<DeviceType, _> = serde_json::from_str(&json);
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
        let result: Result<DeviceType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid DeviceType and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<DeviceType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<DeviceType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        #[allow(deprecated)]
        let values = [
            DeviceType::MobileTablet,
            DeviceType::PersonalComputer,
            DeviceType::ConnectedTv,
            DeviceType::Phone,
            DeviceType::Tablet,
            DeviceType::ConnectedDevice,
            DeviceType::SetTopBox,
            DeviceType::OutOfHome,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DeviceType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
