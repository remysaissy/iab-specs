use serde::{Deserialize, Serialize};

/// Distance metric used for vector similarity computation.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum DistanceMetric {
    /// Cosine similarity (default).
    #[default]
    Cosine,
    /// Dot product similarity.
    Dot,
    /// Euclidean (L2) distance.
    L2,
    /// Manhattan (L1) distance.
    Manhattan,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("cosine", DistanceMetric::Cosine),
            ("dot", DistanceMetric::Dot),
            ("l2", DistanceMetric::L2),
            ("manhattan", DistanceMetric::Manhattan),
        ];

        for (json_str, expected) in values {
            let result: DistanceMetric =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"euclidean\"";
        let result: Result<DistanceMetric, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            DistanceMetric::Cosine,
            DistanceMetric::Dot,
            DistanceMetric::L2,
            DistanceMetric::Manhattan,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: DistanceMetric = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = DistanceMetric::default();
        assert_eq!(default, DistanceMetric::Cosine, "Default should be Cosine");
    }
}
