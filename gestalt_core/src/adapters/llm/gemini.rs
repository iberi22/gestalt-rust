use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;
use futures::stream::BoxStream;
use futures::StreamExt;

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

    async fn stream(&self, request: LlmRequest) -> Result<BoxStream<'static, Result<String, LlmError>>, LlmError> {
        let api_key = env::var("GOOGLE_API_KEY")
            .map_err(|_| LlmError::Auth("GOOGLE_API_KEY not set".to_string()))?;

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?key={}",
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

        let client = self.client.clone();
        
        let stream = async_stream::stream! {
            let resp = client.post(&url)
                .json(&body)
                .send()
                .await;

            match resp {
                Ok(mut response) => {
                    if !response.status().is_success() {
                        let text = response.text().await.unwrap_or_default();
                        yield Err(LlmError::Provider(format!("Gemini Error: {}", text)));
                        return;
                    }

                    while let Some(chunk) = response.chunk().await.unwrap_or(None) {
                        if let Ok(s) = String::from_utf8(chunk.to_vec()) {
                             // Naive parsing: Gemini sends JSON objects in an array.
                             // We try to find valid JSON objects in the chunk.
                             // This is brittle but works for simple cases.
                             let clean_s = s.trim().trim_start_matches('[').trim_end_matches(']').trim_start_matches(',').trim();
                             if clean_s.is_empty() { continue; }
                             
                             if let Ok(json) = serde_json::from_str::<serde_json::Value>(clean_s) {
                                 if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                                     yield Ok(text.to_string());
                                 }
                             }
                        }
                    }
                }
                Err(e) => yield Err(LlmError::Network(e.to_string())),
            }
        };
        
        Ok(Box::pin(stream))
    }
}
