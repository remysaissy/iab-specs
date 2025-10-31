/// OpenRTB 2.5 Data Objects
///
/// This module implements Data and Segment objects for user targeting data.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Segment object representing a specific data point (OpenRTB 2.5 Section 3.2.22)
///
/// A `Segment` object represents a specific data segment about a user from a data provider.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::Segment;
///
/// let segment = Segment {
///     id: Some("123".to_string()),
///     name: Some("Auto Enthusiasts".to_string()),
///     value: Some("high".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Segment {
    /// ID of the data segment; specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Name of the data segment; specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// String representation of the data segment value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

/// Data object representing additional user targeting data (OpenRTB 2.5 Section 3.2.21)
///
/// A `Data` object contains a collection of user data segments from a specific data provider.
///
/// # Example
///
/// ```
/// use iab_specs::openrtb::v25::{Data, Segment};
///
/// let data = Data {
///     id: Some("provider1".to_string()),
///     name: Some("DataProvider Inc".to_string()),
///     segment: Some(vec![
///         Segment {
///             id: Some("123".to_string()),
///             name: Some("Auto Enthusiasts".to_string()),
///             ..Default::default()
///         }
///     ]),
///     ..Default::default()
/// };
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"))]
pub struct Data {
    /// Exchange-specific ID for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,

    /// Exchange-specific name for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,

    /// Array of Segment objects that contain the actual data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub segment: Option<Vec<Segment>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_creation() {
        let segment = Segment {
            id: Some("123".to_string()),
            name: Some("Auto Enthusiasts".to_string()),
            value: Some("high".to_string()),
            ..Default::default()
        };

        assert_eq!(segment.id, Some("123".to_string()));
        assert_eq!(segment.name, Some("Auto Enthusiasts".to_string()));
        assert_eq!(segment.value, Some("high".to_string()));
    }

    #[test]
    fn test_segment_serialization() {
        let segment = Segment {
            id: Some("456".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&segment).unwrap();
        assert!(json.contains("\"id\":\"456\""));
    }

    #[test]
    fn test_data_creation() {
        let segment = Segment {
            id: Some("123".to_string()),
            ..Default::default()
        };

        let data = Data {
            id: Some("provider1".to_string()),
            name: Some("DataProvider Inc".to_string()),
            segment: Some(vec![segment]),
            ..Default::default()
        };

        assert_eq!(data.id, Some("provider1".to_string()));
        assert_eq!(data.name, Some("DataProvider Inc".to_string()));
        assert!(data.segment.is_some());
        assert_eq!(data.segment.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_data_serialization() {
        let data = Data {
            id: Some("provider1".to_string()),
            name: Some("DataProvider Inc".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"id\":\"provider1\""));
        assert!(json.contains("\"name\":\"DataProvider Inc\""));
    }

    #[test]
    fn test_data_deserialization() {
        let json = r#"{"id":"provider1","name":"DataProvider Inc"}"#;
        let data: Data = serde_json::from_str(json).unwrap();

        assert_eq!(data.id, Some("provider1".to_string()));
        assert_eq!(data.name, Some("DataProvider Inc".to_string()));
    }
}
