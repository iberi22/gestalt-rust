//! Gemini Service - Google AI Integration
//!
//! Provides an alternative LLM provider using Google Gemini API.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use reqwest::Client;

use crate::db::SurrealClient;
use crate::models::{EventType, Project, Task};

use crate::services::{Agent, TimelineService, Cognition, LLMResponse, OrchestrationAction, AuthService};

/// Gemini implementation of Cognition trait.
#[derive(Clone)]
pub struct GeminiService {
    client: Client,
    db: SurrealClient,
    timeline: TimelineService,
    model_id: String,
    api_key: Option<String>,
    auth: Option<AuthService>,
}

impl GeminiService {
    /// Create a new GeminiService.
    pub fn new(
        db: SurrealClient,
        timeline: TimelineService,
        model_id: String,
        api_key: Option<String>,
    ) -> Result<Self> {
        info!("🤖 Initializing LLM Service with Google Gemini ({})", model_id);

        let auth = if api_key.is_none() {
            Some(AuthService::new()?)
        } else {
            None
        };

        Ok(Self {
            client: Client::new(),
            db,
            timeline,
            model_id,
            api_key,
            auth,
        })
    }

    /// Internal method to call Gemini API
    async fn call_gemini(&self, prompt: &str) -> Result<String> {
        let mut builder = self.client.post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model_id
        ));

        if let Some(auth) = &self.auth {
            let token = auth.get_valid_token().await
                .context("Failed to get valid OAuth2 token. Ensure you have run 'gestalt login'.")?;
            builder = builder.bearer_auth(token);
        } else if let Some(key) = &self.api_key {
            builder = builder.query(&[("key", key)]);
        } else {
            return Err(anyhow::anyhow!("No API key or OAuth2 credentials configured for Gemini"));
        }

        let body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }],
            "generationConfig": {
                "temperature": 0.2,
                "topK": 40,
                "topP": 0.95,
                "maxOutputTokens": 2048,
            }
        });



        let response = builder
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let err_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Gemini API Error ({}): {}", status, err_text));
        }

        let gemini_resp: serde_json::Value = response.json().await.context("Failed to parse Gemini response")?;

        // Extract text from Gemini response structure
        let text = gemini_resp["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .context("Empty or invalid response from Gemini")?;

        Ok(text.to_string())
    }

    /// Parse the LLM response into orchestration actions (Reusing logic from LLMService).
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

    /// Build the orchestration system prompt (Reusing logic).
    async fn build_orchestration_prompt(&self, project_context: Option<&str>, agent_prompt: Option<&str>) -> Result<String> {
        // Reusing the same builder logic
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

Respond with a JSON array of actions to execute, or a single {{"action": "chat", "response": "your message"}} for conversational responses.

Example: [{{"action": "create_project", "name": "my-app"}}, {{"action": "create_task", "project": "my-app", "description": "Setup database"}}]"#,
            base_prompt,
            if project_list.is_empty() { "(none)" } else { &project_list },
            if task_list.is_empty() { "(none)" } else { &task_list },
            context_section
        ))
    }
}

#[async_trait::async_trait]
impl Cognition for GeminiService {
    async fn chat(&self, agent_id: &str, message: &str) -> Result<LLMResponse> {
        let start = std::time::Instant::now();
        let content = self.call_gemini(message).await?;
        let duration = start.elapsed();

        // Record in timeline
        self.timeline
            .emit(agent_id, EventType::Custom("llm_chat_gemini".to_string()))
            .await?;

        Ok(LLMResponse {
            content,
            model_id: self.model_id.clone(),
            input_tokens: 0, // Gemini API doesn't return this simply in v1beta
            output_tokens: 0,
            duration_ms: duration.as_millis() as u64,
        })
    }

    async fn orchestrate(&self, agent_id: &str, workflow_description: &str, project_context: Option<&str>) -> Result<Vec<OrchestrationAction>> {
        let system_prompt = self.build_orchestration_prompt(project_context, None).await?;
        let combined_msg = format!("{}\n\nWORKFLOW GOAL: {}", system_prompt, workflow_description);

        let content = self.call_gemini(&combined_msg).await?;
        let actions = self.parse_orchestration_response(&content)?;

        // Record in timeline
        self.timeline
            .emit(agent_id, EventType::Custom("llm_orchestrate_gemini".to_string()))
            .await?;

        Ok(actions)
    }

    async fn orchestrate_step(&self, agent_id: &str, goal: &str, history: &[String], project_context: Option<&str>) -> Result<Vec<OrchestrationAction>> {
        info!("🧠 Gemini Autonomous Step for Goal: {}", goal);

        // Fetch agent persona if available
        let agent: Option<Agent> = self.db.select_by_id("agents", agent_id).await.unwrap_or(None);
        let agent_prompt = agent.as_ref().and_then(|a| a.system_prompt.as_deref());

        // Build System Context
        let system_context = self.build_orchestration_prompt(project_context, agent_prompt).await?;

        // Format History
        let history_text = if history.is_empty() {
            "(No history)".to_string()
        } else {
            history.join("\n")
        };

        // Combine into one prompt
        let combined_msg = format!(
            "{}\n\nGOAL: {}\n\nHISTORY OF ACTIONS:\n{}\n\nINSTRUCTION: Review the history. If the goal is met, return empty list []. Otherwise, return the next JSON actions.",
            system_context, goal, history_text
        );

        let content = self.call_gemini(&combined_msg).await?;
        let actions = self.parse_orchestration_response(&content)?;

        Ok(actions)
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }
}
