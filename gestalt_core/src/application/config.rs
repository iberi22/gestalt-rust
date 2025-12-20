use config::{Config, ConfigError, Environment, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub gemini: GeminiConfig,
    pub openai: OpenAIConfig,
    pub qwen: QwenConfig,
    pub ollama: OllamaConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeminiConfig {
    pub model: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIConfig {
    pub model: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QwenConfig {
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaConfig {
    pub model: String,
    pub base_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            gemini: GeminiConfig {
                model: "gemini-2.0-flash".to_string(),
                api_key: None,
            },
            openai: OpenAIConfig {
                model: "gpt-4".to_string(),
                api_key: None,
            },
            qwen: QwenConfig {
                model: "qwen-coder".to_string(),
            },
            ollama: OllamaConfig {
                model: "llama2".to_string(),
                base_url: "http://localhost:11434".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        // 1. Start with defaults
        let defaults = AppConfig::default();
        
        // We need to manually set defaults because Config::builder() doesn't take a struct directly easily
        // without serializing it first, or we can just set individual keys.
        // For simplicity, we'll rely on the config crate's merging.
        // Actually, a better pattern is to build the config and then try to deserialize, 
        // but if keys are missing, it might fail. 
        // Let's set default values explicitly in the builder.
        
        builder = builder
            .set_default("gemini.model", defaults.gemini.model)?
            .set_default("openai.model", defaults.openai.model)?
            .set_default("qwen.model", defaults.qwen.model)?
            .set_default("ollama.model", defaults.ollama.model)?
            .set_default("ollama.base_url", defaults.ollama.base_url)?;

        // 2. Load from config file
        if let Some(proj_dirs) = ProjectDirs::from("com", "gestalt", "gestalt") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("gestalt.toml");
            
            if config_path.exists() {
                builder = builder.add_source(File::from(config_path));
            }
        }

        // 3. Load from Environment Variables
        // GESTALT_GEMINI__MODEL -> gemini.model
        builder = builder.add_source(
            Environment::with_prefix("GESTALT")
                .separator("__")
        );

        builder.build()?.try_deserialize()
    }

    pub fn get_config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "gestalt", "gestalt")
            .map(|d| d.config_dir().join("gestalt.toml"))
    }
}
