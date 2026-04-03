use serde::{Deserialize, Serialize};

/// Subtypes of inventory signals related to available advertising placements and supply.
///
/// Inventory signals relate to characteristics of the advertising inventory, placement attributes,
/// and audience inventory availability across publisher platforms and demand sources.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum InventorySignalSubtype {
    /// Signals related to publisher identity and publisher properties.
    #[default]
    Publisher,
    /// Signals from specific ad placement characteristics and properties.
    Placement,
    /// Signals indicating availability and characteristics of audience inventory.
    AudienceInventory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        let values = vec![
            ("publisher", InventorySignalSubtype::Publisher),
            ("placement", InventorySignalSubtype::Placement),
            (
                "audience_inventory",
                InventorySignalSubtype::AudienceInventory,
            ),
        ];

        for (json_str, expected) in values {
            let result: InventorySignalSubtype =
                serde_json::from_str(&format!("\"{}\"", json_str)).unwrap();
            assert_eq!(result, expected, "Failed for value: {}", json_str);
        }
    }

    #[test]
    fn test_invalid_value_rejected() {
        let json = "\"nonexistent_value\"";
        let result: Result<InventorySignalSubtype, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Invalid value should be rejected");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = vec![
            InventorySignalSubtype::Publisher,
            InventorySignalSubtype::Placement,
            InventorySignalSubtype::AudienceInventory,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: InventorySignalSubtype = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = InventorySignalSubtype::default();
        assert_eq!(
            default,
            InventorySignalSubtype::Publisher,
            "Default should be Publisher"
        );
    }
}
