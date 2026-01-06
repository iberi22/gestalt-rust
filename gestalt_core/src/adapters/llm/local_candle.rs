use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::info;
use hf_hub::api::tokio::ApiBuilder;
use async_trait::async_trait;
use futures::stream::BoxStream;
use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};

pub struct LocalModelManager {
    model_dir: PathBuf,
}

impl LocalModelManager {
    pub fn new<P: AsRef<Path>>(model_dir: P) -> Self {
        Self {
            model_dir: model_dir.as_ref().to_path_buf(),
        }
    }

    pub async fn download_model(&self, repo_id: &str, filename: &str) -> Result<PathBuf> {
        info!("Verifying local model: {}/{}", repo_id, filename);

        // Use hf-hub to download/get the model
        let api = ApiBuilder::new()
            .with_cache_dir(self.model_dir.clone())
            .build()?;

        let repo = api.model(repo_id.to_string());
        let path = repo.get(filename).await?;

        info!("Model available at: {:?}", path);
        Ok(path)
    }
}

#[async_trait]
impl LlmProvider for LocalModelManager {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        info!("Generating response with local model for prompt: {}", request.prompt);
        // Stub for actual Candle inference
        Ok(LlmResponse {
            content: format!("(Local Candle) This is a placeholder for prompt: {}", request.prompt),
            usage: None,
        })
    }

    async fn stream(&self, _request: LlmRequest) -> Result<BoxStream<'static, Result<String, LlmError>>, LlmError> {
        Err(LlmError::Provider("Local streaming not implemented yet".to_string()))
    }
}
