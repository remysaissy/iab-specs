//! Embedding envelope for transport (model, context, embeddings) and campaign scoring types.

mod embedding_context;
mod embedding_model;

pub use embedding_context::{EmbeddingContext, EmbeddingContextBuilder};
pub use embedding_model::{EmbeddingModel, EmbeddingModelBuilder};

mod embedding;
pub use embedding::{Embedding, EmbeddingBuilder};
