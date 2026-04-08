use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::TaskState;

pub const VALID_TASK_TRANSITIONS: &[(TaskState, TaskState)] = &[
    (TaskState::Working, TaskState::InputRequired),
    (TaskState::Working, TaskState::Completed),
    (TaskState::Working, TaskState::Failed),
    (TaskState::Working, TaskState::Cancelled),
    (TaskState::InputRequired, TaskState::Working),
    (TaskState::InputRequired, TaskState::Cancelled),
];

pub fn can_transition_task(from: &TaskState, to: &TaskState) -> bool {
    VALID_TASK_TRANSITIONS
        .iter()
        .any(|(f, t)| f == from && t == to)
}

pub fn valid_task_transitions_from(state: &TaskState) -> Vec<TaskState> {
    VALID_TASK_TRANSITIONS
        .iter()
        .filter(|(f, _)| f == state)
        .map(|(_, t)| *t)
        .collect()
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
pub struct TaskTransition {
    pub from: TaskState,
    pub to: TaskState,
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

impl TaskTransition {
    pub fn builder() -> TaskTransitionBuilder {
        TaskTransitionBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_transitions() {
        let valid_pairs = vec![
            (TaskState::Working, TaskState::InputRequired),
            (TaskState::Working, TaskState::Completed),
            (TaskState::Working, TaskState::Failed),
            (TaskState::Working, TaskState::Cancelled),
            (TaskState::InputRequired, TaskState::Working),
            (TaskState::InputRequired, TaskState::Cancelled),
        ];

        for (from, to) in &valid_pairs {
            assert!(
                can_transition_task(from, to),
                "Expected valid transition from {:?} to {:?}",
                from,
                to
            );
        }

        assert_eq!(valid_pairs.len(), VALID_TASK_TRANSITIONS.len());
    }

    #[test]
    fn test_invalid_transition_per_state() {
        assert!(!can_transition_task(
            &TaskState::Working,
            &TaskState::Working
        ));

        assert!(!can_transition_task(
            &TaskState::InputRequired,
            &TaskState::Completed
        ));

        assert!(!can_transition_task(
            &TaskState::InputRequired,
            &TaskState::Failed
        ));

        assert!(!can_transition_task(
            &TaskState::Completed,
            &TaskState::Working
        ));

        assert!(!can_transition_task(
            &TaskState::Failed,
            &TaskState::Working
        ));

        assert!(!can_transition_task(
            &TaskState::Cancelled,
            &TaskState::Working
        ));
    }

    #[test]
    fn test_terminal_states_have_no_transitions() {
        let terminal_states = vec![
            TaskState::Completed,
            TaskState::Failed,
            TaskState::Cancelled,
        ];

        for state in &terminal_states {
            let transitions = valid_task_transitions_from(state);
            assert!(
                transitions.is_empty(),
                "Terminal state {:?} should have no outgoing transitions, but found: {:?}",
                state,
                transitions
            );
        }
    }

    #[test]
    fn test_working_transitions() {
        let transitions = valid_task_transitions_from(&TaskState::Working);
        assert_eq!(transitions.len(), 4);
        assert!(transitions.contains(&TaskState::InputRequired));
        assert!(transitions.contains(&TaskState::Completed));
        assert!(transitions.contains(&TaskState::Failed));
        assert!(transitions.contains(&TaskState::Cancelled));
    }

    #[test]
    fn test_input_required_transitions() {
        let transitions = valid_task_transitions_from(&TaskState::InputRequired);
        assert_eq!(transitions.len(), 2);
        assert!(transitions.contains(&TaskState::Working));
        assert!(transitions.contains(&TaskState::Cancelled));
    }

    #[test]
    fn test_transition_struct_roundtrip() {
        let transition = TaskTransition::builder()
            .from(TaskState::Working)
            .to(TaskState::Completed)
            .timestamp("2025-03-31T12:00:00Z")
            .reason("Task finished successfully")
            .actor("agent-001")
            .build()
            .unwrap();

        let json = serde_json::to_string(&transition).unwrap();
        let parsed: TaskTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(transition, parsed);
    }

    #[test]
    fn test_self_transitions_are_invalid() {
        // Spec: A2A Protocol — self-transitions are invalid
        let all_states = [
            TaskState::Working,
            TaskState::InputRequired,
            TaskState::Completed,
            TaskState::Failed,
            TaskState::Cancelled,
        ];
        for state in &all_states {
            assert!(
                !can_transition_task(state, state),
                "Self-transition {:?} -> {:?} should be invalid",
                state,
                state
            );
        }
    }

    #[test]
    fn test_task_transition_minimal() {
        // Spec: A2A Protocol — TaskTransition optional fields omitted when absent
        let transition = TaskTransition::builder()
            .from(TaskState::Working)
            .to(TaskState::Completed)
            .build()
            .unwrap();

        let json = serde_json::to_string(&transition).unwrap();
        assert!(!json.contains("timestamp"));
        assert!(!json.contains("reason"));
        assert!(!json.contains("actor"));

        let parsed: TaskTransition = serde_json::from_str(&json).unwrap();
        assert_eq!(transition, parsed);
    }

    #[test]
    fn test_task_transition_default() {
        // Spec: A2A Protocol — TaskTransition default state
        let transition = TaskTransition::default();
        assert_eq!(transition.from, TaskState::Working);
        assert_eq!(transition.to, TaskState::Working);
        assert!(transition.timestamp.is_none());
        assert!(transition.reason.is_none());
        assert!(transition.actor.is_none());
    }
}
