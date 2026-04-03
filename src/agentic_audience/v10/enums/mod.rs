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

mod contextual_signal_subtype;
mod creative_signal_subtype;
mod identity_signal_subtype;
mod inventory_signal_subtype;
mod query_intent_subtype;
mod reinforcement_signal_subtype;
mod signal_type;

pub use contextual_signal_subtype::ContextualSignalSubtype;
pub use creative_signal_subtype::CreativeSignalSubtype;
pub use identity_signal_subtype::IdentitySignalSubtype;
pub use inventory_signal_subtype::InventorySignalSubtype;
pub use query_intent_subtype::QueryIntentSubtype;
pub use reinforcement_signal_subtype::ReinforcementSignalSubtype;
pub use signal_type::SignalType;
