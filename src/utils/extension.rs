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
//!
//! Many types throughout the crate support extensions, including:
//! - **AdCOM types**: `Ad`, `Placement`, `DistributionChannel`, `Site`, `App`, `User`, `Device`, `Content`, `Publisher`, and many more
//! - **OpenRTB 2.5/2.6 types**: `BidRequest`, `BidResponse`, `Imp`, `Banner`, `Video`, `Audio`, `Site`, `App`, `Device`, `User`, and many more
//! - **OpenRTB 3.0 types**: `Request`, `Response`, `Item`, `Bid`, `SeatBid`, `Source`, `SupplyChain`, and many more
//!
//! # Usage Patterns
//!
//! ## Pattern 1: Default JSON Extensions (Most Flexible)
//!
//! Use `serde_json::Value` for maximum flexibility when you don't need compile-time type safety:
//!
//! ```
//! #[cfg(feature = "adcom")]
//! {
//! use iab_specs::adcom::media::Ad;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! // Use default serde_json::Value for flexible, untyped extensions
//! let ad = Ad::builder()
//!     .id(Some("ad123".to_string()))
//!     .ext(Some(Box::new(serde_json::json!({
//!         "vendor_field": "custom_value",
//!         "internal_id": 12345,
//!         "tracking_data": {
//!             "campaign": "summer_sale"
//!         }
//!     }))))
//!     .build()?;
//!
//! // Serialize to JSON
//! let json = serde_json::to_string(&ad)?;
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## Pattern 2: Custom Typed Extensions (Type Safe)
//!
//! Define your own extension type for compile-time type safety and better documentation:
//!
//! ```
//! #[cfg(feature = "adcom")]
//! {
//! use iab_specs::adcom::media::{Ad, AdBuilder};
//! use serde::{Deserialize, Serialize};
//! use derive_builder::Builder;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! // Define your custom extension type with Builder support
//! #[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
//! struct MyAdExtension {
//!     campaign_id: String,
//!     internal_tracking: i64,
//!     priority: u8,
//! }
//!
//! impl MyAdExtension {
//!     pub fn builder() -> MyAdExtensionBuilder {
//!         MyAdExtensionBuilder::create_empty()
//!     }
//! }
//!
//! // Create extension using builder pattern
//! let my_ext = MyAdExtension::builder()
//!     .campaign_id("camp_123".to_string())
//!     .internal_tracking(999)
//!     .priority(5)
//!     .build()?;
//!
//! // Use your custom type with Ad (using AdBuilder with type parameter)
//! let ad = AdBuilder::<MyAdExtension>::default()
//!     .id(Some("ad456".to_string()))
//!     .ext(Some(Box::new(my_ext)))
//!     .build()?;
//!
//! // Type-safe access to extension fields
//! if let Some(ext) = &ad.ext {
//!     println!("Campaign: {}", ext.campaign_id);
//!     println!("Priority: {}", ext.priority);
//! }
//! # Ok(())
//! # }
//! }
//! ```
//!
//! ## Pattern 3: No Extensions
//!
//! Use the unit type `()` when you don't need extensions at all:
//!
//! ```
//! #[cfg(feature = "adcom")]
//! {
//! use iab_specs::adcom::media::Ad;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! // Omit the ext field when you don't need extensions
//! let ad = Ad::builder()
//!     .id(Some("ad789".to_string()))
//!     .build()?;
//! # Ok(())
//! # }
//! }
//! ```
//!
//! # Implementing Custom Extension Types
//!
//! Any type that implements the required traits automatically implements `Extension`.
//! Use the `Builder` derive macro for ergonomic construction:
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use derive_builder::Builder;
//! use iab_specs::Extension;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! // Derive Builder along with other required traits
//! #[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
//! struct MyExtension {
//!     custom_field: String,
//!     vendor_id: i32,
//! }
//!
//! impl MyExtension {
//!     pub fn builder() -> MyExtensionBuilder {
//!         MyExtensionBuilder::create_empty()
//!     }
//! }
//!
//! // MyExtension now automatically implements Extension
//! fn use_extension<E: Extension>(ext: E) {
//!     println!("{:?}", ext);
//! }
//!
//! // Use builder pattern to create instances
//! let my_ext = MyExtension::builder()
//!     .custom_field("value".to_string())
//!     .vendor_id(42)
//!     .build()?;
//!
//! use_extension(my_ext);
//! # Ok(())
//! # }
//! ```
//!
//! # Extension Type Requirements
//!
//! To implement `Extension`, a type must be:
//! - **Serializable**: Implement `Serialize` from serde
//! - **Deserializable**: Implement `Deserialize` from serde
//! - **Cloneable**: Implement `Clone`
//! - **Debuggable**: Implement `Debug`
//! - **Default**: Implement `Default`
//! - **Comparable**: Implement `PartialEq`
//! - **Send**: Safe to transfer between threads
//! - **Sync**: Safe to share between threads
//!
//! These can all be easily derived using `#[derive(...)]` attributes.

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
