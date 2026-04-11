# iab-specs-core

Core types, traits, and error handling for the [iab-specs](https://crates.io/crates/iab-specs) ecosystem.

## Overview

This crate provides the shared foundation used by all `iab-specs-*` sub-crates:

- **`Extension` trait** — Marker trait for type-safe extension fields on IAB specification objects. Any type implementing `Serialize + Deserialize + Clone + Debug + Default + PartialEq + Send + Sync` automatically satisfies `Extension`.
- **`DefaultExt`** — Type alias for `Vec<u8>`, the default opaque-byte extension type.
- **`Error` / `Result`** — Shared error types covering builder errors, serialization errors, and invalid state transitions.
- **`slice_up_to!`** — Macro for safe sub-slicing without panics.

## Features

- `serde_json` — Enables the `SerdeJsonError` variant in `Error` (wraps `serde_json::Error`).

## License

Apache-2.0
