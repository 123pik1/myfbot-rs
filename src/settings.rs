use std::env;

pub struct Settings {
    pub model_name: String,
    pub model_url: String,
    pub discord_bot_token: String,
}

impl Settings {
    pub fn init() -> Self {
        Self {
            model_name: env::var("OLLAMA_MODEL").unwrap_or("Qwen2.5".to_string()),
            model_url: env::var("OLLAMA_HOST").unwrap_or("http://127.0.0.1:11111".to_string()),
            discord_bot_token: env::var("DISCORD_TOKEN").unwrap(),
        }
    }
}
