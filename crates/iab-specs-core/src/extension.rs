//! Extension Trait
//!
//! This module defines the `Extension` trait that all extension objects must implement.
//! The trait ensures that extension types are serializable, deserializable, cloneable,
//! debuggable, and have default values.
//!
//! # Overview
//!
//! The `Extension` trait provides a flexible mechanism for adding custom fields to IAB
//! specification objects. This is particularly useful for:
//! - Vendor-specific data
//! - Internal tracking information
//! - Experimental features
//! - Custom business logic fields

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

/// Default extension type.
///
/// `Vec<u8>` is used as the default extension type, representing opaque bytes.
/// This is serde-agnostic: callers choose the serialization format by specifying
/// an explicit type parameter (e.g., `serde_json::Value` for JSON, or a custom
/// protobuf-decoded type) when they need typed extensions.
pub type DefaultExt = Vec<u8>;

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

    #[test]
    fn test_vec_u8_implements_extension() {
        fn requires_extension<E: Extension>(_ext: E) {}

        let ext: Vec<u8> = vec![0x08, 0x96, 0x01];
        requires_extension(ext);
    }

    #[test]
    fn test_default_ext_is_vec_u8() {
        let ext = DefaultExt::default();
        assert!(ext.is_empty());
        assert_eq!(ext, Vec::<u8>::new());
    }
}
