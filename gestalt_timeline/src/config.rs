use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub cognition: CognitionSettings,
    pub agent: AgentSettings,
    pub telegram: Option<TelegramSettings>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CognitionSettings {
    pub model_id: String,
    pub gemini_api_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentSettings {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramSettings {
    pub bot_token: String,
    pub allowed_users: Option<Vec<String>>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start with default values
            .set_default("database.url", "mem://")?
            .set_default("database.user", "root")?
            .set_default("database.pass", "root")?
            .set_default("database.namespace", "gestalt")?
            .set_default("database.database", "timeline")?
            .set_default("cognition.model_id", "us.anthropic.claude-sonnet-4-5-20250929-v1:0")?
            .set_default("agent.id", "cli_default")?

            // Merge with config file if exists
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config").required(false))

            // Merge with Environment variables
            // E.g. GESTALT_DATABASE_URL will override database.url
            .add_source(Environment::with_prefix("GESTALT").separator("_"))

            // Keep legacy env vars support for backward compatibility (manually if needed, but config crate handles structure mapping well if prefixes match)
            // For now, let's explicitly map legacy vars if we want strict backward compat,
            // but let's assume we are migrating to GESTALT_ prefix or just config file.
            // Actually, let's allow overrides from the old specific names too for smoother transition.
            .add_source(Environment::default().try_parsing(true))

            .build()?;

        s.try_deserialize()
    }
}
