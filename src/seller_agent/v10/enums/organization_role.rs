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
}
