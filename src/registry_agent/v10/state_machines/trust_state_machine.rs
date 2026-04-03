use crate::registry_agent::v10::enums::TrustLevel;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// All valid transitions in the Trust lifecycle.
///
/// The trust lifecycle progresses linearly from Unknown through Preferred,
/// with any non-Unknown state reachable to Blocked (terminal).
pub const VALID_TRUST_TRANSITIONS: &[(TrustLevel, TrustLevel)] = &[
    (TrustLevel::Unknown, TrustLevel::Registered),
    (TrustLevel::Registered, TrustLevel::Verified),
    (TrustLevel::Registered, TrustLevel::Blocked),
    (TrustLevel::Verified, TrustLevel::Preferred),
    (TrustLevel::Verified, TrustLevel::Blocked),
    (TrustLevel::Preferred, TrustLevel::Blocked),
];

/// Check if a trust level transition is valid.
pub fn can_transition_trust(from: &TrustLevel, to: &TrustLevel) -> bool {
    VALID_TRUST_TRANSITIONS
        .iter()
        .any(|(valid_from, valid_to)| valid_from == from && valid_to == to)
}

/// Get all valid transitions from a given trust level.
pub fn valid_trust_transitions_from(level: &TrustLevel) -> Vec<TrustLevel> {
    VALID_TRUST_TRANSITIONS
        .iter()
        .filter(|(from, _)| from == level)
        .map(|(_, to)| *to)
        .collect()
}

/// Records a trust level transition with metadata.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct TrustTransition {
    /// Previous trust level.
    pub from: TrustLevel,
    /// New trust level.
    pub to: TrustLevel,
    /// When the transition occurred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub timestamp: Option<String>,
    /// Reason for the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reason: Option<String>,
    /// Who verified/triggered the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub verified_by: Option<String>,
}

impl TrustTransition {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> TrustTransitionBuilder {
        TrustTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_valid_trust_transition() {
        for (from, to) in VALID_TRUST_TRANSITIONS.iter().copied() {
            assert!(
                can_transition_trust(&from, &to),
                "expected transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_invalid_trust_transitions() {
        let invalid_transitions = [
            (TrustLevel::Unknown, TrustLevel::Verified),
            (TrustLevel::Unknown, TrustLevel::Preferred),
            (TrustLevel::Unknown, TrustLevel::Blocked),
            (TrustLevel::Registered, TrustLevel::Preferred),
            (TrustLevel::Registered, TrustLevel::Unknown),
            (TrustLevel::Verified, TrustLevel::Unknown),
            (TrustLevel::Verified, TrustLevel::Registered),
            (TrustLevel::Preferred, TrustLevel::Unknown),
            (TrustLevel::Preferred, TrustLevel::Registered),
            (TrustLevel::Preferred, TrustLevel::Verified),
            (TrustLevel::Blocked, TrustLevel::Unknown),
            (TrustLevel::Blocked, TrustLevel::Registered),
            (TrustLevel::Blocked, TrustLevel::Verified),
            (TrustLevel::Blocked, TrustLevel::Preferred),
        ];

        for (from, to) in invalid_transitions {
            assert!(
                !can_transition_trust(&from, &to),
                "expected transition {:?} -> {:?} to be invalid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_valid_trust_transitions_from_each_state() {
        assert_eq!(
            valid_trust_transitions_from(&TrustLevel::Unknown),
            vec![TrustLevel::Registered]
        );
        assert_eq!(
            valid_trust_transitions_from(&TrustLevel::Registered),
            vec![TrustLevel::Verified, TrustLevel::Blocked]
        );
        assert_eq!(
            valid_trust_transitions_from(&TrustLevel::Verified),
            vec![TrustLevel::Preferred, TrustLevel::Blocked]
        );
        assert_eq!(
            valid_trust_transitions_from(&TrustLevel::Preferred),
            vec![TrustLevel::Blocked]
        );
        assert_eq!(
            valid_trust_transitions_from(&TrustLevel::Blocked),
            Vec::<TrustLevel>::new()
        );
    }

    #[test]
    fn test_terminal_state_blocked_has_no_outgoing() {
        assert!(valid_trust_transitions_from(&TrustLevel::Blocked).is_empty());
    }

    #[test]
    fn test_trust_happy_path() {
        let happy_path = [
            (TrustLevel::Unknown, TrustLevel::Registered),
            (TrustLevel::Registered, TrustLevel::Verified),
            (TrustLevel::Verified, TrustLevel::Preferred),
        ];

        for (from, to) in happy_path {
            assert!(
                can_transition_trust(&from, &to),
                "expected happy path transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_blocked_reachable_from_registered_verified_preferred() {
        assert!(can_transition_trust(
            &TrustLevel::Registered,
            &TrustLevel::Blocked
        ));
        assert!(can_transition_trust(
            &TrustLevel::Verified,
            &TrustLevel::Blocked
        ));
        assert!(can_transition_trust(
            &TrustLevel::Preferred,
            &TrustLevel::Blocked
        ));
    }

    #[test]
    fn test_trust_transition_builder_and_serde_roundtrip() {
        let original = TrustTransition::builder()
            .from(TrustLevel::Unknown)
            .to(TrustLevel::Registered)
            .timestamp("2026-04-01T12:00:00Z")
            .reason("Agent completed registration")
            .verified_by("registry-admin-001")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: TrustTransition = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed, original);
        assert_eq!(parsed.from, TrustLevel::Unknown);
        assert_eq!(parsed.to, TrustLevel::Registered);
        assert_eq!(parsed.timestamp, Some("2026-04-01T12:00:00Z".to_string()));
        assert_eq!(
            parsed.reason,
            Some("Agent completed registration".to_string())
        );
        assert_eq!(parsed.verified_by, Some("registry-admin-001".to_string()));
    }
}
