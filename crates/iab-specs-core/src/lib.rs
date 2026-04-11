//! Core types, traits, and error handling for the iab-specs ecosystem.
//!
//! This crate provides the shared foundation used by all `iab-specs-*` sub-crates:
//! - [`Extension`] trait for type-safe extension fields
//! - [`DefaultExt`] type alias (`Vec<u8>`) for opaque byte extensions
//! - [`Error`] and [`Result`] types for error handling
//! - [`slice_up_to!`] macro for safe string slicing

mod errors;
mod extension;

pub use errors::*;
pub use extension::*;

#[macro_export]
macro_rules! slice_up_to {
    ($content:expr, $max_len:expr) => {
        $content[..std::cmp::min($content.len(), $max_len)].as_ref()
    };
}
