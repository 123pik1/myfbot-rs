use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
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
