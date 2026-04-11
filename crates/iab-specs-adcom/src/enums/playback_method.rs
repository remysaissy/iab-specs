use serde_repr::{Deserialize_repr, Serialize_repr};

/// Playback methods available for video inventory.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlaybackMethod {
    /// Initiates on page load with sound on
    AutoPlaySoundOn = 1,

    /// Initiates on page load with sound off by default
    AutoPlaySoundOff = 2,

    /// Initiates on click with sound on
    ClickToPlay = 3,

    /// Initiates on mouse-over with sound on
    MouseOver = 4,

    /// Initiates on entering viewport with sound on
    EnterViewportSoundOn = 5,

    /// Initiates on entering viewport with sound off by default
    EnterViewportSoundOff = 6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid PlaybackMethod values (1-6)
        for value in 1..=6 {
            let json = format!("{}", value);
            let result: Result<PlaybackMethod, _> = serde_json::from_str(&json);
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
        let result: Result<PlaybackMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid PlaybackMethod and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<PlaybackMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<PlaybackMethod, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            PlaybackMethod::AutoPlaySoundOn,
            PlaybackMethod::AutoPlaySoundOff,
            PlaybackMethod::ClickToPlay,
            PlaybackMethod::MouseOver,
            PlaybackMethod::EnterViewportSoundOn,
            PlaybackMethod::EnterViewportSoundOff,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: PlaybackMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
