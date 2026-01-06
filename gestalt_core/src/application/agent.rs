use crate::ports::outbound::llm_provider::{LlmProvider, LlmRequest};
use crate::ports::outbound::repo_manager::{VectorDb, RepoManager};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Agent operating modes inspired by OpenCode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentMode {
    /// Full access agent for development work (edits files, runs commands)
    #[default]
    Build,
    /// Read-only agent for analysis and code exploration
    Plan,
}

pub struct AgentOrchestrator {
    llm: Arc<dyn LlmProvider>,
    vector_db: Arc<dyn VectorDb>,
    repo_manager: Arc<dyn RepoManager>,
    mode: std::sync::RwLock<AgentMode>,
    is_read_only: AtomicBool,
}

impl AgentOrchestrator {
    pub fn new(
        llm: Arc<dyn LlmProvider>,
        vector_db: Arc<dyn VectorDb>,
        repo_manager: Arc<dyn RepoManager>,
    ) -> Self {
        Self {
            llm,
            vector_db,
            repo_manager,
            mode: std::sync::RwLock::new(AgentMode::Build),
            is_read_only: AtomicBool::new(false),
        }
    }

    /// Switch agent mode
    pub fn set_mode(&self, mode: AgentMode) {
        *self.mode.write().unwrap() = mode;
        self.is_read_only.store(mode == AgentMode::Plan, Ordering::SeqCst);
        tracing::info!("Agent mode switched to: {:?}", mode);
    }

    /// Get current mode
    pub fn get_mode(&self) -> AgentMode {
        *self.mode.read().unwrap()
    }

    /// Check if write operations are allowed
    pub fn can_write(&self) -> bool {
        !self.is_read_only.load(Ordering::SeqCst)
    }

    pub async fn ask_about_repo(&self, repo_url: &str, question: &str) -> anyhow::Result<String> {
        // RAG: Get similar code chunks from Vector DB
        let similar_chunks = self.vector_db.search_similar("code", vec![0.0; 384], 5).await?;

        // Build context from chunks
        let mut context = String::new();
        for chunk in similar_chunks {
            if let Some(text) = chunk.get("metadata").and_then(|m| m.get("text")).and_then(|t| t.as_str()) {
                context.push_str(text);
                context.push_str("\n---\n");
            }
        }

        // Prompt LLM with context
        let full_prompt = format!(
            "Use the following code context to answer the question about the repository {}:\n\nCONTEXT:\n{}\n\nQUESTION: {}",
            repo_url, context, question
        );

        let request = LlmRequest {
            prompt: full_prompt,
            model: "gemini-1.5-pro".to_string(),
            temperature: 0.2,
            max_tokens: Some(1024),
        };

        let response = self.llm.generate(request).await
            .map_err(|e| anyhow::anyhow!("LLM Error: {:?}", e))?;

        Ok(response.content)
    }

    /// Index a repository (writes to vector DB - requires Build mode)
    pub async fn index_repo(&self, url: &str) -> anyhow::Result<()> {
        if !self.can_write() {
            anyhow::bail!("Cannot index repository in Plan mode. Switch to Build mode first.");
        }

        let repo = self.repo_manager.clone_repo(url).await?;
        // Logic to parse files, create embeddings, and store in vector_db
        tracing::info!("Indexing repo: {}", repo.name);
        Ok(())
    }

    /// Execute a shell command (requires Build mode)
    pub async fn execute_command(&self, _cmd: &str) -> anyhow::Result<String> {
        if !self.can_write() {
            anyhow::bail!("Cannot execute commands in Plan mode. Switch to Build mode first.");
        }
        // Placeholder for actual command execution
        Ok("Command execution not yet implemented".to_string())
    }
}
