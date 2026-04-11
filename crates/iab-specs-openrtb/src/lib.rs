pub use iab_specs_core::{DefaultExt, Error, Extension, Result};

pub mod common;

#[cfg(feature = "openrtb_25")]
pub mod v25;

#[cfg(feature = "openrtb_26")]
pub mod v26;

#[cfg(feature = "openrtb_30")]
pub mod v30;

pub use common::*;
