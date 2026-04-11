use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of content being displayed.
///
/// The nature of the content on the site, app, or other property.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentContext {
    /// Video (i.e., video file or stream such as Internet TV broadcasts)
    Video = 1,

    /// Game (i.e., an interactive software game)
    Game = 2,

    /// Music (i.e., audio file or stream such as Internet radio broadcasts)
    Music = 3,

    /// Application (i.e., an interactive software application)
    Application = 4,

    /// Text (i.e., primarily textual document such as a web page, eBook, or news article)
    Text = 5,

    /// Other (i.e., none of the other categories applies)
    Other = 6,

    /// Unknown
    Unknown = 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid ContentContext values (1-7)
        for value in 1..=7 {
            let json = format!("{}", value);
            let result: Result<ContentContext, _> = serde_json::from_str(&json);
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
        let result: Result<ContentContext, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid ContentContext and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<ContentContext, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<ContentContext, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            ContentContext::Video,
            ContentContext::Game,
            ContentContext::Music,
            ContentContext::Application,
            ContentContext::Text,
            ContentContext::Other,
            ContentContext::Unknown,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: ContentContext = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
