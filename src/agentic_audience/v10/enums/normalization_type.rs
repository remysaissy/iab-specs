use serde::{Deserialize, Serialize};

/// Vector normalization technique applied to embeddings.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationType {
    /// L2 normalization (unit vector scaling, default).
    #[default]
    L2Norm,
    /// Min-max normalization (scales values to [0, 1]).
    MinMax,
    /// Z-score standardization (zero mean, unit variance).
    ZScore,
    /// No normalization applied.
    #[serde(rename = "none")]
    NoNorm,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("l2_norm", NormalizationType::L2Norm),
            ("min_max", NormalizationType::MinMax),
            ("z_score", NormalizationType::ZScore),
            ("none", NormalizationType::NoNorm),
        ];

        for (json_str, expected) in values {
            let result: NormalizationType =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"no_norm\"";
        let result: Result<NormalizationType, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            NormalizationType::L2Norm,
            NormalizationType::MinMax,
            NormalizationType::ZScore,
            NormalizationType::NoNorm,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: NormalizationType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = NormalizationType::default();
        assert_eq!(
            default,
            NormalizationType::L2Norm,
            "Default should be L2Norm"
        );
    }

    #[test]
    fn test_no_norm_serializes_as_none() {
        let no_norm = NormalizationType::NoNorm;
        let json = serde_json::to_string(&no_norm).unwrap();
        assert_eq!(
            json, "\"none\"",
            "NoNorm should serialize as 'none', not 'no_norm'"
        );

        // Verify deserialization works for both forms
        let deserialized: NormalizationType = serde_json::from_str("\"none\"").unwrap();
        assert_eq!(deserialized, NormalizationType::NoNorm);
    }

    #[test]
    fn test_integer_value_rejected() {
        // Spec: Agentic Audience v1.0 — enums are string-serialized, integers must be rejected
        let result: Result<NormalizationType, _> = serde_json::from_str("42");
        assert!(result.is_err(), "Integer value should be rejected");
    }
}
