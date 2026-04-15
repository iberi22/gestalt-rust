use super::tools::create_gestalt_tools;
use super::AgentMode;
use crate::ports::outbound::repo_manager::{RepoManager, VectorDb};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use synapse_agentic::framework::workflow::*;
use synapse_agentic::prelude::*;

#[derive(Debug)]
pub enum GestaltInput {
    Ask {
        repo_url: String,
        question: String,
        reply: oneshot::Sender<String>,
    },
    Index {
        url: String,
    },
    Execute {
        command: String,
    },
    SetMode(AgentMode),
}

pub struct GestaltAgent {
    _vector_db: Arc<dyn VectorDb>,
    _repo_manager: Arc<dyn RepoManager>,
    _mode: AgentMode,
    registry: ToolRegistry,
    llm_provider: Arc<dyn LLMProvider>,
    hive: Arc<Mutex<Hive>>,
}

#[derive(Debug, Clone)]
struct ArcProvider(Arc<dyn LLMProvider>);

#[async_trait]
impl LLMProvider for ArcProvider {
    fn name(&self) -> &str {
        self.0.name()
    }
    fn cost_per_1k_tokens(&self) -> f64 {
        self.0.cost_per_1k_tokens()
    }
    async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        self.0.generate(prompt).await
    }
}

impl GestaltAgent {
    pub async fn new(
        vector_db: Arc<dyn VectorDb>,
        repo_manager: Arc<dyn RepoManager>,
        llm_provider: Arc<dyn LLMProvider>,
        hive: Arc<Mutex<Hive>>,
    ) -> Self {
        let registry =
            create_gestalt_tools(repo_manager.clone(), vector_db.clone(), llm_provider.clone())
                .await;
        Self {
            _vector_db: vector_db,
            _repo_manager: repo_manager,
            _mode: AgentMode::Build,
            registry,
            llm_provider,
            hive,
        }
    }
}

/// Adapter node that executes the Decision Engine for Gestalt.
#[derive(Debug)]
struct GestaltNode {
    id: String,
    engine: DecisionEngine,
    registry: ToolRegistry,
    repo_url: String,
    question: String,
}

#[async_trait]
impl GraphNode for GestaltNode {
    /// Returns the unique ID of the reasoner node.
    fn id(&self) -> &str {
        &self.id
    }

    /// Primary execution loop for the Gestalt reasoning process.
    async fn execute(&mut self, state: &mut ContextState) -> anyhow::Result<NodeResult> {
        let history = state.get_string("reasoning_history").unwrap_or_default();
        let critic_feedback = state.get_string("critic_feedback").unwrap_or_default();

        // Extract current plan progress
        let default_plan = json!([]);
        let plan_val = state.get_value("plan").unwrap_or(&default_plan);
        let step_index = state
            .get_value("current_step_index")
            .and_then(|v: &serde_json::Value| v.as_u64())
            .unwrap_or(0) as usize;

        let plan: Vec<PlannedTask> = serde_json::from_value(plan_val.clone())?;

        if step_index >= plan.len() {
            tracing::info!("All planned steps completed.");
            return Ok(NodeResult::Halt);
        }

        let current_task = &plan[step_index];
        tracing::info!(
            "Executing step {}/{}: {}",
            step_index + 1,
            plan.len(),
            current_task.description
        );

        let mut context = DecisionContext::new(&self.question)
            .with_metadata("repo_url", self.repo_url.clone())
            .with_metadata("history", history.clone())
            .with_metadata("current_step", current_task.description.clone())
            .with_metadata("total_steps", plan.len().to_string())
            .with_metadata("step_index", step_index.to_string());

        if !critic_feedback.is_empty() {
            context = context.with_metadata("critic_feedback", critic_feedback);
        }

        let decision = self.engine.decide(&context).await?;

        if let Some(tool_name) = decision.action.strip_prefix("call:") {
            tracing::info!("Agent decided to call tool: {}", tool_name);
            let args = decision.parameters.unwrap_or(json!({}));

            match self.registry.call(tool_name, &EmptyContext, args).await {
                Ok(result) => {
                    let mut updated_history = history;
                    updated_history.push_str(&format!(
                        "\nStep {}: {} -> Tool {}: {}\n",
                        step_index + 1,
                        current_task.description,
                        tool_name,
                        result
                    ));
                    state.set_value("reasoning_history", json!(updated_history));
                    state.set_value("current_step_index", json!(step_index + 1));
                    Ok(NodeResult::Continue(None))
                }
                Err(e) => {
                    Ok(NodeResult::Error(format!(
                        "Step {} failed at tool {}: {}",
                        step_index + 1,
                        tool_name,
                        e
                    )))
                }
            }
        } else {
            tracing::info!("Agent provided final answer.");
            state.set_value("final_answer", json!(decision.action));
            Ok(NodeResult::Halt)
        }
    }
}

#[async_trait]
impl Agent for GestaltAgent {
    type Input = GestaltInput;

    fn name(&self) -> &str {
        "gestalt-agent"
    }

    async fn handle(&mut self, msg: Self::Input) -> anyhow::Result<()> {
        match msg {
            GestaltInput::Ask {
                repo_url,
                question,
                reply,
            } => {
                tracing::info!("Agent starting multi-agent orchestration for: {}", question);

                let context =
                    DecisionContext::new(&question).with_metadata("repo_url", repo_url.clone());
                let plan = self.plan(&question, &context).await?;
                tracing::info!("Generated plan with {} tasks", plan.len());

                let mut task_results = Vec::new();

                for task in plan {
                    tracing::info!("Delegating task: {}", task.description);

                    // Create a worker engine for the specific task execution
                    let engine = DecisionEngine::builder()
                        .with_provider(ArcProvider(self.llm_provider.clone()))
                        .build();

                    let gestalt_node = GestaltNode {
                        id: format!("worker_{}", task.id),
                        engine: engine.clone(),
                        registry: self.registry.clone(),
                        repo_url: repo_url.clone(),
                        question: task.description.clone(),
                    };

                    let critic = ReflectionNode::new("critic", &format!("worker_{}", task.id), 3);
                    let mut graph = StateGraph::new();
                    graph.add_node(Box::new(gestalt_node));
                    graph.add_node(Box::new(critic));
                    graph.set_entry_point(&format!("worker_{}", task.id));
                    graph.set_error_handler("critic");

                    // Sub-task plan (usually just the task itself)
                    let sub_plan = vec![PlannedTask {
                        id: "sub-1".to_string(),
                        description: task.description.clone(),
                        estimated_tool: task.estimated_tool.clone(),
                        status: TaskStatus::Pending,
                    }];

                    let initial_state = ContextState::new(json!({
                        "plan": sub_plan,
                        "current_step_index": 0
                    }));

                    let final_state = graph.execute(initial_state).await?;
                    let result = final_state
                        .get_string("final_answer")
                        .unwrap_or_else(|| "Task completed without detailed report.".to_string());

                    task_results.push(format!("### Task: {}\nResult: {}", task.description, result));
                }

                let final_report = task_results.join("\n\n---\n\n");
                let _ = reply.send(final_report);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl ExplicitPlanner for GestaltAgent {
    async fn plan(
        &self,
        goal: &str,
        _context: &DecisionContext,
    ) -> anyhow::Result<Vec<PlannedTask>> {
        tracing::info!("Planning for goal: {}", goal);

        let prompt = format!(
            "Decompose the following technical goal into a structured plan. \
             Return ONLY a JSON array of objects with 'id' and 'description' fields. \
             Goal: {}",
            goal
        );

        let response = self.llm_provider.generate(&prompt).await?;

        // Extract JSON
        let json_str = if let Some(start) = response.find('[') {
            if let Some(end) = response.rfind(']') {
                &response[start..=end]
            } else {
                &response
            }
        } else {
            &response
        };

        #[derive(Deserialize)]
        struct TaskTemplate {
            id: String,
            description: String,
        }

        let templates: Vec<TaskTemplate> = serde_json::from_str(json_str).map_err(|e| {
            anyhow::anyhow!("Failed to parse plan JSON: {}. Original: {}", e, response)
        })?;

        let tasks = templates
            .into_iter()
            .map(|t| PlannedTask {
                id: t.id,
                description: t.description,
                estimated_tool: None,
                status: TaskStatus::Pending,
            })
            .collect();

        Ok(tasks)
    }
}
