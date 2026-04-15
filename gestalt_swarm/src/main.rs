use clap::{Parser, Subcommand};
use gestalt_core::adapters::persistence::surreal_db::SurrealDbAdapter;
use gestalt_core::application::agent::gestalt_agent::{GestaltAgent, GestaltInput};
use gestalt_core::ports::outbound::repo_manager::{RepoManager, Repository};
use std::sync::Arc;
use synapse_agentic::prelude::*;
use tokio::sync::{oneshot, Mutex};
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(name = "swarm")]
#[command(about = "Gestalt Swarm CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check the status of the swarm
    Status,
    /// Run a swarm task with multi-agent orchestration
    Run {
        /// The goal for the swarm
        #[arg(short, long)]
        goal: String,

        /// Repository URL (default: current directory)
        #[arg(short, long, default_value = ".")]
        repo: String,
    },
}

pub struct LocalRepoManager;

#[async_trait]
impl RepoManager for LocalRepoManager {
    async fn clone_repo(&self, url: &str) -> anyhow::Result<Repository> {
        Ok(Repository {
            id: "local".to_string(),
            name: "local-workspace".to_string(),
            url: url.to_string(),
            local_path: Some(".".to_string()),
        })
    }

    async fn list_repos(&self) -> anyhow::Result<Vec<Repository>> {
        Ok(vec![Repository {
            id: "local".to_string(),
            name: "local-workspace".to_string(),
            url: ".".to_string(),
            local_path: Some(".".to_string()),
        }])
    }
}

fn get_provider() -> anyhow::Result<Arc<dyn LLMProvider>> {
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        info!("Using Gemini provider");
        return Ok(Arc::new(GeminiProvider::new(key, "gemini-1.5-pro".into())));
    }

    if let Ok(key) = std::env::var("MINIMAX_API_KEY") {
        info!("Using MiniMax provider");
        let group_id = std::env::var("MINIMAX_GROUP_ID").unwrap_or_default();
        return Ok(Arc::new(MinimaxProvider::new(
            key,
            group_id,
            "abab6.5-chat".into(),
        )));
    }

    warn!("No LLM API keys found in environment. Using mock provider.");
    Ok(Arc::new(MockProvider))
}

#[derive(Debug, Clone)]
struct MockProvider;

#[async_trait]
impl LLMProvider for MockProvider {
    fn name(&self) -> &str {
        "mock"
    }
    fn cost_per_1k_tokens(&self) -> f64 {
        0.0
    }
    async fn generate(&self, _prompt: &str) -> anyhow::Result<String> {
        Ok(r#"[{"id": "1", "description": "Scan the workspace structure"}, {"id": "2", "description": "Analyze key components"}]"#.to_string())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level)))
        .init();

    match args.command {
        Some(Commands::Status) => {
            println!("🐝 Gestalt Swarm is active and ready.");
            println!("Configuration:");
            println!("  LLM Provider: {}", get_provider()?.name());
            println!("  Vector DB: SurrealDB (In-Memory)");
        }
        Some(Commands::Run { goal, repo }) => {
            info!("Starting swarm orchestration for goal: {}", goal);
            println!("🚀 Executing goal: {}", goal);

            // 1. Initialize Infrastructure
            let vector_db = Arc::new(SurrealDbAdapter::new().await?);
            let repo_manager = Arc::new(LocalRepoManager);
            let llm_provider = get_provider()?;
            let hive = Arc::new(Mutex::new(Hive::new()));

            // 2. Initialize Gestalt Coordinator Agent
            let mut coordinator = GestaltAgent::new(
                vector_db,
                repo_manager,
                llm_provider,
                hive.clone(),
            ).await;

            // 3. Orchestration
            let (tx, rx) = oneshot::channel();

            println!("🧠 Decomposing goal into tasks...");

            coordinator.handle(GestaltInput::Ask {
                repo_url: repo,
                question: goal,
                reply: tx,
            }).await?;

            println!("🐝 Hive active. Agents are working...");

            match rx.await {
                Ok(report) => {
                    println!("\n✨ Swarm Execution Result:\n");
                    println!("{}", report);
                }
                Err(_) => {
                    println!("❌ Swarm orchestration failed to return a result.");
                }
            }
        }
        None => {
            println!("Gestalt Swarm CLI. Use --help for available commands.");
        }
    }

    Ok(())
}
