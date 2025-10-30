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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uninitialized_field_error() {
        let err = Error::UninitializedFieldError("test_field");
        assert_eq!(err.to_string(), "test_field");
    }

    #[test]
    fn test_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(json_err.is_err());
        let err: Error = json_err.unwrap_err().into();
        assert!(err.to_string().contains("expected value"));
    }

    #[test]
    fn test_serde_plain_error() {
        use serde::de::Error as _;
        let plain_err = serde_plain::Error::custom("custom error");
        let err: Error = plain_err.into();
        assert_eq!(err.to_string(), "custom error");
    }

    #[test]
    fn test_from_builder_uninitialized_field_error() {
        use derive_builder::UninitializedFieldError;
        let builder_err = UninitializedFieldError::new("field_name");
        let err: Error = builder_err.into();
        assert_eq!(err.to_string(), "field_name");
    }
}
