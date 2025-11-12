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
