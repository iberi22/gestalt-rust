//! Gemini LLM Provider using OAuth2 authentication.
//!
//! Uses the Code Assist API endpoint (same as gemini-cli) which is compatible
//! with the `cloud-platform` OAuth scope.

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures::stream::BoxStream;

use uuid::Uuid;

use crate::adapters::auth::google_oauth;
use crate::ports::outbound::llm_provider::{LlmError, LlmProvider, LlmRequest, LlmResponse};

/// Code Assist API endpoint (same as gemini-cli uses)
const CODE_ASSIST_ENDPOINT: &str = "https://cloudcode-pa.googleapis.com";
const CODE_ASSIST_API_VERSION: &str = "v1internal";

pub struct GeminiOAuthProvider {
    client: Client,
    model: String,
}

impl GeminiOAuthProvider {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            model,
        }
    }
}

/// Request format matching gemini-cli's Code Assist API format
#[derive(Debug, Serialize)]
struct CAGenerateContentRequest {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<String>,
    user_prompt_id: String,
    request: VertexGenerateContentRequest,
}

#[derive(Debug, Serialize)]
struct VertexGenerateContentRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(rename = "maxOutputTokens", skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
}

/// Response format from Code Assist API
#[derive(Debug, Deserialize)]
struct CAGenerateContentResponse {
    response: VertexGenerateContentResponse,
    #[serde(rename = "traceId")]
    #[allow(dead_code)]
    trace_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VertexGenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Debug, Deserialize)]
struct CandidateContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: Option<String>,
}

#[async_trait]
impl LlmProvider for GeminiOAuthProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        // Get a valid access token (refreshes if needed)
        let access_token = google_oauth::get_valid_access_token()
            .await
            .map_err(|e| LlmError::Auth(e.to_string()))?;

        // Code Assist endpoint (same as gemini-cli)
        let url = format!(
            "{}/{}:generateContent",
            CODE_ASSIST_ENDPOINT, CODE_ASSIST_API_VERSION
        );

        // Build request in gemini-cli format
        let ca_request = CAGenerateContentRequest {
            model: self.model.clone(),
            project: std::env::var("GOOGLE_CLOUD_PROJECT").ok(),
            user_prompt_id: Uuid::new_v4().to_string(),
            request: VertexGenerateContentRequest {
                contents: vec![Content {
                    role: "user".to_string(),
                    parts: vec![Part {
                        text: request.prompt.clone(),
                    }],
                }],
                generation_config: Some(GenerationConfig {
                    temperature: Some(request.temperature),
                    max_output_tokens: request.max_tokens,
                }),
            },
        };

        let resp = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&ca_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !resp.status().is_success() {
             let status = resp.status();
             let text = resp.text().await.unwrap_or_default();
             return Err(LlmError::Provider(format!("Gemini Code Assist Error {}: {}", status, text)));
        }

        let resp_json: CAGenerateContentResponse = resp.json().await
            .map_err(|e| LlmError::Parse(format!("Failed to parse response: {}", e)))?;

        // Extract text from response
        let content = resp_json.response.candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .ok_or_else(|| LlmError::Parse("No text in response".to_string()))?;

        Ok(LlmResponse {
            content,
            usage: None,
        })
    }

    async fn stream(&self, _request: LlmRequest) -> Result<BoxStream<'static, Result<String, LlmError>>, LlmError> {
        Err(LlmError::Provider("Streaming not implemented yet for GeminiOAuth".to_string()))
    }
}
