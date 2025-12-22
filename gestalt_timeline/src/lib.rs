//! Gestalt Timeline Orchestrator
//!
//! Meta-Agent CLI for coordinating projects and tasks with a universal timeline.

pub mod cli;
pub mod db;
pub mod models;
pub mod services;
pub mod config;

#[cfg(test)]
mod tests;

pub use db::SurrealClient;
pub use models::{EventType, Project, Task, TimelineEvent};
pub use services::{
    Agent, AgentService, AgentStatus, AgentType,
    LLMResponse, LLMService, OrchestrationAction,
    ProjectService, TaskService, TimelineService, WatchService
};
