use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use crate::ports::outbound::llm_provider::{LlmProvider, LlmRequest, LlmResponse};

/// A specialized subagent that can process prompts with a specific focus.
#[async_trait]
pub trait Subagent: Send + Sync {
    /// The unique name of the subagent (e.g., "coder", "researcher").
    fn name(&self) -> &str;

    /// The system prompt that defines this agent's persona and instructions.
    fn system_prompt(&self) -> &str;

    /// Optional: Custom logic for processing the prompt before sending to LLM.
    async fn process_request(&self, request: LlmRequest) -> LlmRequest {
        let mut req = request;
        let system = self.system_prompt();
        req.prompt = format!("SYSTEM: {}\n\nUSER: {}", system, req.prompt);
        req
    }
}

pub struct SubagentRegistry {
    agents: HashMap<String, Arc<dyn Subagent>>,
}

impl SubagentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            agents: HashMap::new(),
        };

        // Register default subagents
        registry.register(Arc::new(CoderAgent));
        registry.register(Arc::new(ResearcherAgent));

        registry
    }

    pub fn register(&mut self, agent: Arc<dyn Subagent>) {
        self.agents.insert(agent.name().to_string(), agent);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Subagent>> {
        self.agents.get(name).cloned()
    }
}

/// Specialized agent for code generation and technical implementation.
pub struct CoderAgent;

#[async_trait]
impl Subagent for CoderAgent {
    fn name(&self) -> &str { "coder" }

    fn system_prompt(&self) -> &str {
        "You are a Senior Software Engineer specializing in high-precision code generation. \
         Provide optimized, well-documented, and production-ready code. \
         Be concise and focus on the technical implementation."
    }
}

/// Specialized agent for repository analysis and research.
pub struct ResearcherAgent;

#[async_trait]
impl Subagent for ResearcherAgent {
    fn name(&self) -> &str { "researcher" }

    fn system_prompt(&self) -> &str {
        "You are a Technical Researcher specializing in repository exploration and RAG. \
         Analyze codebases to find patterns, architectural decisions, and specific implementation details. \
         Provide clear explanations and cite specific files when possible."
    }
}
