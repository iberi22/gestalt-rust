//! CLI command definitions using Clap

use clap::{Parser, Subcommand};

/// Gestalt Timeline - Meta-Agent CLI Orchestrator
#[derive(Parser, Debug)]
#[command(name = "gestalt")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Output in JSON format for programmatic access
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Register a new project
    #[command(name = "add-project")]
    AddProject {
        /// Name of the project
        name: String,
    },

    /// Add a task to a project
    #[command(name = "add-task")]
    AddTask {
        /// Project name
        project: String,
        /// Task description
        description: String,
    },

    /// Execute a task asynchronously
    #[command(name = "run-task")]
    RunTask {
        /// Task ID to execute
        task_id: String,
    },

    /// List all projects
    #[command(name = "list-projects")]
    ListProjects,

    /// List tasks, optionally filtered by project
    #[command(name = "list-tasks")]
    ListTasks {
        /// Optional project name to filter by
        project: Option<String>,
    },

    /// Show project status and progress
    #[command(name = "status")]
    Status {
        /// Project name
        project: String,
    },

    /// Show timeline of events
    #[command(name = "timeline")]
    Timeline {
        /// Filter events since duration (e.g., "1h", "30m", "1d")
        #[arg(long)]
        since: Option<String>,
    },

    /// Watch timeline events in real-time (persistent process)
    #[command(name = "watch")]
    Watch {
        /// Optional project to filter events
        #[arg(long)]
        project: Option<String>,

        /// Show only specific event types (comma-separated)
        #[arg(long)]
        events: Option<String>,
    },

    /// Broadcast a message to all connected agents
    #[command(name = "broadcast")]
    Broadcast {
        /// Message to broadcast
        message: String,

        /// Optional project context
        #[arg(long)]
        project: Option<String>,
    },

    /// Subscribe to events from a specific project
    #[command(name = "subscribe")]
    Subscribe {
        /// Project name to subscribe to
        project: String,
    },

    /// Register this agent with the system
    #[command(name = "agent-connect")]
    AgentConnect {
        /// Optional custom agent name
        #[arg(long)]
        name: Option<String>,
    },

    /// Disconnect this agent from the system
    #[command(name = "agent-disconnect")]
    AgentDisconnect,

    /// List all connected agents
    #[command(name = "list-agents")]
    ListAgents {
        /// Show only online agents
        #[arg(long)]
        online: bool,
    },

    /// Chat with AI orchestrator (Claude Sonnet 4.5)
    #[command(name = "ai-chat")]
    AiChat {
        /// Message to send to Claude
        message: String,
    },

    /// Execute workflow via AI orchestrator
    #[command(name = "ai-orchestrate")]
    AiOrchestrate {
        /// Natural language workflow description
        workflow: String,

        /// Optional project context
        #[arg(long)]
        project: Option<String>,

        /// Dry run - show actions without executing
        #[arg(long)]
        dry_run: bool,
    },
}
