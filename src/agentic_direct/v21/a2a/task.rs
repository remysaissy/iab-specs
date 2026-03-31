use crate::Extension;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::super::enums::TaskState;

/// Part of an A2A artifact.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(rename_all = "camelCase")]
pub struct A2AArtifactPart {
    /// Content type of this part.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub type_: String,

    /// Content payload.
    #[builder(setter(into))]
    pub content: String,
}

impl A2AArtifactPart {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> A2AArtifactPartBuilder {
        A2AArtifactPartBuilder::create_empty()
    }
}

/// An artifact produced by an A2A task.
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(rename_all = "camelCase")]
pub struct A2AArtifact {
    /// Name of the artifact.
    #[builder(setter(into))]
    pub name: String,

    /// Description of the artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    /// Parts composing this artifact.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub parts: Vec<A2AArtifactPart>,
}

impl A2AArtifact {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> A2AArtifactBuilder {
        A2AArtifactBuilder::create_empty()
    }
}

/// A2A Protocol Task.
///
/// Represents a unit of work submitted to an agent.
///
/// # Generic Parameters
///
/// * `Ext` - Extension object type (must implement [`Extension`]). Defaults to [`DefaultExt`](crate::DefaultExt).
#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[builder(build_fn(error = "crate::Error"), default)]
#[serde(bound(serialize = "Ext: Extension", deserialize = "Ext: Extension"))]
#[serde(rename_all = "camelCase")]
pub struct A2ATask<Ext: Extension = crate::DefaultExt> {
    /// Unique identifier for the task.
    #[builder(setter(into))]
    pub id: String,

    /// Current state of the task.
    pub state: TaskState,

    /// Human-readable message about the task state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub message: Option<String>,

    /// Result payload of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub result: Option<serde_json::Value>,

    /// Artifacts produced by the task.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub artifacts: Vec<A2AArtifact>,

    /// Timestamp when the task was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub created_at: Option<String>,

    /// Timestamp when the task was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub updated_at: Option<String>,

    /// Extension object for custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ext: Option<Box<Ext>>,
}

impl A2ATask {
    /// Convenience method to create a new instance using the builder pattern.
    pub fn builder() -> A2ATaskBuilder {
        A2ATaskBuilder::create_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2a_task_creation() {
        let task = A2ATask::builder()
            .id("task-001")
            .state(TaskState::Working)
            .build()
            .unwrap();

        assert_eq!(task.id, "task-001");
        assert_eq!(task.state, TaskState::Working);
        assert!(task.message.is_none());
        assert!(task.result.is_none());
        assert!(task.artifacts.is_empty());
        assert!(task.created_at.is_none());
        assert!(task.updated_at.is_none());
    }

    #[test]
    fn test_a2a_task_serialization() {
        let task = A2ATask::builder()
            .id("task-002")
            .state(TaskState::Completed)
            .message("Done")
            .created_at("2025-03-31T12:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("\"created_at\""));
    }

    #[test]
    fn test_a2a_task_deserialization() {
        let json = r#"{"id":"task-003","state":"working","message":"Processing"}"#;
        let task: A2ATask = serde_json::from_str(json).unwrap();

        assert_eq!(task.id, "task-003");
        assert_eq!(task.state, TaskState::Working);
        assert_eq!(task.message, Some("Processing".to_string()));
    }

    #[test]
    fn test_a2a_task_roundtrip() {
        let task = A2ATask::builder()
            .id("task-004")
            .state(TaskState::InputRequired)
            .message("Need more info")
            .result(Some(serde_json::json!({"key": "value"})))
            .created_at("2025-03-31T10:00:00Z")
            .updated_at("2025-03-31T11:00:00Z")
            .build()
            .unwrap();

        let json = serde_json::to_string(&task).unwrap();
        let parsed: A2ATask = serde_json::from_str(&json).unwrap();
        assert_eq!(task, parsed);
    }

    #[test]
    fn test_a2a_task_default() {
        let task = A2ATask::builder().id("task-005").build().unwrap();

        assert_eq!(task.id, "task-005");
        assert_eq!(task.state, TaskState::Working);
        assert!(task.message.is_none());
        assert!(task.artifacts.is_empty());
    }

    #[test]
    fn test_a2a_artifact_creation() {
        let artifact = A2AArtifact::builder()
            .name("output.txt")
            .description("Generated output")
            .parts(vec![A2AArtifactPart::builder()
                .type_("text/plain")
                .content("Hello, world!")
                .build()
                .unwrap()])
            .build()
            .unwrap();

        assert_eq!(artifact.name, "output.txt");
        assert_eq!(artifact.description, Some("Generated output".to_string()));
        assert_eq!(artifact.parts.len(), 1);
        assert_eq!(artifact.parts[0].type_, "text/plain");
        assert_eq!(artifact.parts[0].content, "Hello, world!");
    }

    #[test]
    fn test_a2a_artifact_roundtrip() {
        let artifact = A2AArtifact::builder()
            .name("data.json")
            .parts(vec![
                A2AArtifactPart::builder()
                    .type_("application/json")
                    .content("{\"key\":\"value\"}")
                    .build()
                    .unwrap(),
                A2AArtifactPart::builder()
                    .type_("text/plain")
                    .content("Summary text")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&artifact).unwrap();
        let parsed: A2AArtifact = serde_json::from_str(&json).unwrap();
        assert_eq!(artifact, parsed);
    }

    #[test]
    fn test_a2a_artifact_part_type_serde() {
        let part = A2AArtifactPart::builder()
            .type_("text/html")
            .content("<p>Hello</p>")
            .build()
            .unwrap();

        let json = serde_json::to_string(&part).unwrap();
        assert!(json.contains("\"type\""));
        assert!(!json.contains("\"type_\""));

        let parsed: A2AArtifactPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, parsed);
    }

    #[test]
    fn test_a2a_task_with_artifacts() {
        let task = A2ATask::builder()
            .id("task-006")
            .state(TaskState::Completed)
            .artifacts(vec![
                A2AArtifact::builder()
                    .name("report.pdf")
                    .description("Generated report")
                    .build()
                    .unwrap(),
                A2AArtifact::builder()
                    .name("summary.txt")
                    .parts(vec![A2AArtifactPart::builder()
                        .type_("text/plain")
                        .content("Task completed successfully")
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let json = serde_json::to_string(&task).unwrap();
        let parsed: A2ATask = serde_json::from_str(&json).unwrap();
        assert_eq!(task, parsed);
        assert_eq!(parsed.artifacts.len(), 2);
    }
}
