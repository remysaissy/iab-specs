use serde_repr::{Deserialize_repr, Serialize_repr};

/// Display context type.
///
/// Context in which a display ad appears.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayContextType {
    /// Content-centric context (e.g., newsfeed, article)
    ContentCentric = 1,

    /// Social-centric context (e.g., social network feed)
    SocialCentric = 2,

    /// Product context (e.g., product details, reviews)
    ProductContext = 3,
}
