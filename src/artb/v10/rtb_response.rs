use super::metadata::Metadata;
use super::mutation::Mutation;
use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ARTB RTB Response returned by an agent to the orchestrator.
///
/// Contains the agent's proposed mutations to the OpenRTB payload,
/// along with response metadata. Each mutation is atomic and can be
/// accepted or rejected independently by the orchestrator.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to `serde_json::Value`.
///
/// # Example
///
/// ```
/// use iab_specs::artb::v10::{
///     RTBResponse, Mutation, Metadata, Intent, Operation, IDsPayload
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let response = RTBResponse::builder()
///     .id("req-12345".to_string())
///     .mutations(vec![
///         Mutation::builder()
///             .intent(Intent::ActivateSegments)
///             .op(Operation::Add)
///             .path("/user/data/segment".to_string())
///             .ids(Some(IDsPayload::builder()
///                 .id(vec!["seg-001".to_string()])
///                 .build()?))
///             .build()?,
///     ])
///     .metadata(Some(Metadata::builder()
///         .api_version("1.0")
///         .model_version("v0.10.0")
///         .build()?))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(
    serialize = "P: Extension, Ext: Extension",
    deserialize = "P: Extension, Ext: Extension"
))]
pub struct RTBResponse<P: Extension = crate::DefaultExt, Ext: Extension = crate::DefaultExt> {
    /// Must match the `id` from the corresponding `RTBRequest`.
    /// **Required field**
    #[builder(setter(into))]
    pub id: String,

    /// List of proposed mutations to the OpenRTB payload.
    #[builder(default, setter(into))]
    pub mutations: Vec<Mutation<P, Ext>>,

    /// Response metadata (API version, model version).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metadata: Option<Metadata<Ext>>,

    /// Extension object for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl RTBResponse {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> RTBResponseBuilder<crate::DefaultExt, crate::DefaultExt> {
        RTBResponseBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artb::v10::enums::{Intent, Operation};
    use crate::artb::v10::ids_payload::IDsPayload;

    #[test]
    fn test_rtb_response_creation() {
        let response = RTBResponse::builder()
            .id("req-001".to_string())
            .mutations(vec![
                Mutation::builder()
                    .intent(Intent::ActivateSegments)
                    .op(Operation::Add)
                    .path("/user/data/segment".to_string())
                    .ids(Some(
                        IDsPayload::builder()
                            .id(vec!["seg-1".to_string()])
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .metadata(Some(
                Metadata::builder()
                    .api_version("1.0")
                    .model_version("v0.10.0")
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(response.id, "req-001");
        assert_eq!(response.mutations.len(), 1);
        assert!(response.metadata.is_some());
    }

    #[test]
    fn test_rtb_response_empty_mutations() {
        let response = RTBResponse::builder()
            .id("req-002".to_string())
            .build()
            .unwrap();

        assert_eq!(response.id, "req-002");
        assert!(response.mutations.is_empty());
        assert!(response.metadata.is_none());
    }

    #[test]
    fn test_rtb_response_multiple_mutations() {
        let response = RTBResponse::builder()
            .id("req-003".to_string())
            .mutations(vec![
                Mutation::builder()
                    .intent(Intent::ActivateSegments)
                    .op(Operation::Add)
                    .path("/user/data/segment".to_string())
                    .ids(Some(
                        IDsPayload::builder()
                            .id(vec!["seg-1".to_string()])
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
                Mutation::builder()
                    .intent(Intent::ActivateDeals)
                    .op(Operation::Add)
                    .path("/imp/imp-1".to_string())
                    .ids(Some(
                        IDsPayload::builder()
                            .id(vec!["deal-100".to_string()])
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(response.mutations.len(), 2);
        assert_eq!(response.mutations[0].intent, Intent::ActivateSegments);
        assert_eq!(response.mutations[1].intent, Intent::ActivateDeals);
    }

    #[test]
    fn test_rtb_response_serialization() {
        let response = RTBResponse::builder()
            .id("req-004".to_string())
            .mutations(vec![
                Mutation::builder()
                    .intent(Intent::BidShade)
                    .op(Operation::Replace)
                    .path("/seatbid/0/bid/1".to_string())
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"id\":\"req-004\""));
        assert!(json.contains("\"mutations\""));
        assert!(json.contains("\"intent\":6"));
    }

    #[test]
    fn test_rtb_response_deserialization() {
        let json = r#"{
            "id": "req-005",
            "mutations": [
                {
                    "intent": 1,
                    "op": 1,
                    "path": "/user/data/segment",
                    "ids": {"id": ["seg-1"]}
                }
            ],
            "metadata": {
                "api_version": "1.0",
                "model_version": "v1.0.0"
            }
        }"#;

        let response: RTBResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "req-005");
        assert_eq!(response.mutations.len(), 1);
        assert_eq!(response.mutations[0].intent, Intent::ActivateSegments);
        assert!(response.metadata.is_some());
    }

    #[test]
    fn test_rtb_response_roundtrip() {
        let response = RTBResponse::builder()
            .id("req-006".to_string())
            .mutations(vec![
                Mutation::builder()
                    .intent(Intent::ActivateSegments)
                    .op(Operation::Add)
                    .path("/user/data/segment".to_string())
                    .ids(Some(
                        IDsPayload::builder()
                            .id(vec!["seg-1".to_string()])
                            .build()
                            .unwrap(),
                    ))
                    .build()
                    .unwrap(),
            ])
            .metadata(Some(
                Metadata::builder().api_version("1.0").build().unwrap(),
            ))
            .build()
            .unwrap();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: RTBResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response, parsed);
    }

    #[test]
    fn test_rtb_response_default() {
        let response = RTBResponse::builder().build().unwrap();
        assert!(response.id.is_empty());
        assert!(response.mutations.is_empty());
        assert!(response.metadata.is_none());
    }
}
