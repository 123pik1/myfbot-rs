use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

impl OllamaRequest {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            stream: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    total_duration: usize,
    load_duration: usize,
    prompt_eval_count: usize,
    prompt_eval_duration: usize,
    eval_count: usize,
    eval_duration: usize,
}
