//! # Agentic Audience v1.0 (Draft)
//!
//! ⚠️ **Draft Specification**: Based on Agentic Audience v1.0 Draft. Breaking changes may occur.
//!
//! Data models for the embedding exchange protocol, including:
//! - Signal taxonomy enums and embedding type classification
//! - Embedding envelope for transport (model, context, embeddings)
//! - Campaign scoring types (head, request, response, score)
//! - OpenRTB bid stream extension (`EmbeddingSegmentExt`)

pub mod enums;
pub mod models;
pub mod scoring;
