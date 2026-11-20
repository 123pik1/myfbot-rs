use std::time::Duration;

use reqwest::Error;
use serde::{Deserialize, Serialize};

// Maybe these consts move later to settings and environment variables

/// System prompt for ollama
const OLLAMA_SYSTEM_PROMPT: &str =
    "Answer in English. Do not be too verbose. Prompts can be in Polish but answer only in English";

/// Ollama timeout set to 15 minutes - maximum answer time for discord bot
const OLLAMA_TIMEOUT: u64 = 900;

#[derive(Debug, Serialize, Deserialize)]
struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<i64>>,
    pub system: String,
    pub stream: bool,
}

impl OllamaRequest {
    pub fn new(model: String, system: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            system,
            context: None,
            stream: false,
        }
    }

    pub fn with_context(model: String, system: String, prompt: String, context: Vec<i64>) -> Self {
        Self {
            model,
            prompt,
            system,
            context: Some(context),
            stream: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub context: Option<Vec<i64>>,
    pub done: bool,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u64>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u64>,
    pub eval_duration: Option<u64>,
}

pub async fn ask_ollama(
    ollama_address: String,
    model: String,
    prompt: String,
    context: Option<Vec<i64>>,
) -> Option<OllamaResponse> {
    let request = if let Some(cont) = context {
        OllamaRequest::with_context(model, OLLAMA_SYSTEM_PROMPT.to_string(), prompt, cont)
    } else {
        OllamaRequest::new(model, OLLAMA_SYSTEM_PROMPT.to_string(), prompt)
    };

    let url = if ollama_address.ends_with("/api/generate") {
        ollama_address
    } else {
        format!("{}/api/generate", ollama_address.trim_end_matches('/'))
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(OLLAMA_TIMEOUT))
        .build()
        .ok()?;

    let response = client.post(url).json(&request).send().await.ok()?;

    response.json::<OllamaResponse>().await.ok()
}
