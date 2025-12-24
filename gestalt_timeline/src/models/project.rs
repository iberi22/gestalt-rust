use super::FlexibleTimestamp;
use serde::{Deserialize, Serialize};
use std::fmt;
use surrealdb::sql::Thing;

/// Represents a project in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique identifier
    pub id: Option<Thing>,

    /// Project name
    pub name: String,

    /// Current status
    pub status: ProjectStatus,

    /// Priority level (1 = highest)
    pub priority: u8,

    /// Creation timestamp
    #[serde(with = "super::timestamp")]
    pub created_at: FlexibleTimestamp,

    /// Last update timestamp
    #[serde(with = "super::timestamp")]
    pub updated_at: FlexibleTimestamp,

    /// Agent that created the project
    pub created_by: String,
}

impl Project {
    /// Create a new project with default values.
    pub fn new(name: &str, created_by: &str) -> Self {
        let now = FlexibleTimestamp::now();
        Self {
            id: None, // Let DB or Builder handle ID, or we can set it if we want custom ULID
            name: name.to_string(),
            status: ProjectStatus::Active,
            priority: 5,
            created_at: now.clone(),
            updated_at: now,
            created_by: created_by.to_string(),
        }
    }
}

/// Project status enumeration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    /// Project is actively being worked on
    Active,
    /// Project is paused
    Paused,
    /// Project is completed
    Completed,
    /// Project is archived
    Archived,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::Paused => write!(f, "paused"),
            ProjectStatus::Completed => write!(f, "completed"),
            ProjectStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Project status information for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusInfo {
    pub id: String,
    pub name: String,
    pub status: ProjectStatus,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub progress_percent: u8,
}
