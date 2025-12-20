use flutter_rust_bridge::frb;
use std::sync::Arc;

use gestalt_core::adapters::llm::gemini::GeminiProvider;
use gestalt_core::adapters::llm::ollama::OllamaProvider;
use gestalt_core::application::consensus::ConsensusService;
use gestalt_core::ports::outbound::llm_provider::LlmProvider;


// Define local types for FRB to generate DTOs
pub struct ServerResponse {
    pub text_response: String,
    pub ui_component: Option<UiElement>,
}

pub struct UiElement {
    pub widget_type: WidgetType,
    pub content: String,
    pub id: String,
}

pub enum WidgetType {
    Button,
    Markdown,
    CodeBlock,
    Input,
    Card,
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - e.g. logging
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn ask_agent(prompt: String) -> anyhow::Result<ServerResponse> {
    // In a real app, we would hold the service in a static/singleton or pass it around.
    // For this prototype, we re-initialize it per request (stateless adapters).

    let mut providers: Vec<(String, Arc<dyn LlmProvider>)> = Vec::new();

    // Gemini - Try to get key from env, but in mobile env variables are tricky.
    // For now, hardcode or assume it's passed?
    // Let's rely on compile-time env or skip Gemini if not set.
    // BETTER: Use a hardcoded placeholder or logic to inject key from Flutter side later.
    // We will just skip Gemini if not found, but since we are on Windows/Emulator, it might inherit env.
    if let Ok(key) = std::env::var("GOOGLE_API_KEY") {
        if !key.is_empty() {
             let gemini = GeminiProvider::new("gemini-pro".to_string());
             providers.push(("Gemini".to_string(), Arc::new(gemini)));
        }
    }

    // Ollama - Assume localhost accessible (emulator special alias if Android, but Windows is localhost)
    // Android Emulator loopback is 10.0.2.2.
    // We can try multiple URLs or just default to localhost if on Windows.
    let ollama_url = "http://localhost:11434".to_string();
    let ollama = OllamaProvider::new(ollama_url, "llama2".to_string());
    providers.push(("Ollama".to_string(), Arc::new(ollama)));

    let service = ConsensusService::new(providers);
    let domain_response = service.ask_all(&prompt).await;

    // Convert domain types to local types
    Ok(ServerResponse {
        text_response: domain_response.text_response,
        ui_component: domain_response.ui_component.map(|u| UiElement {
            widget_type: match u.widget_type {
                gestalt_core::domain::genui::WidgetType::Button => WidgetType::Button,
                gestalt_core::domain::genui::WidgetType::Markdown => WidgetType::Markdown,
                gestalt_core::domain::genui::WidgetType::CodeBlock => WidgetType::CodeBlock,
                gestalt_core::domain::genui::WidgetType::Input => WidgetType::Input,
                gestalt_core::domain::genui::WidgetType::Card => WidgetType::Card,
            },
            content: u.content,
            id: u.id,
        }),
    })
}
