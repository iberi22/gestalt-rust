use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("Parsing error: {0}")]
    Parse(String),
}

#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a single response from the LLM
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;

    /// Stream the response from the LLM
    /// Returns a stream of strings chunks
    async fn stream(&self, request: LlmRequest) -> Result<tokio::sync::mpsc::Receiver<Result<String, LlmError>>, LlmError>;
}
