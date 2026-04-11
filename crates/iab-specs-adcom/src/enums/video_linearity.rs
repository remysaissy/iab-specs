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

#[cfg(test)]
mod tests {
    use super::*;

    /// AdCOM 1.0 Table: Video Linearity
    #[test]
    fn test_all_valid_values() {
        // Test all valid VideoLinearity values (1-2)
        for value in 1..=2 {
            let json = format!("{}", value);
            let result: Result<VideoLinearity, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    /// AdCOM 1.0 Table: Video Linearity — 0 is not a valid variant
    #[test]
    fn test_invalid_value_zero() {
        let json = "0";
        let result: Result<VideoLinearity, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid VideoLinearity and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Linearity — out of range values are invalid
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<VideoLinearity, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Linearity — negative values are invalid
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<VideoLinearity, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Linearity — roundtrip serialization
    #[test]
    fn test_serialization_roundtrip() {
        let values = [VideoLinearity::Linear, VideoLinearity::NonLinear];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: VideoLinearity = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
