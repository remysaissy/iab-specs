use serde_repr::{Deserialize_repr, Serialize_repr};

/// Video placement subtype.
///
/// More specific video placement types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoPlacementSubtype {
    /// In-stream placement (pre/mid/post-roll)
    InStream = 1,

    /// In-banner video
    InBanner = 2,

    /// In-article video
    InArticle = 3,

    /// In-feed video
    InFeed = 4,

    /// Interstitial/floating video
    Interstitial = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AdCOM 1.0 Table: Video Placement Subtype
    #[test]
    fn test_all_valid_values() {
        // Test all valid VideoPlacementSubtype values (1-5)
        for value in 1..=5 {
            let json = format!("{}", value);
            let result: Result<VideoPlacementSubtype, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    /// AdCOM 1.0 Table: Video Placement Subtype — 0 is not a valid variant
    #[test]
    fn test_invalid_value_zero() {
        let json = "0";
        let result: Result<VideoPlacementSubtype, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid VideoPlacementSubtype and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Placement Subtype — out of range values are invalid
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<VideoPlacementSubtype, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Placement Subtype — negative values are invalid
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<VideoPlacementSubtype, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    /// AdCOM 1.0 Table: Video Placement Subtype — roundtrip serialization
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            VideoPlacementSubtype::InStream,
            VideoPlacementSubtype::InBanner,
            VideoPlacementSubtype::InArticle,
            VideoPlacementSubtype::InFeed,
            VideoPlacementSubtype::Interstitial,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: VideoPlacementSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
