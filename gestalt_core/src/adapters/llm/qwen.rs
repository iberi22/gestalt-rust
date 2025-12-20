//! Qwen LLM Provider using OAuth2 authentication.

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::adapters::auth::qwen_oauth;
use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};

pub struct QwenProvider {
    client: Client,
    model: String,
}

impl QwenProvider {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            model,
        }
    }
}

#[async_trait]
impl LlmProvider for QwenProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        // Get a valid access token (refreshes if needed)
        let creds = qwen_oauth::load_cached_credentials()
            .await
            .ok_or_else(|| LlmError::Auth("No Qwen credentials. Run login first.".to_string()))?;

        // Check expiry and refresh if needed
        let access_token = if let Some(expiry) = creds.expiry_date {
            let now = chrono::Utc::now().timestamp_millis();
            if now >= expiry - 60_000 {
                if let Some(ref refresh) = creds.refresh_token {
                    let new_creds = qwen_oauth::refresh_access_token(refresh)
                        .await
                        .map_err(|e| LlmError::Auth(e.to_string()))?;
                    new_creds.access_token
                } else {
                    return Err(LlmError::Auth("Qwen token expired and no refresh token".to_string()));
                }
            } else {
                creds.access_token
            }
        } else {
            creds.access_token
        };

        // Use the resource_url if available, otherwise default to Qwen API
        let base_url = creds.resource_url
            .unwrap_or_else(|| "https://chat.qwen.ai".to_string());
        let url = format!("{}/api/v1/chat/completions", base_url);

        let body = json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": request.prompt
            }],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens
        });

        let resp = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !resp.status().is_success() {
             let status = resp.status();
             let text = resp.text().await.unwrap_or_default();
             return Err(LlmError::Provider(format!("Qwen Error {}: {}", status, text)));
        }

        let resp_json: serde_json::Value = resp.json().await
            .map_err(|e| LlmError::Parse(e.to_string()))?;

        let content = resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| LlmError::Parse("Invalid Qwen response structure".to_string()))?
            .to_string();

        Ok(LlmResponse {
            content,
            usage: None,
        })
    }

    async fn stream(&self, _request: LlmRequest) -> Result<tokio::sync::mpsc::Receiver<Result<String, LlmError>>, LlmError> {
        Err(LlmError::Provider("Streaming not implemented yet for Qwen".to_string()))
    }
}
