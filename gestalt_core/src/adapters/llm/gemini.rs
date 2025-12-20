use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;

use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};

pub struct GeminiProvider {
    client: Client,
    model: String,
}

impl GeminiProvider {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            model,
        }
    }
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let api_key = env::var("GOOGLE_API_KEY")
            .map_err(|_| LlmError::Auth("GOOGLE_API_KEY not set".to_string()))?;

        // Gemini REST API URL
        // https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, api_key
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": request.prompt
                }]
            }],
            "generationConfig": {
                "temperature": request.temperature,
                "maxOutputTokens": request.max_tokens
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
             return Err(LlmError::Provider(format!("Gemini Error {}: {}", status, text)));
        }

        let resp_json: serde_json::Value = resp.json().await
            .map_err(|e| LlmError::Parse(e.to_string()))?;

        // Extract content from Gemini response structure
        // candidates[0].content.parts[0].text
        let content = resp_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| LlmError::Parse("Invalid Gemini response structure".to_string()))?
            .to_string();

        Ok(LlmResponse {
            content,
            usage: None, // Simplified for now
        })
    }

    async fn stream(&self, _request: LlmRequest) -> Result<tokio::sync::mpsc::Receiver<Result<String, LlmError>>, LlmError> {
        Err(LlmError::Provider("Streaming not implemented yet for Gemini".to_string()))
    }
}
