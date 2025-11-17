use serde_repr::{Deserialize_repr, Serialize_repr};

/// The various types of creative attributes.
///
/// Creative attributes that describe ad creatives in detail. They can be used to
/// indicate restrictions on what kinds of creatives can be displayed.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CreativeAttribute {
    /// Audio ad (autoplay)
    AudioAuto = 1,

    /// Audio ad (user initiated)
    AudioUser = 2,

    /// Expandable (automatic)
    ExpandableAuto = 3,

    /// Expandable (user initiated - click)
    ExpandableClick = 4,

    /// Expandable (user initiated - rollover)
    ExpandableRollover = 5,

    /// In-banner video ad (autoplay)
    VideoBannerAuto = 6,

    /// In-banner video ad (user initiated)
    VideoBannerUser = 7,

    /// Pop (e.g., over, under, or upon exit)
    Pop = 8,

    /// Provocative or suggestive imagery
    Provocative = 9,

    /// Shaky, flashing, flickering, extreme animation, smileys
    Annoying = 10,

    /// Surveys
    Surveys = 11,

    /// Text only
    TextOnly = 12,

    /// User interactive (e.g., embedded games)
    UserInteractive = 13,

    /// Windows dialog or alert style
    Alert = 14,

    /// Has audio on/off button
    AudioOnOffButton = 15,

    /// Ad can be skipped (e.g., skip button)
    Skippable = 16,

    /// Adobe Flash
    AdobeFlash = 17,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid CreativeAttribute values (1-17)
        for value in 1..=17 {
            let json = format!("{}", value);
            let result: Result<CreativeAttribute, _> = serde_json::from_str(&json);
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
        let result: Result<CreativeAttribute, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid CreativeAttribute and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<CreativeAttribute, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<CreativeAttribute, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            CreativeAttribute::AudioAuto,
            CreativeAttribute::AudioUser,
            CreativeAttribute::ExpandableAuto,
            CreativeAttribute::ExpandableClick,
            CreativeAttribute::ExpandableRollover,
            CreativeAttribute::VideoBannerAuto,
            CreativeAttribute::VideoBannerUser,
            CreativeAttribute::Pop,
            CreativeAttribute::Provocative,
            CreativeAttribute::Annoying,
            CreativeAttribute::Surveys,
            CreativeAttribute::TextOnly,
            CreativeAttribute::UserInteractive,
            CreativeAttribute::Alert,
            CreativeAttribute::AudioOnOffButton,
            CreativeAttribute::Skippable,
            CreativeAttribute::AdobeFlash,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CreativeAttribute = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
