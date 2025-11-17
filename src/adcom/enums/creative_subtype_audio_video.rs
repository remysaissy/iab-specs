use serde_repr::{Deserialize_repr, Serialize_repr};

/// Creative subtype for audio/video ads.
///
/// Categorization of audio and video creative formats.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeSubtypeAudioVideo {
    /// VAST (Video Ad Serving Template)
    Vast = 1,

    /// DAAST (Digital Audio Ad Serving Template)
    Daast = 2,

    /// VPAID (Video Player-Ad Interface Definition)
    Vpaid = 3,

    /// Proprietary format
    Proprietary = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid CreativeSubtypeAudioVideo values (1-4)
        for value in 1..=4 {
            let json = format!("{}", value);
            let result: Result<CreativeSubtypeAudioVideo, _> = serde_json::from_str(&json);
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
        let result: Result<CreativeSubtypeAudioVideo, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid CreativeSubtypeAudioVideo and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<CreativeSubtypeAudioVideo, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<CreativeSubtypeAudioVideo, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            CreativeSubtypeAudioVideo::Vast,
            CreativeSubtypeAudioVideo::Daast,
            CreativeSubtypeAudioVideo::Vpaid,
            CreativeSubtypeAudioVideo::Proprietary,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CreativeSubtypeAudioVideo = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
