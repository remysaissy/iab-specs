use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DisplayFormat Object (Section 4.3)
///
/// Display creative format constraints including size and expandability.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
#[derive(Builder, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct DisplayFormat<Ext: Extension = serde_json::Value> {
    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Width as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Height as a ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// Directions in which creative can expand
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<i32>>,

    /// Extension object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Box<Ext>>,
}

impl DisplayFormat {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> DisplayFormatBuilder {
        DisplayFormatBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_format_builder() {
        let format = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .build()
            .unwrap();

        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
    }

    #[test]
    fn test_display_format_default() {
        let format = DisplayFormat::builder().build().unwrap();

        assert!(format.w.is_none());
        assert!(format.h.is_none());
        assert!(format.wratio.is_none());
        assert!(format.hratio.is_none());
    }

    #[test]
    fn test_display_format_with_ratio() {
        let format = DisplayFormat::builder()
            .wratio(Some(16))
            .hratio(Some(9))
            .build()
            .unwrap();

        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
    }

    #[test]
    fn test_display_format_with_expansion() {
        let format = DisplayFormat::builder()
            .w(Some(300))
            .h(Some(250))
            .expdir(Some(vec![1, 2, 3, 4]))
            .build()
            .unwrap();

        assert_eq!(format.expdir, Some(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_display_format_serialization() {
        let format = DisplayFormat::builder()
            .w(Some(728))
            .h(Some(90))
            .build()
            .unwrap();

        let json = serde_json::to_string(&format).unwrap();
        assert!(json.contains("\"w\":728"));
        assert!(json.contains("\"h\":90"));
    }

    #[test]
    fn test_display_format_deserialization() {
        let json = r#"{"w":300,"h":250,"wratio":16,"hratio":9}"#;
        let format: DisplayFormat = serde_json::from_str(json).unwrap();

        assert_eq!(format.w, Some(300));
        assert_eq!(format.h, Some(250));
        assert_eq!(format.wratio, Some(16));
        assert_eq!(format.hratio, Some(9));
    }
}
