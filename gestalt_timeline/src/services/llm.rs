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
use crate::services::TimelineService;

/// Default Claude Sonnet 4.5 inference profile ID (US region, on-demand supported)
const DEFAULT_MODEL_ID: &str = "us.anthropic.claude-sonnet-4-5-20250929-v1:0";

/// LLM Service for AI-powered orchestration.
#[derive(Clone)]
pub struct LLMService {
    bedrock_client: BedrockClient,
    db: SurrealClient,
    timeline: TimelineService,
    model_id: String,
}

/// Response from the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub model_id: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
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
}

impl LLMService {
    /// Create a new LLMService with AWS Bedrock client.
    pub async fn new(db: SurrealClient, timeline: TimelineService) -> Result<Self> {
        info!("🤖 Initializing LLM Service with AWS Bedrock...");

        // Load AWS config from environment
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let bedrock_client = BedrockClient::new(&config);

        // Get model ID from env or use default
        let model_id = std::env::var("BEDROCK_MODEL_ID")
            .unwrap_or_else(|_| DEFAULT_MODEL_ID.to_string());

        info!("📦 Using model: {}", model_id);

        Ok(Self {
            bedrock_client,
            db,
            timeline,
            model_id,
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

        // Record in timeline
        self.timeline
            .emit(agent_id, EventType::Custom("llm_chat".to_string()))
            .await?;

        info!("✅ Claude response received ({} tokens)", output_tokens);

        Ok(LLMResponse {
            content,
            model_id: self.model_id.clone(),
            input_tokens,
            output_tokens,
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

        // Build combined prompt with context and request
        let system_prompt = self.build_orchestration_prompt(project_context).await?;
        let combined_message = format!("{}\n\nUSER REQUEST:\n{}", system_prompt, workflow_description);

        // Build single user message
        let user_message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(combined_message))
            .build()
            .context("Failed to build message")?;

        // Call Bedrock with single message
        let response = self
            .bedrock_client
            .converse()
            .model_id(&self.model_id)
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

    /// Build the orchestration system prompt with current context.
    async fn build_orchestration_prompt(&self, project_context: Option<&str>) -> Result<String> {
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

        Ok(format!(
            r#"You are a workflow orchestrator for Gestalt Timeline, a project and task management system.

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

Respond with a JSON array of actions to execute, or a single {{"action": "chat", "response": "your message"}} for conversational responses.

Example: [{{"action": "create_project", "name": "my-app"}}, {{"action": "create_task", "project": "my-app", "description": "Setup database"}}]"#,
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
        let db = SurrealClient::connect().await.unwrap();
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
}
