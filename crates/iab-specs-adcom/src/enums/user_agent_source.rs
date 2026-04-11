use serde_repr::{Deserialize_repr, Serialize_repr};

/// User agent source.
///
/// Source of the user agent string.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UserAgentSource {
    /// Unknown source
    Unknown = 0,

    /// User-agent HTTP header
    HttpHeader = 1,

    /// Client hints
    ClientHints = 2,

    /// Server-side detection
    ServerSide = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AdCOM 1.0 Table: User Agent Source
    #[test]
    fn test_all_valid_values() {
        // Test all valid UserAgentSource values (0-3)
        for value in 0..=3 {
            let json = format!("{}", value);
            let result: Result<UserAgentSource, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    /// AdCOM 1.0 Table: User Agent Source — out of range values are invalid
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<UserAgentSource, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: User Agent Source — negative values are invalid
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<UserAgentSource, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: User Agent Source — roundtrip serialization
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            UserAgentSource::Unknown,
            UserAgentSource::HttpHeader,
            UserAgentSource::ClientHints,
            UserAgentSource::ServerSide,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: UserAgentSource = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
