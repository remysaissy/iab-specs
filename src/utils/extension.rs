//! Extension Trait
//!
//! This module defines the `Extension` trait that all extension objects must implement.
//! The trait ensures that extension types are serializable, deserializable, cloneable,
//! debuggable, and have default values.
//!
//! # Examples
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use iab_specs::Extension;
//!
//! // Custom extension type
//! #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
//! struct MyExtension {
//!     custom_field: String,
//!     vendor_id: i32,
//! }
//!
//! // Automatically implements Extension trait
//! fn use_extension<E: Extension>(ext: E) {
//!     println!("{:?}", ext);
//! }
//! ```

use serde::{Deserialize, Serialize};

/// Extension trait for extension objects.
///
/// This trait defines the requirements for any type that can be used as an extension
/// field (`ext`). All extension types must be:
///
/// - **Serializable**: Can be converted to various formats (JSON, YAML, etc.)
/// - **Deserializable**: Can be parsed from various formats
/// - **Cloneable**: Can be duplicated
/// - **Debuggable**: Can be formatted for debugging
/// - **Default**: Has a sensible default value
/// - **Comparable**: Can be compared for equality
/// - **Send**: Can be transferred between threads
/// - **Sync**: Can be shared between threads
///
/// # Automatic Implementation
///
/// Any type that implements `Serialize`, `Deserialize`, `Clone`, `Debug`, `Default`,
/// `PartialEq`, `Send`, and `Sync` automatically implements `Extension`.
pub trait Extension:
    Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug + Default + PartialEq + Send + Sync
{
}

// Blanket implementation for any type that satisfies the trait bounds
impl<T> Extension for T where
    T: Serialize
        + for<'de> Deserialize<'de>
        + Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Send
        + Sync
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
    struct TestExtension {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_custom_type_implements_extension() {
        fn requires_extension<E: Extension>(_ext: E) {}

        let ext = TestExtension {
            field1: "test".to_string(),
            field2: 42,
        };

        requires_extension(ext);
    }

    #[test]
    fn test_json_value_implements_extension() {
        fn requires_extension<E: Extension>(_ext: E) {}

        let ext = serde_json::json!({"key": "value"});
        requires_extension(ext);
    }

    #[test]
    fn test_unit_type_implements_extension() {
        fn requires_extension<E: Extension>(_ext: E) {}

        requires_extension(());
    }

    #[test]
    fn test_option_implements_extension() {
        fn requires_extension<E: Extension>(_ext: Option<E>) {}

        let ext: Option<TestExtension> = Some(TestExtension::default());
        requires_extension(ext);
    }
}
