use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use std::sync::Arc;
use tracing::{info, error, debug};
use crate::services::llm::Cognition;

#[derive(Clone)]
pub struct TelegramService {
    token: String,
    cognition: Arc<dyn Cognition>,
    allowed_users: Option<Vec<String>>,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "chat with the agent.")]
    Chat(String),
    #[command(description = "check system STATUS.")]
    Status,
}

impl TelegramService {
    pub fn new(token: String, cognition: Arc<dyn Cognition>, allowed_users: Option<Vec<String>>) -> Self {
        Self {
            token,
            cognition,
            allowed_users,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!("🤖 Starting Telegram Bot...");
        let bot = Bot::new(&self.token);
        let service = self.clone();

        let handler = Update::filter_message()
            .filter_command::<Command>()
            .endpoint(move |bot: Bot, msg: Message, cmd: Command| {
                let service = service.clone();
                async move {
                    service.answer(bot, msg, cmd).await
                }
            });

        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;

        Ok(())
    }

    async fn answer(&self, bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        // Access control
        if let Some(allowed) = &self.allowed_users {
            let username = msg.chat.username().unwrap_or("unknown");
            if !allowed.contains(&username.to_string()) {
                bot.send_message(msg.chat.id, "⛔ Access denied.").await?;
                return Ok(());
            }
        }

        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Command::Start => {
                bot.send_message(msg.chat.id, "👋 Hello! I am your Neural-Link Agent. Use /chat to talk to me.").await?;
            }
            Command::Chat(text) => {
                if text.trim().is_empty() {
                    bot.send_message(msg.chat.id, "Please provide a message. Usage: /chat <message>").await?;
                    return Ok(());
                }

                bot.send_message(msg.chat.id, "🤔 Thinking...").await?;

                // Use the Cognition service (which supports subagents)
                match self.cognition.chat("telegram_user", &text).await {
                    Ok(response) => {
                         // Split long messages if needed (Telegram limit is 4096)
                         for chunk in response.content.chars().collect::<Vec<char>>().chunks(4000) {
                             let chunk_str: String = chunk.iter().collect();
                             bot.send_message(msg.chat.id, chunk_str).await?;
                         }
                    }
                    Err(e) => {
                        error!("Error processing chat: {:?}", e);
                        bot.send_message(msg.chat.id, format!("❌ Error: {}", e)).await?;
                    }
                }
            }
            Command::Status => {
                let model = self.cognition.model_id();
                bot.send_message(msg.chat.id, format!("✅ System Online\nUsing Model: {}", model)).await?;
            }
        };

        Ok(())
    }
}
