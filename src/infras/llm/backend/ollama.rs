use serde::Deserialize;
use serde_json::json;
use tokio::runtime::Runtime;

use super::LlmBackend;

pub struct OllamaBackend {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaBackend {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            model: model.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl LlmBackend for OllamaBackend {
    fn generate(&self, prompt: &str) -> Result<String, String> {
        let rt = Runtime::new()
            .map_err(|e| format!("Failed to create async runtime: {}", e))?;
        rt.block_on(self.generate_async(prompt))
    }
}

impl OllamaBackend {
    async fn generate_async(&self, prompt: &str) -> Result<String, String> {
        let url = format!("{}/api/generate", self.base_url);
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
        })
        .to_string();

        let resp_text = self
            .client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| format!("Failed to reach Ollama server: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to read Ollama response: {}", e))?;

        let ollama_resp: OllamaResponse = serde_json::from_str(&resp_text)
            .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

        Ok(ollama_resp.response)
    }
}
