//! Signal taxonomy enums and embedding type classification.

mod composition_type;
mod distance_metric;
mod embedding_type;
mod model_type;
mod normalization_type;
mod temporal_scope;

pub use composition_type::CompositionType;
pub use distance_metric::DistanceMetric;
pub use embedding_type::EmbeddingType;
pub use model_type::ModelType;
pub use normalization_type::NormalizationType;
pub use temporal_scope::TemporalScope;
