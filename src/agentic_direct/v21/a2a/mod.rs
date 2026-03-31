//! A2A (Autonomous-to-Autonomous) protocol messages

pub mod agent_card;
pub mod task;
pub mod task_state_machine;

pub use agent_card::{
    AgentCapabilities, AgentCapabilitiesBuilder, AgentCard, AgentCardBuilder, AgentInterface,
    AgentInterfaceBuilder, SecurityScheme, SecuritySchemeBuilder, Skill, SkillBuilder,
};
pub use task::{
    A2AArtifact, A2AArtifactBuilder, A2AArtifactPart, A2AArtifactPartBuilder, A2ATask,
    A2ATaskBuilder,
};
pub use task_state_machine::{
    can_transition_task, valid_task_transitions_from, TaskTransition, TaskTransitionBuilder,
    VALID_TASK_TRANSITIONS,
};
