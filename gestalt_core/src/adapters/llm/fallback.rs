use async_trait::async_trait;
use futures::stream::BoxStream;
use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};
use std::sync::Arc;

pub struct FallbackProvider {
    primary: Arc<dyn LlmProvider>,
    fallback: Arc<dyn LlmProvider>,
}

impl FallbackProvider {
    pub fn new(primary: Arc<dyn LlmProvider>, fallback: Arc<dyn LlmProvider>) -> Self {
        Self { primary, fallback }
    }
}

#[async_trait]
impl LlmProvider for FallbackProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        match self.primary.generate(request.clone()).await {
            Ok(resp) => Ok(resp),
            Err(e) => {
                tracing::warn!("Primary LLM failed: {:?}. Falling back to local/secondary.", e);
                self.fallback.generate(request).await
            }
        }
    }

    async fn stream(&self, request: LlmRequest) -> Result<BoxStream<'static, Result<String, LlmError>>, LlmError> {
        match self.primary.stream(request.clone()).await {
            Ok(stream) => Ok(stream),
            Err(e) => {
                tracing::warn!("Primary LLM streaming failed: {:?}. Falling back to local/secondary.", e);
                self.fallback.stream(request).await
            }
        }
    }
}
