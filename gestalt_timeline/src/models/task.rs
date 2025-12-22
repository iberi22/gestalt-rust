//! Task model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a task within a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier
    pub id: String,

    /// Associated project ID
    pub project_id: String,

    /// Task description
    pub description: String,

    /// Current status
    pub status: TaskStatus,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Completion timestamp (if completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    /// Agent that created the task
    pub created_by: String,

    /// Agent that executed the task (if executed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,

    /// Duration of execution in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

impl Task {
    /// Create a new task.
    pub fn new(project_id: &str, description: &str, created_by: &str) -> Self {
        let now = Utc::now();
        Self {
            id: ulid::Ulid::new().to_string(),
            project_id: project_id.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            created_at: now,
            updated_at: now,
            completed_at: None,
            created_by: created_by.to_string(),
            executed_by: None,
            duration_ms: None,
        }
    }
}

/// Task status enumeration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    /// Task is pending execution
    Pending,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was cancelled
    Cancelled,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Running => write!(f, "running"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Failed => write!(f, "failed"),
            TaskStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Result of task execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    pub message: String,
    pub duration_ms: u64,
    pub completed_at: DateTime<Utc>,
}
