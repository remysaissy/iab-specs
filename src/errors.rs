use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    UninitializedFieldError(&'static str),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    SerdePlainError(#[from] serde_plain::Error),

    #[error("{0}")]
    StdFmtError(#[from] std::fmt::Error),
}

impl From<derive_builder::UninitializedFieldError> for Error {
    fn from(e: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedFieldError(e.field_name())
    }
}

/// Alias for a `Result` with the error type `iab_specs::Error`.
pub type Result<T> = result::Result<T, serde_json::Error>;
