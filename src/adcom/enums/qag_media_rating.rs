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
