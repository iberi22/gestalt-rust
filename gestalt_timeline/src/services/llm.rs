//! LLM Service - AWS Bedrock / Claude Sonnet Integration
//!
//! Orchestrates workflows using Claude Sonnet 4.5 as the AI backbone.

use anyhow::{Context, Result};
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::db::SurrealClient;
use crate::models::{EventType, Project, Task};
use crate::services::{Agent, TimelineService};



#[async_trait::async_trait]
pub trait Cognition: Send + Sync {
    async fn chat(&self, agent_id: &str, message: &str) -> Result<LLMResponse>;
    async fn orchestrate(&self, agent_id: &str, workflow_description: &str, project_context: Option<&str>) -> Result<Vec<OrchestrationAction>>;
    async fn orchestrate_step(&self, agent_id: &str, goal: &str, history: &[String], project_context: Option<&str>) -> Result<Vec<OrchestrationAction>>;
    fn model_id(&self) -> &str;
}

/// LLM Service for AI-powered orchestration.
#[derive(Clone)]
pub struct LLMService {
    bedrock_client: BedrockClient,
    db: SurrealClient,
    timeline: TimelineService,
    model_id: String,
}

#[async_trait::async_trait]
impl Cognition for LLMService {
    async fn chat(&self, agent_id: &str, message: &str) -> Result<LLMResponse> {
        self.chat(agent_id, message).await
    }

    async fn orchestrate(&self, agent_id: &str, workflow_description: &str, project_context: Option<&str>) -> Result<Vec<OrchestrationAction>> {
        self.orchestrate(agent_id, workflow_description, project_context).await
    }

    async fn orchestrate_step(&self, agent_id: &str, goal: &str, history: &[String], project_context: Option<&str>) -> Result<Vec<OrchestrationAction>> {
        self.orchestrate_step(agent_id, goal, history, project_context).await
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }
}

/// Response from the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub model_id: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: u64,
}

/// Orchestration action parsed from LLM response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrchestrationAction {
    CreateProject { name: String, description: Option<String> },
    CreateTask { project: String, description: String },
    RunTask { task_id: String },
    ListProjects,
    ListTasks { project: Option<String> },
    GetStatus { project: String },
    Chat { response: String },
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    ExecuteShell { command: String },
    StartJob { name: String, command: String },
    StopJob { name: String },
    ListJobs,
    DelegateTask { agent: String, goal: String },
}

impl LLMService {
    /// Create a new LLMService with AWS Bedrock client.
    pub async fn new(
        db: SurrealClient,
        timeline: TimelineService,
        config: &crate::config::CognitionSettings
    ) -> Result<Self> {
        info!("🤖 Initializing LLM Service with AWS Bedrock...");

        // Load AWS config from environment
        let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let bedrock_client = BedrockClient::new(&aws_config);

        info!("📦 Using model: {}", config.model_id);

        Ok(Self {
            bedrock_client,
            db,
            timeline,
            model_id: config.model_id.clone(),
        })
    }

    /// Create LLMService with a custom Bedrock client (for testing).
    pub fn with_client(
        bedrock_client: BedrockClient,
        db: SurrealClient,
        timeline: TimelineService,
        model_id: String,
    ) -> Self {
        Self {
            bedrock_client,
            db,
            timeline,
            model_id,
        }
    }

    /// Send a chat message to Claude and get a response.
    pub async fn chat(&self, agent_id: &str, message: &str) -> Result<LLMResponse> {
        debug!("💬 Sending message to Claude: {}", message);
        let start = std::time::Instant::now();

        // Build the message
        let user_message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(message.to_string()))
            .build()
            .context("Failed to build message")?;

        // Call Bedrock Converse API
        let response = self
            .bedrock_client
            .converse()
            .model_id(&self.model_id)
            .messages(user_message)
            .send()
            .await
            .context("Failed to invoke Bedrock model")?;

        // Extract response content
        let output = response.output().context("No output in response")?;
        let content = match output {
            aws_sdk_bedrockruntime::types::ConverseOutput::Message(msg) => {
                msg.content()
                    .iter()
                    .filter_map(|block| {
                        if let ContentBlock::Text(text) = block {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => String::new(),
        };

        // Extract token usage
        let usage = response.usage();
        let input_tokens = usage.map(|u| u.input_tokens()).unwrap_or(0);
        let output_tokens = usage.map(|u| u.output_tokens()).unwrap_or(0);

        let end = start.elapsed().as_millis() as u64;

        // Record in timeline
        self.timeline
            .emit(agent_id, EventType::Custom("llm_chat".to_string()))
            .await?;

        info!("✅ Claude response received ({} tokens in {}ms)", output_tokens, end);

        Ok(LLMResponse {
            content,
            model_id: self.model_id.clone(),
            input_tokens,
            output_tokens,
            duration_ms: end,
        })
    }

    /// Orchestrate a workflow based on natural language description.
    pub async fn orchestrate(
        &self,
        agent_id: &str,
        workflow_description: &str,
        project_context: Option<&str>,
    ) -> Result<Vec<OrchestrationAction>> {
        info!("🎯 Orchestrating workflow: {}", workflow_description);

        // Fetch agent persona if available
        let agent: Option<Agent> = self.db.select_by_id("agents", agent_id).await.unwrap_or(None);
        let agent_prompt = agent.as_ref().and_then(|a| a.system_prompt.as_deref());
        let dynamic_model_id = agent.as_ref().and_then(|a| a.model_id.as_deref()).unwrap_or(&self.model_id);

        // Build combined prompt with context and request
        let system_prompt = self.build_orchestration_prompt(project_context, agent_prompt).await?;
        let combined_message = format!("{}\n\nUSER REQUEST:\n{}", system_prompt, workflow_description);

        // Build single user message
        let user_message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(combined_message))
            .build()
            .context("Failed to build message")?;

        info!("🧠 Using model: {} (Agent Persona: {})", dynamic_model_id, agent_prompt.is_some());

        // Call Bedrock with single message
        let response = self
            .bedrock_client
            .converse()
            .model_id(dynamic_model_id)
            .messages(user_message)
            .send()
            .await
            .context("Failed to invoke Bedrock model")?;

        // Extract and parse response
        let output = response.output().context("No output in response")?;
        let content = match output {
            aws_sdk_bedrockruntime::types::ConverseOutput::Message(msg) => {
                msg.content()
                    .iter()
                    .filter_map(|block| {
                        if let ContentBlock::Text(text) = block {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => String::new(),
        };

        // Parse actions from response
        let actions = self.parse_orchestration_response(&content)?;

        // Record in timeline
        self.timeline
            .emit(agent_id, EventType::Custom("llm_orchestrate".to_string()))
            .await?;

        Ok(actions)
    }

    /// Execute a single step of the autonomous loop with history.
    pub async fn orchestrate_step(
        &self,
        agent_id: &str,
        goal: &str,
        history: &[String],
        project_context: Option<&str>,
    ) -> Result<Vec<OrchestrationAction>> {
        info!("🧠 Autonomous Step for Goal: {}", goal);

        // Fetch agent persona if available
        let agent: Option<Agent> = self.db.select_by_id("agents", agent_id).await.unwrap_or(None);
        let agent_prompt = agent.as_ref().and_then(|a| a.system_prompt.as_deref());
        let dynamic_model_id = agent.as_ref().and_then(|a| a.model_id.as_deref()).unwrap_or(&self.model_id);

        // Build System Context
        let system_context = self.build_orchestration_prompt(project_context, agent_prompt).await?;

        // Format History
        let history_text = if history.is_empty() {
            "(No history)".to_string()
        } else {
            history.join("\n")
        };

        // Combine into one prompt (Simulating "Context Window")
        let combined_msg = format!(
            "{}\n\nGOAL: {}\n\nHISTORY OF ACTIONS:\n{}\n\nINSTRUCTION: Review the history. If the goal is met, return empty list []. Otherwise, return the next JSON actions.",
            system_context, goal, history_text
        );

        // Build single user message
        let user_message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(combined_msg))
            .build()
            .context("Failed to build message")?;

        info!("🧠 Using model: {} (Step)", dynamic_model_id);

        // Call Bedrock
        let response = self
            .bedrock_client
            .converse()
            .model_id(dynamic_model_id)
            .messages(user_message)
            .send()
            .await
            .context("Failed to invoke Bedrock model")?;

        // Extract and parse response
        let output = response.output().context("No output in response")?;
        let content = match output {
            aws_sdk_bedrockruntime::types::ConverseOutput::Message(msg) => {
                msg.content()
                    .iter()
                    .filter_map(|block| {
                        if let ContentBlock::Text(text) = block {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => String::new(),
        };

        // Parse actions from response
        self.parse_orchestration_response(&content)
    }

    /// Build the orchestration system prompt with current context.
    async fn build_orchestration_prompt(&self, project_context: Option<&str>, agent_prompt: Option<&str>) -> Result<String> {
        // Get current projects and tasks for context
        let projects: Vec<Project> = self.db.select_all("projects").await.unwrap_or_default();
        let tasks: Vec<Task> = self.db.select_all("tasks").await.unwrap_or_default();

        let project_list = projects
            .iter()
            .map(|p| format!("- {} ({})", p.name, p.status))
            .collect::<Vec<_>>()
            .join("\n");

        let task_list = tasks
            .iter()
            .take(10)
            .map(|t| format!("- [{}] {}", t.status, t.description))
            .collect::<Vec<_>>()
            .join("\n");

        let context_section = if let Some(ctx) = project_context {
            format!("\nPROJECT CONTEXT: {}\n", ctx)
        } else {
            String::new()
        };

        let base_prompt = agent_prompt.unwrap_or("You are a workflow orchestrator for Gestalt Timeline, a project and task management system.");

        Ok(format!(
            r#"{}

CURRENT STATE:
Projects:
{}

Recent Tasks:
{}
{}
CAPABILITIES:
You can perform these actions by responding with JSON:
1. Create project: {{"action": "create_project", "name": "project-name", "description": "optional desc"}}
2. Create task: {{"action": "create_task", "project": "project-name", "description": "task description"}}
3. Run task: {{"action": "run_task", "task_id": "task-id"}}
4. List projects: {{"action": "list_projects"}}
5. List tasks: {{"action": "list_tasks", "project": "optional-project-name"}}
6. Get status: {{"action": "get_status", "project": "project-name"}}
7. Read file: {{"action": "read_file", "path": "path/to/file"}}
8. Write file: {{"action": "write_file", "path": "path/to/file", "content": "file content"}}
11. Execute shell command: {{"action": "execute_shell", "command": "ls -la"}} (WARNING: Executes on host machine)
12. Start background job: {{"action": "start_job", "name": "server", "command": "python3 -m http.server 8080"}}
13. Stop background job: {{"action": "stop_job", "name": "server"}}
14. List background jobs: {{"action": "list_jobs"}}
15. Delegate task to another agent: {{"action": "delegate_task", "agent": "<developer|researcher|reviewer>", "goal": "<goal>"}}
Respond with a JSON array of actions to execute, or a single {{"action": "chat", "response": "your message"}} for conversational responses.

Example: [{{"action": "create_project", "name": "my-app"}}, {{"action": "execute_shell", "command": "git init"}}]"#,
            base_prompt,
            if project_list.is_empty() { "(none)" } else { &project_list },
            if task_list.is_empty() { "(none)" } else { &task_list },
            context_section
        ))
    }

    /// Parse the LLM response into orchestration actions.
    fn parse_orchestration_response(&self, response: &str) -> Result<Vec<OrchestrationAction>> {
        // Try to extract JSON from the response
        let json_str = if response.contains('[') {
            // Array of actions
            let start = response.find('[').unwrap();
            let end = response.rfind(']').map(|i| i + 1).unwrap_or(response.len());
            &response[start..end]
        } else if response.contains('{') {
            // Single action
            let start = response.find('{').unwrap();
            let end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
            &format!("[{}]", &response[start..end])
        } else {
            // No JSON, treat as chat response
            return Ok(vec![OrchestrationAction::Chat {
                response: response.to_string(),
            }]);
        };

        // Parse JSON
        let raw_actions: Vec<serde_json::Value> = serde_json::from_str(json_str)
            .unwrap_or_else(|_| vec![serde_json::json!({"action": "chat", "response": response})]);

        let mut actions = Vec::new();

        for raw in raw_actions {
            let action_type = raw.get("action").and_then(|v| v.as_str()).unwrap_or("chat");

            let action = match action_type {
                "create_project" => OrchestrationAction::CreateProject {
                    name: raw.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed").to_string(),
                    description: raw.get("description").and_then(|v| v.as_str()).map(String::from),
                },
                "create_task" => OrchestrationAction::CreateTask {
                    project: raw.get("project").and_then(|v| v.as_str()).unwrap_or("default").to_string(),
                    description: raw.get("description").and_then(|v| v.as_str()).unwrap_or("task").to_string(),
                },
                "run_task" => OrchestrationAction::RunTask {
                    task_id: raw.get("task_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                "list_projects" => OrchestrationAction::ListProjects,
                "list_tasks" => OrchestrationAction::ListTasks {
                    project: raw.get("project").and_then(|v| v.as_str()).map(String::from),
                },
                "get_status" => OrchestrationAction::GetStatus {
                    project: raw.get("project").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                "read_file" => OrchestrationAction::ReadFile {
                    path: raw.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                "write_file" => OrchestrationAction::WriteFile {
                    path: raw.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    content: raw.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                "execute_shell" => OrchestrationAction::ExecuteShell {
                    command: raw.get("command").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                "start_job" => OrchestrationAction::StartJob {
                    name: raw.get("name").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
                    command: raw.get("command").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                "stop_job" => OrchestrationAction::StopJob {
                    name: raw.get("name").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
                },
                "list_jobs" => OrchestrationAction::ListJobs,
                "delegate_task" => OrchestrationAction::DelegateTask {
                    agent: raw.get("agent").and_then(|v| v.as_str()).unwrap_or("general").to_string(),
                    goal: raw.get("goal").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                },
                _ => OrchestrationAction::Chat {
                    response: raw.get("response").and_then(|v| v.as_str()).unwrap_or(response).to_string(),
                },
            };

            actions.push(action);
        }

        Ok(actions)
    }

    /// Get the current model ID.
    pub fn model_id(&self) -> &str {
        &self.model_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::SurrealClient;
    use crate::services::TimelineService;

    async fn setup_llm() -> LLMService {
        // Use memory database for tests
        let db_settings = crate::config::DatabaseSettings {
            url: "mem://".to_string(),
            user: "root".to_string(),
            pass: "root".to_string(),
            namespace: "test".to_string(),
            database: "test".to_string(),
        };

        let db = SurrealClient::connect(&db_settings).await.unwrap();
        let timeline = TimelineService::new(db.clone());

        // We can pass a default client since we only test non-async logic here
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let bedrock_client = BedrockClient::new(&config);

        LLMService::with_client(bedrock_client, db, timeline, "test-model".to_string())
    }

    #[tokio::test]
    async fn test_parse_orchestration_response_json_array() {
        let llm = setup_llm().await;
        let response = r#"[{"action": "create_project", "name": "api", "description": "test"}, {"action": "create_task", "project": "api", "description": "setup"}]"#;
        let actions = llm.parse_orchestration_response(response).unwrap();

        assert_eq!(actions.len(), 2);
        if let OrchestrationAction::CreateProject { name, .. } = &actions[0] {
            assert_eq!(name, "api");
        } else {
            panic!("Expected CreateProject");
        }
    }

    #[tokio::test]
    async fn test_parse_orchestration_response_single_object() {
        let llm = setup_llm().await;
        let response = r#"{"action": "create_project", "name": "api"}"#;
        let actions = llm.parse_orchestration_response(response).unwrap();

        assert_eq!(actions.len(), 1);
        if let OrchestrationAction::CreateProject { name, .. } = &actions[0] {
            assert_eq!(name, "api");
        } else {
            panic!("Expected CreateProject");
        }
    }

    #[tokio::test]
    async fn test_parse_orchestration_response_chat_fallback() {
        let llm = setup_llm().await;
        let response = "I don't understand that request.";
        let actions = llm.parse_orchestration_response(response).unwrap();

        assert_eq!(actions.len(), 1);
        if let OrchestrationAction::Chat { response: r } = &actions[0] {
            assert_eq!(r, "I don't understand that request.");
        } else {
            panic!("Expected Chat");
        }
    }

    #[tokio::test]
    async fn test_parse_orchestration_response_with_markdown() {
        let llm = setup_llm().await;
        let response = "Sure! Here are the actions:\n```json\n[{\"action\": \"create_task\", \"project\": \"p1\", \"description\": \"t1\"}]\n```";
        let actions = llm.parse_orchestration_response(response).unwrap();

        assert_eq!(actions.len(), 1);
        if let OrchestrationAction::CreateTask { description, .. } = &actions[0] {
            assert_eq!(description, "t1");
        } else {
            panic!("Expected CreateTask");
        }
    }

    #[tokio::test]
    async fn test_parse_orchestration_response_new_actions() {
        let llm = setup_llm().await;
        let response = r#"[
            {"action": "read_file", "path": "test.txt"},
            {"action": "write_file", "path": "out.txt", "content": "hello"},
            {"action": "execute_shell", "command": "echo hi"}
        ]"#;
        let actions = llm.parse_orchestration_response(response).unwrap();

        assert_eq!(actions.len(), 3);

        if let OrchestrationAction::ReadFile { path } = &actions[0] {
            assert_eq!(path, "test.txt");
        } else {
            panic!("Expected ReadFile");
        }

        if let OrchestrationAction::WriteFile { path, content } = &actions[1] {
            assert_eq!(path, "out.txt");
            assert_eq!(content, "hello");
        } else {
            panic!("Expected WriteFile");
        }

        if let OrchestrationAction::ExecuteShell { command } = &actions[2] {
            assert_eq!(command, "echo hi");
        } else {
            panic!("Expected ExecuteShell");
        }
    }
}
