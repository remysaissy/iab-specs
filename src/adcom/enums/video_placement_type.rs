use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video placement types.
///
/// These values are derived from the IAB's Digital Video Guidelines for programmatic
/// video advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoPlacementType {
    /// In-Stream: Played before, during or after the streaming video content that the
    /// consumer has requested (Pre-roll, Mid-roll, Post-roll).
    InStream = 1,

    /// In-Banner: Exists within a web banner that leverages the banner space to deliver
    /// a video experience as opposed to another static or rich media format.
    InBanner = 2,

    /// In-Article: Loads and plays dynamically between paragraphs of editorial content;
    /// existing as a standalone branded message.
    InArticle = 3,

    /// In-Feed: Found in content, social, or product feeds.
    InFeed = 4,

    /// Interstitial/Slider/Floating: Covers the entire or a portion of screen area, but
    /// is always on screen while displayed (i.e. cannot be scrolled out of view).
    Interstitial = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AdCOM 1.0 Table: Video Placement Type
    #[test]
    fn test_all_valid_values() {
        // Test all valid VideoPlacementType values (1-5)
        for value in 1..=5 {
            let json = format!("{}", value);
            let result: Result<VideoPlacementType, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    /// AdCOM 1.0 Table: Video Placement Type — 0 is not a valid variant
    #[test]
    fn test_invalid_value_zero() {
        let json = "0";
        let result: Result<VideoPlacementType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid VideoPlacementType and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Placement Type — out of range values are invalid
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<VideoPlacementType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Placement Type — negative values are invalid
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<VideoPlacementType, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Placement Type — roundtrip serialization
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            VideoPlacementType::InStream,
            VideoPlacementType::InBanner,
            VideoPlacementType::InArticle,
            VideoPlacementType::InFeed,
            VideoPlacementType::Interstitial,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: VideoPlacementType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
