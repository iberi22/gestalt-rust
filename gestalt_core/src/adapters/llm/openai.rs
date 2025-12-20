use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;

use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};

pub struct OpenAIProvider {
    client: Client,
    model: String,
}

impl OpenAIProvider {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            model,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAIProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| LlmError::Auth("OPENAI_API_KEY not set".to_string()))?;

        let url = "https://api.openai.com/v1/chat/completions";

        let body = json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": request.prompt
            }],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens
        });

        let resp = self.client.post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !resp.status().is_success() {
             let status = resp.status();
             let text = resp.text().await.unwrap_or_default();
             return Err(LlmError::Provider(format!("OpenAI Error {}: {}", status, text)));
        }

        let resp_json: serde_json::Value = resp.json().await
            .map_err(|e| LlmError::Parse(e.to_string()))?;

        let content = resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| LlmError::Parse("Invalid OpenAI response structure".to_string()))?
            .to_string();

        Ok(LlmResponse {
            content,
            usage: None,
        })
    }

    async fn stream(&self, _request: LlmRequest) -> Result<tokio::sync::mpsc::Receiver<Result<String, LlmError>>, LlmError> {
        Err(LlmError::Provider("Streaming not implemented yet for OpenAI".to_string()))
    }
}
