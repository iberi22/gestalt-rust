use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use futures::stream::BoxStream;

use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};

pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let url = format!("{}/api/chat", self.base_url);

        let body = json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": request.prompt
            }],
            "stream": false,
            "options": {
                "temperature": request.temperature,
                "num_predict": request.max_tokens
            }
        });

        let resp = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !resp.status().is_success() {
             let status = resp.status();
             let text = resp.text().await.unwrap_or_default();
             return Err(LlmError::Provider(format!("Ollama Error {}: {}", status, text)));
        }

        let resp_json: serde_json::Value = resp.json().await
            .map_err(|e| LlmError::Parse(e.to_string()))?;

        let content = resp_json["message"]["content"]
            .as_str()
            .ok_or_else(|| LlmError::Parse("Invalid Ollama response structure".to_string()))?
            .to_string();

        Ok(LlmResponse {
            content,
            usage: None,
        })
    }

    async fn stream(&self, _request: LlmRequest) -> Result<BoxStream<'static, Result<String, LlmError>>, LlmError> {
        Err(LlmError::Provider("Streaming not implemented yet for Ollama".to_string()))
    }
}
