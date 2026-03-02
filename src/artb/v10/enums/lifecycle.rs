use serde_repr::{Deserialize_repr, Serialize_repr};

/// Auction lifecycle stage indicating when in the bidstream processing
/// the agent is being invoked.
///
/// This determines what data is available to the agent and what types
/// of mutations it can propose.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Lifecycle {
    /// Unspecified lifecycle stage.
    #[default]
    Unspecified = 0,

    /// During bid request processing (publisher/SSP side).
    /// The agent receives the OpenRTB bid request and can propose
    /// mutations before it is sent to DSPs.
    PublisherBidRequest = 1,

    /// During bid response processing (DSP side).
    /// The agent receives both the bid request and bid response,
    /// and can propose mutations to the response.
    DspBidResponse = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_values() {
        for value in 0..=2 {
            let json = format!("{}", value);
            let result: Result<Lifecycle, _> = serde_json::from_str(&json);
            assert!(
                result.is_ok(),
                "Valid value {} should deserialize successfully",
                value
            );
        }
    }

    #[test]
    fn test_invalid_value_out_of_range() {
        let json = "99";
        let result: Result<Lifecycle, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Value 99 is out of range and should fail deserialization"
        );
    }

    #[test]
    fn test_invalid_value_negative() {
        let json = "-1";
        let result: Result<Lifecycle, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Negative values should fail deserialization"
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            Lifecycle::Unspecified,
            Lifecycle::PublisherBidRequest,
            Lifecycle::DspBidResponse,
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Lifecycle = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn test_default_value() {
        let default = Lifecycle::default();
        assert_eq!(
            default,
            Lifecycle::Unspecified,
            "Default should be Unspecified"
        );
    }

    #[test]
    fn test_specific_values() {
        let json = "0";
        let result: Lifecycle = serde_json::from_str(json).unwrap();
        assert_eq!(result, Lifecycle::Unspecified);

        let json = "1";
        let result: Lifecycle = serde_json::from_str(json).unwrap();
        assert_eq!(result, Lifecycle::PublisherBidRequest);

        let json = "2";
        let result: Lifecycle = serde_json::from_str(json).unwrap();
        assert_eq!(result, Lifecycle::DspBidResponse);
    }
}
