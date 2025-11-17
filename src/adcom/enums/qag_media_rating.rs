use serde_repr::{Deserialize_repr, Serialize_repr};

/// IQG Media Ratings.
///
/// The content rating from the IAB Quality Assurance Guidelines (IQG) Taxonomy.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum QagMediaRating {
    /// All Audiences
    AllAudiences = 1,

    /// Everyone Over 12
    Over12 = 2,

    /// Mature Audiences (17+)
    Mature = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid QagMediaRating values (1-3)
        for value in 1..=3 {
            let json = format!("{}", value);
            let result: Result<QagMediaRating, _> = serde_json::from_str(&json);
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
        let result: Result<QagMediaRating, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid QagMediaRating and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<QagMediaRating, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<QagMediaRating, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            QagMediaRating::AllAudiences,
            QagMediaRating::Over12,
            QagMediaRating::Mature,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: QagMediaRating = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
