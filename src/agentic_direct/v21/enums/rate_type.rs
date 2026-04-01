use serde::{Deserialize, Serialize};

/// Type of pricing model for advertising inventory.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum RateType {
    /// Cost per thousand impressions (CPM).
    #[default]
    Cpm,
    /// Cost per click (CPC).
    Cpc,
    /// Cost per action (CPA).
    Cpa,
    /// Flat rate.
    Flat,
    /// Cost per completed video view (CPV).
    CpvCompleted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("cpm", RateType::Cpm),
            ("cpc", RateType::Cpc),
            ("cpa", RateType::Cpa),
            ("flat", RateType::Flat),
            ("cpv_completed", RateType::CpvCompleted),
        ];

        for (json_str, expected) in values {
            let result: RateType = serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<RateType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            RateType::Cpm,
            RateType::Cpc,
            RateType::Cpa,
            RateType::Flat,
            RateType::CpvCompleted,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: RateType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = RateType::default();
        assert_eq!(default, RateType::Cpm, "Default should be Cpm");
    }
}
