//! Agent Registry Service
//!
//! Manages connected agents and their sessions in the timeline system.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::db::SurrealClient;
use crate::models::EventType;
use crate::services::TimelineService;

/// Represents a connected agent in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique agent identifier
    pub id: String,

    /// Human-readable agent name
    pub name: String,

    /// Agent type (cli, copilot, antigravity, etc.)
    pub agent_type: AgentType,

    /// Connection status
    pub status: AgentStatus,

    /// When the agent connected
    pub connected_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_seen: DateTime<Utc>,

    /// Number of commands executed
    pub command_count: u64,

    /// Optional metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

/// Types of agents that can connect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentType {
    Cli,
    VsCodeCopilot,
    Antigravity,
    GeminiCli,
    Custom(String),
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::Cli => write!(f, "cli"),
            AgentType::VsCodeCopilot => write!(f, "vscode_copilot"),
            AgentType::Antigravity => write!(f, "antigravity"),
            AgentType::GeminiCli => write!(f, "gemini_cli"),
            AgentType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

/// Agent connection status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Online,
    Idle,
    Busy,
    Offline,
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Online => write!(f, "online"),
            AgentStatus::Idle => write!(f, "idle"),
            AgentStatus::Busy => write!(f, "busy"),
            AgentStatus::Offline => write!(f, "offline"),
        }
    }
}

impl Agent {
    /// Create a new agent.
    pub fn new(id: &str, name: &str, agent_type: AgentType) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            name: name.to_string(),
            agent_type,
            status: AgentStatus::Online,
            connected_at: now,
            last_seen: now,
            command_count: 0,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Detect agent type from environment or name.
    pub fn detect_type(name: &str) -> AgentType {
        let lower = name.to_lowercase();
        if lower.contains("copilot") {
            AgentType::VsCodeCopilot
        } else if lower.contains("antigravity") {
            AgentType::Antigravity
        } else if lower.contains("gemini") {
            AgentType::GeminiCli
        } else if lower.contains("cli") {
            AgentType::Cli
        } else {
            AgentType::Custom(name.to_string())
        }
    }
}

/// Service for managing agent connections.
#[derive(Clone)]
pub struct AgentService {
    db: SurrealClient,
    timeline: TimelineService,
}

impl AgentService {
    /// Create a new AgentService.
    pub fn new(db: SurrealClient, timeline: TimelineService) -> Self {
        Self { db, timeline }
    }

    /// Register a new agent connection.
    pub async fn connect(&self, agent_id: &str, name: Option<&str>) -> Result<Agent> {
        let agent_name = name.unwrap_or(agent_id);
        let agent_type = Agent::detect_type(agent_name);

        info!("🤖 Agent connecting: {} ({})", agent_id, agent_type);

        // Check if agent already exists
        if let Some(mut existing) = self.get_agent(agent_id).await? {
            // Update existing agent
            existing.status = AgentStatus::Online;
            existing.last_seen = Utc::now();
            let updated = self.db.update("agents", agent_id, &existing).await?;

            self.timeline
                .emit(agent_id, EventType::AgentConnected)
                .await?;

            return Ok(updated);
        }

        // Create new agent
        let agent = Agent::new(agent_id, agent_name, agent_type);
        let created: Agent = self.db.create("agents", &agent).await?;

        self.timeline
            .emit(agent_id, EventType::AgentConnected)
            .await?;

        Ok(created)
    }

    /// Disconnect an agent.
    pub async fn disconnect(&self, agent_id: &str) -> Result<Option<Agent>> {
        info!("👋 Agent disconnecting: {}", agent_id);

        if let Some(mut agent) = self.get_agent(agent_id).await? {
            agent.status = AgentStatus::Offline;
            agent.last_seen = Utc::now();

            let updated = self.db.update("agents", agent_id, &agent).await?;

            self.timeline
                .emit(agent_id, EventType::AgentDisconnected)
                .await?;

            return Ok(Some(updated));
        }

        Ok(None)
    }

    /// Get an agent by ID.
    pub async fn get_agent(&self, agent_id: &str) -> Result<Option<Agent>> {
        self.db.select_by_id("agents", agent_id).await
    }

    /// List all agents.
    pub async fn list_agents(&self) -> Result<Vec<Agent>> {
        self.db.select_all("agents").await
    }

    /// List only online agents.
    pub async fn list_online_agents(&self) -> Result<Vec<Agent>> {
        let query = "SELECT * FROM agents WHERE status = 'online' ORDER BY last_seen DESC";
        self.db.query_with(query, ()).await
    }

    /// Update agent's last seen timestamp.
    pub async fn heartbeat(&self, agent_id: &str) -> Result<()> {
        debug!("Heartbeat from agent: {}", agent_id);

        if let Some(mut agent) = self.get_agent(agent_id).await? {
            agent.last_seen = Utc::now();
            agent.command_count += 1;
            self.db.update("agents", agent_id, &agent).await?;
        }

        Ok(())
    }

    /// Set agent status.
    pub async fn set_status(&self, agent_id: &str, status: AgentStatus) -> Result<Option<Agent>> {
        if let Some(mut agent) = self.get_agent(agent_id).await? {
            agent.status = status;
            agent.last_seen = Utc::now();
            let updated = self.db.update("agents", agent_id, &agent).await?;
            return Ok(Some(updated));
        }
        Ok(None)
    }
}
