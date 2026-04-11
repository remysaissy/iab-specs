use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of connection.
///
/// The various options for the type of device connectivity.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConnectionType {
    /// Unknown
    Unknown = 0,

    /// Ethernet
    Ethernet = 1,

    /// WIFI
    Wifi = 2,

    /// Cellular Network - Unknown Generation
    CellularUnknown = 3,

    /// Cellular Network - 2G
    Cellular2G = 4,

    /// Cellular Network - 3G
    Cellular3G = 5,

    /// Cellular Network - 4G
    Cellular4G = 6,

    /// Cellular Network - 5G
    Cellular5G = 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid ConnectionType values (0-7)
        for value in 0..=7 {
            let json = format!("{}", value);
            let result: Result<ConnectionType, _> = serde_json::from_str(&json);
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
        let result: Result<ConnectionType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<ConnectionType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            ConnectionType::Unknown,
            ConnectionType::Ethernet,
            ConnectionType::Wifi,
            ConnectionType::CellularUnknown,
            ConnectionType::Cellular2G,
            ConnectionType::Cellular3G,
            ConnectionType::Cellular4G,
            ConnectionType::Cellular5G,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ConnectionType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
