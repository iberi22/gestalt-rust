//! TimelineEvent model - Core of the timeline system
//!
//! The timestamp is the PRIMARY variable of the entire system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents an event in the universal timeline.
///
/// Every action in the system is recorded as a TimelineEvent with a UTC timestamp.
/// This enables full traceability and coordination between multiple agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// Unique identifier (ULID format)
    pub id: String,

    /// UTC timestamp - PRIMARY VARIABLE
    pub timestamp: DateTime<Utc>,

    /// ID of the agent that triggered this event
    pub agent_id: String,

    /// Type of event
    pub event_type: EventType,

    /// Associated project ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// Associated task ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    /// Event payload data
    #[serde(default)]
    pub payload: serde_json::Value,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl TimelineEvent {
    /// Create a new timeline event with current UTC timestamp.
    pub fn new(agent_id: &str, event_type: EventType) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            timestamp: Utc::now(),
            agent_id: agent_id.to_string(),
            event_type,
            project_id: None,
            task_id: None,
            payload: serde_json::Value::Null,
            metadata: HashMap::new(),
        }
    }

    /// Set the project ID for this event.
    pub fn with_project(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_string());
        self
    }

    /// Set the task ID for this event.
    pub fn with_task(mut self, task_id: &str) -> Self {
        self.task_id = Some(task_id.to_string());
        self
    }

    /// Set the payload for this event.
    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }

    /// Add metadata to this event.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Types of events that can occur in the timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// A new project was created
    ProjectCreated,
    /// A project was updated
    ProjectUpdated,
    /// A project was deleted
    ProjectDeleted,
    /// A new task was created
    TaskCreated,
    /// A task execution started
    TaskStarted,
    /// A task completed successfully
    TaskCompleted,
    /// A task failed
    TaskFailed,
    /// An agent connected to the system
    AgentConnected,
    /// An agent disconnected from the system
    AgentDisconnected,
    /// A CLI command was executed
    CommandExecuted,
    /// Custom event type
    Custom(String),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::ProjectCreated => write!(f, "project_created"),
            EventType::ProjectUpdated => write!(f, "project_updated"),
            EventType::ProjectDeleted => write!(f, "project_deleted"),
            EventType::TaskCreated => write!(f, "task_created"),
            EventType::TaskStarted => write!(f, "task_started"),
            EventType::TaskCompleted => write!(f, "task_completed"),
            EventType::TaskFailed => write!(f, "task_failed"),
            EventType::AgentConnected => write!(f, "agent_connected"),
            EventType::AgentDisconnected => write!(f, "agent_disconnected"),
            EventType::CommandExecuted => write!(f, "command_executed"),
            EventType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}
