use serde_repr::{Deserialize_repr, Serialize_repr};

/// Agent type.
///
/// Type of user agent, distinguishing between human users and automated agents.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AgentType {
    /// Human user
    Human = 1,

    /// Robot, crawler, or spider
    Robot = 2,
}
