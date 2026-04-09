use serde::{Deserialize, Serialize};

/// The role of an organization in the Seller Agent workflow.
///
/// Organizations can have different roles depending on their relationship
/// with the advertising ecosystem. All serialization uses snake_case format
/// (e.g., `"seller"` for `Seller`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationRole {
    /// Organization acting as a buyer of advertising inventory.
    Buyer,

    /// Organization acting as a seller of advertising inventory (default).
    #[default]
    Seller,

    /// Organization acting as an agent facilitating advertising transactions.
    Agent,

    /// Organization acting as a content curator.
    Curator,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let variants = [
            OrganizationRole::Buyer,
            OrganizationRole::Seller,
            OrganizationRole::Agent,
            OrganizationRole::Curator,
        ];

        for variant in &variants {
            let serialized = serde_json::to_string(variant).expect("Failed to serialize");
            assert!(
                serialized.starts_with('"') && serialized.ends_with('"'),
                "Serialized value {} should be a JSON string",
                serialized
            );
            let unquoted = &serialized[1..serialized.len() - 1];
            assert!(
                unquoted.chars().all(|c| c.is_lowercase() || c == '_'),
                "Serialized value {} should be snake_case",
                unquoted
            );
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_role\"";
        let result: Result<OrganizationRole, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid role should fail deserialization");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let variants = [
            OrganizationRole::Buyer,
            OrganizationRole::Seller,
            OrganizationRole::Agent,
            OrganizationRole::Curator,
        ];

        for original in &variants {
            let serialized = serde_json::to_string(original).expect("Failed to serialize");
            let deserialized: OrganizationRole =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(
                original, &deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = OrganizationRole::default();
        assert_eq!(
            default,
            OrganizationRole::Seller,
            "Default should be Seller"
        );
    }

    /// Seller Agent 1.0 § OrganizationRole — Clone and Copy traits enable value semantics
    #[test]
    fn test_clone_copy_traits() {
        let a = OrganizationRole::Seller;
        let b = a; // Copy semantics
        assert_eq!(a, b);
        assert_eq!(a, OrganizationRole::Seller);
    }

    /// Seller Agent 1.0 § OrganizationRole — Hash trait enables HashSet usage
    #[test]
    fn test_hash_trait_with_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(OrganizationRole::Buyer);
        set.insert(OrganizationRole::Seller);
        set.insert(OrganizationRole::Agent);
        set.insert(OrganizationRole::Curator);

        assert_eq!(set.len(), 4);
        assert!(set.contains(&OrganizationRole::Buyer));
        assert!(set.contains(&OrganizationRole::Curator));
    }

    /// Seller Agent 1.0 § OrganizationRole — PartialEq and Eq verify inequality of different variants
    #[test]
    fn test_eq_different_variants() {
        assert_ne!(OrganizationRole::Buyer, OrganizationRole::Seller);
        assert_ne!(OrganizationRole::Seller, OrganizationRole::Agent);
        assert_ne!(OrganizationRole::Agent, OrganizationRole::Curator);
    }

    /// Seller Agent 1.0 § OrganizationRole — serde rename_all = "snake_case" rejects PascalCase
    #[test]
    fn test_case_sensitivity_rejected() {
        let pascal_case_examples = ["\"Buyer\"", "\"Seller\""];

        for example in &pascal_case_examples {
            let result: Result<OrganizationRole, _> = serde_json::from_str(example);
            assert!(result.is_err(), "PascalCase {} should be rejected", example);
        }
    }

    /// Seller Agent 1.0 § OrganizationRole — Exact snake_case serialization values per spec
    #[test]
    fn test_exact_snake_case_values() {
        let expected = [
            (OrganizationRole::Buyer, "\"buyer\""),
            (OrganizationRole::Seller, "\"seller\""),
            (OrganizationRole::Agent, "\"agent\""),
            (OrganizationRole::Curator, "\"curator\""),
        ];

        for (variant, expected_json) in &expected {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(
                &json, expected_json,
                "Mismatch for {:?}: got {}, expected {}",
                variant, json, expected_json
            );
        }
    }
}
