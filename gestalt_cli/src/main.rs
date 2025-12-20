use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use gestalt_core::adapters::auth::{google_oauth, qwen_oauth};
use gestalt_core::adapters::llm::gemini::GeminiProvider;
use gestalt_core::adapters::llm::gemini_oauth::GeminiOAuthProvider;
use gestalt_core::adapters::llm::ollama::OllamaProvider;
use gestalt_core::adapters::llm::openai::OpenAIProvider;
use gestalt_core::adapters::llm::qwen::QwenProvider;
use gestalt_core::application::consensus::ConsensusService;
use gestalt_core::application::mcp_service::McpService;
use gestalt_core::ports::outbound::llm_provider::LlmProvider;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The prompt to send to the agents (when not using subcommands)
    #[arg(short, long)]
    prompt: Option<String>,

    /// Google Gemini Model (default: gemini-2.0-flash)
    #[arg(long, default_value = "gemini-2.0-flash")]
    gemini_model: String,

    /// OpenAI Model (default: gpt-4)
    #[arg(long, default_value = "gpt-4")]
    openai_model: String,

    /// Qwen Model (default: qwen-coder)
    #[arg(long, default_value = "qwen-coder")]
    qwen_model: String,

    /// Ollama Model (default: llama2)
    #[arg(long, default_value = "llama2")]
    ollama_model: String,

    /// Ollama Base URL
    #[arg(long, default_value = "http://localhost:11434")]
    ollama_url: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login with Qwen OAuth2 (Device Code flow)
    #[command(name = "qwen-login")]
    QwenLogin,
    /// Logout from Qwen
    #[command(name = "qwen-logout")]
    QwenLogout,
    /// Show authentication status
    Status,
}

#[tokio::main]
async fn main() {
    // 1. Initialize Logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // 2. Load Environment Variables (.env)
    dotenv().ok();

    // 3. Parse Args
    let cli = Cli::parse();

    // Handle subcommands
    match cli.command {
        Some(Commands::QwenLogin) => {
            match qwen_oauth::run_device_flow_login().await {
                Ok(_) => {
                    println!("✅ Qwen login successful!");
                }
                Err(e) => {
                    eprintln!("❌ Qwen login failed: {}", e);
                }
            }
            return;
        }
        Some(Commands::QwenLogout) => {
            match qwen_oauth::clear_credentials().await {
                Ok(_) => println!("✅ Qwen logged out successfully."),
                Err(e) => eprintln!("❌ Qwen logout failed: {}", e),
            }
            return;
        }
        Some(Commands::Status) => {
            println!("🔐 Authentication Status:\n");

            // Gemini (from gemini-cli)
            if google_oauth::has_gemini_cli_credentials().await {
                println!("  ✅ Gemini: Logged in (via gemini-cli)");
            } else {
                println!("  ❌ Gemini: Not logged in. Run `gemini` CLI to authenticate.");
            }

            // Qwen
            if qwen_oauth::has_qwen_credentials().await {
                println!("  ✅ Qwen: Logged in");
            } else {
                println!("  ❌ Qwen: Not logged in. Run `gestalt_cli qwen-login`");
            }

            // OpenAI
            if std::env::var("OPENAI_API_KEY").is_ok() {
                println!("  ✅ OpenAI: API key set");
            } else {
                println!("  ❌ OpenAI: OPENAI_API_KEY not set");
            }

            println!("\n  ℹ️  Ollama: Always available (local)");
            return;
        }
        None => {
            // Continue with the ask flow
        }
    }

    // Require prompt for ask flow
    let prompt = match cli.prompt {
        Some(p) => p,
        None => {
            eprintln!("Error: --prompt is required. Use --help for usage.");
            eprintln!("\nExamples:");
            eprintln!("  gestalt_cli --prompt \"Explain Rust\"");
            eprintln!("  gestalt_cli status");
            eprintln!("  gestalt_cli qwen-login");
            return;
        }
    };

    info!("Starting Gestalt Agent Consensus...");
    info!("Prompt: {}", prompt);

    // 4. Initialize MCP Service (Experimental)
    info!("Initializing MCP Service...");
    let mut mcp_service = McpService::new();
    if let Err(e) = mcp_service.initialize_all().await {
        tracing::warn!("Failed to initialize MCP service: {}", e);
    } else {
        let count = mcp_service.get_clients().len();
        if count > 0 {
            info!("✅ Initialized {} MCP servers.", count);
            let tools = mcp_service.list_tools().await;
            info!("🛠️  Available MCP Tools: {:?}", tools);
        } else {
            info!("No MCP servers found in configuration.");
        }
    }

    // 5. Initialize Providers
    let mut providers: Vec<(String, Arc<dyn LlmProvider>)> = Vec::new();

    // Gemini: Use gemini-cli credentials if available
    if google_oauth::has_gemini_cli_credentials().await {
        info!("Using Gemini with gemini-cli OAuth credentials.");
        let gemini = GeminiOAuthProvider::new(cli.gemini_model.clone());
        providers.push(("Gemini".to_string(), Arc::new(gemini)));
    } else if std::env::var("GOOGLE_API_KEY").is_ok() || std::env::var("GEMINI_API_KEY").is_ok() {
        if let Ok(key) = std::env::var("GEMINI_API_KEY") {
            std::env::set_var("GOOGLE_API_KEY", key);
        }
        let gemini = GeminiProvider::new(cli.gemini_model.clone());
        providers.push(("Gemini (API Key)".to_string(), Arc::new(gemini)));
    } else {
        info!("No Gemini credentials. Run `gemini` CLI to login.");
    }

    // Qwen: Use Qwen credentials if available
    if qwen_oauth::has_qwen_credentials().await {
        info!("Using Qwen with OAuth credentials.");
        let qwen = QwenProvider::new(cli.qwen_model.clone());
        providers.push(("Qwen".to_string(), Arc::new(qwen)));
    } else {
        info!("No Qwen credentials. Run `gestalt_cli qwen-login` to login.");
    }

    // OpenAI
    if std::env::var("OPENAI_API_KEY").is_ok() {
        let openai = OpenAIProvider::new(cli.openai_model.clone());
        providers.push(("OpenAI".to_string(), Arc::new(openai)));
    } else {
        info!("OPENAI_API_KEY not found. Skipping OpenAI.");
    }

    // Ollama (Always assumed available or fails gracefully in network call)
    let ollama = OllamaProvider::new(cli.ollama_url.clone(), cli.ollama_model.clone());
    providers.push(("Ollama".to_string(), Arc::new(ollama)));

    if providers.is_empty() {
        eprintln!("No LLM providers available. Run `gestalt_cli status` to see auth status.");
        return;
    }

    // 5. Run Consensus
    let service = ConsensusService::new(providers);
    let result = service.ask_all(&prompt).await;

    // 6. Print Output as JSON for parsing
    match serde_json::to_string_pretty(&result) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Failed to serialize result: {}", e),
    }
}
