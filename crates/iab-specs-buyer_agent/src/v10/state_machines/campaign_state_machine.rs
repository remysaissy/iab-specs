use crate::v10::enums::CampaignStatus;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub const VALID_CAMPAIGN_TRANSITIONS: &[(CampaignStatus, CampaignStatus)] = &[
    (CampaignStatus::Initialized, CampaignStatus::BriefReceived),
    (CampaignStatus::Initialized, CampaignStatus::Cancelled),
    (
        CampaignStatus::BriefReceived,
        CampaignStatus::BudgetAllocated,
    ),
    (CampaignStatus::BriefReceived, CampaignStatus::Cancelled),
    (CampaignStatus::BudgetAllocated, CampaignStatus::Researching),
    (CampaignStatus::BudgetAllocated, CampaignStatus::Cancelled),
    (
        CampaignStatus::Researching,
        CampaignStatus::AwaitingApproval,
    ),
    (CampaignStatus::Researching, CampaignStatus::Cancelled),
    (
        CampaignStatus::AwaitingApproval,
        CampaignStatus::ExecutingBookings,
    ),
    (
        CampaignStatus::AwaitingApproval,
        CampaignStatus::Researching,
    ),
    (CampaignStatus::AwaitingApproval, CampaignStatus::Cancelled),
    (CampaignStatus::ExecutingBookings, CampaignStatus::Completed),
    (CampaignStatus::ExecutingBookings, CampaignStatus::Failed),
    (CampaignStatus::ExecutingBookings, CampaignStatus::Cancelled),
    (CampaignStatus::Failed, CampaignStatus::Cancelled),
];

pub fn can_transition_campaign(from: &CampaignStatus, to: &CampaignStatus) -> bool {
    VALID_CAMPAIGN_TRANSITIONS
        .iter()
        .any(|(valid_from, valid_to)| valid_from == from && valid_to == to)
}

pub fn valid_campaign_transitions_from(state: &CampaignStatus) -> Vec<CampaignStatus> {
    VALID_CAMPAIGN_TRANSITIONS
        .iter()
        .filter_map(|(from, to)| if from == state { Some(*to) } else { None })
        .collect()
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct CampaignTransition {
    pub from: CampaignStatus,

    pub to: CampaignStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub actor: Option<String>,
}

impl CampaignTransition {
    pub fn builder() -> CampaignTransitionBuilder {
        CampaignTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_valid_campaign_transition_returns_true() {
        for (from, to) in VALID_CAMPAIGN_TRANSITIONS.iter().copied() {
            assert!(
                can_transition_campaign(&from, &to),
                "expected transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_invalid_campaign_transition_for_each_state_returns_false() {
        let invalid_transitions = [
            (CampaignStatus::Initialized, CampaignStatus::BudgetAllocated),
            (CampaignStatus::BriefReceived, CampaignStatus::Researching),
            (
                CampaignStatus::BudgetAllocated,
                CampaignStatus::AwaitingApproval,
            ),
            (
                CampaignStatus::Researching,
                CampaignStatus::ExecutingBookings,
            ),
            (CampaignStatus::AwaitingApproval, CampaignStatus::Completed),
            (
                CampaignStatus::ExecutingBookings,
                CampaignStatus::Researching,
            ),
            (CampaignStatus::Completed, CampaignStatus::Cancelled),
            (CampaignStatus::Failed, CampaignStatus::Completed),
            (CampaignStatus::Cancelled, CampaignStatus::Initialized),
        ];

        for (from, to) in invalid_transitions {
            assert!(
                !can_transition_campaign(&from, &to),
                "expected transition {:?} -> {:?} to be invalid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_valid_campaign_transitions_from_non_terminal_states() {
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::Initialized),
            vec![CampaignStatus::BriefReceived, CampaignStatus::Cancelled]
        );
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::BriefReceived),
            vec![CampaignStatus::BudgetAllocated, CampaignStatus::Cancelled]
        );
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::BudgetAllocated),
            vec![CampaignStatus::Researching, CampaignStatus::Cancelled]
        );
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::Researching),
            vec![CampaignStatus::AwaitingApproval, CampaignStatus::Cancelled]
        );
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::AwaitingApproval),
            vec![
                CampaignStatus::ExecutingBookings,
                CampaignStatus::Researching,
                CampaignStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::ExecutingBookings),
            vec![
                CampaignStatus::Completed,
                CampaignStatus::Failed,
                CampaignStatus::Cancelled,
            ]
        );
        assert_eq!(
            valid_campaign_transitions_from(&CampaignStatus::Failed),
            vec![CampaignStatus::Cancelled]
        );
    }

    #[test]
    fn test_terminal_states_have_no_outgoing_transitions() {
        for terminal_state in [CampaignStatus::Completed, CampaignStatus::Cancelled] {
            assert!(valid_campaign_transitions_from(&terminal_state).is_empty());
        }
    }

    #[test]
    fn test_campaign_lifecycle_happy_path() {
        let happy_path = [
            (CampaignStatus::Initialized, CampaignStatus::BriefReceived),
            (
                CampaignStatus::BriefReceived,
                CampaignStatus::BudgetAllocated,
            ),
            (CampaignStatus::BudgetAllocated, CampaignStatus::Researching),
            (
                CampaignStatus::Researching,
                CampaignStatus::AwaitingApproval,
            ),
            (
                CampaignStatus::AwaitingApproval,
                CampaignStatus::ExecutingBookings,
            ),
            (CampaignStatus::ExecutingBookings, CampaignStatus::Completed),
        ];

        for (from, to) in happy_path {
            assert!(
                can_transition_campaign(&from, &to),
                "expected happy path transition {:?} -> {:?} to be valid",
                from,
                to
            );
        }
    }

    #[test]
    fn test_approval_rejection_loops_back_to_research() {
        assert!(can_transition_campaign(
            &CampaignStatus::AwaitingApproval,
            &CampaignStatus::Researching,
        ));
        assert!(!can_transition_campaign(
            &CampaignStatus::AwaitingApproval,
            &CampaignStatus::Completed,
        ));
    }

    #[test]
    fn test_campaign_transition_builder_and_serde_roundtrip() {
        let original = CampaignTransition::builder()
            .from(CampaignStatus::AwaitingApproval)
            .to(CampaignStatus::Researching)
            .timestamp("2026-04-01T12:00:00Z")
            .reason("Human reviewer requested deeper research")
            .actor("buyer-approver-123")
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let parsed: CampaignTransition = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed, original);
        assert_eq!(parsed.from, CampaignStatus::AwaitingApproval);
        assert_eq!(parsed.to, CampaignStatus::Researching);
    }

    /// Buyer Agent 1.0 § Campaign State Machine — exhaustive 9×9 transition matrix
    #[test]
    fn test_exhaustive_9x9_campaign_transition_matrix() {
        let all_states = [
            CampaignStatus::Initialized,
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated,
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings,
            CampaignStatus::Completed,
            CampaignStatus::Failed,
            CampaignStatus::Cancelled,
        ];

        for from in &all_states {
            for to in &all_states {
                let expected = VALID_CAMPAIGN_TRANSITIONS.contains(&(*from, *to));
                assert_eq!(
                    can_transition_campaign(from, to),
                    expected,
                    "Mismatch for {:?} -> {:?}: expected {}, got {}",
                    from,
                    to,
                    expected,
                    !expected
                );
            }
        }
    }

    /// Buyer Agent 1.0 § Campaign State Machine — self-transitions are never valid
    #[test]
    fn test_self_transition_campaign_always_rejected() {
        let all_states = [
            CampaignStatus::Initialized,
            CampaignStatus::BriefReceived,
            CampaignStatus::BudgetAllocated,
            CampaignStatus::Researching,
            CampaignStatus::AwaitingApproval,
            CampaignStatus::ExecutingBookings,
            CampaignStatus::Completed,
            CampaignStatus::Failed,
            CampaignStatus::Cancelled,
        ];

        for state in &all_states {
            assert!(
                !can_transition_campaign(state, state),
                "Self-transition for {:?} should be rejected",
                state
            );
        }
    }

    /// Buyer Agent 1.0 § CampaignTransition — Default trait produces valid zero-state
    #[test]
    fn test_campaign_transition_record_default() {
        let t: CampaignTransition = CampaignTransition::default();
        assert_eq!(t.from, CampaignStatus::Initialized);
        assert_eq!(t.to, CampaignStatus::Initialized);
        assert!(t.timestamp.is_none());
        assert!(t.reason.is_none());
        assert!(t.actor.is_none());
    }

    /// Buyer Agent 1.0 § CampaignTransition — optional fields skipped when None
    #[test]
    fn test_campaign_transition_optional_fields_none() {
        let t = CampaignTransition::builder()
            .from(CampaignStatus::Researching)
            .to(CampaignStatus::AwaitingApproval)
            .build()
            .unwrap();

        assert!(t.timestamp.is_none());
        assert!(t.reason.is_none());
        assert!(t.actor.is_none());

        let json = serde_json::to_string(&t).unwrap();
        assert!(!json.contains("timestamp"));
        assert!(!json.contains("reason"));
        assert!(!json.contains("actor"));
    }
}
