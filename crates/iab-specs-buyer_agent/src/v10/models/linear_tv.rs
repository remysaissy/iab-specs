use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// LinearTVParams captures specific parameters for linear TV inventory, including flighting schedules,
/// cancellation terms, and makegood policies.
///
/// Linear TV parameters define the operational constraints and policies for broadcast television
/// campaigns, including when ads will air, what happens if they don't, and compensation mechanisms.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
///
/// # Example
///
/// ```
/// use iab_specs_buyer_agent::v10::models::LinearTVParams;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let params = LinearTVParams::builder()
///     .flighting(Some(serde_json::json!({
///         "schedule": [
///             {"start": "2024-04-01", "end": "2024-04-30"},
///             {"start": "2024-06-01", "end": "2024-06-30"}
///         ]
///     })))
///     .cancellation_terms("30 days notice required")
///     .makegood_policy("50% value replacement")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
pub struct LinearTVParams<Ext: Extension = crate::DefaultExt> {
    /// Flighting schedules as an arbitrary JSON blob (e.g., start/end dates, blackout periods).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flighting: Option<serde_json::Value>,

    /// Cancellation terms and notice requirements (e.g., "30 days notice required").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub cancellation_terms: Option<String>,

    /// Makegood policy for unfilled or underperforming inventory (e.g., "50% value replacement").
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub makegood_policy: Option<String>,

    /// Extension object for linear TV-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl LinearTVParams {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> LinearTVParamsBuilder {
        LinearTVParamsBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_tv_params_default_empty() {
        let params = LinearTVParams::builder().build().unwrap();

        assert!(params.flighting.is_none());
        assert!(params.cancellation_terms.is_none());
        assert!(params.makegood_policy.is_none());
        assert!(params.ext.is_none());
    }

    #[test]
    fn test_linear_tv_params_with_flighting_json() {
        let flighting = serde_json::json!({
            "schedule": [
                {"start": "2024-04-01", "end": "2024-04-30"},
                {"start": "2024-06-01", "end": "2024-06-30"}
            ]
        });

        let params = LinearTVParams::builder()
            .flighting(Some(flighting.clone()))
            .build()
            .unwrap();

        assert_eq!(params.flighting, Some(flighting));
        assert!(params.cancellation_terms.is_none());
        assert!(params.makegood_policy.is_none());
    }

    #[test]
    fn test_linear_tv_params_with_cancellation_terms() {
        let params = LinearTVParams::builder()
            .cancellation_terms("30 days notice required")
            .build()
            .unwrap();

        assert!(params.flighting.is_none());
        assert_eq!(
            params.cancellation_terms,
            Some("30 days notice required".to_string())
        );
        assert!(params.makegood_policy.is_none());
    }

    #[test]
    fn test_linear_tv_params_with_makegood_policy() {
        let params = LinearTVParams::builder()
            .makegood_policy("50% value replacement")
            .build()
            .unwrap();

        assert!(params.flighting.is_none());
        assert!(params.cancellation_terms.is_none());
        assert_eq!(
            params.makegood_policy,
            Some("50% value replacement".to_string())
        );
    }

    #[test]
    fn test_linear_tv_params_full() {
        let flighting = serde_json::json!({
            "schedule": [
                {"start": "2024-04-01", "end": "2024-04-30"},
                {"start": "2024-06-01", "end": "2024-06-30"}
            ],
            "blackout_periods": ["2024-05-01", "2024-05-02"]
        });

        let params = LinearTVParams::builder()
            .flighting(Some(flighting.clone()))
            .cancellation_terms("45 days notice required")
            .makegood_policy("100% make-good guarantee")
            .build()
            .unwrap();

        assert_eq!(params.flighting, Some(flighting));
        assert_eq!(
            params.cancellation_terms,
            Some("45 days notice required".to_string())
        );
        assert_eq!(
            params.makegood_policy,
            Some("100% make-good guarantee".to_string())
        );
        assert!(params.ext.is_none());
    }

    #[test]
    fn test_linear_tv_params_full_roundtrip() {
        let flighting = serde_json::json!({
            "schedule": [
                {
                    "start": "2024-04-01",
                    "end": "2024-04-30",
                    "frequency": 5
                }
            ]
        });

        let params = LinearTVParams::builder()
            .flighting(Some(flighting.clone()))
            .cancellation_terms("30 days notice")
            .makegood_policy("50% replacement")
            .build()
            .unwrap();

        // Serialize to JSON and deserialize
        let json = serde_json::to_string(&params).unwrap();
        let parsed: LinearTVParams = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.flighting, Some(flighting));
        assert_eq!(
            parsed.cancellation_terms,
            Some("30 days notice".to_string())
        );
        assert_eq!(parsed.makegood_policy, Some("50% replacement".to_string()));
    }

    #[test]
    fn test_linear_tv_params_complex_flighting_json() {
        let flighting = serde_json::json!({
            "daypart_schedule": {
                "monday_friday": {
                    "morning": {"start": "06:00", "end": "12:00", "cpm": 5.50},
                    "afternoon": {"start": "12:00", "end": "18:00", "cpm": 6.00},
                    "evening": {"start": "18:00", "end": "23:00", "cpm": 8.00}
                },
                "weekend": {
                    "prime": {"start": "19:00", "end": "23:00", "cpm": 10.00}
                }
            },
            "flight_dates": [
                {"start": "2024-04-01", "end": "2024-04-30", "impressions": 1000000},
                {"start": "2024-06-01", "end": "2024-06-30", "impressions": 1200000}
            ],
            "blackout_periods": ["2024-05-01", "2024-07-04"],
            "program_restrictions": ["news", "documentaries"],
            "demographic_targeting": {
                "age": "25-54",
                "gender": "all",
                "income": "middle-upper"
            }
        });

        let params = LinearTVParams::builder()
            .flighting(Some(flighting.clone()))
            .build()
            .unwrap();

        // Verify the complex JSON is preserved
        assert_eq!(params.flighting, Some(flighting));

        // Verify roundtrip preserves all nested structure
        let json = serde_json::to_string(&params).unwrap();
        let parsed: LinearTVParams = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.flighting, params.flighting);
    }

    #[test]
    fn test_linear_tv_params_serialization_skips_none() {
        let params = LinearTVParams::builder()
            .cancellation_terms("30 days")
            .build()
            .unwrap();

        let json = serde_json::to_string(&params).unwrap();

        // Verify that None fields are not serialized
        assert!(!json.contains("flighting"));
        assert!(!json.contains("makegood_policy"));
        assert!(!json.contains("ext"));

        // Verify that the set field is present
        assert!(json.contains("\"cancellation_terms\":\"30 days\""));
    }

    #[test]
    fn test_linear_tv_params_cancellation_terms_into_converter() {
        // Test the `into` converter for String
        let params = LinearTVParams::builder()
            .cancellation_terms("Flexible terms")
            .build()
            .unwrap();

        assert_eq!(
            params.cancellation_terms,
            Some("Flexible terms".to_string())
        );
    }

    #[test]
    fn test_linear_tv_params_makegood_policy_into_converter() {
        // Test the `into` converter for String
        let params = LinearTVParams::builder()
            .makegood_policy("Full replacement")
            .build()
            .unwrap();

        assert_eq!(params.makegood_policy, Some("Full replacement".to_string()));
    }

    #[test]
    fn test_linear_tv_params_explicit_default_trait() {
        let params: LinearTVParams = LinearTVParams::default();
        assert!(params.flighting.is_none());
        assert!(params.cancellation_terms.is_none());
        assert!(params.makegood_policy.is_none());
        assert!(params.ext.is_none());
    }

    #[test]
    fn test_linear_tv_params_deserialization_from_raw_json() {
        let json = r#"{"cancellation_terms":"14 days","makegood_policy":"full replacement"}"#;
        let params: LinearTVParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.cancellation_terms, Some("14 days".to_string()));
        assert_eq!(params.makegood_policy, Some("full replacement".to_string()));
        assert!(params.flighting.is_none());
    }

    #[test]
    fn test_linear_tv_params_null_flighting_in_json() {
        let json = r#"{"flighting":null}"#;
        let params: LinearTVParams = serde_json::from_str(json).unwrap();
        assert!(params.flighting.is_none());
    }

    #[test]
    fn test_linear_tv_params_with_json_extension() {
        let params = LinearTVParamsBuilder::<serde_json::Value>::default()
            .cancellation_terms("30 days")
            .ext(Some(Box::new(serde_json::json!({"network": "NBC"}))))
            .build()
            .unwrap();

        assert!(params.ext.is_some());
        assert_eq!(params.ext.as_ref().unwrap()["network"], "NBC");

        let json = serde_json::to_string(&params).unwrap();
        let parsed: LinearTVParams<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ext.as_ref().unwrap()["network"], "NBC");
    }
}
