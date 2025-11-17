use serde_repr::{Deserialize_repr, Serialize_repr};

/// Category taxonomy.
///
/// Taxonomy used for content categorization.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CategoryTaxonomy {
    /// IAB Content Category Taxonomy 1.0
    IabContentCategory1_0 = 1,

    /// IAB Content Category Taxonomy 2.0
    IabContentCategory2_0 = 2,

    /// IAB Ad Product Taxonomy 1.0
    IabAdProduct1_0 = 3,

    /// Publisher-specific proprietary taxonomy
    PublisherSpecific = 4,

    /// IAB Content Category Taxonomy 2.1
    IabContentCategory2_1 = 5,

    /// IAB Content Category Taxonomy 2.2
    IabContentCategory2_2 = 6,

    /// IAB Content Category Taxonomy 3.0
    IabContentCategory3_0 = 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        // Test all valid CategoryTaxonomy values (1-7)
        for value in 1..=7 {
            let json = format!("{}", value);
            let result: Result<CategoryTaxonomy, _> = serde_json::from_str(&json);
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
        let result: Result<CategoryTaxonomy, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 0 is not a valid CategoryTaxonomy and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<CategoryTaxonomy, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }
    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<CategoryTaxonomy, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            CategoryTaxonomy::IabContentCategory1_0,
            CategoryTaxonomy::IabContentCategory2_0,
            CategoryTaxonomy::IabAdProduct1_0,
            CategoryTaxonomy::PublisherSpecific,
            CategoryTaxonomy::IabContentCategory2_1,
            CategoryTaxonomy::IabContentCategory2_2,
            CategoryTaxonomy::IabContentCategory3_0,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: CategoryTaxonomy = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
