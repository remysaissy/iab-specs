use serde_repr::{Deserialize_repr, Serialize_repr};

/// Volume normalization modes.
///
/// Volume normalization modes for audio content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VolumeNormalizationMode {
    /// None
    None = 0,

    /// Ad Volume Average Normalized to Content
    AverageVolume = 1,

    /// Ad Volume Peak Normalized to Content
    PeakVolume = 2,

    /// Ad Loudness Normalized to Content
    Loudness = 3,

    /// Custom Volume Normalization
    Custom = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AdCOM 1.0 Table: Volume Normalization Mode
    #[test]
    fn test_all_valid_values() {
        // Test all valid VolumeNormalizationMode values (0-4)
        for value in 0..=4 {
            let json = format!("{}", value);
            let result: Result<VolumeNormalizationMode, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    /// AdCOM 1.0 Table: Volume Normalization Mode — out of range values are invalid
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<VolumeNormalizationMode, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Volume Normalization Mode — negative values are invalid
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<VolumeNormalizationMode, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Volume Normalization Mode — roundtrip serialization
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            VolumeNormalizationMode::None,
            VolumeNormalizationMode::AverageVolume,
            VolumeNormalizationMode::PeakVolume,
            VolumeNormalizationMode::Loudness,
            VolumeNormalizationMode::Custom,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: VolumeNormalizationMode = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
