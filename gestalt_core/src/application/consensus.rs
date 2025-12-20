use std::sync::Arc;
use futures::future::join_all;
use tracing::info;

use crate::domain::{AgentResponse};
use crate::ports::outbound::llm_provider::{LlmProvider, LlmRequest};

pub struct ConsensusService {
    providers: Vec<(String, Arc<dyn LlmProvider>)>, // (Model Name, Provider)
}

impl ConsensusService {
    pub fn new(providers: Vec<(String, Arc<dyn LlmProvider>)>) -> Self {
        Self { providers }
    }

    pub async fn ask_all(&self, prompt: &str) -> crate::domain::genui::ServerResponse {
        info!("Starting consensus for prompt: {}", prompt);

        // 1. Parallel Collection
        let futures = self.providers.iter().map(|(name, provider)| {
            let name = name.clone();
            let provider = provider.clone();
            let prompt = prompt.to_string();

            // Spawn task for each provider
            tokio::spawn(async move {
                info!("Querying model: {}", name);
                let request = LlmRequest {
                    prompt,
                    model: name.clone(),
                    temperature: 0.7,
                    max_tokens: None,
                };

                match provider.generate(request).await {
                    Ok(resp) => {
                        info!("Model {} finished successfully", name);
                        Some(AgentResponse {
                            model_name: name,
                            content: resp.content,
                        })
                    },
                    Err(e) => {
                        tracing::error!("Model {} failed: {}", name, e);
                        None
                    }
                }
            })
        });

        let results = join_all(futures).await;

        let mut responses = Vec::new();
        for res in results {
            if let Ok(Some(agent_resp)) = res {
                responses.push(agent_resp);
            }
        }

        info!("Consensus finished with {} responses", responses.len());

        // 2. The Judge Step
        // Find the "Smartest" provider (Gemini > *).
        let judge_provider = self.providers.iter()
            .find(|(name, _)| name.to_lowercase().contains("gemini"))
            .or_else(|| self.providers.first());

        if let Some((judge_name, judge)) = judge_provider {
             info!("Selected Judge: {}", judge_name);

             // Construct Judge Prompt
             let context = responses.iter()
                .map(|r| format!("--- {} SAID ---\n{}\n", r.model_name, r.content))
                .collect::<Vec<_>>()
                .join("\n");

            let judge_prompt = format!(
                "You are a Consensus Judge. Analyze these responses:\n{}\n\
                \n\
                Task:\n\
                1. Synthesize a single, correct answer based on the responses.\n\
                2. Recommend a UI component to show the user. Allowed types: Button, Markdown, CodeBlock, Input, Card.\n\
                \n\
                Return ONLY valid JSON matching this structure:\n\
                {{\n\
                    \"text_response\": \"The synthesized answer\",\n\
                    \"ui_component\": {{\n\
                        \"type\": \"CodeBlock\",\n\
                        \"content\": \"def hello(): print('world')\",\n\
                        \"id\": \"code_1\"\n\
                    }}\n\
                }}\n\
                If no specific UI is needed (just text), set \"ui_component\" to null.",
                context
            );

            let request = LlmRequest {
                prompt: judge_prompt,
                model: judge_name.clone(),
                temperature: 0.2, // Lower temp for structured output
                max_tokens: None,
            };

            match judge.generate(request).await {
                Ok(resp) => {
                    // Clean markdown code blocks if provider adds them (Gemini often does ```json ... ```)
                    let clean_json = resp.content.trim()
                        .trim_start_matches("```json")
                        .trim_start_matches("```")
                        .trim_end_matches("```")
                        .trim();

                    match serde_json::from_str::<crate::domain::genui::ServerResponse>(clean_json) {
                        Ok(parsed) => return parsed,
                        Err(e) => {
                             tracing::error!("Judge JSON parse failed: {}. Raw: {}", e, resp.content);
                        }
                    }
                },
                Err(e) => tracing::error!("Judge generation failed: {}", e),
            }
        }

        // Fallback
        crate::domain::genui::ServerResponse {
            text_response: "Consensus failed or Judge could not synthesize.".to_string(),
            ui_component: None,
        }
    }
}
