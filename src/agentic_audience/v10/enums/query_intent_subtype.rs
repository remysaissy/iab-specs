use serde::{Deserialize, Serialize};

/// Subtypes of query intent signals derived from user search behavior and stated intent.
///
/// Query intent signals capture the explicit intent signaled through search queries, purchase
/// intent signals, and seller-provided offers, providing high-confidence indication of user
/// interest and purchase readiness.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum QueryIntentSubtype {
    /// Signals derived from user search queries and search behavior.
    #[default]
    SearchQuery,
    /// Signals indicating explicit buyer purchase intent and readiness.
    BuyerIntent,
    /// Signals from seller-provided offers and inventory offers.
    SellerOffer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("search_query", QueryIntentSubtype::SearchQuery),
            ("buyer_intent", QueryIntentSubtype::BuyerIntent),
            ("seller_offer", QueryIntentSubtype::SellerOffer),
        ];

        for (json_str, expected) in values {
            let result: QueryIntentSubtype =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<QueryIntentSubtype, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            QueryIntentSubtype::SearchQuery,
            QueryIntentSubtype::BuyerIntent,
            QueryIntentSubtype::SellerOffer,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: QueryIntentSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = QueryIntentSubtype::default();
        assert_eq!(
            default,
            QueryIntentSubtype::SearchQuery,
            "Default should be SearchQuery"
        );
    }

    #[test]
    fn test_integer_value_rejected() {
        // Spec: Agentic Audience v1.0 — enums are string-serialized, integers must be rejected
        let result: Result<QueryIntentSubtype, _> = serde_json::from_str("42");
        assert!(result.is_err(), "Integer value should be rejected");
    }
}
